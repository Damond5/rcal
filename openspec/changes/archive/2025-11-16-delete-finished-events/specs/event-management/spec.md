# event-management Specification Delta

## ADDED Requirements

### Requirement: Automatic Cleanup of Finished Events
Application MUST support automatic deletion of events that finished more than 2 months prior to current date when `auto_cleanup_old_events` config option is enabled (default: true). Cleanup runs on every launch and does not require confirmation to allow batch operations.

#### Scenario: Cleanup on Launch
Given application launched with `auto_cleanup_old_events` enabled in config,
When events are loaded,
Then finished events older than 2 months are automatically deleted.

#### Scenario: Finished Event Identification
Given event with end_date,
When checking if finished before cutoff,
Then uses end_date for multi-day events and start_date for single-day events.

#### Scenario: Safe Cleanup Process
Given cleanup operation,
When deleting old events,
Then uses existing safe deletion with sync support to maintain consistency.

#### Scenario: Cleanup Failure Handling
Given cleanup operation encounters a deletion failure (e.g., file locked),
When attempting to delete an event,
Then logs the error and continues with remaining events.</content>
<parameter name="filePath">openspec/changes/delete-finished-events/specs/event-management/spec.md