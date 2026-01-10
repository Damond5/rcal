# Specification: Real-Time Time Format Validation

## Overview

This specification defines the requirements for real-time format validation of time and end time input fields in the event management system.

## ADDED Requirements

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

## Requirement: Real-Time End Time Field Validation

End time field input MUST provide real-time format validation with immediate error feedback during input, matching the validation behavior of end date fields.

#### Scenario: Valid End Time Format Detection

**Given** end time field with valid input (HH:MM, HH, or H format)  
**When** user types each character  
**Then** validation passes silently (no error displayed)

**Acceptance Criteria**:
- [ ] No error message appears during valid input
- [ ] Error state remains None
- [ ] Input continues normally without interruption

#### Scenario: Invalid End Time Format Detection

**Given** end time field with invalid input  
**When** user types characters that create invalid format  
**Then** error message displays immediately: "Invalid time format. Use HH:MM, HH, or H (e.g., 14:30, 14, 9)"

**Acceptance Criteria**:
- [ ] Error message appears within 100ms of invalid input
- [ ] Error message text is exactly "Invalid time format. Use HH:MM, HH, or H (e.g., 14:30, 14, 9)"
- [ ] Error message displays in red color
- [ ] Error message positioned below end time input field

#### Scenario: Empty End Time Field Validation

**Given** end time field is empty  
**When** user tabs to or away from field  
**Then** no error is displayed (empty is valid - defaults to start time)

**Acceptance Criteria**:
- [ ] Empty field does not trigger error
- [ ] Error state remains None
- [ ] Behavior consistent with optional end time

#### Scenario: End Time Field Error Clearing

**Given** end time field with active error  
**When** user corrects the input to valid format  
**Then** error message clears immediately

**Acceptance Criteria**:
- [ ] Error clears within 100ms of valid input
- [ ] No visual delay in error removal
- [ ] Field returns to normal visual state

## Requirement: Consistent Validation Experience

All date/time input fields MUST provide consistent real-time validation behavior to ensure predictable user experience.

#### Scenario: Unified Validation Timing

**Given** any date/time input field (time, end time, end date)  
**When** user modifies input  
**Then** validation occurs on each character input with immediate feedback

**Acceptance Criteria**:
- [ ] Consistent validation timing across all fields
- [ ] Feedback within 100ms of input
- [ ] No field-specific timing differences

#### Scenario: Error Message Consistency

**Given** any date/time input field with validation error  
**When** displaying error  
**Then** error message uses consistent format and positioning

**Acceptance Criteria**:
- [ ] Same error message styling (color, font)
- [ ] Consistent error position (below input field)
- [ ] Same error message format and language

## Supported Time Formats

The validation MUST support the following input formats:

| Format | Pattern | Examples | Validation |
|--------|---------|----------|------------|
| HH:MM | 24-hour with colon | "14:30", "09:00", "23:59" | Valid |
| HH | Hour only (0-23) | "14", "9", "0" | Valid |
| H | Single digit hour | "0", "1", "9" | Valid |
| Empty | No input | "" | Valid |

## Validation Rules

1. **HH:MM Format**:
   - Hours: 00-23 (two digits, leading zero optional)
   - Minutes: 00-59 (two digits required)
   - Separator: Colon (:)

2. **HH Format**:
   - Hours: 0-23 (one or two digits)
   - No separator required

3. **H Format**:
   - Hours: 0-9 (single digit)
   - Short form of HH

4. **Empty Field**:
   - Considered valid
   - Represents all-day event or unset end time

## Error States

### Time Field Error State
- **State Field**: `app.time_input_error`
- **Type**: `Option<String>`
- **None**: No error, valid input
- **Some(message)**: Error message to display

### End Time Field Error State
- **State Field**: `app.end_time_input_error`
- **Type**: `Option<String>`
- **None**: No error, valid input
- **Some(message)**: Error message to display

## UI Requirements

### Error Display
- **Color**: Red text
- **Position**: Below the corresponding input field
- **Visibility**: Conditional (only when error present)
- **Timing**: Immediate display on error, immediate hide on correction

### Input Field
- **Target Field**: `popup_event_time` (time), `popup_event_end_time` (end time)
- **Trigger**: Content change event
- **Validation**: Called on each character input

## Implementation Requirements

### Validation Function
```rust
fn validate_time_input(input: &str) -> Result<(), String> {
    // Returns Ok(()) for valid formats
    // Returns Err("Invalid time format. Use HH:MM, HH, or H (e.g., 14:30, 14, 9)") for invalid
}
```

### Error State Updates
```rust
// On validation failure
app.time_input_error = Some("Invalid time format. Use HH:MM, HH, or H (e.g., 14:30, 14, 9)".to_string());

// On validation success
app.time_input_error = None;
```

## Testing Requirements

### Unit Tests
1. Valid HH:MM formats (12 examples)
2. Valid HH formats (10 examples)
3. Valid H formats (9 examples)
4. Invalid formats (15 examples)
5. Empty string (1 example)

### Integration Tests
1. Error display on invalid input
2. Error clearing on valid input
3. Empty field behavior
4. Real-time validation timing
5. Cross-field consistency

### Manual Tests
1. Typing valid time shows no error
2. Typing invalid time shows error immediately
3. Correcting invalid time clears error immediately
4. Empty field shows no error
5. Both time and end time fields work consistently

## Acceptance Criteria

- [ ] Time field shows real-time validation feedback
- [ ] End time field shows real-time validation feedback
- [ ] Error messages display immediately on invalid input
- [ ] Errors clear immediately on valid input
- [ ] All existing time input formats still work
- [ ] Tests pass for validation logic
- [ ] Manual testing confirms improved user experience
