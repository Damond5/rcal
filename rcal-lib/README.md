# rcal-lib

A reusable Rust library for calendar event management, extracted from the rcal terminal application. This library provides the core business logic for storing, retrieving, and managing calendar events with support for recurring events, file-based persistence, Git synchronization, and optional desktop notifications.

[![Crates.io](https://img.shields.io/crates/v/rcal-lib)](https://crates.io/crates/rcal-lib)
[![License: CC0-1.0](https://img.shields.io/badge/License-CC0%201.0-blue)](LICENSE)

## Overview

rcal-lib is designed to be the foundation for building calendar applications. It handles all the complexity of event management so you can focus on building your UI—whether that's a terminal interface, web application, mobile app, or desktop GUI.

Key benefits:

- **Pure Business Logic**: Core event management without UI dependencies
- **Flexible Storage**: File-based persistence with customizable path providers
- **Recurring Events**: Full support for daily, weekly, monthly, and yearly recurrence
- **Optional Sync**: Git-based synchronization for cross-device sharing
- **Desktop Notifications**: Optional D-Bus notifications on Linux (feature-gated)

## Features

### Core Event Management

- Create, update, delete, and query calendar events
- Support for single and multi-day events
- All-day events (events without a specific time)
- Event validation with detailed error messages

### Recurring Events

- Daily, weekly, monthly, and yearly recurrence patterns
- Lazy instance generation for efficient memory usage
- Automatic February 29th fallback to February 28th in non-leap years
- Session-level caching for performance

### Storage

- Markdown-based file storage (one file per event)
- Customizable storage paths via the `CalendarPathProvider` trait
- Automatic cleanup of old events (configurable)
- Sanitized filenames for cross-platform compatibility

### Synchronization (Optional)

- Git-based synchronization provider included
- Pluggable `SyncProvider` trait for custom backends
- Status tracking: UpToDate, Ahead, Behind, Conflicts, Error

### Notifications (Linux only, optional)

- Desktop notification support via D-Bus
- Feature-gated implementation for minimal dependencies

## Installation

Add rcal-lib to your `Cargo.toml`:

```toml
[dependencies]
rcal-lib = "0.1"
```

### Feature Flags

Configure optional features:

```toml
[dependencies]
rcal-lib = { version = "0.1", features = ["desktop-notifications"] }
```

| Feature | Description | Default |
|---------|-------------|---------|
| `desktop-notifications` | Enable desktop notification support (Linux with D-Bus) | Disabled |

## Usage Example

Here's a basic example showing how to use rcal-lib in your project:

```rust
use rcal_lib::{
    CalendarEvent, EventService, FileEventRepository, DefaultPathProvider,
    GitSyncProvider, Recurrence,
};
use chrono::{NaiveDate, NaiveTime};

// Create a new calendar event
let mut event = CalendarEvent::new(
    "Team Meeting".to_string(),
    "Weekly standup".to_string(),
    NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
    NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
);

// Make it recurring weekly
event.recurrence = Recurrence::Weekly;

// Use the event service for business logic
let mut service = EventService::new();
service.add_event(event);

// Load events from file storage
let repo = FileEventRepository::with_default_path().unwrap();
let events = repo.load().unwrap();

println!("Loaded {} events", events.len());

// Query events for a date range
let range_start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
let range_end = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
let all_events = service.get_all_events_for_range(range_start, range_end);

for evt in &all_events {
    if evt.is_recurring_instance {
        println!("  {} (instance)", evt.title);
    } else {
        println!("  {}", evt.title);
    }
}
```

## Architecture

The library is organized into clear module layers:

```
rcal-lib/
├── src/
│   ├── lib.rs              # Public API exports
│   ├── models/             # Domain types
│   │   ├── calendar_event.rs
│   │   └── sync_status.rs
│   ├── core/               # Business logic
│   │   ├── event_service.rs
│   │   └── date_utils.rs
│   ├── storage/            # Persistence
│   │   ├── traits.rs       # EventRepository, CalendarPathProvider traits
│   │   └── file_storage.rs
│   ├── sync/               # Synchronization
│   │   ├── traits.rs       # SyncProvider trait
│   │   └── git_sync.rs     # Git implementation
│   └── notifications/      # Desktop notifications
│       ├── mod.rs          # Notifier trait
│       └── daemon.rs       # Notification daemon
```

### Domain Models (`models/`)

The core data types:

- `CalendarEvent`: Represents an event with title, description, dates, times, and recurrence
- `Recurrence`: Enum for recurrence patterns (None, Daily, Weekly, Monthly, Yearly)
- `SyncStatus`: Enum for sync state (Idle, Syncing, UpToDate, Ahead, Behind, Conflicts, Error)

### Business Logic (`core/`)

- `EventService`: Handles event CRUD operations, validation, and recurring instance generation
- `date_utils`: Date parsing, validation, and formatting utilities

### Storage Abstraction (`storage/`)

Traits for pluggable storage backends:

- `EventRepository`: Load, save, delete events
- `CalendarPathProvider`: Provide calendar directory paths

The `FileEventRepository` provides a Markdown-based implementation.

### Sync Abstraction (`sync/`)

- `SyncProvider` trait: Implement custom sync backends
- `GitSyncProvider`: Git-based implementation for file synchronization

### Notifications (`notifications/`)

- `Notifier` trait: Send desktop notifications
- `NotificationDaemon`: Background service for scheduled notifications

## Storage Format

Events are stored as Markdown files in the calendar directory:

```markdown
# Event: Team Meeting

- **Date**: 2024-01-15
- **Time**: 10:00 to 11:00
- **Description**: Weekly standup
- **Recurrence**: weekly
```

For multi-day events:

```markdown
# Event: Conference

- **Date**: 2024-03-15 to 2024-03-17
- **Time**: all-day
- **Description**: Annual tech conference
- **Recurrence**: yearly
```

## Custom Storage Implementation

You can implement custom storage by providing the `EventRepository` trait:

```rust
use rcal_lib::storage::{CalendarPathProvider, EventRepository};
use rcal_lib::models::CalendarEvent;
use std::path::PathBuf;
use std::error::Error;

struct MyDatabase {
    path: PathBuf,
}

impl CalendarPathProvider for MyDatabase {
    fn calendar_dir(&self) -> PathBuf {
        self.path.clone()
    }
}

impl EventRepository for MyDatabase {
    fn load(&self) -> Result<Vec<CalendarEvent>, Box<dyn Error>> {
        // Load from your database
    }

    fn save(&self, event: &CalendarEvent) -> Result<(), Box<dyn Error>> {
        // Save to your database
    }

    fn delete(&self, id: &uuid::Uuid) -> Result<(), Box<dyn Error>> {
        // Delete from your database
    }
}
```

## Custom Sync Implementation

Implement the `SyncProvider` trait for custom synchronization:

```rust
use rcal_lib::sync::SyncProvider;
use rcal_lib::models::SyncStatus;
use std::path::Path;
use std::error::Error;

struct MyCloudSync {
    api_key: String,
}

impl SyncProvider for MyCloudSync {
    fn init(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        // Initialize sync
    }

    fn pull(&self, path: &Path) -> Result<SyncStatus, Box<dyn Error>> {
        // Pull from cloud
    }

    fn push(&self, path: &Path) -> Result<SyncStatus, Box<dyn Error>> {
        // Push to cloud
    }

    fn status(&self, path: &Path) -> Result<SyncStatus, Box<dyn Error>> {
        // Check sync status
    }
}
```

## Platform Requirements

- **Rust**: 1.70 or later
- **Linux (notifications)**: D-Bus session bus for desktop notifications

## Use Cases

rcal-lib is ideal for:

- Building terminal calendar applications (like the rcal CLI)
- Creating GUI calendar applications
- Developing mobile calendar apps (with Rust mobile bindings)
- Implementing calendar-backed services
- Adding calendar features to existing applications

## Related Projects

- [rcal](https://github.com/Damond5/rcal): Terminal calendar application using this library
- [icalendar](https://crates.io/crates/icalendar): iCalendar format support for interoperability
- [notify-rust](https://crates.io/crates/notify-rust): Desktop notifications on Linux

## License

CC0 1.0 Universal - see LICENSE file for details.
