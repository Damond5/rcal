# Remove Sync Status Keybind Instruction

## Summary
Remove the "s: Check status" instruction from the sync popup UI since status is now automatically updated on popup entry.

## Motivation
The sync popup currently displays "s: Check status" as an available action, but this keybind was removed when automatic status updates were implemented. The instruction creates confusion for users as pressing 's' has no effect in sync mode. Removing this obsolete instruction will clean up the UI and prevent user confusion.

## Impact
- Eliminates misleading UI instruction
- Simplifies sync popup interface and improves usability by reducing cognitive load from irrelevant instructions
- Maintains existing automatic status functionality
- No functional changes to sync operations

## Implementation Approach
Remove the "s: Check status" ListItem from the instructions list in the sync popup rendering code in `src/ui.rs` (search for "s: Check status" to locate the exact line).

## Potential Risks/Edge Cases
- Ensure no other references to 's' keybind in sync context
- Verify that automatic status update still works correctly

## Testing Considerations
- Manually inspect the sync popup UI to confirm the instruction is removed and layout remains clean
- Verify that "f: Pull from remote" and "p: Push to remote" instructions are still displayed
- Test in different terminal sizes to ensure no rendering issues</content>
<parameter name="filePath">openspec/changes/2025-11-08-remove-sync-status-keybind-instruction/proposal.md