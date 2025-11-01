# Agent Guidelines for rcal

This document outlines the conventions and commands for agents working in the `rcal` Rust project.

## Commands

- **Build**: `cargo build`
- **Lint**: `cargo clippy`
- **Test**: `cargo test`
- **Run a single test**: `cargo test <test_name>`
- **Format**: `cargo fmt`

## Code Style Guidelines

Adhere to standard Rust conventions as enforced by `cargo fmt` and `cargo clippy`.

- **Imports**: Organize `use` statements alphabetically and group them.
- **Formatting**: Use `cargo fmt` to automatically format code.
- **Types**: Leverage Rust's strong type system.
- **Naming Conventions**:
    - `snake_case` for functions, variables, and modules.
    - `PascalCase` for types (structs, enums, traits).
    - `SCREAMING_SNAKE_CASE` for constants.
- **Error Handling**: Prefer `Result` and `Option` for error handling. Avoid `panic!` for recoverable errors.
- **Import Management**: Remove unused imports to avoid warnings, e.g., fixed unused UUID import.

## Tests

Always add accompanying test(s) when implementing new functionality.

### Testing Strategy

- **Integration Tests**: Use `cargo test` with `ratatui::backend::TestBackend` for comprehensive TUI testing
- **Event Simulation**: Test user interactions by simulating `crossterm::event::Event` inputs
- **State Verification**: Assert application state changes and UI behavior including mode transitions
- **Workflow Testing**: Test complete user workflows (view → add → delete) for end-to-end functionality
- **Avoid External Tools**: Do not use `xdotool` or similar tools; keep testing within the Rust ecosystem
- **Daemon Testing**: Isolate notification logic into testable functions; avoid testing infinite loops or external D-Bus dependencies

### Testable Architecture

- **Event Handler Separation**: Extract `handle_event()` function for isolated testing of event logic
- **Library + Binary**: Make crate both library and binary (`[lib]` and `[[bin]]` in Cargo.toml) to expose modules for testing
- **Debug Traits**: Add `#[derive(Debug)]` to enums used in tests for assertion formatting
- **Comprehensive Coverage**: Test all user interactions, state transitions, and edge cases

### Test Organization

- **Integration Tests**: Place in `tests/` directory
- **Test Naming**: Use descriptive names like `test_navigation_left`, `test_create_event_success`
- **Test Isolation**: Each test should be independent and not rely on other tests
- **Event Simulation**: Use `Event::Key(KeyEvent::from(KeyCode::Char('a')))` pattern for input testing
- **Workflow Coverage**: Test complete user journeys including error cases and state transitions

## Design Choices

### TUI Architecture

- **Popup Layout**: Use centered, fixed-size rectangles (`Rect::new()`) for consistent popup positioning
- **Input Handling**: Implement custom cursor management with character-based indexing for Unicode support
- **State Management**: Use enum-based input modes (`InputMode`) for clear state transitions including confirmation dialogs
- **Event Filtering**: Filter events by date using iterator methods for efficient display
- **Modal Interactions**: Support nested popup states (view → add → confirm) with proper state restoration
- **Keybindings**: Always prefer vim-like keybindings (h/j/k/l for navigation) over modifier-based shortcuts (e.g., avoid Ctrl+S in favor of single-key alternatives). Use lowercase letters for commands, reserving uppercase (shift+letter) for future features, except where vim conventions apply (e.g., H/L for page navigation). Added Shift+Tab support for backwards navigation in event popup fields.
- **Keybind Hints**: Display contextual keybind hints on all screens and popups to guide user interactions, positioned at the bottom of each interface element. Fixed keybind hints display as footer instead of text field.
- **Cycling Navigation**: In the events view popup, up/down navigation cycles to the opposite end when reaching bounds.

### UI Rendering

- **Clear Widget**: Use `ratatui::widgets::Clear` to properly render popups over calendar background
- **Block Borders**: Wrap input fields in `Block` widgets with titles for clear field identification
- **Cursor Positioning**: Calculate cursor coordinates relative to input field positions using character-based indexing for Unicode support
- **Style Management**: Use conditional styling based on `selected_input_field` for visual feedback
- **Event Selection**: Highlight selected events in view popup with black text on light blue background for improved readability
- **Text Field Heights**: Fixed text field heights in event creation/editing popup for better visibility

### Error Handling

- **Graceful Degradation**: Invalid time formats prevent event creation but don't crash the application
- **Input Validation**: Parse time strings with flexible format support (HH:MM, HH, H) and handle failures
- **Boundary Checks**: Prevent cursor movement beyond string boundaries using character-based indexing
- **Unicode Support**: Handle multi-byte characters properly in text input fields
- **Notification Failures**: Gracefully handle D-Bus errors (e.g., no notification daemon) by logging without crashing the daemon

### Event Management

- **Flexible Time Input**: Support multiple time formats (HH:MM, HH, H) with automatic normalization
- **End Date Input Format**: Changed to DD/MM with automatic year assumption
- **Event Deletion**: Safe deletion with confirmation dialog to prevent accidental data loss
- **Context-Aware Adding**: Add events directly from view popup without losing context
- **Persistent Storage**: Events saved as individual markdown files in the user's home directory, one per event, with filenames based on sanitized event titles (replacing spaces with underscores, filtering to alphanumeric and underscore characters, with fallback to "untitled" for invalid titles), and duplicates handled by appending a number (e.g., "Team_Meeting.md", "Team_Meeting_1.md").
- **Multi-day Events**: Support for events spanning multiple days with start_date, end_date, start_time, and end_time fields
- **UI Enhancements**: Updated event creation and editing popup to include input fields for end date and end time
- **Event Format Specification**: Documented in EVENT_FORMAT.md for standardized event file format
- **Real-time Updates**: View popup refreshes automatically after adding/deleting events
- **Daemon Notifications**: Run with `--daemon` flag for background notifications independent of TUI, sending desktop notifications via `notify-rust` approximately 30 minutes before upcoming timed events and at midday the day before for all-day events. Notifications are checked for all events regardless of date, and sent only once per event per daemon session, with retriggering allowed on calendar file changes.
- **File Watching**: Use `notify` crate to monitor `~/calendar` directory for real-time event updates without restarting the daemon
- **Notification Deduplication**: Track notified events in a `HashSet` to prevent duplicate alerts per session
- **Systemd Integration**: Recommend systemd user services for reliable daemon startup and management in environments like i3/Arch Linux
- **Mandatory Non-Empty Titles**: Events require non-empty titles to ensure meaningful identification.
- **Default End Date/Time**: If end date is not specified during event creation/editing, it defaults to the start date, making the event last only on that single day. Similarly, if end time is not specified, it defaults to the start time, making it a point event.
- **Single-Day Event Handling**: Single-day events now properly have end_date set to start_date internally, and are saved without ' to ' in the file format. This ensures they display stars only on the single day.
- **All-Day Events**: Events without a specified time are treated as all-day events. All-day events are notified the day before at midday. In the UI, all-day events display "All day" instead of a time. In storage, all-day events have time set to "all-day" in the markdown file.
- **ID Removal**: Removed the `id` field from `CalendarEvent` struct to align with EVENT_FORMAT.md. Event title serves as the unique identifier for file operations, assuming titles are unique. Deletion generates filename from sanitized title without appending numbers for duplicates.
- **Configurable Calendar Directory**: App supports configurable calendar directory via `new_with_calendar_dir` constructor, enabling tests to use temporary directories and avoid interfering with real user events.

### Sync Implementation
- **Provider Abstraction**: Sync functionality uses a `SyncProvider` trait for extensibility, allowing future implementations (e.g., cloud storage, rsync) beyond the initial Git provider. This ensures the core app remains agnostic to sync mechanisms.
- **Git as Initial Provider**: Starts with Git for version control and cross-device sync, leveraging markdown files' human-readable format for easy conflict resolution. Uses system `git` commands via `std::process::Command` for full SSH config support (e.g., host aliases), ensuring compatibility with user authentication setups.
- **Asynchronous Auto-Sync**: Automatic sync operations (pull on launch, push on save/delete) are performed asynchronously in background threads to avoid blocking the TUI. Manual sync operations (via TUI menu or CLI) remain synchronous for user feedback.
- **Configuration Storage**: Sync settings (e.g., remote URL) stored in `~/.config/rcal/config.toml` for persistence across sessions. Uses TOML for human-editable config.
- **Error Handling and Safety**: Sync operations fail gracefully with user-friendly messages (e.g., conflict notifications). No automatic conflict resolution; users must manually edit markdown files. Operations do not crash the app or daemon. Background sync errors are currently logged but not displayed to avoid interrupting the user experience.
- **Integration with Existing Architecture**: Sync hooks into persistence layer for optional auto-push on save/delete via background threads. Daemon reloads events via file watching after sync, maintaining notification integrity without background sync loops.
- **Security and Privacy**: Relies on user's Git/SSH setup; no app-level secrets stored. Markdown files remain local until explicitly pushed.
- **Testing Strategy**: Unit tests for provider logic; integration tests for end-to-end sync workflows using temp directories. Avoids external Git repos in tests for isolation.

### Packaging for Arch Linux AUR
- **PKGBUILD Structure**: Standard Rust package with `cargo build --release`; installs binary to `/usr/bin/rcal`, license to `/usr/share/licenses/rcal/`, and systemd user service to `/usr/lib/systemd/user/rcal.service`.
- **Systemd User Service**: Provides `rcal.service` for daemon mode, enabling background notifications with `systemctl --user enable rcal.service`.
- **Service Enabling**: Manual by users to avoid interference with existing setups; automatic enabling on first TUI run considered but rejected for simplicity, compatibility, and to respect user preferences.
- **Dependencies**: No runtime deps; build deps include `cargo` and `rust`.
- **Source**: Uses GitHub git repo with release tags for versioning.
- **Maintenance**: Update `pkgver` and checksums on upstream releases; ensure AUR package stays current.

## Test Coverage

The application includes comprehensive test coverage (67 tests) for all functionality:

### Navigation Tests
- Day navigation (Left/Right, h/l)
- Week navigation (Up/Down, k/j)
- Month navigation (PageUp/PageDown, H/L)

### Popup Tests
- Add event popup opening/closing
- View events popup opening/closing
- Input field switching and cursor movement
- Add events from view popup

### Input Handling Tests
- Character input in title/time/description fields
- Unicode character support
- Backspace functionality
- Tab switching between fields
- Enter to save events
- Flexible time input (HH:MM, HH, H)

### Event Management Tests
- Event creation with valid/invalid data
- Event deletion with confirmation
- Event filtering by date
- Event sorting by time
- State management during operations
- Persistence across application restarts

### Edge Case Tests
- Empty input handling
- Boundary cursor movement
- Invalid time format handling
- No events scenarios
- Unicode text handling

### Daemon Tests
- Notification logic for upcoming events
- Deduplication of notifications
- Handling of past events
