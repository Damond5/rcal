# Update README Installation Format

## Why
The current installation instructions in README.md combine human and AI agent needs into a single section, making it harder for users to quickly get started and for AI agents to provide comprehensive assistance. Users often want quick, simple commands, while AI agents need detailed verification steps and configuration guidance to ensure successful setup.

## What Changes
Restructure README.md Installation section into two subsections following oh-my-opencode format:

**"For humans" section:**
- One-line AUR installation: `paru -S rcal`
- Simple daemon enablement: `systemctl --user enable --now rcal.service`
- Basic verification: `rcal` to launch app, `rcal --version` to confirm
- Simple troubleshooting tip for daemon issues

**"For LLM agents" section (hidden in collapsible details):**
- Prerequisite verification: Rust 1.70+ check, D-Bus availability check
- Detailed installation breakdown for each method (AUR, source build from source, cargo install)
- Post-installation setup with specific commands:
  - Service enablement with status verification
  - Configuration file location and structure (~/.config/rcal/config.toml)
  - Sync repository initialization and authentication
  - Directory creation verification (~/.local/share/rcal/)
- Troubleshooting guide covering:
  - Common installation errors and resolution
  - Permission issues and fixes
  - Service startup failures and debugging
  - Sync authentication problems and setup
- Verification checklist with exit codes and expected outputs

Current "Prerequisites" subsection will be integrated into both sections appropriately. All existing installation methods (AUR, source build, cargo install) will be preserved.

## Impact
- **Users**: Faster time-to-first-run with simple, focused installation commands
- **AI Agents**: Comprehensive guide for automated setup, verification, and troubleshooting
- **Documentation**: Better alignment with modern open-source documentation standards
- **Specs**: packaging/spec.md - new requirements for installation documentation structure and quality
