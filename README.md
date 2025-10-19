# rcal

A terminal-based calendar application built with Rust and Ratatui.

## Description

rcal is a simple yet powerful calendar application that runs entirely in your terminal. It provides an intuitive text-based user interface for managing events, with support for notifications and persistent storage.

Key benefits:
- **TUI Interface**: Navigate and manage your calendar without leaving the terminal
- **Notifications**: Get desktop notifications for upcoming events via daemon mode
- **Persistence**: Events are stored as markdown files in your home directory

## Features

### Core Functionality
- Interactive calendar view with month/week navigation
- Add, view, and delete events
- Event details including title, time, and description
- Flexible time input formats (HH:MM, HH, H)

### Advanced Features
- Daemon mode for background notifications
- Real-time file watching for event updates
- Unicode support in text input
- Confirmation dialogs for safe operations

### Supported Formats
- Markdown-based event storage
- Desktop notifications via D-Bus
- Terminal-based UI with keyboard navigation

## Installation

### Prerequisites
- Rust 1.70 or later
- Cargo package manager

### Build from Source
```bash
git clone <repository-url>
cd rcal
cargo build --release
```

### Optional: Install Binary
After building, you can install the binary:
```bash
cargo install --path .
```

## Usage

### Basic Commands
- `rcal`: Launch the interactive calendar
- `rcal --daemon`: Run in daemon mode for notifications
- `rcal --help`: Show help information

### Keybindings

#### Calendar Navigation
- **Day**: Left/Right arrows or `h`/`l`
- **Week**: Up/Down arrows or `k`/`j`
- **Month**: PageUp/PageDown or `H`/`L`
- **Quit**: `q` or `Q`

#### Actions
- **Add Event**: `a`
- **View Events**: `o`
- **Delete Event**: `d` or Delete (in view popup)
- **Edit Event**: `e` (in view popup)

#### Input Forms
- **Switch Fields**: Tab
- **Save**: Enter
- **Cancel**: Esc
- **Cursor Movement**: Left/Right arrows
- **Delete Character**: Backspace
- **Confirm Delete**: `y` or `Y` (in confirmation dialog)

### Examples
```bash
# View calendar
rcal

# Run notifications daemon
rcal --daemon

# Add an event (interactive)
rcal
# Then press Enter on a date and fill in details
```

## Configuration

### Event Storage
Events are stored as markdown files in `~/calendar/` directory:
- Each day has its own file (e.g., `2024-01-15.md`)
- Events include title, time, and description

### Notification Settings
- Daemon checks for events 30 minutes ahead
- Notifications use system desktop notification service
- Duplicate notifications are prevented per session

### Customization
Currently, rcal uses default settings. Future versions may support configuration files for themes and notification preferences.

## Development

### Build Commands
```bash
cargo build          # Debug build
cargo build --release # Release build
cargo test           # Run tests
cargo clippy         # Lint code
cargo fmt            # Format code
```

### Code Style
Follow standard Rust conventions enforced by `cargo fmt` and `cargo clippy`:
- `snake_case` for functions and variables
- `PascalCase` for types
- Comprehensive test coverage for all features

### Contributing
1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run `cargo test` and `cargo clippy`
5. Submit a pull request

## License

CC0 1.0 Universal - see LICENSE file for details.

## Demo

Run `rcal` to launch the interactive calendar interface. Use the keybindings above to navigate and manage events.