# Prevent Duplicate Notifications on File Changes

## Why
The daemon currently sends duplicate notifications for the same events when file changes occur in the calendar directory without altering event data. This happens due to sync operations modifying files (e.g., Git metadata), triggering unnecessary reloads and clearing the notification deduplication state.

## What Changes
- Modify daemon file change handling to compare loaded events with current events before clearing notified set
- Add error handling for event loading failures to preserve last good state
- Ensure order-independent event comparison via sorting
- Update persistence functions to return Results for better error propagation

## Impact
- Improves user experience by eliminating duplicate notifications
- Maintains correct behavior when events actually change (re-notifies appropriately)
- No breaking changes to existing functionality

## Related Specs
- event-management: Notification Deduplication requirement</content>
<parameter name="filePath">openspec/changes/prevent-duplicate-notifications-on-file-changes/proposal.md