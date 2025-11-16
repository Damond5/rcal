# event-management Specification Delta

## MODIFIED Requirements

### Requirement: Lazy Instance Generation
Recurring event instances MUST be generated in memory on-demand when viewing date ranges or when events are modified, not persisted to disk. Instances are cached per session to avoid regeneration but invalidated when events change.

#### Scenario: Instance Display on View
Given recurring event created,
When viewing future dates,
Then instances are generated and displayed for that range.

#### Scenario: Cache Invalidation on Event Change
Given cached recurring instances,
When event is added, deleted, or edited,
Then affected cached instances are invalidated and refreshed on next display.

#### Scenario: UI Update After Operations
Given event modification operation,
When operation completes,
Then lazy loading triggers to update displayed events accurately.

#### Scenario: Selective Invalidation
Given multiple recurring events,
When one event is modified,
Then only instances for that event within cached date ranges are invalidated, not all cached instances.

#### Scenario: Error Handling on Invalidation Failure
Given cache invalidation attempt,
When it fails (e.g., due to I/O error),
Then log error and trigger full refresh on next UI render.</content>
<parameter name="filePath">openspec/changes/extend-lazy-loading-event-triggers/specs/event-management/spec.md