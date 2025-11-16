# Tasks for Modifying Recurring Event Deletion Behavior

1. **Update Event Management Spec**: Modify the "Instance vs Base Event Deletion" requirement in event-management/spec.md to reflect that deleting a recurring instance now deletes the entire series persistently. Add new requirements for error handling and edge cases.

2. **Modify Deletion Logic**: Update the delete confirmation handler in event_handling.rs to detect recurring instances and delete the base event instead of just the instance.

3. **Add Helper Function**: Create a helper function to find the base event for a given recurring instance.

4. **Create Unit Tests**: Add unit tests for the helper function, deletion logic, and edge cases (e.g., deletion failures, instance detection).

5. **Update Integration Tests**: Ensure existing tests reflect the new behavior and add new tests for the series deletion functionality, including persistence across restarts.

6. **Update CHANGELOG.md**: Add an entry under "Changed" in the [Unreleased] section documenting the breaking change in recurring event deletion behavior.

7. **Update README.md**: Document the new deletion behavior in the user-facing sections (e.g., event management features).

8. **Perform Code Review**: Use the @review subagent to review the implementation code and implement feedback.

9. **Full Build and Test**: Fully build the project and run all tests to ensure no regressions.</content>
<parameter name="filePath">openspec/changes/modify-recurring-event-deletion-behavior/tasks.md