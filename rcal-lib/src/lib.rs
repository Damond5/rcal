//! rcal-lib - Calendar library for Rust.
//!
//! A clean architecture library for calendar event management,
//! providing storage, synchronization, and notification capabilities.
//!
//! ## Features
//!
//! - **Event Management**: Create, update, delete, and query calendar events
//! - **Recurring Events**: Support for daily, weekly, monthly, and yearly recurrence
//! - **File Storage**: Markdown-based event storage in the file system
//! - **Git Sync**: Optional Git-based synchronization for calendar sharing
//! - **Desktop Notifications**: Optional desktop notification support (Linux with D-Bus)
//!
//! ## Usage
//!
//! ```rust
//! use rcal_lib::{CalendarEvent, EventRepository, EventService, FileEventRepository, DefaultPathProvider};
//! use chrono::{NaiveDate, NaiveTime};
//!
//! // Create an event
//! let event = CalendarEvent::new(
//!     "Meeting".to_string(),
//!     "Team standup".to_string(),
//!     NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
//!     NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
//! );
//!
//! // Use the event service
//! let mut service = EventService::new();
//! service.add_event(event);
//!
//! // Load events from file storage
//! let repo = FileEventRepository::with_default_path().unwrap();
//! let events = repo.load().unwrap();
//! ```

// Re-export models
pub mod models;
pub use models::{CalendarEvent, Recurrence, SyncStatus};

// Re-export core
pub mod core;
pub use core::{
    date_utils, get_date_suggestions, validate_date_input, validate_time_input, EventService,
};

// Re-export storage
pub mod storage;
pub use storage::{
    CalendarPathProvider, DefaultPathProvider, EventRepository, FileEventRepository,
};

// Re-export sync
pub mod sync;
pub use sync::{GitSyncProvider, SyncProvider};

// Re-export notifications
pub mod notifications;
pub use notifications::{DefaultNotifier, NotificationDaemon, Notifier};

// Re-export validation
pub mod validation;
pub use validation::{
    sanitize_title_for_filename, validate_event, validate_event_with_details, validate_filename,
    ValidationError,
};
