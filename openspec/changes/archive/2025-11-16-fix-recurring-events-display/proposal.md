# Change: Fix Recurring Events Display

## Why
Recurring events (daily, weekly, monthly) only show on the creation day and do not display instances on subsequent dates until the application reloads events from disk. This breaks the expected behavior of recurring events appearing automatically on future dates.

## What Changes
- Modify event addition logic to generate recurring instances in memory immediately after creating a recurring event.
- Update event deletion logic to handle recurring instances without deleting the base event file.
- Add requirements for recurring event support to the event-management spec.
- Add tests for recurring event instance generation and display.

## Impact
- Affected specs: event-management
- Affected code: src/event_handling.rs, src/persistence.rs, tests
- No breaking changes; improves existing functionality.