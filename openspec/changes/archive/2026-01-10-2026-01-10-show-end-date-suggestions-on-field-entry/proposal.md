# Change: Show End Date Suggestions on Field Entry

## Why
Currently, autocomplete suggestions for the end date field only appear after the user starts typing. Users would benefit from seeing common date suggestions immediately when they enter the end date field, providing better discoverability and guidance for date input options.

## What Changes
- Modify the end date field entry behavior to show suggestions immediately when tabbing into the field, even when empty
- Display the top 5 most common relative date suggestions in priority order: 1) Tomorrow, 2) Next week, 3) End of month, 4) Next month, 5) Same day when the field is first entered
- Maintain existing behavior of updating suggestions as the user types
- Keep the overlay positioning and navigation unchanged

## Impact
- Affected specs: event-management (suggestion display timing)
- Affected code: src/event_handling.rs (Tab handling for EndDate field entry), src/date_utils.rs (get_date_suggestions function for empty input handling)
- Affected tests: integration_test.rs (suggestion display tests), date_utils.rs unit tests
- No breaking changes to existing functionality</content>
<parameter name="filePath">openspec/changes/2026-01-10-show-end-date-suggestions-on-field-entry/proposal.md