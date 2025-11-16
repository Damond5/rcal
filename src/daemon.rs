use std::{collections::HashSet, error::Error, thread, time::Duration};

use chrono::Local;
use dirs;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use notify_rust::Notification;

use crate::persistence;

pub fn run_daemon() -> Result<(), Box<dyn Error>> {
    let home = dirs::home_dir().expect("Could not find home directory");
    let calendar_dir = home.join("calendar");

    let mut events = persistence::load_events().unwrap_or_else(|e| {
        eprintln!("Failed to load initial events: {e}");
        Vec::new()
    });
    let mut notified = HashSet::new();

    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(&calendar_dir, RecursiveMode::Recursive)?;

    loop {
        // Check for upcoming events
        let now = Local::now();

        // Handle all-day events
        for event in &events {
            if event.is_all_day {
                let should_notify = {
                    // Notify all-day events the day before at midday
                    let tomorrow = now.date_naive() + chrono::Duration::days(1);
                    event.date == tomorrow
                        && now.time() < chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap()
                };
                if should_notify {
                    let key = (event.date, event.time, event.title.clone());
                    if !notified.contains(&key) {
                        let body = format!("{} (all day)", event.title);
                        if let Err(e) = Notification::new()
                            .summary("Upcoming Event")
                            .body(&body)
                            .show()
                        {
                            eprintln!("Notification failed: {e}");
                        }
                        notified.insert(key);
                    }
                }
            }
        }

        // Handle timed events: notify all at the next upcoming time slot within 30 minutes
        let mut next_timed_events = Vec::new();
        let mut min_diff = i64::MAX;
        for event in &events {
            if !event.is_all_day {
                let event_datetime = event.date.and_time(event.time);
                let diff = event_datetime.signed_duration_since(now.naive_local());
                if diff.num_minutes() <= 30 && diff.num_minutes() > 0 {
                    if diff.num_minutes() < min_diff {
                        min_diff = diff.num_minutes();
                        next_timed_events.clear();
                        next_timed_events.push(event);
                    } else if diff.num_minutes() == min_diff {
                        next_timed_events.push(event);
                    }
                }
            }
        }
        for event in next_timed_events {
            let key = (event.date, event.time, event.title.clone());
            if !notified.contains(&key) {
                let body = format!("{} at {}", event.title, event.time.format("%H:%M"));
                if let Err(e) = Notification::new()
                    .summary("Upcoming Event")
                    .body(&body)
                    .show()
                {
                    eprintln!("Notification failed: {e}");
                }
                notified.insert(key);
            }
        }

        // Check for file changes
        match rx.try_recv() {
            Ok(_) => {
                match persistence::load_events() {
                    Ok(mut new_events) => {
                        // Sort both for order-independent comparison
                        new_events.sort_by(|a, b| a.date.cmp(&b.date).then(a.time.cmp(&b.time)));
                        let mut current_sorted = events.clone();
                        current_sorted
                            .sort_by(|a, b| a.date.cmp(&b.date).then(a.time.cmp(&b.time)));
                        if new_events != current_sorted {
                            events = new_events;
                            // Reset notified to allow re-notifying if events change
                            notified.clear();
                        } else {
                            events = new_events;
                        }
                    }
                    Err(e) => eprintln!("Failed to load events on file change: {e}"),
                }
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
    use chrono::{Duration, NaiveDate, NaiveTime, TimeZone};
    use std::collections::HashSet;

    // Mock function to check upcoming events
    fn check_upcoming_events(
        events: &[CalendarEvent],
        now: chrono::DateTime<Local>,
        notified: &mut HashSet<(NaiveDate, NaiveTime, String)>,
    ) -> Vec<String> {
        let mut notifications = Vec::new();

        // Handle all-day events
        for event in events {
            if event.is_all_day {
                let should_notify = {
                    // Notify all-day events the day before at midday
                    let tomorrow = now.date_naive() + chrono::Duration::days(1);
                    event.date == tomorrow
                        && now.time() < chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap()
                };
                if should_notify {
                    let key = (event.date, event.time, event.title.clone());
                    if !notified.contains(&key) {
                        let body = format!("{} (all day)", event.title);
                        notifications.push(body);
                        notified.insert(key);
                    }
                }
            }
        }

        // Handle timed events: notify all at the next upcoming time slot within 30 minutes
        let mut next_timed_events = Vec::new();
        let mut min_diff = i64::MAX;
        for event in events {
            if !event.is_all_day {
                let event_datetime = event.date.and_time(event.time);
                let diff = event_datetime.signed_duration_since(now.naive_local());
                if diff.num_minutes() <= 30 && diff.num_minutes() > 0 {
                    if diff.num_minutes() < min_diff {
                        min_diff = diff.num_minutes();
                        next_timed_events.clear();
                        next_timed_events.push(event);
                    } else if diff.num_minutes() == min_diff {
                        next_timed_events.push(event);
                    }
                }
            }
        }
        for event in next_timed_events {
            let key = (event.date, event.time, event.title.clone());
            if !notified.contains(&key) {
                let body = format!("{} at {}", event.title, event.time.format("%H:%M"));
                notifications.push(body);
                notified.insert(key);
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
            id: uuid::Uuid::new_v4().to_string(),
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
            is_all_day: false,
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
            id: uuid::Uuid::new_v4().to_string(),
            is_all_day: false,
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
        }];

        let mut notified = HashSet::new();
        let notifications = check_upcoming_events(&events, now, &mut notified);

        assert_eq!(notifications.len(), 0);
    }

    #[test]
    fn test_notification_for_15_minutes_ahead() {
        let now = Local::now();
        let today = now.date_naive();
        let future_time = now.time() + Duration::minutes(15);

        let events = vec![CalendarEvent {
            id: uuid::Uuid::new_v4().to_string(),
            is_all_day: false,
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

        assert_eq!(notifications.len(), 1);
        assert_eq!(
            notifications[0],
            format!("Test Event at {}", future_time.format("%H:%M"))
        );
    }

    #[test]
    fn test_all_day_event_notification() {
        let now = Local::now();
        let today = now.date_naive();
        let tomorrow = today + chrono::Duration::days(1);
        // Set time to before midday
        let before_midday = chrono::NaiveTime::from_hms_opt(11, 0, 0).unwrap();
        let now_before_midday = today.and_time(before_midday);

        let events = vec![CalendarEvent {
            id: uuid::Uuid::new_v4().to_string(),
            date: tomorrow,
            time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            title: "All Day Event".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: tomorrow,
            end_date: None,
            start_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            end_time: None,
            is_all_day: true,
        }];

        let mut notified = HashSet::new();
        let now_dt = Local.from_local_datetime(&now_before_midday).unwrap();
        let notifications = check_upcoming_events(&events, now_dt, &mut notified);

        assert_eq!(notifications.len(), 1);
        assert_eq!(notifications[0], "All Day Event (all day)");
    }

    #[test]
    fn test_no_duplicate_notifications() {
        let now = Local::now();
        let today = now.date_naive();
        let future_time = now.time() + Duration::minutes(30);

        let events = vec![CalendarEvent {
            id: uuid::Uuid::new_v4().to_string(),
            is_all_day: false,
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

    #[test]
    fn test_multiple_close_events() {
        let now = Local::now();
        let today = now.date_naive();
        let time1 = now.time() + Duration::minutes(15);
        let time2 = now.time() + Duration::minutes(20);
        let time3 = now.time() + Duration::minutes(25);

        let events = vec![
            CalendarEvent {
                id: uuid::Uuid::new_v4().to_string(),
                is_all_day: false,
                date: today,
                time: time1,
                title: "Event 1".to_string(),
                description: String::new(),
                recurrence: crate::app::Recurrence::None,
                is_recurring_instance: false,
                base_date: None,
                start_date: today,
                end_date: None,
                start_time: time1,
                end_time: None,
            },
            CalendarEvent {
                id: uuid::Uuid::new_v4().to_string(),
                is_all_day: false,
                date: today,
                time: time2,
                title: "Event 2".to_string(),
                description: String::new(),
                recurrence: crate::app::Recurrence::None,
                is_recurring_instance: false,
                base_date: None,
                start_date: today,
                end_date: None,
                start_time: time2,
                end_time: None,
            },
            CalendarEvent {
                id: uuid::Uuid::new_v4().to_string(),
                is_all_day: false,
                date: today,
                time: time3,
                title: "Event 3".to_string(),
                description: String::new(),
                recurrence: crate::app::Recurrence::None,
                is_recurring_instance: false,
                base_date: None,
                start_date: today,
                end_date: None,
                start_time: time3,
                end_time: None,
            },
        ];

        let mut notified = HashSet::new();
        let notifications = check_upcoming_events(&events, now, &mut notified);

        assert_eq!(notifications.len(), 1);
        assert_eq!(
            notifications[0],
            format!("Event 1 at {}", time1.format("%H:%M"))
        );
    }

    #[test]
    fn test_no_notification_for_31_minutes_ahead() {
        let now = Local::now();
        let today = now.date_naive();
        let future_time = now.time() + Duration::minutes(31);

        let events = vec![CalendarEvent {
            id: uuid::Uuid::new_v4().to_string(),
            is_all_day: false,
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
    fn test_multiple_events_same_time() {
        let now = Local::now();
        let today = now.date_naive();
        let same_time = now.time() + Duration::minutes(15);

        let events = vec![
            CalendarEvent {
                id: uuid::Uuid::new_v4().to_string(),
                is_all_day: false,
                date: today,
                time: same_time,
                title: "Event 1".to_string(),
                description: String::new(),
                recurrence: crate::app::Recurrence::None,
                is_recurring_instance: false,
                base_date: None,
                start_date: today,
                end_date: None,
                start_time: same_time,
                end_time: None,
            },
            CalendarEvent {
                id: uuid::Uuid::new_v4().to_string(),
                is_all_day: false,
                date: today,
                time: same_time,
                title: "Event 2".to_string(),
                description: String::new(),
                recurrence: crate::app::Recurrence::None,
                is_recurring_instance: false,
                base_date: None,
                start_date: today,
                end_date: None,
                start_time: same_time,
                end_time: None,
            },
        ];

        let mut notified = HashSet::new();
        let notifications = check_upcoming_events(&events, now, &mut notified);

        assert_eq!(notifications.len(), 2);
        assert!(notifications.contains(&format!("Event 1 at {}", same_time.format("%H:%M"))));
        assert!(notifications.contains(&format!("Event 2 at {}", same_time.format("%H:%M"))));
    }

    #[test]
    fn test_event_comparison_for_reload() {
        let now = Local::now();
        let today = now.date_naive();
        let future_time = now.time() + Duration::minutes(30);

        let events = vec![CalendarEvent {
            id: uuid::Uuid::new_v4().to_string(),
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
            is_all_day: false,
        }];

        // Simulate initial notification
        let mut notified = HashSet::new();
        let _ = check_upcoming_events(&events, now, &mut notified);
        assert_eq!(notified.len(), 1);

        // Simulate reload with same events (different order)
        let mut new_events = events.clone();
        new_events.reverse(); // Change order
        let mut new_sorted = new_events.clone();
        new_sorted.sort_by(|a, b| a.date.cmp(&b.date).then(a.time.cmp(&b.time)));
        let mut current_sorted = events.clone();
        current_sorted.sort_by(|a, b| a.date.cmp(&b.date).then(a.time.cmp(&b.time)));

        // Should be equal after sorting
        assert_eq!(new_sorted, current_sorted);

        // If events are the same, notified should not be cleared
        if new_sorted == current_sorted {
            // In real code, events = new_events; but notified not cleared
            assert_eq!(notified.len(), 1); // Still notified
        }
    }
}
