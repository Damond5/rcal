//! Storage traits for event persistence.
//!
//! Defines the core abstractions for storing and retrieving calendar events.

use std::any::Any;
use std::error::Error;
use std::path::{Path, PathBuf};

use chrono::NaiveDate;

use crate::models::CalendarEvent;

/// A type-erased sync provider trait object.
pub type DynSyncProvider = dyn Any + Send + Sync;

/// Trait for loading, saving, and deleting calendar events.
///
/// Implementations can provide various backends (file system, database, etc.)
pub trait EventRepository: Send + Sync {
    /// Loads all events from the repository.
    fn load(&self) -> Result<Vec<CalendarEvent>, Box<dyn Error>>;

    /// Saves an event to the repository.
    /// If an event with the same title AND start_date already exists, it will be replaced.
    fn save(&self, event: &CalendarEvent) -> Result<(), Box<dyn Error>>;

    /// Deletes an event by its title and start_date.
    ///
    /// This is the key for file-based storage since UUIDs are not persisted
    /// and are regenerated on each load.
    fn delete(&self, title: &str, start_date: NaiveDate) -> Result<(), Box<dyn Error>>;

    /// Saves an event with optional sync provider.
    /// The sync_provider should be cast to the appropriate type using `as_any()`.
    #[allow(unused_variables)]
    fn save_with_sync(
        &self,
        event: &CalendarEvent,
        sync_provider: Option<&DynSyncProvider>,
        calendar_dir: &Path,
    ) -> Result<(), Box<dyn Error>> {
        self.save(event)
    }

    /// Deletes an event with optional sync provider.
    #[allow(unused_variables)]
    fn delete_with_sync(
        &self,
        title: &str,
        start_date: NaiveDate,
        sync_provider: Option<&DynSyncProvider>,
        calendar_dir: &Path,
    ) -> Result<(), Box<dyn Error>> {
        self.delete(title, start_date)
    }
}

/// Trait for providing the calendar directory path.
///
/// This allows for different calendar directory configurations
/// (e.g., custom location, multiple calendars).
pub trait CalendarPathProvider: Send + Sync {
    /// Returns the path to the calendar directory.
    fn calendar_dir(&self) -> PathBuf;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Recurrence;
    use chrono::{NaiveDate, NaiveTime};
    use std::sync::Arc;

    // Simple in-memory implementation for testing
    struct InMemoryRepository {
        events: std::sync::Mutex<Vec<CalendarEvent>>,
    }

    impl InMemoryRepository {
        fn new() -> Self {
            Self {
                events: std::sync::Mutex::new(Vec::new()),
            }
        }
    }

    impl EventRepository for InMemoryRepository {
        fn load(&self) -> Result<Vec<CalendarEvent>, Box<dyn Error>> {
            let events = self.events.lock().unwrap().clone();
            Ok(events)
        }

        fn save(&self, event: &CalendarEvent) -> Result<(), Box<dyn Error>> {
            let mut events = self.events.lock().unwrap();
            // Find existing event by id
            if let Some(pos) = events.iter().position(|e| e.id == event.id) {
                events[pos] = event.clone();
            } else {
                events.push(event.clone());
            }
            Ok(())
        }

        fn delete(&self, title: &str, start_date: NaiveDate) -> Result<(), Box<dyn Error>> {
            let mut events = self.events.lock().unwrap();
            events.retain(|e| !(e.title == title && e.start_date == start_date));
            Ok(())
        }
    }

    #[test]
    fn test_event_repository_save_and_load() {
        let repo = Arc::new(InMemoryRepository::new());

        let event = CalendarEvent {
            id: "test-id".to_string(),
            title: "Test Event".to_string(),
            description: "Description".to_string(),
            recurrence: Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };

        repo.save(&event).unwrap();

        let events = repo.load().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].title, "Test Event");
    }

    #[test]
    fn test_event_repository_update() {
        let repo = Arc::new(InMemoryRepository::new());

        let mut event = CalendarEvent {
            id: "test-id".to_string(),
            title: "Test Event".to_string(),
            description: "Description".to_string(),
            recurrence: Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };

        repo.save(&event).unwrap();

        event.title = "Updated Title".to_string();
        repo.save(&event).unwrap();

        let events = repo.load().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].title, "Updated Title");
    }

    #[test]
    fn test_event_repository_delete() {
        let repo = Arc::new(InMemoryRepository::new());

        let event = CalendarEvent {
            id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
            title: "Test Event".to_string(),
            description: "Description".to_string(),
            recurrence: Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };

        repo.save(&event).unwrap();
        repo.delete("Test Event", event.start_date).unwrap();

        let events = repo.load().unwrap();
        assert!(events.is_empty());
    }

    // Simple path provider for testing
    struct TestPathProvider {
        path: PathBuf,
    }

    impl CalendarPathProvider for TestPathProvider {
        fn calendar_dir(&self) -> PathBuf {
            self.path.clone()
        }
    }

    #[test]
    fn test_calendar_path_provider() {
        let provider = TestPathProvider {
            path: PathBuf::from("/tmp/test_calendar"),
        };

        assert_eq!(provider.calendar_dir(), PathBuf::from("/tmp/test_calendar"));
    }
}
