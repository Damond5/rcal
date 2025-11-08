1. Modify event handler in `src/event_handling.rs` (around line 137) to call sync status check after setting `app.input_mode = InputMode::Sync`
2. Update `app.sync_status` and `app.sync_message` fields in app state upon successful status retrieval, mirroring the 's' key handler logic
3. Handle status check errors gracefully by setting `SyncStatus::Error` and appropriate error message when `provider.status()` fails
4. Handle case where sync provider is None (skip status check and set status to None)
5. Test the implementation by opening sync popup and verifying status is displayed immediately without pressing 's'
6. Run existing tests to ensure no regressions in sync functionality
7. Update CHANGELOG.md and README.md to reflect the improved sync popup behavior</content>
<parameter name="filePath">openspec/changes/auto-update-sync-status-on-popup-entry/tasks.md