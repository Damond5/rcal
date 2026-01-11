# event-management Specification

## MODIFIED Requirements

### Requirement: All-Day Event Handling
Events without **start time** MUST be treated as all-day with "All day" display.

#### Scenario: All-Day Display
Given event without **start time**,
When viewing,
Then shows "All day".

#### Scenario: All-Day Storage
Given all-day event,
When saving,
Then time set to "all-day" in file.

### Requirement: Real-Time Start Time Field Validation
**Start time** field input MUST provide real-time format validation with immediate error feedback during input, matching the validation behavior of end date fields.

#### Scenario: Valid Time Format Detection

**Given** **start time** field with valid input (HH:MM, HH, or H format)
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

**Given** **start time** field with invalid input
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

**Given** **start time** field is empty
**When** user tabs to or away from field
**Then** no error is displayed (empty is valid for all-day events)

**Acceptance Criteria**:
- [ ] Empty field does not trigger error
- [ ] Error state remains None
- [ ] Behavior consistent with all-day event support

#### Scenario: Time Field Error Clearing

**Given** **start time** field with active error
**When** user corrects the input to valid format
**Then** error message clears immediately

**Acceptance Criteria**:
- [ ] Error clears within 100ms of valid input
- [ ] No visual delay in error removal
- [ ] Field returns to normal visual state
