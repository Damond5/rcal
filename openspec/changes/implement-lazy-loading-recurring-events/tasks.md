## 1. Core Implementation
- [ ] 1.1 Modify `load_events_from_path()` in persistence.rs to load only base events without pre-generating instances
- [ ] 1.2 Add `generate_instances_for_range(base_event, start_date, end_date)` function in persistence.rs
- [ ] 1.3 Update `CalendarEvent` struct or add helper to track generated ranges per base event
- [ ] 1.4 Modify UI rendering in ui.rs to call instance generation for visible date ranges
- [ ] 1.5 Update event handling in event_handling.rs to work with lazy-loaded instances

## 2. Caching and Performance
- [ ] 2.1 Implement session-level cache for generated instances to avoid regeneration
- [ ] 2.2 Add logic to expand generation range when navigating to new dates
- [ ] 2.3 Optimize instance generation to avoid duplicates

## 3. Testing and Validation
- [ ] 3.1 Update existing tests in persistence.rs to work with lazy loading
- [ ] 3.2 Add integration tests for lazy loading behavior
- [ ] 3.3 Test performance with large numbers of recurring events
- [ ] 3.4 Verify yearly events appear indefinitely in calendar view
- [ ] 3.5 Add unit tests for `generate_instances_for_range()` (edge cases: invalid dates, large ranges, cache hits/misses)
- [ ] 3.6 Add integration tests for UI scrolling (e.g., navigate to 2030, verify yearly events appear without lag)
- [ ] 3.7 Test concurrency (e.g., sync operations during lazy generation)
- [ ] 3.8 Benchmark performance (e.g., load time with 1000 recurring events, UI responsiveness)

## 4. Documentation and Cleanup
- [ ] 4.1 Update comments in code explaining lazy loading
- [ ] 4.2 Run full test suite and fix any regressions
- [ ] 4.3 Update README if user-facing behavior changes

## 5. Code Review and Quality Assurance
- [ ] 5.1 Perform code review using `@review` subagent on all modified files
- [ ] 5.2 Fix any warnings or clippy issues immediately
- [ ] 5.3 Run full build and test suite; address regressions

## 6. Documentation and Release
- [ ] 6.1 Update `README.md` if user-facing behavior changes (e.g., note indefinite event display)
- [ ] 6.2 Update `CHANGELOG.md` per semver (likely a minor version bump for new feature)</content>
<parameter name="filePath">openspec/changes/implement-lazy-loading-recurring-events/tasks.md