## Context
The current implementation pre-generates all recurring event instances in memory upon loading, limited to 365 days. This works for short-term planning but fails for long-term recurring events like birthdays that should persist indefinitely. The app loads events once on startup and keeps them in memory.

## Goals / Non-Goals
- Goals: Enable yearly events to show indefinitely, reduce memory usage for long-term events, maintain fast UI responsiveness
- Non-Goals: Change event persistence format, add database storage, support complex recurrence rules

## Decisions
- Decision: Implement lazy generation where base events are loaded eagerly, but instances are generated on-demand for visible date ranges
- Alternatives considered: Extend generation period (simple but still finite), add recurrence end dates (adds complexity), pre-generate all possible instances (memory intensive)

## Risks / Trade-offs
- Performance: On-demand generation may cause slight UI lag when scrolling to new date ranges → Mitigation: Cache generated instances per session
- Memory: Reduced memory for short-term events, but potential duplication if same instances generated multiple times → Mitigation: Implement session-level caching
- Complexity: More complex loading/rendering logic → Mitigation: Encapsulate in dedicated functions

## Migration Plan
No data migration needed. Existing events will work as base events are unchanged. Rollback by reverting code changes.

## Open Questions
- Resolved: Generate instances 1 year ahead and behind the visible date range to balance responsiveness and memory usage.</content>
<parameter name="filePath">openspec/changes/implement-lazy-loading-recurring-events/design.md