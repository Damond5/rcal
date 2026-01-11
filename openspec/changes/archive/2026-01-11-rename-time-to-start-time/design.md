## Context
Straightforward rename of "time" to "start time" for clarity. This is a simple refactoring with no architectural implications.

## Goals
- Improve UI clarity by distinguishing start time from end time
- Align terminology across UI labels, code, and documentation
- Complete the field migration from generic "time" to specific "start time"

## Non-Goals
- Changing the underlying data model structure
- Modifying validation logic or business rules
- Adding new functionality or features

## Decisions
The naming convention "Start Time" was chosen over alternatives (e.g., "Begin Time", "From Time") because:
- It is the most widely recognized and intuitive terminology in calendar applications
- It pairs naturally with "End Time" for consistent UI labeling
- It clearly distinguishes the field from end time without requiring additional context
- The capitalization follows standard English title case conventions

## Migration Plan
1. Rename PopupInputField::Time to PopupInputField::StartTime in enum definition
2. Rename popup_event_time to popup_event_start_time in App struct
3. Update all references across files
4. Update spec documentation (event-management spec and EVENT_FORMAT.md)

## Open Questions
- None
