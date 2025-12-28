# packaging Specification Delta

## ADDED Requirements

### Requirement: Installation Documentation Structure
Installation documentation MUST provide separate sections for human users and AI agents.

#### Scenario: Human Section Content
Given the README installation section "For humans",
When reading installation instructions,
- **WHEN** counting commands per installation method
- **THEN** no more than 3 commands per method are included

#### Scenario: LLM Section Content
Given the README installation section "For LLM agents",
When reading installation instructions,
- **WHEN** reviewing the content
- **THEN** it includes prerequisite checks, detailed configuration guidance, and troubleshooting steps

### Requirement: Installation Verification
Installation documentation MUST include verification steps for both human and AI audiences.

#### Scenario: Human Verification Commands
Given a completed installation,
When following human verification steps,
- **WHEN** running `rcal --version`
- **THEN** version number is displayed
- **WHEN** running `systemctl --user status rcal.service`
- **THEN** service shows active state

#### Scenario: LLM Verification
Given installation by AI agent,
When running LLM verification steps,
- **WHEN** checking binary existence
- **THEN** `/usr/bin/rcal` exists
- **WHEN** verifying version
- **THEN** version matches expected format
- **WHEN** checking service
- **THEN** daemon is running
- **WHEN** checking configuration
- **THEN** config files are in place at `~/.config/rcal/config.toml`

### Requirement: Configuration Guidance
Installation documentation MUST include post-installation configuration instructions.

#### Scenario: Daemon Configuration
Given installed rcal,
When configuring daemon mode,
- **WHEN** running `systemctl --user enable --now rcal.service`
- **THEN** daemon starts and persists across reboots

#### Scenario: Sync Configuration
Given installed rcal,
When configuring git synchronization,
- **WHEN** initializing with `rcal --sync-init <URL>`
- **THEN** remote repository is configured
- **WHEN** checking `~/.config/rcal/config.toml`
- **THEN** remote URL is stored in configuration

#### Scenario: Configuration File Creation
Given fresh installation,
When accessing configuration,
- **WHEN** first running rcal
- **THEN** `~/.config/rcal/config.toml` is created with default values

### Requirement: Platform-Specific Instructions
Installation documentation MUST provide platform-specific guidance where applicable.

#### Scenario: Arch Linux AUR Installation
Given Arch Linux system,
When following installation instructions,
- **WHEN** using `paru -S rcal`
- **THEN** package is installed
- **WHEN** using alternative AUR helpers (yay, pikaur)
- **THEN** equivalent commands are documented

#### Scenario: Build from Source Verification
Given source installation method,
When building from source,
- **WHEN** running `cargo build --release`
- **THEN** build succeeds without errors
- **WHEN** checking target/release/rcal
- **THEN** binary is produced
- **WHEN** running `cargo install --path .`
- **THEN** binary is installed to cargo bin directory
