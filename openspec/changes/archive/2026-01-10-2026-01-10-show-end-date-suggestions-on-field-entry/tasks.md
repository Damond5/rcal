## 1. Implementation
- [ ] 1.1 Modify Tab handling in event_handling.rs to show suggestions when entering EndDate field (verified by manual testing showing overlay appears immediately)
- [ ] 1.2 Update get_date_suggestions function to return exactly 5 top common suggestions for empty input: Tomorrow, Next week, End of month, Next month, Same day (verified by unit tests)
- [ ] 1.3 Update tests to verify suggestions appear on field entry (integration tests pass)

## 2. Code Review
- [ ] 2.1 Perform code review using @code-review subagent on all implementation changes
- [ ] 2.2 Implement all suggestions from code review

## 3. Testing
- [ ] 3.1 Test that suggestions appear immediately when tabbing to end date field (manual verification)
- [ ] 3.2 Verify existing typing behavior still works (manual verification)
- [ ] 3.3 Test overlay positioning and navigation (manual verification)
- [ ] 3.4 Run full test suite and ensure no regressions (cargo test passes with 127 tests)

## 4. Documentation
- [ ] 4.1 Update README.md to reflect improved end date autocomplete behavior using @docs-writer subagent
- [ ] 4.2 Update CHANGELOG.md with new feature entry using @docs-writer subagent

## 5. Validation
- [ ] 5.1 Validate change against OpenSpec requirements (openspec validate passes)
- [ ] 5.2 Confirm UX improvement provides better discoverability (manual assessment)</content>
<parameter name="filePath">openspec/changes/2026-01-10-show-end-date-suggestions-on-field-entry/tasks.md