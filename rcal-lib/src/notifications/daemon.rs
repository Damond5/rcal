//! Notification daemon for desktop notifications.
//!
//! Provides background notification checking and delivery.

#[allow(unused_imports)]
use std::collections::HashSet;

use chrono::{Local, NaiveDate, NaiveTime};

use crate::models::CalendarEvent;
use crate::notifications::Notifier;

/// Notification daemon that monitors events and sends notifications.
pub struct NotificationDaemon {
    notifier: Box<dyn Notifier>,
    notified: HashSet<(NaiveDate, NaiveTime, String)>,
    events: Vec<CalendarEvent>,
}

impl NotificationDaemon {
    /// Creates a new NotificationDaemon with the given notifier.
    pub fn new(notifier: Box<dyn Notifier>) -> Self {
        Self {
            notifier,
            notified: HashSet::new(),
            events: Vec::new(),
        }
    }

    /// Sets the events to monitor for notifications.
    pub fn set_events(&mut self, events: Vec<CalendarEvent>) {
        self.events = events;
    }

    /// Checks for upcoming events and sends notifications.
    /// Call this periodically (e.g., every minute).
    pub fn check_and_notify(&mut self) {
        let now = Local::now();

        // Handle all-day events
        for event in &self.events {
            if event.is_all_day {
                let should_notify = {
                    // Notify all-day events the day before at midday
                    let tomorrow = now.date_naive() + chrono::Duration::days(1);
                    event.start_date == tomorrow
                        && now.time() < chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap()
                };
                if should_notify {
                    let key = (event.start_date, event.start_time, event.title.clone());
                    if !self.notified.contains(&key) {
                        let body = format!("{} (all day)", event.title);
                        if let Err(e) = self.notifier.notify("Upcoming Event", &body) {
                            eprintln!("Notification failed: {e}");
                        }
                        self.notified.insert(key);
                    }
                }
            }
        }

        // Handle timed events: notify all at the next upcoming time slot within 30 minutes
        let mut next_timed_events = Vec::new();
        let mut min_diff = i64::MAX;
        for event in &self.events {
            if !event.is_all_day {
                let event_datetime = event.start_date.and_time(event.start_time);
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
            let key = (event.start_date, event.start_time, event.title.clone());
            if !self.notified.contains(&key) {
                let body = format!("{} at {}", event.title, event.start_time.format("%H:%M"));
                if let Err(e) = self.notifier.notify("Upcoming Event", &body) {
                    eprintln!("Notification failed: {e}");
                }
                self.notified.insert(key);
            }
        }
    }

    /// Clears the notification history.
    pub fn clear_notifications(&mut self) {
        self.notified.clear();
    }

    /// Returns the number of events being monitored.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
}

/// Runs the notification daemon in a background thread.
///
/// This function blocks and should be run in its own thread.
/// It will periodically check for upcoming events and send notifications.
#[cfg(feature = "desktop-notifications")]
pub fn run_daemon<R: crate::storage::EventRepository>(
    repository: &R,
    notifier: Box<dyn Notifier>,
) -> Result<(), Box<dyn Error>> {
    use crate::sync::SyncProvider;

    let mut daemon = NotificationDaemon::new(notifier);

    // Load initial events
    daemon.set_events(repository.load().unwrap_or_else(|e| {
        eprintln!("Failed to load initial events: {e}");
        Vec::new()
    }));

    loop {
        daemon.check_and_notify();

        // Reload events
        daemon.set_events(repository.load().unwrap_or_else(|e| {
            eprintln!("Failed to load events: {e}");
            Vec::new()
        }));

        thread::sleep(Duration::from_secs(60));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, NaiveDate, NaiveTime};

    struct TestNotifier {
        notifications: std::sync::Mutex<Vec<(String, String)>>,
    }

    impl TestNotifier {
        fn new() -> Self {
            Self {
                notifications: std::sync::Mutex::new(Vec::new()),
            }
        }

        fn get_notifications(&self) -> Vec<(String, String)> {
            self.notifications.lock().unwrap().clone()
        }
    }

    impl Notifier for TestNotifier {
        fn notify(&self, title: &str, body: &str) -> Result<(), Box<dyn Error>> {
            self.notifications
                .lock()
                .unwrap()
                .push((title.to_string(), body.to_string()));
            Ok(())
        }
    }

    #[test]
    fn test_notification_daemon_new() {
        let notifier = Box::new(TestNotifier::new());
        let daemon = NotificationDaemon::new(notifier);
        assert_eq!(daemon.event_count(), 0);
    }

    #[test]
    fn test_notification_daemon_set_events() {
        let notifier = Box::new(TestNotifier::new());
        let mut daemon = NotificationDaemon::new(notifier);

        let event = CalendarEvent {
            id: "test".to_string(),
            title: "Test Event".to_string(),
            description: "".to_string(),
            recurrence: crate::models::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: Local::now().date_naive(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };

        daemon.set_events(vec![event]);
        assert_eq!(daemon.event_count(), 1);
    }

    #[test]
    fn test_check_and_notify_no_events() {
        let notifier = Box::new(TestNotifier::new());
        let mut daemon = NotificationDaemon::new(notifier);
        daemon.check_and_notify();

        // No events, so no notifications should be sent
        // The test passes if no panic occurs
    }

    #[test]
    fn test_clear_notifications() {
        let notifier = Box::new(TestNotifier::new());
        let mut daemon = NotificationDaemon::new(notifier);

        daemon.notified.insert((
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            "Test".to_string(),
        ));

        daemon.clear_notifications();
        assert!(daemon.notified.is_empty());
    }

    #[test]
    fn test_notification_sent() {
        let test_notifier = Box::new(TestNotifier::new());
        let test_notifier_clone = Box::new(TestNotifier::new());

        let mut daemon = NotificationDaemon::new(test_notifier_clone);

        let now = Local::now();
        let future_time = now.time() + Duration::minutes(15);

        let event = CalendarEvent {
            id: "test".to_string(),
            title: "Test Event".to_string(),
            description: "".to_string(),
            recurrence: crate::models::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: now.date_naive(),
            end_date: None,
            start_time: future_time,
            end_time: None,
            is_all_day: false,
        };

        daemon.set_events(vec![event]);
        daemon.check_and_notify();

        let notifications = test_notifier.get_notifications();
        assert!(!notifications.is_empty());
    }
}
