## Why

The "Time" field in the event model represents both start time AND end time (when present), similar to how the "Date" field represents both start date and end date. The previous naming "Start Time" was semantically misleading because:
- It's not just the start time - users input both start and end times in this field
- Creates inconsistency with the "Date" field naming pattern (both are single fields representing time spans)
- This reverses the decision made in "2026-01-11-rename-time-to-start-time"

## What Changes

Rename "Start Time" to "Time" throughout the entire codebase:
- UI labels in src/ui.rs
- User-facing documentation (README.md, EVENT_FORMAT.md)
- Internal identifiers (PopupInputField::StartTime → PopupInputField::Time, popup_event_start_time → popup_event_time)
- File format specification labels
- Test assertions and references
- Remove the old breaking change note from CHANGELOG.md

## Capabilities

### New Capabilities
None - this is a pure rename/refactoring with no functional changes.

### Modified Capabilities
None - this is a pure rename/refactoring with no functional changes.

## Impact

- Breaking change for users (new event format uses "Time" instead of "Start Time")
- No backwards compatibility required per user request
- Internal data model keeps start_time/end_time semantics for actual time tracking
