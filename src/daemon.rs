use std::{collections::HashSet, error::Error, thread, time::Duration};

use chrono::Local;
use dirs;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use notify_rust::Notification;

use crate::persistence;

    pub fn run_daemon() -> Result<(), Box<dyn Error>> {
        let home = dirs::home_dir().expect("Could not find home directory");
        let calendar_dir = home.join("calendar");

        let mut events = persistence::load_events();
        let mut notified = HashSet::new();

        let (tx, rx) = std::sync::mpsc::channel();

        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
        watcher.watch(&calendar_dir, RecursiveMode::Recursive)?;

        loop {
            // Check for upcoming events
            let now = Local::now();

        for event in &events {
            let event_datetime = event.date.and_time(event.time);
            let diff = event_datetime.signed_duration_since(now.naive_local());
            if diff.num_minutes() >= 30 {
                let key = (event.date, event.time, event.title.clone());
                if !notified.contains(&key) {
                    Notification::new()
                        .summary("Upcoming Event")
                        .body(&format!(
                            "{} at {}",
                            event.title,
                            event.time.format("%H:%M")
                        ))
                        .show()?;
                    notified.insert(key);
                }
            }
        }

        // Check for file changes
        match rx.try_recv() {
            Ok(_) => {
                events = persistence::load_events();
                // Reset notified to allow re-notifying if events change
                notified.clear();
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(e) => eprintln!("Watch error: {e:?}"),
        }

        thread::sleep(Duration::from_secs(60));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::CalendarEvent;
    use chrono::{Duration, NaiveDate, NaiveTime};
    use std::collections::HashSet;

    // Mock function to check upcoming events
    fn check_upcoming_events(
        events: &[CalendarEvent],
        now: chrono::DateTime<Local>,
        notified: &mut HashSet<(NaiveDate, NaiveTime, String)>,
    ) -> Vec<String> {
        let mut notifications = Vec::new();

        for event in events {
            let event_datetime = event.date.and_time(event.time);
            let diff = event_datetime.signed_duration_since(now.naive_local());
            if diff.num_minutes() >= 30 {
                let key = (event.date, event.time, event.title.clone());
                if !notified.contains(&key) {
                    notifications.push(format!(
                        "{} at {}",
                        event.title,
                        event.time.format("%H:%M")
                    ));
                    notified.insert(key);
                }
            }
        }
        notifications
    }

    #[test]
    fn test_check_upcoming_events() {
        let now = Local::now();
        let today = now.date_naive();
        let future_time = now.time() + Duration::minutes(30);

        let events = vec![CalendarEvent {
            date: today,
            time: future_time,
            title: "Test Event".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: today,
            end_date: None,
            start_time: future_time,
            end_time: None,
            id: String::new(),
        }];

        let mut notified = HashSet::new();
        let notifications = check_upcoming_events(&events, now, &mut notified);

        assert_eq!(notifications.len(), 1);
        assert_eq!(
            notifications[0],
            format!("Test Event at {}", future_time.format("%H:%M"))
        );
    }

    #[test]
    fn test_no_notification_for_past_event() {
        let now = Local::now();
        let today = now.date_naive();
        let past_time = now.time() - Duration::minutes(15);

        let events = vec![CalendarEvent {
            date: today,
            time: past_time,
            title: "Test Event".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: today,
            end_date: None,
            start_time: past_time,
            end_time: None,
            id: String::new(),
        }];

        let mut notified = HashSet::new();
        let notifications = check_upcoming_events(&events, now, &mut notified);

        assert_eq!(notifications.len(), 0);
    }

    #[test]
    fn test_no_notification_for_15_minutes_ahead() {
        let now = Local::now();
        let today = now.date_naive();
        let future_time = now.time() + Duration::minutes(15);

        let events = vec![CalendarEvent {
            id: String::new(),
            date: today,
            time: future_time,
            title: "Test Event".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: today,
            end_date: None,
            start_time: future_time,
            end_time: None,
        }];

        let mut notified = HashSet::new();
        let notifications = check_upcoming_events(&events, now, &mut notified);

        assert_eq!(notifications.len(), 0);
    }

    #[test]
    fn test_no_duplicate_notifications() {
        let now = Local::now();
        let today = now.date_naive();
        let future_time = now.time() + Duration::minutes(30);

        let events = vec![CalendarEvent {
            id: String::new(),
            date: today,
            time: future_time,
            title: "Test Event".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: today,
            end_date: None,
            start_time: future_time,
            end_time: None,
        }];

        let mut notified = HashSet::new();
        let notifications1 = check_upcoming_events(&events, now, &mut notified);
        let notifications2 = check_upcoming_events(&events, now, &mut notified);

        assert_eq!(notifications1.len(), 1);
        assert_eq!(notifications2.len(), 0);
    }
}
