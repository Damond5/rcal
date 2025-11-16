# Checklist: Add Suggestions Overlay
# Change ID: add-suggestions-overlay

- [x] **Remove inline suggestions rendering**: In `src/ui.rs`, remove the code that renders suggestions directly below the end date field to eliminate overlap.
- [x] **Add overlay rendering logic**: Implement conditional rendering of a suggestions overlay in the add event popup, using `Clear` and `Block` widgets.
- [x] **Calculate overlay position**: Compute the overlay's `Rect` to position it to the right of the end date field, vertically aligned, ensuring no overlap with other fields.
- [x] **Style the overlay**: Apply appropriate styling (borders, colors) to the overlay for visual distinction, consistent with other popups.
- [x] **Handle overlay content**: Format and display the suggestions list within the overlay, handling multiple suggestions with scrolling if needed.
- [x] **Handle overlay boundary constraints**: Implement logic to reposition or resize the overlay if it would exceed terminal width/height, ensuring it remains visible.
- [x] **Write unit tests**: Add unit tests in appropriate test files (e.g., new file or extend existing) to test overlay positioning logic, conditional rendering, and styling.
- [x] **Write integration tests**: Update `tests/integration_test.rs` to include tests for the suggestions overlay behavior, such as visibility when suggestions are present and absence when not.
- [x] **Perform code review**: Use the @review subagent to review the implementation code for quality, best practices, and spec compliance, then implement all suggested improvements.
- [x] **Run all tests**: Execute `cargo test` to run unit and integration tests, ensuring all pass.
- [x] **Run lint and typecheck**: Execute `cargo check` and `cargo clippy` to ensure no warnings or errors from the changes.
- [x] **Update documentation**: Use the @docs-writer subagent to update @CHANGELOG.md and @README.md with details of the new suggestions overlay feature.