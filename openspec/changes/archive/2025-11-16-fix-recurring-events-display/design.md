# Design Decisions for Recurring Events Display Fix

## Architectural Overview
- Recurring instances are generated dynamically in memory upon event creation/loading, not persisted to disk. This avoids file bloat while ensuring immediate display.
- Instances are tied to the base event by title/ID, allowing deletion of individual instances without affecting the base or others.

## Key Decisions
- **Instance Generation**: Use a simple loop to create instances up to a reasonable future limit (e.g., 1 year) to prevent unbounded growth. Regenerate on app load or event changes.
- **Memory Management**: Store instances in a map/hash keyed by base event ID. Clear on reload to avoid leaks.
- **Integration**: Hook into existing event loading in `persistence.rs` and UI rendering in `ui.rs`. Ensure sync operations (push/pull) only affect base events.

## Considerations
- **Performance**: For calendars with many recurrences, consider lazy generation or caching.
- **Edge Cases**: Handle invalid recurrence patterns gracefully; ensure multi-day recurrences work correctly.
- **Security**: No new file exposures; rely on existing safe deletion.