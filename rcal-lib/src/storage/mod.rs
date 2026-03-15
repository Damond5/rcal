//! Storage module - persistence layer for calendar events.
//!
//! This module provides traits and implementations for storing and retrieving
//! calendar events from various backends.

pub mod file_storage;
pub mod traits;

pub use file_storage::{DefaultPathProvider, FileEventRepository};
pub use traits::{CalendarPathProvider, EventRepository};
