# Delete Finished Events

## Why
Over time, calendar applications accumulate many past events that are no longer relevant. Users may want to automatically clean up old events to reduce clutter and improve performance. The 2-month threshold provides a reasonable buffer while ensuring recent past events remain accessible. This feature runs automatically on every launch when enabled via configuration to maintain ongoing cleanliness.

## What Changes
- Add a configuration option `auto_cleanup_old_events` in the TOML config file (default: true) to enable automatic cleanup on every launch
- Implement logic to identify finished events based on end_date (or start_date for single-day events)
- Use current date minus 2 months as the cutoff (using local timezone)
- Integrate with existing persistence layer for safe deletion with sync support
- Ensure cleanup only affects valid event files and handles errors gracefully

## Impact
- Reduces storage usage by removing old event files
- Improves performance by reducing the number of events to load and display
- Maintains data integrity by only deleting truly finished events
- Preserves user data by keeping events within the 2-month window
- Affected specs: event-management
- Affected code: main.rs (CLI), persistence.rs (cleanup logic), sync.rs (cross-device consistency)</content>
<parameter name="filePath">openspec/changes/delete-finished-events/proposal.md