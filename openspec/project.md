# Project Context

## Purpose
rcal is a terminal-based calendar application built with Rust and Ratatui. It provides an intuitive text-based user interface for managing events, with support for notifications and persistent storage. The goals are to offer a simple yet powerful calendar tool that runs entirely in the terminal, enabling users to manage their schedules without leaving the command line.

## Tech Stack
- **Rust**: Primary programming language (edition 2024)
- **Ratatui**: Terminal user interface framework
- **Crossterm**: Cross-platform terminal manipulation
- **Chrono**: Date and time handling
- **Notify-rust**: Desktop notifications via D-Bus
- **Notify**: File system watching for real-time updates
- **Clap**: Command-line argument parsing
- **Dirs**: Cross-platform directory handling
- **TOML**: Configuration file parsing

## Project Conventions

### Code Style
Adhere to standard Rust conventions as enforced by `cargo fmt` and `cargo clippy`:
- `snake_case` for functions, variables, and modules
- `PascalCase` for types (structs, enums, traits)
- `SCREAMING_SNAKE_CASE` for constants
- Organize `use` statements alphabetically and group them
- Remove unused imports to avoid warnings

### Architecture Patterns
- **TUI Architecture**: Enum-based input modes for clear state transitions, centered popup layouts with fixed-size rectangles, custom cursor management with character-based indexing for Unicode support
- **Event Management**: Events stored as individual markdown files in user's home directory, one per event with title-based filenames
- **State Management**: Separation of concerns between UI rendering, event handling, and persistence layers
- **Provider Abstraction**: Sync functionality uses a `SyncProvider` trait for extensibility
- **Asynchronous Operations**: Background threads for sync operations to avoid blocking TUI

### Testing Strategy
- **Integration Tests**: Use `cargo test` with `ratatui::backend::TestBackend` for comprehensive TUI testing
- **Event Simulation**: Test user interactions by simulating `crossterm::event::Event` inputs
- **State Verification**: Assert application state changes and UI behavior including mode transitions
- **Workflow Testing**: Test complete user workflows (view → add → delete) for end-to-end functionality
- **Testable Architecture**: Extract `handle_event()` function for isolated testing, make crate both library and binary to expose modules
- **Comprehensive Coverage**: Test all user interactions, state transitions, and edge cases (67 tests currently)

### Git Workflow
- Use Git for version control and synchronization
- Branching strategy: Feature branches for development
- Commit conventions: Follow standard practices with descriptive messages
- Sync operations: Automatic pull on launch, push on save/delete via background threads
- Conflict resolution: Manual resolution in markdown files, no automatic merging

## Domain Context
- **Calendar Events**: Support for timed and all-day events, multi-day events with start/end dates and times
- **Time Formats**: Flexible input (HH:MM, HH, H), automatic normalization
- **Date Formats**: DD/MM with automatic year assumption for end dates
- **Notifications**: Desktop notifications 30 minutes before timed events, midday the day before for all-day events
- **Persistence**: Markdown-based storage in `~/calendar/` directory, real-time file watching
- **Synchronization**: Git-based cross-device sync with SSH authentication

## Important Constraints
- **Terminal Environment**: Must work in various terminal emulators with proper Unicode support
- **D-Bus Compatibility**: Notifications require D-Bus compatible desktop environment
- **File System Access**: Requires write access to home directory for event storage
- **Rust Version**: Minimum Rust 1.70
- **No External Services**: All functionality works offline, no cloud dependencies required

## External Dependencies
- **D-Bus**: For desktop notifications (notify-rust)
- **Git**: For synchronization features (system command execution)
- **Terminal Emulator**: For TUI rendering (ratatui/crossterm)
