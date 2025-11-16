# Prevent Invalid Date Acceptance

## Summary
Modify event creation/editing to validate date inputs on submission, preventing acceptance of invalid dates and displaying error messages similar to title validation, instead of proceeding with invalid data or closing the popup silently.

## Motivation
Currently, invalid time inputs cause the popup to close silently without feedback, and invalid end dates allow submission despite real-time validation errors. This leads to poor user experience and potential data inconsistencies. By adding submission-time validation with clear error messages, users get immediate feedback and cannot accidentally create events with invalid dates.

## Impact
- Improves user experience by providing clear feedback for invalid date inputs
- Prevents creation of events with invalid dates
- Maintains consistency with existing title validation behavior
- No breaking changes to existing functionality

## Implementation Approach
Add validation checks in the Enter key handler for EditingEventPopup mode, checking for date_input_error and invalid time parsing, setting error_message and returning early similar to title validation. Error messages will persist until the user corrects the invalid input to provide clear feedback on resubmission.</content>
<parameter name="filePath">openspec/changes/prevent-invalid-date-acceptance/proposal.md