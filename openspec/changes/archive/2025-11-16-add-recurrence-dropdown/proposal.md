# Change: Add Recurrence Dropdown

## Why
Users struggle to remember the exact recurrence syntax (none/daily/weekly/monthly/yearly) when creating events, leading to errors or frustration. A dropdown selection prevents invalid inputs and improves usability.

## What Changes
- Add a new input mode for recurrence selection with keyboard navigation
- Modify the event creation/editing popup to show a selectable list when recurrence field is active
- Ensure only valid recurrence options can be chosen, eliminating free-text errors

## Impact
- Affected specs: event-management (new UI interaction for recurrence input)
- Affected code: src/ui.rs (rendering), src/event_handling.rs (input handling), src/app.rs (new InputMode)
- No breaking changes to existing functionality or file formats</content>
<parameter name="filePath">openspec/changes/add-recurrence-dropdown/proposal.md