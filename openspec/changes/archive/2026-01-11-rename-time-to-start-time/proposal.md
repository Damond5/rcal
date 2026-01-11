# Change: Rename Time to Start Time

## Why
The current label "Time" is ambiguous for events that span multiple days or have both start and end times. Renaming to "Start Time" clarifies that this field represents the event's start time, improving user understanding and consistency with "End Time".

## What Changes
- Rename UI label "Time" to "Start Time" in src/ui.rs
- **BREAKING**: Rename PopupInputField::Time to PopupInputField::StartTime in src/app.rs
- Rename popup_event_time field to popup_event_start_time in src/app.rs
- Update all references to event.time to event.start_time across the codebase
- Update event-management spec to use "start time" consistently
- Update EVENT_FORMAT.md to use "Start Time" terminology

## Impact
- Affected specs: event-management, EVENT_FORMAT.md
- Affected code: src/ui.rs, src/app.rs, src/event_handling.rs, src/persistence.rs, src/daemon.rs
- Breaking: Yes (API change for PopupInputField enum variant)

## Scope Clarification
This change includes the complete migration from the deprecated `event.time` field to `event.start_time` field across the entire codebase. The goal is to fully clean up all deprecated field references after successful migration, ensuring consistent terminology throughout the application and its documentation.
