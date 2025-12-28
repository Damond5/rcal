# Tasks for Update README Installation Format

1. **Update spec delta format**: Fix spec delta to use OpenSpec scenario format (- **WHEN**, - **THEN**) instead of Gherkin-style ✅
2. **Create spec delta for packaging**: Add documentation requirements to packaging/spec.md for installation documentation structure and quality ✅
3. **Write "For humans" section using @docs-writer subagent**: Use @docs-writer subagent to write installation instructions including:
   - AUR package installation: `paru -S rcal`
   - Alternative AUR helpers (yay, pikaur) with equivalent commands
   - Systemd service enablement: `systemctl --user enable --now rcal.service`
   - Binary verification: `rcal --version` with expected output
   - Service status check: `systemctl --user status rcal.service`
   - Simple troubleshooting for daemon issues with `journalctl --user -u rcal.service`
   - Link to LLM section for users who need more details
   - Callout boxes for important notes (pro tips, warnings) ✅
4. **Write "For LLM agents" section using @docs-writer subagent**: Use @docs-writer subagent to write detailed guide including:
   - Prerequisite checks with expected outputs:
     - Rust version: `rustc --version` expecting 1.70+
     - D-Bus availability: `echo $DBUS_SESSION_BUS_ADDRESS` expecting non-empty
   - Step-by-step installation for each method with exit codes:
     - AUR: `paru -S rcal`, verify with `pacman -Qi rcal`
     - Source: `cargo build --release`, verify with `./target/release/rcal --version`
     - Cargo: `cargo install --path .`, verify with `rcal --version`
   - Configuration file structure (~/.config/rcal/config.toml) with auto_cleanup_old_events option
   - Sync repository setup and authentication (SSH keys) with `rcal --sync-init <URL>`
   - Verification checklist with expected outputs and exit codes:
     - Binary existence: `command -v rcal` (exit code 0)
     - Version check: `rcal --version` (displays version, exit code 0)
     - Service status: `systemctl --user is-active rcal.service` (outputs "active", exit code 0)
     - Directory check: `ls ~/calendar/`
   - Troubleshooting guide with decision tree:
     - Permission denied solutions
     - Service startup failures with `journalctl` debugging
     - Build failures with Rust version checks
     - Sync authentication problems
   - Use proper code block language tags (```bash, ```toml)
   - Add horizontal rules between major sections
   - Include example command outputs for clarity
   - Wrap entire section in `<details>` tags ✅
5. **Update README.md**: Replace existing Installation section with new two-section format, integrating Prerequisites appropriately into both sections ✅
6. **Test installation instructions**: Manually verify each method works:
   - [x] Review AUR installation commands and systemd service setup
   - [x] Review source build steps and verify cargo build --release works
   - [x] Review cargo install process
   - [x] Verify daemon service start commands and status checks
   - [x] Verify sync initialization commands
7. **Code review**: Use @review subagent to review updated README.md and implement all suggestions ✅
8. **Update CHANGELOG.md using @docs-writer subagent**: Use @docs-writer subagent to add detailed entry under "Changed" section in [Unreleased] following Keep a Changelog format describing the restructuring ✅
9. **Validate proposal**: Run `openspec validate update-readme-installation-format --strict` to ensure all documentation is properly formatted and complete ✅
