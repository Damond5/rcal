# Tasks for Enhance End Date Input Assistance

## Implementation Tasks

1. **[ ] Add date validation helper function**
   - Create `validate_date_input(input: &str) -> Result<NaiveDate, String>` in a new date_utils module
   - Implement DD/MM parsing with automatic year assumption
   - Return validation errors for malformed input

2. **[ ] Add auto-completion suggestions**
   - Create `get_date_suggestions(input: &str, start_date: NaiveDate) -> Vec<String>` function
   - Generate suggestions for common patterns: "tomorrow", "next week", "end of month", etc.
   - Include relative dates based on start_date context

3. **[ ] Extend App state for input assistance**
   - Add `date_input_error: Option<String>` field to App struct
   - Add `date_suggestions: Vec<String>` field for current suggestions
   - Add `show_date_suggestions: bool` flag for suggestion display

4. **[ ] Update UI rendering for end date field**
   - Modify end date input rendering in `ui.rs` to show validation errors in red
   - Add suggestion display below input field when available
   - Use conditional styling for error states

5. **[ ] Enhance event handling for end date input**
   - Update `PopupInputField::EndDate` handling in `event_handling.rs`
   - Add real-time validation on input changes
   - Implement suggestion navigation with Tab/Arrow keys
   - Clear errors on successful validation

## Testing Tasks

6. **[ ] Add unit tests for date validation**
    - Test DD/MM parsing with year assumption
    - Test invalid input error messages
    - Test edge cases (leap years, month boundaries)

7. **[ ] Add integration tests for input assistance**
    - Test auto-completion suggestions
    - Test error display and clearing

8. **[ ] Manual testing of user experience**
    - Verify visual feedback works correctly
    - Test with various date inputs and edge cases
    - Ensure no regression in existing functionality

## Validation Tasks

9. **[ ] Run full test suite**
    - Execute `cargo test` to ensure no regressions
    - Run integration tests with new functionality

10. **[ ] Build and test application**
    - Run `cargo build --release`
    - Manually test end date input in event creation/editing
    - Verify compatibility with existing date formats

11. **[ ] Code review using @review subagent**
    - Review all new and modified code for quality, best practices, potential bugs, edge cases, performance implications, and security considerations
    - Implement all suggestions from the review before proceeding

12. **[ ] Run comprehensive tests**
    - Execute unit tests, integration tests, and manual tests
    - Ensure no regressions and full coverage of new functionality

13. **[ ] Documentation updates using @docs-writer subagent**
    - Update CHANGELOG.md with details of the enhancement (e.g., "Added: Real-time validation and auto-completion for end date input in event creation/editing")
    - Update README.md if the feature introduces new user instructions