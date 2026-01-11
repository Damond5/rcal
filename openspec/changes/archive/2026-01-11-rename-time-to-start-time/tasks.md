## 1. Implementation
- [x] 1.1 Rename PopupInputField::Time to PopupInputField::StartTime in src/app.rs (depends on -)
- [x] 1.2 Rename popup_event_time field to popup_event_start_time in src/app.rs (depends on 1.1)
- [x] 1.3 Update UI label "Time" to "Start Time" in src/ui.rs (depends on 1.1)
- [x] 1.4 Update all references in src/event_handling.rs (depends on 1.2)
- [x] 1.5 Update all references in src/persistence.rs (depends on 1.2)
- [x] 1.6 Update all references in src/daemon.rs (depends on 1.2)
- [x] 1.7 Update event.time to event.start_time in src/event.rs and src/event_handling.rs (depends on 1.2)

## 2. Testing
- [x] 2.1 Run existing tests to verify no regressions
- [x] 2.2 Run cargo clippy to check for warnings
- [x] 2.3 Run cargo fmt to ensure formatting

## 3. Documentation
- [x] 3.1 Update event-management spec with "start time" terminology
- [x] 3.2 EVENT_FORMAT.md already uses "Start Time" terminology (verified)

## 4. Documentation Changes
- [x] 4.1 EVENT_FORMAT.md already uses "Start Time" in field definition (verified)
- [x] 4.2 EVENT_FORMAT.md already uses "### Start Time" (verified)
- [x] 4.3 EVENT_FORMAT.md already uses "**Start Time**:" in example events (verified)

## 5. Code Review & Quality
- [x] 5.1 Review implementation using @code-review subagent
- [x] 5.2 Implement all code review suggestions (completed as part of fixing critical issues)
- [x] 5.3 Update README.md using @docs-writer subagent
- [x] 5.4 Update CHANGELOG.md using @docs-writer subagent
