# Auto-Update Sync Status on Popup Entry

## Summary
Automatically fetch and display the current sync status when entering the sync popup, eliminating the need for users to manually check status first.

## Motivation
Currently, when users open the sync popup, the status shows "Unknown" until they press 's' to check status. This creates unnecessary friction in the user experience. By automatically updating the status on entry, users can immediately see the current sync state without additional interaction.

## Impact
- Improves user experience by providing immediate feedback
- Reduces the number of key presses required for sync operations
- Maintains existing functionality while enhancing responsiveness

## Implementation Approach
Modify the event handler in `src/event_handling.rs` to call the sync status check synchronously when transitioning to `InputMode::Sync`, updating `app.sync_status` and `app.sync_message` accordingly. This mirrors the existing 's' key behavior but executes automatically on entry.

## Potential Risks/Edge Cases
- Brief UI blocking if status check is slow (e.g., network issues with remote Git repo)
- Behavior when sync provider is not configured (should gracefully handle None provider)
- Error handling for failed status checks (set appropriate error status and message)</content>
<parameter name="filePath">openspec/changes/auto-update-sync-status-on-popup-entry/proposal.md