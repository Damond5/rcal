# Modify Recurring Event Deletion Behavior

## Why
Currently, deleting a recurring event instance only removes that specific instance from memory, leaving the base event file intact. This means the recurring series continues to generate new instances on future application launches. Users expect that deleting any part of a recurring event should cancel the entire series, similar to how calendar applications typically behave.

## What Changes
- **BREAKING**: Change deletion behavior so deleting any recurring instance deletes the entire series persistently
- Update event management spec to reflect new behavior
- Modify deletion logic in event_handling.rs to detect recurring instances and delete base event
- Add helper function to find base event for instances
- Update tests, CHANGELOG.md, and README.md

## Impact
- **User Experience**: More intuitive deletion behavior for recurring events
- **Data Consistency**: Ensures recurring events are properly managed across sessions
- **Breaking Change**: Changes existing behavior where instance deletion was non-persistent. Existing recurring events may behave differently on next launch if instances were previously deleted.

## Risks and Mitigations
- **Risk: Accidental Series Deletion**: Users might delete an instance expecting only that instance to be removed, but the whole series disappears.
  - **Mitigation**: Enhance confirmation dialog to clearly indicate that deleting a recurring instance will delete the entire series. Consider adding a config option to toggle between old and new behavior (default to new).
- **Risk: Data Loss**: If base event file deletion fails (e.g., permissions), instances remain but series is broken.
  - **Mitigation**: Implement robust error handling with user notifications and logging. Ensure partial failures don't leave inconsistent state.
- **Risk: Sync Conflicts**: Ongoing sync operations might conflict with series deletion.
  - **Mitigation**: Coordinate with sync provider to handle deletions gracefully, potentially pausing sync during deletion.

## Implementation Approach
Modify the event deletion logic in the ViewEventsPopup to detect when a recurring instance is being deleted (via event metadata like `is_recurring_instance` and `base_date`) and instead delete the base recurring event, which will remove all instances persistently. Add a helper function to locate the base event for a given instance.</content>
<parameter name="filePath">openspec/changes/modify-recurring-event-deletion-behavior/proposal.md