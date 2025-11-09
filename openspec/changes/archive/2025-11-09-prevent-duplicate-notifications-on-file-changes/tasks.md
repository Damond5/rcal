# Tasks for Preventing Duplicate Notifications on File Changes

- [x] **Analyze current notification logic**: Review `src/daemon.rs` to understand how `notified` is managed and cleared on file changes.
- [x] **Implement event comparison**: Modify the file change handling in `run_daemon()` to compare loaded events with current events before clearing `notified`, ensuring order-independent comparison by sorting both vectors.
- [x] **Add error handling**: Wrap `persistence::load_events()` in error handling to log failures and preserve the last good state on load errors.
- [x] **Add unit tests**: Add tests in `src/daemon.rs` to verify `notified` behavior on simulated file changes with unchanged/changed events, including edge cases like event reordering.
- [x] **Test integration**: Run the daemon with simulated file changes (e.g., touch files) to ensure notifications don't repeat for unchanged events.
- [x] **Validate with sync scenarios**: Test with Git sync operations to confirm no duplicate notifications during pulls that don't change events.</content>
<parameter name="filePath">openspec/changes/prevent-duplicate-notifications-on-file-changes/tasks.md