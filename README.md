# rcal

A terminal-based calendar application built with Rust and Ratatui.

[![Test Coverage](https://img.shields.io/badge/coverage-57.5%25-yellow)](coverage-report/html/index.html)
[![Tests](https://img.shields.io/badge/tests-127_passed-green)](coverage.sh)

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
- Recurrence options can be selected from a popup overlay that appears when navigating to the recurrence field in the event creation and editing popups, instead of typing them, guiding users and preventing errors
- Support for recurring events (daily, weekly, monthly, yearly) with automatic instance display for indefinite periods
- Deleting any recurring event instance deletes the entire series persistently
- Support for multi-day events with start and end dates and times
- All-day events (leave Start Time field empty when creating)
- Event details including title, start date, Start Time, end date, End Time, recurrence, and description
- Flexible Start Time input formats (HH:MM, HH, H) with real-time validation and immediate error feedback to prevent invalid Start Times
- Real-time validation for date and Start Time input fields: End date input with real-time validation and auto-completion, and Start Time input fields with immediate format validation
- Note: Yearly recurring events on February 29th automatically fall back to February 28th in non-leap years to ensure annual occurrence (e.g., for birthdays and anniversaries)

### Advanced Features
- Daemon mode for background notifications (30 minutes before events with Start Time, midday the day before for all-day events)
- Real-time file watching for event updates
- Immediate refresh of recurring event instances after add, edit, or delete operations for accurate and responsive calendar display
- Unicode support in text input
- Confirmation dialogs for safe operations
- Asynchronous Git-based synchronization for cross-device event sharing (auto-sync happens in background)

### Supported Formats
- Markdown-based event storage
- Desktop notifications via D-Bus
- Terminal-based UI with keyboard navigation
- Git synchronization for remote repositories

## Installation

### For humans

Quick installation for Arch Linux users using the AUR.

> **Note**: These instructions are for Arch Linux. For other platforms or detailed setup, see [For LLM Agents](#for-llm-agents).

#### Quick install on Arch Linux (AUR)

```bash
paru -S rcal
```

Alternative AUR helpers:
```bash
yay -S rcal
pikaur -S rcal
```

> **Prerequisites**: Arch Linux system and a D-Bus compatible desktop environment for notifications.

#### Daemon setup

Enable and start background notification service:

```bash
systemctl --user enable --now rcal.service
```

> **Pro tip**: The daemon sends notifications 30 minutes before events with Start Time and midday the day before all-day events.

> **Note**: On first run, rcal creates `~/.config/rcal/config.toml` and `~/calendar/` automatically.

#### Verification

Confirm installation and service status:

```bash
rcal --version
systemctl --user status rcal.service
```

#### Troubleshooting

If you encounter issues with the daemon:

```bash
journalctl --user -u rcal.service
```

> **Need more details?** See the [For LLM Agents](#for-llm-agents) section for comprehensive installation and configuration information.

### For LLM Agents

<details>
<summary>📋 Instructions for AI assistants (click to expand)</summary>

If you're an LLM agent helping install rcal, follow these comprehensive steps:

### Step 0: Prerequisite Verification

Check Rust installation:

```bash
rustc --version
```

Expected output: `rustc 1.70.0` or later. If missing, install from https://rustup.rs.

Verify D-Bus is available:

```bash
echo $DBUS_SESSION_BUS_ADDRESS
```

Expected output: Non-empty string like `unix:path=/run/user/1000/bus`. If empty, D-Bus may not be running.

### Step 1: Installation Methods

#### Method A: Arch Linux AUR (Recommended)

Install using paru:

```bash
paru -S rcal
```

Verify installation:

```bash
pacman -Qi rcal
```

Expected: Package information displayed (exit code: 0)

Alternative AUR helpers:

```bash
yay -S rcal
pikaur -S rcal
```

#### Method B: Build from Source

Clone and build:

```bash
git clone https://github.com/Damond5/rcal
cd rcal
cargo build --release
```

Expected: Build completes without errors (exit code: 0)

Verify binary:

```bash
./target/release/rcal --version
```

Expected: Version string displayed (exit code: 0)

#### Method C: Cargo Install

Install to cargo bin directory:

```bash
cargo install --path .
```

Expected: Installation completes (exit code: 0)

Verify:

```bash
rcal --version
```

Expected: Version string displayed (exit code: 0)

### Step 2: Configuration

#### Daemon Service Setup

Enable and start the notification service:

```bash
systemctl --user enable --now rcal.service
```

Verify service is running:

```bash
systemctl --user status rcal.service
```

Expected output:

```
● rcal.service - rcal calendar daemon
     Loaded: loaded (/usr/lib/systemd/user/rcal.service; enabled)
     Active: active (running) since Mon 2025-12-28 10:00:00 UTC; 5min ago
```

Check active status programmatically:

```bash
systemctl --user is-active rcal.service
```

Expected: `active` (exit code: 0)

> **Note**: First run of rcal creates configuration directory and files automatically.

#### Configuration File

Location: `~/.config/rcal/config.toml`

First run creates this file with default values. Example structure:

```toml
# ~/.config/rcal/config.toml

[sync]
remote = "git@github.com:user/my-calendar.git"

auto_cleanup_old_events = true
```

View current configuration:

```bash
cat ~/.config/rcal/config.toml
```

#### Sync Repository Setup (Optional)

Initialize Git synchronization with a remote repository:

```bash
rcal --sync-init https://github.com/user/my-calendar.git
```

Expected: Repository initialized, config updated (exit code: 0)

Verify configuration:

```bash
grep remote_url ~/.config/rcal/config.toml
```

Expected: `remote_url = "https://github.com/user/my-calendar.git"`

**Requirements**: SSH keys must be configured for Git authentication.

Test SSH access:

```bash
ssh -T git@github.com
```

Expected: Authentication succeeds (for GitHub) or equivalent for your Git host.

### Step 3: Verification Checklist

Run these checks to confirm installation is successful:

| Check | Command | Expected Result |
|-------|----------|-----------------|
| Binary exists | `command -v rcal` | Returns path (exit code: 0) |
| Version check | `rcal --version` | Version displayed (exit code: 0) |
| Service active | `systemctl --user is-active rcal.service` | `active` (exit code: 0) |
| Config created | `test -f ~/.config/rcal/config.toml` | File exists (exit code: 0) |
| Data directory | `test -d ~/calendar/` | Directory created (exit code: 0) |

Quick verification script:

```bash
#!/bin/bash
set -e

echo "Checking rcal installation..."

command -v rcal >/dev/null 2>&1 || { echo "❌ rcal not found in PATH"; exit 1; }
echo "✅ Binary found"

rcal --version >/dev/null 2>&1 || { echo "❌ rcal --version failed"; exit 1; }
echo "✅ Version check passed"

systemctl --user is-active rcal.service >/dev/null 2>&1 || { echo "❌ Service not active"; exit 1; }
echo "✅ Service is active"

test -f ~/.config/rcal/config.toml || { echo "❌ Config file not found"; exit 1; }
echo "✅ Configuration file exists"

test -d ~/calendar/ || { echo "⚠️  Data directory not created yet (normal if not first run)"; }
echo "✅ All critical checks passed!"
```

### Step 4: Troubleshooting

#### Installation Issues

**Permission denied when installing**

```bash
# Check if running as root (not recommended)
# Use sudo only if necessary for package installation
sudo paru -S rcal
```

**Build from source fails**

```bash
# Check Rust version
rustc --version

# If < 1.70, update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

**Binary not found after install**

```bash
# Check if cargo bin is in PATH
echo $PATH | grep -o 'cargo/bin'

# Add to ~/.bashrc or ~/.zshrc if missing
export PATH="$HOME/.cargo/bin:$PATH"
```

#### Service Issues

**Service won't start**

```bash
# Check service status
systemctl --user status rcal.service

# View recent logs (last 50 lines)
journalctl --user -u rcal.service -n 50

# View all logs
journalctl --user -u rcal.service
```

Common issues:
- D-Bus not running: Start desktop session
- Permission denied: Check file permissions on config directory
- Binary missing: Re-install package

**Service not found**

```bash
# Check if systemd user instance is running
systemctl --user list-units

# If no user instance, systemd user service may not be available
# Use daemon mode directly instead:
rcal --daemon
```

#### Sync Issues

**Authentication failed**

```bash
# Test SSH connection to Git host
ssh -T git@github.com
ssh -T git@gitlab.com

# If fails, set up SSH keys
ssh-keygen -t ed25519 -C "your_email@example.com"
# Add key to Git host account
```

**Connection refused**

```bash
# Test network connectivity
ping github.com

# Check URL format
rcal --sync-init git@github.com:user/repo.git
```

**Conflicts after pull**

```bash
# Navigate to calendar directory
cd ~/calendar/

# Resolve merge conflicts in markdown files
# Then push resolved changes
rcal --sync-push
```

---

</details>

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
- **Recurrence Selection**: When navigating to the recurrence field, a popup overlay appears; use j/k to navigate options, Enter to select, Esc to cancel

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

### Testing

rcal has comprehensive test coverage with 127 automated tests (34 unit tests + 93 integration tests).

#### Running Tests
```bash
cargo test                    # Run all tests
cargo test --test integration_test  # Run only integration tests
./coverage.sh                 # Generate coverage report
```

#### Coverage
- **Core Logic** (persistence, date utilities, daemon): 80%+ coverage
- **TUI/UI Code**: 60%+ coverage (challenging due to terminal I/O)
- **Current Coverage**: 57.5% overall (HTML report: `coverage-report/html/index.html`)

#### Test Types
- **Unit Tests**: Test individual functions and modules
- **Integration Tests**: Test complete workflows including TUI interactions, sync operations, and daemon functionality
- **Coverage Tooling**: Uses `cargo-llvm-cov` for accurate coverage measurement

### Contributing
1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run `cargo test` and `cargo clippy`
5. Submit a pull request

## License

CC0 1.0 Universal - see LICENSE file for details.

## Version

Current version: 1.4.0

## Demo

Run `rcal` to launch the interactive calendar interface. Use the keybindings above to navigate and manage events.

---

### Step 0: Prerequisite Verification

Before installing rcal, verify your environment meets the minimum requirements.

#### Rust Version Check

Verify Rust 1.70 or later is installed:

```bash
rustc --version
```

**Expected output:**
```
rustc 1.70.0 (or higher)
```

**If check fails:**
- If Rust is not installed: Install via [rustup](https://rustup.rs/)
- If version is below 1.70: Run `rustup update`

#### D-Bus Availability Check

Verify D-Bus session is available for notifications:

```bash
echo $DBUS_SESSION_BUS_ADDRESS
```

**Expected output:**
```
unix:path=/run/user/1000/bus
```

**If check fails:**
- Empty output indicates no D-Bus session
- Ensure you're running in a graphical session with D-Bus
- Notifications will still work without D-Bus, but desktop notifications won't be displayed

---

### Step 1: Installation Methods

Choose one of the following installation methods based on your environment.

#### Method A: Arch Linux AUR (Recommended)

Install rcal from the Arch User Repository using paru:

```bash
paru -S rcal
```

**Expected exit code:** `0`

Verify package installation:

```bash
pacman -Qi rcal
```

**Expected output:** Package information including version, description, and dependencies

**Alternative AUR helpers:**
```bash
yay -S rcal      # Using yay
pikaur -S rcal   # Using pikaur
```

#### Method B: Build from Source

Clone the repository and build:

```bash
git clone https://github.com/Damond5/rcal && cd rcal
cargo build --release
```

**Expected output:** Build completes without errors, binary created at `./target/release/rcal`

**Expected exit code:** `0`

Verify the build:

```bash
./target/release/rcal --version
```

**Expected output:** Version information (e.g., `rcal 1.4.0`)

#### Method C: Cargo Install

Install from the local directory:

```bash
cargo install --path .
```

**Expected output:** Compilation and installation completes successfully

**Expected exit code:** `0`

Verify the installation:

```bash
rcal --version
```

**Expected output:** Version information

---

### Step 2: Configuration

#### Daemon Service

Enable and start the rcal notification daemon:

```bash
systemctl --user enable --now rcal.service
```

**Expected exit code:** `0`

Check service status:

```bash
systemctl --user status rcal.service
```

**Expected output:** Service shows "active (running)" status

> **Note:** The daemon service runs in the background and sends notifications for upcoming events:
> - Events with Start Time: 30 minutes before
> - All-day events: Midday the day before

#### Configuration File

On first run, rcal creates a configuration file automatically:

**Location:** `~/.config/rcal/config.toml`

Example configuration:

```toml
[sync]
remote = "https://github.com/user/repo.git"

auto_cleanup_old_events = true
```

**Configuration options:**
- `auto_cleanup_old_events` (default: `true`): Automatically delete finished events older than 2 months. Set to `false` to preserve all events.

> **Warning:** When `auto_cleanup_old_events` is enabled, deleted events cannot be recovered.

#### Sync Repository

Initialize Git synchronization with a remote repository:

```bash
rcal --sync-init https://github.com/user/repo.git
```

**Expected output:** `Sync initialized with remote: https://github.com/user/repo.git`

**Prerequisites:**
- Git must be installed on the system
- SSH keys must be configured for Git authentication with the remote

**Storage:** The remote URL is stored in `~/.config/rcal/config.toml` under `[sync.remote]`

---

### Step 3: Verification Checklist

Verify that rcal is properly installed and configured using the following checks:

| Check | Command | Expected Result | Exit Code |
|-------|---------|-----------------|-----------|
| Binary exists | `command -v rcal` | Path to rcal binary | `0` |
| Version correct | `rcal --version` | Version number displayed | `0` |
| Service active | `systemctl --user is-active rcal.service` | `active` | `0` |
| Config created | `ls ~/.config/rcal/config.toml` | File exists | `0` |
| Data directory | `ls ~/.local/share/rcal/` | Directory created on first run | `0` |

**Quick verification script:**

```bash
#!/bin/bash
# Run all verification checks

echo "Checking binary..."
command -v rcal && echo "✓ Binary found" || echo "✗ Binary not found"

echo "Checking version..."
rcal --version && echo "✓ Version check passed" || echo "✗ Version check failed"

echo "Checking service..."
systemctl --user is-active rcal.service | grep -q "active" && echo "✓ Service active" || echo "✗ Service not active"

echo "Checking config..."
ls ~/.config/rcal/config.toml > /dev/null 2>&1 && echo "✓ Config exists" || echo "✗ Config not found"

echo "Checking data directory..."
ls ~/.local/share/rcal/ > /dev/null 2>&1 && echo "✓ Data directory exists" || echo "✗ Data directory not found"
```

---

### Step 4: Troubleshooting

#### Installation Issues

**Problem: Permission denied during installation**

```bash
# Check permissions
ls -la target/release/rcal

# Fix permissions if needed
chmod +x target/release/rcal

# Or use sudo for system-wide install
sudo install -m 755 target/release/rcal /usr/local/bin/rcal
```

**Problem: Build failed**

```bash
# Verify Rust version
rustc --version

# Clean and rebuild
cargo clean
cargo build --release

# Check for dependency issues
cargo update
```

#### Service Issues

**Problem: Service won't start**

```bash
# Check service logs for errors
journalctl --user -u rcal.service -n 50
```

**Problem: Service not found**

```bash
# Verify systemd user instance is running
systemctl --user

# If not running, start systemd user session
loginctl enable-linger $USER
```

#### Sync Issues

**Problem: Authentication failed**

```bash
# Verify SSH keys are configured
ssh -T git@github.com

# Expected output: Hi username! You've successfully authenticated...
```

**Problem: Connection refused**

```bash
# Check Git remote URL
cat ~/.config/rcal/config.toml

# Test network connectivity
ping github.com

# Verify remote repository exists
git ls-remote https://github.com/user/repo.git
```

**Problem: Merge conflicts**

```bash
# Check current sync status
rcal --sync-status

# Manually resolve conflicts in ~/calendar/ directory
cd ~/calendar
git status
# Edit conflicted files, then:
git add .
git commit -m "Resolve conflicts"
git push
```

</details>