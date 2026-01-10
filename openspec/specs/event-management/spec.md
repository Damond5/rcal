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
End date inputs MUST use DD/MM format with automatic year assumption and provide comprehensive real-time validation, auto-completion suggestions, and visual feedback while maintaining the required DD/MM format. Suggestions MUST include expanded relative dates, fuzzy matching, enhanced partial completion, user experience improvements, validation feedback, and performance optimizations. Suggestions MUST appear immediately when entering the end date field, showing common relative date options even when the field is empty. Digit-based inputs MUST suggest dates using the current month, and edge cases around end of month MUST be handled by providing valid alternatives or adjustments.

#### Scenario: Suggestion Display on Field Entry
Given end date input field,
When user enters the field via Tab navigation,
Then shows top 5 common date suggestions below field including tomorrow, next week, end of month, next month, and same day.

#### Scenario: Comprehensive Suggestion Display
Given end date input field,
When user starts typing,
Then shows relevant date suggestions below field including relative dates, fuzzy matches, and partial completions.

#### Scenario: Suggestion Display on Field Entry
Given end date input field,
When user enters the field via Tab navigation,
Then shows top 5 common date suggestions below field including Tomorrow, Next week, End of month, Next month, and Same day.

#### Scenario: Enhanced Suggestion Selection
Given displayed suggestions,
When user presses Tab or arrow keys,
Then cycles through available suggestions with descriptions.

#### Scenario: Expanded Relative Date Suggestions
Given event with start date,
When entering end date,
Then suggestions include comprehensive relative dates like "tomorrow", "next week", "next monday", "in 3 days", "next month", "end of year", "1 day", "1 week", "2 weeks", "1 month", "same day".

#### Scenario: Fuzzy Matching Support
Given partial or mistyped input,
When matching suggestions,
Then supports fuzzy matching for typos and partial word matching beyond prefixes.

#### Scenario: Enhanced Date Completion
Given partial date input,
When completing,
Then supports advanced patterns like "15/" completion, " /10" completion, and common date patterns.

#### Scenario: Suggestion Descriptions
Given displayed suggestions,
When showing options,
Then includes descriptive text alongside dates (e.g., "Tomorrow (02/11)").

#### Scenario: Validation Feedback in Suggestions
Given suggestions with potential invalid dates,
When displaying,
Then shows validation feedback and suggests corrections for invalid inputs.

#### Scenario: Performance Optimization
Given suggestion generation,
When processing,
Then limits to reasonable number of suggestions and handles edge cases efficiently.

#### Scenario: Digit Input Completion with Multiple Months
Given end date input field,
When user types digits representing a day (e.g., "12"),
Then suggests the date for current month, next month, and month after next (e.g., "12/01", "12/02", "12/03" if current month is January).

#### Scenario: Edge Case Handling for Invalid Day Numbers
Given end date input field,
When user types a day number that would make an invalid date for the current month (e.g., "32" in January),
Then provides valid alternatives, such as the last day of the current month or the equivalent day in the next month if applicable.

#### Scenario: Real-Time Suggestion Updates on Each Character
Given end date input field,
When user types or deletes characters,
Then suggestions update immediately after each input change, ensuring responsiveness.

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
Events MUST support recurring patterns (daily, weekly, monthly, yearly) with automatic instance generation for indefinite periods.

#### Scenario: Recurrence Pattern Creation
Given recurrence pattern (daily/weekly/monthly/yearly),
When creating event,
Then base event is saved with recurrence metadata.

### Requirement: Lazy Instance Generation
Recurring event instances MUST be generated in memory on-demand when viewing date ranges, not persisted to disk. Instances are cached per session to avoid regeneration.

#### Scenario: Instance Display
Given recurring event created,
When viewing future dates,
Then instances are generated and displayed for that range.

#### Scenario: Memory Efficiency
Given many recurring events,
When viewing limited date range,
Then only instances for visible dates are generated.

#### Scenario: Cache Reuse
Given previously generated instances,
When revisiting the same date range,
Then cached instances are reused without regeneration.

#### Scenario: Notification Integration
Given lazy loading,
When daemon checks for upcoming events,
Then instances are generated for relevant notification periods without pre-loading all events.</content>
<parameter name="filePath">openspec/changes/implement-lazy-loading-recurring-events/specs/event-management/spec.md

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

### Requirement: Recurrence Dropdown Selection
The event creation/editing popup MUST provide a selectable dropdown for recurrence options instead of free-text input when the recurrence field is active.

#### Scenario: Dropdown Activation
Given the recurrence field is selected via Tab navigation,
When the field becomes active,
Then display a list of valid recurrence options (none, daily, weekly, monthly, yearly) for selection.

#### Scenario: Keyboard Navigation
Given the recurrence dropdown is active,
When user presses j/k keys,
Then highlight the next/previous option in the list.

#### Scenario: Selection Confirmation
Given a recurrence option is highlighted,
When user presses Enter,
Then set the recurrence field to the selected option and return to normal editing mode.

#### Scenario: Selection Cancellation
Given the recurrence dropdown is active,
When user presses Esc,
Then keep the recurrence field unchanged and return to normal editing mode.

### Requirement: Invalid Recurrence Prevention
Event creation MUST only accept valid recurrence values, preventing user errors from free-text input.

#### Scenario: Dropdown Enforcement
Given event creation with recurrence dropdown,
When saving the event,
Then only predefined options are possible, eliminating invalid recurrence strings.</content>
<parameter name="filePath">openspec/changes/add-recurrence-dropdown/specs/event-management/spec.md

### Requirement: Date Validation on Submission
Event creation and editing MUST validate date inputs on submission, preventing acceptance of invalid dates and displaying error messages instead of proceeding or closing silently.

#### Scenario: Invalid Time Input Rejection
Given event popup with invalid time input,
When submitting with Enter,
Then displays error message and prevents submission.

#### Scenario: Invalid End Date Rejection
Given event popup with invalid end date input,
When submitting with Enter,
Then displays error message and prevents submission.

#### Scenario: Mixed Valid/Invalid Input Rejection
Given event popup with valid time but invalid end date,
When submitting with Enter,
Then displays error message and prevents submission.

#### Scenario: Valid Date Acceptance
Given event popup with valid date inputs,
When submitting with Enter,
Then proceeds with event creation/editing normally.</content>
<parameter name="filePath">openspec/changes/prevent-invalid-date-acceptance/specs/event-management/spec.md

### Requirement: February 29th Leap Year Fallback for Yearly Recurrence
Yearly recurring events with February 29th as the base date MUST automatically fall back to February 28th in non-leap years, ensuring events continue occurring annually.

#### Scenario: February 29th Fallback to February 28th
Given a yearly recurring event with February 29th as the base date,
When generating instances for a non-leap year,
Then the instance occurs on February 28th instead of being skipped.

#### Scenario: February 29th Occurs on Leap Years
Given a yearly recurring event with February 29th as the base date,
When generating instances for a leap year,
Then the instance occurs on February 29th.

#### Scenario: Non-February 29th Yearly Events Unaffected
Given a yearly recurring event with a base date other than February 29th,
When generating instances across all years,
Then instances occur on the same day and month annually.

#### Scenario: February 29th Multi-Day Event Fallback
Given a yearly recurring multi-day event starting February 29th and spanning multiple days,
When generating instances for a non-leap year,
Then both start_date and end_date fall back to February 28th, preserving the event duration.

#### Scenario: Century Year Transition Handling
Given a yearly recurring February 29th event crossing century year boundaries (e.g., 1899→1900, 2099→2100),
When generating instances,
Then events fall back correctly on non-leap years regardless of century leap year rules.

#### Scenario: February 29th Event Notification Timing
Given an all-day yearly event on February 29th falling back to February 28th in a non-leap year,
When the notification is due (midday of day before event),
Then the notification triggers on February 27th (day before actual occurrence on February 28th).

### Requirement: Real-Time Time Field Validation

Time field input MUST provide real-time format validation with immediate error feedback during input, matching the validation behavior of end date fields.

#### Scenario: Valid Time Format Detection

**Given** time field with valid input (HH:MM, HH, or H format)  
**When** user types each character  
**Then** validation passes silently (no error displayed)

**Acceptance Criteria**:
- [ ] No error message appears during valid input
- [ ] Error state remains None
- [ ] Input continues normally without interruption

**Example Inputs**:
| Input | Valid |
|-------|-------|
| "9" | Yes |
| "14" | Yes |
| "09:00" | Yes |
| "14:30" | Yes |

#### Scenario: Invalid Time Format Detection

**Given** time field with invalid input  
**When** user types characters that create invalid format  
**Then** error message displays immediately: "Invalid time format. Use HH:MM, HH, or H (e.g., 14:30, 14, 9)"

**Acceptance Criteria**:
- [ ] Error message appears within 100ms of invalid input
- [ ] Error message text is exactly "Invalid time format. Use HH:MM, HH, or H (e.g., 14:30, 14, 9)"
- [ ] Error message displays in red color
- [ ] Error message positioned below input field

**Example Invalid Inputs**:
| Input | Reason |
|-------|--------|
| "24:00" | Hour out of range (24) |
| "12:60" | Minute out of range (60) |
| "abc" | Non-numeric characters |
| "12:" | Incomplete format |

#### Scenario: Empty Time Field Validation

**Given** time field is empty  
**When** user tabs to or away from field  
**Then** no error is displayed (empty is valid for all-day events)

**Acceptance Criteria**:
- [ ] Empty field does not trigger error
- [ ] Error state remains None
- [ ] Behavior consistent with all-day event support

#### Scenario: Time Field Error Clearing

**Given** time field with active error  
**When** user corrects the input to valid format  
**Then** error message clears immediately

**Acceptance Criteria**:
- [ ] Error clears within 100ms of valid input
- [ ] No visual delay in error removal
- [ ] Field returns to normal visual state

