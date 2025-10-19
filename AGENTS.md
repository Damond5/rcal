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

## Tests

Always add accompanying test(s) when implementing new functionality.

### Testing Strategy

- **Integration Tests**: Use `cargo test` with `ratatui::backend::TestBackend` for comprehensive TUI testing
- **Event Simulation**: Test user interactions by simulating `crossterm::event::Event` inputs
- **State Verification**: Assert application state changes and UI behavior including mode transitions
- **Workflow Testing**: Test complete user workflows (view → add → delete) for end-to-end functionality
- **Avoid External Tools**: Do not use `xdotool` or similar tools; keep testing within the Rust ecosystem

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

### UI Rendering

- **Clear Widget**: Use `ratatui::widgets::Clear` to properly render popups over calendar background
- **Block Borders**: Wrap input fields in `Block` widgets with titles for clear field identification
- **Cursor Positioning**: Calculate cursor coordinates relative to input field positions using character-based indexing for Unicode support
- **Style Management**: Use conditional styling based on `selected_input_field` for visual feedback
- **Event Selection**: Highlight selected events in view popup with background color

### Error Handling

- **Graceful Degradation**: Invalid time formats prevent event creation but don't crash the application
- **Input Validation**: Parse time strings with flexible format support (HH:MM, HH, H) and handle failures
- **Boundary Checks**: Prevent cursor movement beyond string boundaries using character-based indexing
- **Unicode Support**: Handle multi-byte characters properly in text input fields

### Event Management

- **Flexible Time Input**: Support multiple time formats (HH:MM, HH, H) with automatic normalization
- **Event Deletion**: Safe deletion with confirmation dialog to prevent accidental data loss
- **Context-Aware Adding**: Add events directly from view popup without losing context
- **Persistent Storage**: Events saved to markdown files in user's home directory
- **Real-time Updates**: View popup refreshes automatically after adding/deleting events

## Test Coverage

The application includes comprehensive test coverage (43 tests) for all functionality:

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
