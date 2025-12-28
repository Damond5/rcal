# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Enhanced end date autocomplete suggestions: Expanded to include more relative dates ("next monday", "1 day", "next month", "end of year", "same day"), improved fuzzy matching for typos and partial inputs, added descriptive suggestions with arrow key navigation, enhanced partial date completion for inputs like "15/" or "/10", and optimized performance with edge case handling.

### Changed
- Restructured README Installation section into two targeted subsections to improve accessibility: "For humans" provides quick installation with 1-3 commands per method, while "For LLM Agents" includes comprehensive guidance with prerequisites, configuration details, verification steps using exit codes, and troubleshooting for installation, service, and sync issues in collapsible details format. This follows oh-my-opencode documentation standards to reduce time-to-first-run for human users while maintaining detailed automation documentation. All installation methods (AUR, source build, cargo install) are preserved with enhanced clarity.

### Fixed
- February 29th leap year fallback for yearly recurring events: Yearly events on February 29th now automatically fall back to February 28th in non-leap years, ensuring birthdays, anniversaries, and other annual events continue occurring annually without being skipped.

## [1.4.0] - 2025-11-16

### Added
- Prevent invalid date acceptance: Added validation on event submission to prevent creation of events with invalid dates, displaying error messages for invalid time input and invalid end date input instead of proceeding silently.
- Recurring event support (daily, weekly, monthly, yearly) with automatic instance generation
- Automatic cleanup of finished events older than 2 months on launch (configurable via `auto_cleanup_old_events`)
- Lazy loading for recurring event instances, enabling indefinite display of long-term recurring events
- Recurrence dropdown for event creation/editing implemented as a popup overlay instead of inline to prevent invalid syntax and improve usability
- Suggestions overlay for end date input: Implemented a dedicated overlay popup to display date suggestions without overlapping other input fields.
- Real-time validation and auto-completion for end date input in event creation/editing, providing immediate feedback for invalid dates, suggesting common date patterns like "tomorrow", "next week", "end of month", while maintaining DD/MM format with automatic year assumption

### Changed
- **Breaking Change**: Changed recurring event deletion behavior - deleting any recurring instance now deletes the entire series persistently. Previously, only the specific instance was removed from memory.
- Enhanced lazy loading to refresh cached recurring event instances after add, edit, or delete operations, ensuring UI accurately reflects changes without manual refresh.

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
