# event-management Specification

## Purpose
TBD - created by archiving change add-agent-guidelines-specs. Update Purpose after archive.
## Requirements
### Requirement: Flexible Time Input Support
Time inputs MUST support multiple formats (HH:MM, HH, H) with automatic normalization.

#### Scenario: Time Format Acceptance
Given time input in various formats,
When parsing,
Then all valid formats are accepted and normalized.

### Requirement: End Date Format Handling
End date inputs MUST use DD/MM format with automatic year assumption.

#### Scenario: Date Input Parsing
Given DD/MM date,
When assuming current/next year,
Then date is correctly interpreted.

### Requirement: Safe Event Deletion
Event deletion MUST require confirmation to prevent accidental data loss.

#### Scenario: Confirmation Dialog
Given delete action,
When confirming deletion,
Then event is safely removed.

### Requirement: Context-Aware Event Addition
Events MUST be addable from view popup without losing current context.

#### Scenario: Contextual Adding
Given view popup,
When adding event,
Then returns to view with new event shown.

### Requirement: Markdown-Based Persistence
Events MUST be stored as individual markdown files in user's home directory.

#### Scenario: File Storage
Given event data,
When saving,
Then creates markdown file in ~/calendar/.

### Requirement: Title-Based Filenames
Filenames MUST be based on sanitized event titles with duplicate handling.

#### Scenario: Filename Generation
Given event title,
When sanitizing (spaces to underscores, alphanumeric+underscore),
Then creates valid filename.

#### Scenario: Duplicate Handling
Given duplicate titles,
When saving,
Then appends number (e.g., title_1.md).

### Requirement: Multi-Day Event Support
Events MUST support start/end dates and times for spanning multiple days.

#### Scenario: Multi-Day Creation
Given start and end dates,
When creating event,
Then spans multiple days in calendar.

### Requirement: UI Fields for Multi-Day
Event creation/editing popup MUST include input fields for end date and end time.

#### Scenario: Field Availability
Given event popup,
When editing,
Then end date/time fields are present.

### Requirement: Event Format Specification
Events MUST follow format documented in EVENT_FORMAT.md.

#### Scenario: Format Compliance
Given event file,
When following spec,
Then parsing works correctly.

### Requirement: Real-Time View Updates
View popup MUST refresh automatically after adding/deleting events.

#### Scenario: Post-Operation Refresh
Given add/delete operation,
When returning to view,
Then list shows updated events.

### Requirement: Daemon Notification Mode
Application MUST support `--daemon` flag for background notifications.

#### Scenario: Daemon Launch
Given --daemon flag,
When running,
Then monitors events in background.

### Requirement: Timed Event Notifications
Timed events MUST trigger desktop notifications 30 minutes before.

#### Scenario: Notification Timing
Given timed event in 30 minutes,
When daemon running,
Then notification appears.

### Requirement: All-Day Event Notifications
All-day events MUST trigger notifications midday the day before.

#### Scenario: All-Day Timing
Given all-day event tomorrow,
When midday today,
Then notification appears.

### Requirement: Notification Deduplication
Notifications MUST be sent only once per event, even when file changes occur without altering event data.

#### Scenario: Unchanged Events on File Change
Given daemon monitoring events,
When file changes but events remain identical,
Then notifications are not re-sent for previously notified events.

#### Scenario: Changed Events on File Change
Given daemon monitoring events,
When file changes alter event data,
Then notifications are re-sent appropriately for modified events.

### Requirement: File Watching Integration
Daemon MUST use `notify` crate to monitor ~/calendar directory for updates.

#### Scenario: Real-Time Monitoring
Given file changes,
When daemon watching,
Then reloads events automatically.

### Requirement: Mandatory Event Titles
Events MUST require non-empty titles for meaningful identification.

#### Scenario: Title Validation
Given empty title,
When creating event,
Then creation fails.

### Requirement: Default End Date/Time
Unspecified end date/time MUST default to start date/time.

#### Scenario: Default Values
Given no end specified,
When creating event,
Then end equals start.

### Requirement: Single-Day Event Storage
Single-day events MUST have end_date set to start_date and omit ' to ' in file.

#### Scenario: Storage Format
Given single-day event,
When saving,
Then no ' to ' in markdown.

### Requirement: All-Day Event Handling
Events without time MUST be treated as all-day with "All day" display.

#### Scenario: All-Day Display
Given event without time,
When viewing,
Then shows "All day".

#### Scenario: All-Day Storage
Given all-day event,
When saving,
Then time set to "all-day" in file.

### Requirement: Title-Based Uniqueness
Event titles MUST serve as unique identifiers for file operations.

#### Scenario: Title-Based Operations
Given event title,
When performing operations,
Then uses title for identification.

### Requirement: Configurable Calendar Directory
App MUST support configurable calendar directory via constructor.

#### Scenario: Custom Directory
Given calendar directory path,
When initializing,
Then uses specified directory.

### Requirement: Robust Event Reloading
Event reloading on file changes MUST handle load errors gracefully without crashing the daemon.

#### Scenario: Load Error Handling
Given file change detection,
When event loading fails (e.g., corrupted files),
Then daemon logs the error and continues with the last good event state.</content>
<parameter name="filePath">openspec/changes/prevent-duplicate-notifications-on-file-changes/specs/event-management/spec.md

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
Then logs the error and continues with remaining events.

### Requirement: Recurring Event Support
Events MUST support recurring patterns (daily, weekly, monthly, yearly) with automatic instance generation.

#### Scenario: Recurrence Pattern Creation
Given recurrence pattern (daily/weekly/monthly/yearly),
When creating event,
Then base event is saved with recurrence metadata.

### Requirement: In-Memory Instance Generation
Recurring event instances MUST be generated in memory immediately upon event creation/loading, not persisted to disk.

#### Scenario: Instance Display
Given recurring event created,
When viewing future dates,
Then instances appear without reload.

### Requirement: Instance vs Base Event Deletion
Deleting any recurring instance MUST delete the entire recurring series persistently by removing the base event file, ensuring the deletion persists across application restarts.

#### Scenario: Instance Deletion
Given recurring instance,
When deleting,
Then base event file is deleted and all instances are removed persistently.

#### Scenario: Base Event Deletion
Given base recurring event,
When deleting,
Then all instances are removed from memory and the series is deleted persistently.

### Requirement: Recurring Event Deletion Error Handling
Deletion of recurring events MUST handle errors gracefully, notifying the user of failures and maintaining data consistency.

#### Scenario: Deletion Failure Handling
Given deletion operation fails (e.g., file permissions),
When attempting to delete recurring series,
Then logs error, notifies user, and prevents partial state.

### Requirement: Recurring Event Deletion Confirmation
Deletion of recurring instances MUST include clear confirmation indicating that the entire series will be deleted.

#### Scenario: Series Deletion Confirmation
Given recurring instance deletion,
When confirming,
Then dialog clearly states series deletion.

