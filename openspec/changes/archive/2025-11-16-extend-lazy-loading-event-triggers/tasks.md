## 1. Core Implementation
- [x] 1.1 Identify locations in event_handling.rs where events are added, deleted, or edited
- [x] 1.2 Add calls to refresh lazy-loaded instances after successful event operations
- [x] 1.3 Update cache invalidation logic in persistence.rs to clear affected ranges when events change
- [x] 1.4 Ensure UI rendering in ui.rs triggers lazy loading refresh when needed

## 2. Cache Management
- [x] 2.1 Implement selective cache invalidation (only affected date ranges, not all cached instances)
- [x] 2.1.1 Define "affected ranges" as date ranges overlapping the modified event's recurrence pattern and cached periods
- [x] 2.2 Add logic to detect which recurring events are affected by changes (use event ID and recurrence rules)
- [x] 2.3 Optimize to avoid unnecessary re-generation of unchanged instances

## 3. Testing and Validation
- [x] 3.1 Write unit tests for cache invalidation logic on event changes (test selective invalidation, performance)
- [x] 3.2 Write integration tests for UI updates after add/delete/edit operations (verify lazy loading triggers correctly)
- [x] 3.3 Write unit tests for lazy loading refresh functions (edge cases: invalid dates, large ranges, cache hits/misses)
- [x] 3.4 Write integration tests for end-to-end workflow (add recurring event, verify display; edit recurrence, verify changes; delete event, verify removal)
- [x] 3.5 Write tests for error handling during invalidation (e.g., simulate disk write failures, ensure graceful degradation)
- [x] 3.6 Test performance impact of frequent invalidations (benchmark with 100+ recurring events)
- [x] 3.7 Verify recurring events appear/disappear correctly after operations
- [x] 3.8 Test edge cases (editing recurrence pattern mid-series, deleting base event with many instances)
- [x] 3.9 Run full test suite and fix any regressions

## 4. Code Review and Quality Assurance
- [x] 4.1 Perform code review using `@review` subagent on all modified files (persistence.rs, event_handling.rs, ui.rs)
- [x] 4.2 Implement all suggestions from the code review
- [x] 4.3 Fix any warnings or clippy issues immediately
- [x] 4.4 Run full build and test suite; address regressions

## 5. Documentation and Release
- [x] 5.1 Update `README.md` using `@docs-writer` subagent if user-facing behavior changes (e.g., note improved responsiveness after event operations)
- [x] 5.2 Update `CHANGELOG.md` using `@docs-writer` subagent per semver (likely a minor version bump for enhanced lazy loading)</content>
<parameter name="filePath">openspec/changes/extend-lazy-loading-event-triggers/tasks.md