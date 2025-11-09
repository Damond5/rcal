# Design for Preventing Duplicate Notifications

## Overview
The daemon currently reloads events and unconditionally clears the `notified` HashSet on any file change. This causes re-notification of events that haven't changed. To fix this, we compare the newly loaded events with the current events and only clear `notified` if they differ.

## Implementation Approach
- Load new events on file change detection
- Sort both current and new event vectors by a stable key (e.g., `(date, time, title)`) to ensure order-independent comparison
- Compare sorted new events vector with sorted current events vector using `==` (CalendarEvent derives PartialEq)
- If events changed, update events and clear notified
- If events unchanged, update events but preserve notified
- Add error handling around event loading to log failures and skip updates on errors, preserving the last good state
- This ensures deduplication persists across irrelevant file changes while allowing re-notification when events are genuinely modified

## Trade-offs
- **Pros**: Prevents annoying duplicate notifications from file system noise (e.g., sync metadata updates); robust against event reordering
- **Cons**: Slight performance cost from sorting and comparing event vectors on each reload (negligible for typical event counts); added error handling complexity
- **Alternatives Considered**: Persistent notified storage (more complex, unnecessary if daemon doesn't frequently restart)

## Dependencies
- Relies on CalendarEvent's PartialEq implementation for accurate comparison
- No external dependencies added</content>
<parameter name="filePath">openspec/changes/prevent-duplicate-notifications-on-file-changes/design.md