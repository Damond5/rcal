# AGENTS.md - rcal Project Development Guide

This document provides comprehensive guidance for developers working on the rcal project, a terminal calendar application built with Rust and Ratatui. It covers development workflows, coding standards, project structure, and tooling requirements.

## Project Overview

rcal is a terminal-based calendar application written in Rust that provides a rich text user interface for managing calendar events. The application features a terminal user interface (TUI) built with Ratatui, persistent event storage using Markdown files in the user's home directory, and Git-based synchronization for calendar sharing across devices. The project emphasizes terminal-native UX with custom Unicode cursor management, keyboard-driven navigation, and support for desktop notifications through D-Bus on Linux systems.

## Development Commands

### Build Commands

The project uses Cargo for all build operations. The workspace contains both a library (`rcal-lib`) and a binary (`rcal`), allowing for modular development and testing.

```bash
cargo build                                    # Debug build with full symbols
cargo build --release                          # Optimized release build
cargo build -p rcal-lib                       # Build library only
cargo build --bin rcal                        # Build binary only
```

Debug builds include full debug symbols and disable most optimizations, making them suitable for development and debugging. Release builds enable optimizations for better performance but lack debug symbols. Building individual targets is useful when working on specific components without rebuilding the entire workspace.

### Lint & Format Commands

Code quality is enforced through Rust's standard tooling. These commands must pass before any commit is made to the repository.

```bash
cargo fmt                                      # Format code automatically
cargo clippy                                   # Run linter for code quality
cargo clippy -- -D warnings                    # Treat all warnings as errors
```

The `cargo fmt` command enforces consistent code formatting according to the project's rustfmt configuration. Running this before commits ensures that code reviews focus on logic and design rather than style nitpicks. The `cargo clippy` command performs static analysis to identify common mistakes, anti-patterns, and opportunities for improvement. Using the `-D warnings` flag ensures that even minor issues are addressed, maintaining high code quality standards.

### Test Commands

The project maintains comprehensive test coverage through unit tests, integration tests, and property-based testing where appropriate.

```bash
cargo test                                    # Run all tests
cargo test --release                          # Run tests in release mode
cargo test --lib                              # Run library tests only
cargo test --test integration_test           # Run integration tests only
cargo test test_function_name                # Run single test by name
cargo test -p rcal-lib test_function_name    # Run single library test
cargo test --test integration_test test_name # Run single integration test
cargo test -- --nocapture                    # Show output for passing tests
cargo test -- --test-threads=1               # Run tests serially
```

Tests are organized into unit tests within source files (in `#[cfg(test)]` modules) and integration tests in the `tests/` directory. The `-p` flag targets specific workspace members, while pattern matching after `--` filters tests by name. The `--nocapture` flag is particularly useful when debugging test failures, as it shows all `println!` output even for passing tests. Running with `--test-threads=1` ensures serial execution, which is required for tests that involve file system operations or shared state.

## Code Style Guidelines

### Naming Conventions

The project follows standard Rust naming conventions as defined in the project configuration at `openspec/config.yaml`. These conventions ensure consistency across the codebase and align with Rust community standards.

Functions, variables, and modules must use `snake_case`, which consists entirely of lowercase letters with underscores separating words. Types and enums must use `PascalCase`, where each word begins with an uppercase letter. Constants, including static immutable values, must use `SCREAMING_SNAKE_CASE`, which uses uppercase letters and underscores. Trait names should follow `PascalCase` convention, and generic type parameters typically use single uppercase letters (e.g., `T`, `U`, `E` for error types).

### Imports and Organization

Import organization follows a specific hierarchy that improves readability and makes dependency relationships explicit. Standard library imports appear first, grouped together using nested import syntax for related types. External crate imports follow, also organized alphabetically within their group. Local imports use the `crate::` prefix to distinguish them from external dependencies.

```rust
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;

use chrono::{DateTime, Local, NaiveDate, TimeZone};
use clap::Parser;
use ratatui::{Frame, Terminal};

use crate::app::{App, CalendarEvent, SyncStatus};
use crate::persistence::load_events;
```

Avoid glob imports (`use crate::*`) as they obscure the origin of types and can lead to name conflicts. When importing multiple items from the same crate, use nested imports to show the relationship between types. This approach makes it easier to understand dependencies when reading code.

### Formatting

The project relies on `cargo fmt` for automatic formatting, which enforces Rust's standard conventions. Manual formatting adjustments are generally unnecessary and may be overwritten by the formatter. The default Rust formatting settings apply, including 4-space indentation (no tabs), a maximum line length of 100 characters, and blank lines between function definitions.

```rust
fn calculate_ical_date(date: &NaiveDate) -> String {
    date.format("%Y%m%dT%H%M%S").to_string()
}
```

Trailing whitespace is prohibited throughout the codebase. Each file should end with a single newline character. Function definitions should have blank lines between them to improve visual separation. Match arms and their bodies should have consistent spacing, with complex match expressions formatted across multiple lines for readability.

### Error Handling

Error handling follows Rust's idiomatic `Result` type pattern, with specific conventions for different contexts. The main entry point returns `Result<(), Box<dyn Error>>`, allowing for flexible error types while maintaining a consistent interface. I/O operations return `Result<T, Box<dyn Error>>` for operations that produce values or `Result<(), std::io::Error>` for operations that only indicate success or failure.

```rust
fn load_events_from_file(path: &PathBuf) -> Result<Vec<CalendarEvent>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    serde_yaml::from_str(&contents).map_err(Into::into)
}
```

Validation functions that report user-friendly errors return `Result<(), String>` rather than propagating low-level errors. Domain-specific error states use custom enums, such as `SyncStatus` for representing the state of calendar synchronization. The `?` operator is preferred over explicit match statements for error propagation, as it produces more concise and readable code. For non-critical errors, the application provides graceful degradation with warning messages rather than failing completely.

### Data Structures

Public structs in the project generally expose all fields publicly for simplicity and ease of use. Data types that need to be compared, cloned, or printed for debugging should derive the appropriate traits.

```rust
#[derive(Clone, PartialEq, Debug)]
pub struct CalendarEvent {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(PartialEq, Debug)]
pub enum SyncStatus {
    Idle,
    Syncing,
    Success,
    Failed(String),
}
```

State enums derive `PartialEq` and `Debug` but avoid deriving `Clone`, as cloning application state can lead to inconsistent state if updates are not carefully managed. Large application state structures bundle related state together, such as the `App` struct that holds the calendar data, UI state, and synchronization status.

### Testing

Tests follow Rust's conventions with unit tests in `#[cfg(test)]` modules embedded within source files and integration tests in the dedicated `tests/` directory. Tests use `tempfile::TempDir` for isolated file system operations, preventing interference between test runs.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_event_creation_with_valid_date() {
        let event = CalendarEvent::new(
            "Team Meeting",
            "Weekly sync",
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        );
        assert_eq!(event.title, "Team Meeting");
    }

    #[test]
    fn test_date_parsing_with_valid_input() {
        let result = parse_date_string("2024-01-15");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), NaiveDate::from_ymd_opt(2024, 1, 15).unwrap());
    }
}
```

Test names should be descriptive and indicate the action being tested and the expected outcome, such as `test_event_creation_with_valid_date` rather than `test_create`. Tests should cover edge cases, invalid inputs, and state transitions. Helper functions for test setup reduce duplication and ensure consistent initialization.

### Documentation

Public functions should include triple-slash documentation comments that describe their purpose, parameters, return values, and any important behavior or edge cases.

```rust
/// Parses a date string in YYYY-MM-DD format into a NaiveDate.
///
/// # Arguments
///
/// * `date_str` - A string slice containing the date to parse.
///
/// # Returns
///
/// Returns `Ok(NaiveDate)` if parsing succeeds, or `Err(String)` with
/// a user-friendly error message if the date is invalid.
///
/// # Examples
///
/// ```
/// let date = parse_date_string("2024-01-15");
/// assert!(date.is_ok());
/// ```
```

Complex functions should include implementation comments that explain the reasoning behind algorithmic choices, especially when those choices relate to design decisions documented elsewhere. Section comments can help navigate large files by describing the purpose of groups of related functions.

### Module Structure

The project uses a flat module structure within the `src/` directory, with clear separation of concerns by functionality rather than layering. All modules are declared in `lib.rs` and re-exported as public, allowing consumers to import from a consistent path.

```rust
// src/lib.rs
pub mod app;
pub mod date_utils;
pub mod daemon;
pub mod event_handling;
pub mod persistence;
pub mod sync;
pub mod ui;
```

Cross-module imports use the `crate::` prefix to distinguish local imports from external dependencies. For example, the `ui` module imports from `crate::app` rather than `super::app`. This convention makes the dependency structure explicit and avoids confusion about whether a module is local or external.

## Project Structure

```
rcal/
├── src/
│   ├── lib.rs                    # Library root, module declarations, public API
│   ├── main.rs                   # Binary entry point, argument parsing
│   ├── app.rs                    # Core application state (App, CalendarEvent, enums)
│   ├── event_handling.rs         # Terminal event processing and input handling
│   ├── persistence.rs           # File I/O operations, event storage/retrieval
│   ├── sync.rs                  # Git sync provider, SyncProvider trait definition
│   ├── ui.rs                    # Ratatui terminal UI rendering, component layout
│   ├── date_utils.rs            # Date parsing, validation, formatting utilities
│   └── daemon.rs               # Background notification daemon, D-Bus integration
├── tests/
│   └── integration_test.rs     # Integration tests for public API and workflows
├── openspec/
│   ├── config.yaml             # Project configuration, tech stack, rules
│   └── changes/               # Change proposals, design documents, archives
├── Cargo.toml                 # Workspace manifest, dependencies, target configurations
└── AGENTS.md                  # This file, development guide and conventions
```

The `src/lib.rs` file serves as the library root and declares all modules. The `src/main.rs` file contains the binary entry point that uses the library. Each module in `src/` has a single, focused responsibility. The `tests/` directory contains integration tests that exercise the public API from outside the library, simulating real usage scenarios.

## Key Dependencies

The project relies on several carefully selected dependencies that provide essential functionality while maintaining compatibility and stability.

- `ratatui` (0.26.1) - The terminal UI framework that provides the foundation for building the text-based user interface, offering a declarative approach to rendering terminal content.
- `chrono` (0.4.31) - Date and time handling library that provides robust parsing, formatting, and arithmetic operations for calendar dates.
- `crossterm` (0.27.0) - Cross-platform terminal capabilities including input handling, screen manipulation, and cursor control.
- `clap` (4.0) - Command-line argument parsing library that provides a declarative interface for defining CLI arguments and subcommands.
- `notify` / `notify-rust` - File system watching and desktop notification integration through D-Bus on Linux systems.
- `uuid` (1.0) - UUID generation for unique event identifiers, ensuring reliable event tracking across synchronization.

## Important Notes

Developers should be aware of several project-specific constraints and requirements that affect development and runtime behavior.

The minimum supported Rust version is 1.70, which ensures access to stable features used throughout the codebase. New dependencies must be compatible with this version requirement. D-Bus library and header files are required for desktop notification functionality on Linux systems; on systems without D-Bus, notification features will gracefully degrade.

Calendar events are stored as Markdown files in the `~/calendar/` directory, with each event represented as a separate file. The application uses Git for synchronization, with SSH authentication expected for remote repositories. The TUI implements custom Unicode cursor management for navigation feedback, which requires terminal support for Unicode block characters.

Input modes are managed through enums that represent different states of the application, such as navigation mode, editing mode, and command mode. This pattern ensures type-safe state transitions and prevents invalid state combinations.

## OpenSpec Integration

For significant changes, features, or architectural decisions, the project follows the OpenSpec workflow as defined in `openspec/AGENTS.md`. This workflow ensures that major changes are properly documented, reviewed, and tracked throughout their lifecycle.

The workflow proceeds through several phases: creating a change proposal in the `openspec/changes/` directory with detailed design documents, documenting design decisions with their rationale and alternatives, obtaining approval from project maintainers before implementation begins, updating specifications to reflect the implemented changes, and archiving the completed change with all relevant artifacts.

This process ensures that the project maintains high-quality documentation, that design decisions are traceable, and that future developers can understand the context behind architectural choices. See `openspec/AGENTS.md` for detailed workflow instructions and templates.
