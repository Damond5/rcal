# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Recurring event support (daily, weekly, monthly, yearly) with automatic instance generation
- Automatic cleanup of finished events older than 2 months on launch (configurable via `auto_cleanup_old_events`)
- Lazy loading for recurring event instances, enabling indefinite display of long-term recurring events
- Recurrence dropdown for event creation/editing implemented as a popup overlay instead of inline to prevent invalid syntax and improve usability

### Changed
- **Breaking Change**: Changed recurring event deletion behavior - deleting any recurring instance now deletes the entire series persistently. Previously, only the specific instance was removed from memory.

## [1.3.1] - 2025-11-09

### Fixed
- Prevent duplicate notifications when file changes occur without altering events.

## [1.3.0] - 2025-11-08

### Fixed
- Removed duplicate 'Up to date' message in sync popup

## [1.2.0] - 2025-11-08

### Added
- Three-month calendar display

### Changed
- Improved navigation with full view shifting
- Migrated to OpenSpec for project specifications and change management
- Optimized AGENTS.md and OpenSpec configuration
- Sync popup now automatically displays current status on entry without requiring manual check

### Removed
- Removed H/L and PageUp/PageDown navigation keys for three-month paging, simplifying navigation to rely solely on h/j/k/l with automatic view shifting.
- General agent instructions to prevent automatic OpenSpec proposal implementation
- Removed obsolete "s: Check status" instruction from sync popup UI since status is now automatically displayed on entry

### Fixed
- Fixed bug where "Launch sync completed" message would incorrectly appear in sync popup, interfering with current sync operations

## [1.1.0] - 2025-11-01

### Added
- All-day event support with notifications the day before at midday
- Asynchronous sync functionality for background operations
- Added automatic pull on launch when sync is configured, performed asynchronously in the background
- One file per event storage system
- Multi-day event support
- Event format specification documentation

### Changed
- Event files are now named by event title instead of date-based naming
- Removed ID field from CalendarEvent struct, using title as unique identifier for file operations
