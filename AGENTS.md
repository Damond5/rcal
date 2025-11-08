<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# Agent Guidelines for rcal

## Build/Lint/Test Commands

### Building
- `cargo build` - Compile the application
- `cargo build --release` - Compile optimized release build

### Linting
- `cargo clippy` - Run the Rust linter for code quality checks
- `cargo clippy --fix` - Auto-fix clippy warnings where possible

### Testing
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run a specific test (e.g., `cargo test test_quit_application`)
- `cargo test --lib` - Run only unit tests (library tests)
- `cargo test --test integration_test` - Run only integration tests

### Formatting
- `cargo fmt` - Format code according to Rust standards
- `cargo fmt --check` - Check if code is properly formatted

## Code Style Guidelines

### Naming Conventions
- **Functions and variables**: snake_case (e.g., `get_config_path`, `popup_event_title`)
- **Types (structs, enums)**: PascalCase (e.g., `CalendarEvent`, `InputMode`)
- **Constants**: SCREAMING_SNAKE_CASE
- **Modules**: snake_case

### Import Organization
- Group imports by category: std, external crates, then local modules
- Sort imports alphabetically within each group
- Use explicit imports over glob imports (`use std::path::PathBuf` not `use std::path::*`)
- Remove unused imports

### Formatting Standards
- Use `cargo fmt` for consistent formatting
- Follow standard Rust indentation and spacing
- Use 4 spaces for indentation (cargo fmt default)

### Type System Usage
- Leverage Rust's strong type system with appropriate structs and enums
- Use meaningful type names that describe purpose
- Prefer specific types over generic ones where appropriate

### Error Handling
- Use `Result<T, E>` for operations that can fail
- Use `Option<T>` for optional values instead of null
- Prefer graceful error handling over panics for recoverable errors
- Log errors appropriately without crashing the application

### Code Structure
- Keep functions focused and single-purpose
- Use meaningful variable and function names
- Add comments for complex logic, but avoid obvious comments
- Follow the existing patterns in the codebase for consistency

## Testing Guidelines

### Test Organization
- Integration tests go in `tests/` directory
- Use descriptive test function names starting with `test_`
- Each test should be independent and not rely on other tests

### Testing Patterns
- Use `ratatui::backend::TestBackend` for TUI testing
- Simulate user interactions with `crossterm::event::Event`
- Test complete user workflows end-to-end
- Assert both application state changes and UI behavior
- Cover error cases and edge conditions

### Test Structure
- Setup test data using helper functions (e.g., `setup_app()`)
- Use temporary directories for file operations
- Verify state transitions and mode changes
- Test both success and failure scenarios

## Development Workflow

1. Write code following the style guidelines above
2. Add tests for new functionality
3. Run `cargo fmt` to format code
4. Run `cargo clippy` to check for issues
5. Run `cargo test` to verify functionality
6. Commit changes following conventional commit format
