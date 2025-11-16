# Change: Extend Lazy Loading Triggers for Event Changes

## Why
Currently, lazy loading for recurring events only triggers when viewing new date ranges. However, when events are added, deleted, or edited, the displayed events may change (e.g., new recurring events appear, deleted ones disappear, edited recurrence patterns alter instances). The lazy loading system needs to refresh or invalidate caches to ensure accurate display after these operations.

## What Changes
- Extend lazy loading triggers to include event modification actions (add, delete, edit)
- Add cache invalidation or refresh logic when events change
- Ensure UI updates correctly reflect changes after event operations
- **BREAKING**: May affect performance if invalidation causes re-generation of many instances

## Impact
- Affected specs: event-management
- Affected code: src/event_handling.rs (event operations), src/ui.rs (display updates), src/persistence.rs (cache management)

## Risks and Mitigations
- Performance: Frequent invalidations could increase CPU usage by 20-50% for calendars with 100+ recurring events during edits → Mitigation: Implement selective invalidation to only clear affected ranges, not all cached instances
- UI Responsiveness: Cache invalidation during UI rendering may cause brief lag → Mitigation: Defer invalidation to post-operation and debounce rapid changes
- Error Handling: Invalidation failures (e.g., due to I/O errors) could leave stale cache → Mitigation: Log errors and trigger full refresh on next UI render</content>
<parameter name="filePath">openspec/changes/extend-lazy-loading-event-triggers/proposal.md