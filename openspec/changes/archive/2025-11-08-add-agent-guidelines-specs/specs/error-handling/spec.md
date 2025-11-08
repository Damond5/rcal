## ADDED Requirements

### Requirement: Graceful Degradation
Invalid inputs MUST prevent operations but not crash the application.

#### Scenario: Invalid Time Format
Given invalid time input,
When creating event,
Then creation fails gracefully without crash.

### Requirement: Input Validation Implementation
Time strings MUST be parsed with flexible format support and failure handling.

#### Scenario: Flexible Time Parsing
Given various time formats (HH:MM, HH, H),
When parsing input,
Then valid formats are accepted.

#### Scenario: Invalid Input Handling
Given invalid time format,
When parsing,
Then failure is handled gracefully.

### Requirement: Boundary Check Enforcement
Cursor movement MUST be prevented beyond string boundaries using character-based indexing.

#### Scenario: Cursor Bounds
Given text input,
When moving cursor to end,
Then it doesn't go beyond string length.

### Requirement: Unicode Character Handling
Multi-byte characters MUST be handled properly in text input fields.

#### Scenario: Unicode Input
Given Unicode text,
When editing,
Then characters are processed correctly.

### Requirement: Notification Failure Handling
D-Bus errors MUST be logged without crashing the daemon.

#### Scenario: Notification Daemon Unavailable
Given no notification daemon,
When sending notification,
Then logs error but continues running.

## Cross-references
- See `event-management` for time input validation details