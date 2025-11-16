## 1. Add SelectingRecurrence InputMode
- [x] Add InputMode::SelectingRecurrence variant to src/app.rs
- [x] Add selected_recurrence_index: usize to App struct, initialize to 0
- [x] Update input mode handling in src/event_handling.rs

## 2. Implement Dropdown Rendering
- [x] Modify ui.rs to render recurrence options list when in SelectingRecurrence mode
- [x] Use ratatui::widgets::List for the dropdown display
- [x] Position list within the Recurrence field's area
- [x] Update UI hints for SelectingRecurrence mode

## 3. Handle Dropdown Navigation
- [x] Add key handling for j/k navigation in SelectingRecurrence mode
- [x] Implement Enter to confirm selection, Esc to cancel
- [x] Implement pre-selection: On mode entry, set selected_recurrence_index based on popup_event_recurrence
- [x] Update app state to store selected recurrence option

## 4. Integrate with Existing Flow
- [x] Modify Tab navigation to enter SelectingRecurrence when reaching recurrence field
- [x] Modify EditingEventPopup to disable Char input when selected_input_field == Recurrence
- [x] Ensure selection updates the popup_event_recurrence field
- [x] Test transition back to EditingEventPopup after selection

## 5. Write Tests
- [x] Write unit tests for new functions (e.g., index mapping, mode transitions)
- [x] Write integration tests for dropdown navigation and selection
- [x] Test pre-selection for edits and invalid recurrence strings
- [x] Test edge cases: empty selection, mode transitions
- [x] Ensure no regression in existing event creation flow

## 6. Code Review
- [x] Use @review subagent to review the implementation
- [x] Implement all suggestions from the code review

## 7. Run All Tests
- [x] Run cargo test to ensure all tests pass
- [x] Run integration tests with ratatui::backend::TestBackend

## 8. Update Documentation
- [x] Update CHANGELOG.md with the new feature using @docs-writer subagent
- [x] Update README.md with recurrence dropdown details using @docs-writer subagent

## 9. Validate and Lint
- [x] Run cargo clippy and fix any warnings
- [x] Run openspec validate add-recurrence-dropdown --strict</content>
<parameter name="filePath">openspec/changes/add-recurrence-dropdown/tasks.md