# Change: Implement Lazy Loading for Recurring Events

## Why
Yearly recurring events like birthdays currently only generate instances for the next 365 days, causing them to disappear after one year. Users expect these events to persist indefinitely without manual recreation.

## What Changes
- Change recurring event instance generation from eager (immediate upon loading) to lazy (on-demand when viewing dates)
- Modify event loading to store only base recurring events, not pre-generated instances
- Add on-demand instance generation for date ranges during UI rendering
- **BREAKING**: Changes how recurring events are handled in memory, may affect performance and memory usage

## Impact
- Affected specs: event-management
- Affected code: src/persistence.rs (loading/generation), src/ui.rs (rendering), src/event_handling.rs (event management)</content>
<parameter name="filePath">openspec/changes/implement-lazy-loading-recurring-events/proposal.md