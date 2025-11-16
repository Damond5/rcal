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
- Interactive three-month calendar view with day/week/month navigation
- Add, view, edit, and delete events
- Support for recurring events (daily, weekly, monthly) with automatic instance display
- Support for multi-day events with start and end dates and times
- All-day events (leave time field empty when creating)
- Event details including title, start/end dates, start/end times, recurrence, and description
- Flexible time input formats (HH:MM, HH, H)
- End date input format: DD/MM with automatic year assumption

### Advanced Features
- Daemon mode for background notifications (30 minutes before timed events, midday the day before for all-day events)
- Real-time file watching for event updates
- Unicode support in text input
- Confirmation dialogs for safe operations
- Asynchronous Git-based synchronization for cross-device event sharing (auto-sync happens in background)

### Supported Formats
- Markdown-based event storage
- Desktop notifications via D-Bus
- Terminal-based UI with keyboard navigation
- Git synchronization for remote repositories

## Installation

### Prerequisites
- Rust 1.70 or later
- Cargo package manager
- For notifications: A D-Bus compatible desktop environment

### Arch Linux (AUR)
Install the `rcal` package from the Arch User Repository:
```bash
paru -S rcal  # or your preferred AUR helper
```
After installation, enable the daemon service:
```bash
systemctl --user enable rcal.service
systemctl --user start rcal.service
```

### Build from Source
```bash
git clone https://github.com/Damond5/rcal
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
- `rcal --sync-init <URL>`: Initialize sync with a Git remote
- `rcal --sync-pull`: Pull events from remote
- `rcal --sync-push`: Push events to remote
- `rcal --sync-status`: Check sync status
- `rcal --help`: Show help information

### Keybindings

#### Calendar Navigation
- **Day/Week**: Left/Right arrows or `h`/`l` for day, Up/Down arrows or `k`/`j` for week, with automatic view shifting for month changes.
- **Quit**: `q` or `Q`

#### Actions
- **Add Event**: `a`
- **View Events**: `o`
- **Delete Event**: `d` or Delete (in view popup)
- **Edit Event**: `e` (in view popup)
- **Sync Menu**: `s` (in main view)

#### Input Forms
- **Switch Fields**: Tab
- **Backwards Field Switch**: Shift+Tab
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

# Initialize sync with GitHub repo
rcal --sync-init https://github.com/user/my-calendar.git

# Pull latest events
rcal --sync-pull

# Push local changes
rcal --sync-push

# Check sync status
rcal --sync-status

# Add an event (interactive)
rcal
# Then press Enter on a date and fill in details

# Access sync menu in TUI
rcal
# Press s to open sync popup (status shown automatically), then f/p to pull/push
```

## Configuration

### Event Storage
Events are stored as individual markdown files in `~/calendar/` directory, one per event with title-based filenames. See EVENT_FORMAT.md for the detailed event format specification.
- Events include title, start date, start time, end date, end time, and description
- **Warning**: By default, non-recurring events that finished more than 2 months ago are automatically deleted on application launch to reduce clutter. Recurring events are not cleaned up to preserve ongoing schedules. This can be disabled in the configuration.

### Notification Settings
- Daemon checks for events 30 minutes ahead
- Notifications use system desktop notification service
- Duplicate notifications are prevented per session

### Sync Configuration
- Remote URL stored in `~/.config/rcal/config.toml`
- Uses SSH keys for Git authentication
- Supports rebase-based pulling to avoid merge commits
- Automatic pull on launch and push on save/delete happen asynchronously in background threads
- Conflicts must be resolved manually in the markdown files

### Configuration Options
Configuration is stored in `~/.config/rcal/config.toml`.

- `auto_cleanup_old_events` (default: true): Enable automatic cleanup of finished events older than 2 months on every launch. Set to false to disable this feature. Note: Deleted events cannot be recovered.

### Customization
Currently, rcal uses default settings. Future versions may support additional configuration options for themes and notification preferences.

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

## Version

Current version: 1.3.1

## Demo

Run `rcal` to launch the interactive calendar interface. Use the keybindings above to navigate and manage events.