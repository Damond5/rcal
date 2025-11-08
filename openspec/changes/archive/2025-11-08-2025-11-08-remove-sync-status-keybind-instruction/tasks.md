1. Remove the "s: Check status" ListItem from the instructions list in sync popup rendering (grep for "s: Check status" in src/ui.rs)
2. Verify that the sync popup still displays "f: Pull from remote" and "p: Push to remote" instructions
3. Test that automatic status update on popup entry still works correctly
4. Run integration tests (tests/integration_test.rs) to ensure no regressions in sync functionality
5. Manually test sync popup rendering in different terminal sizes to ensure clean layout
6. Update CHANGELOG.md to document the UI cleanup</content>
<parameter name="filePath">openspec/changes/2025-11-08-remove-sync-status-keybind-instruction/tasks.md