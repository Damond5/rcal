# Remove Duplicate "Up to Date" in Sync Popup

## Why
The sync popup displays "Up to date" redundantly in both the message and status areas when the sync status is up to date, cluttering the UI.

## What Changes
- Modify `src/event_handling.rs` to set `app.sync_message` to an empty string for `SyncStatus::UpToDate`, preventing the duplicate message display.
- Update the ui-rendering spec to require no redundant "Up to date" messages.

## Impact
- Minor UI improvement: eliminates duplicate text in sync popup.
- No functional changes to sync operations.</content>
<parameter name="filePath">openspec/changes/remove-duplicate-up-to-date-sync-popup/proposal.md