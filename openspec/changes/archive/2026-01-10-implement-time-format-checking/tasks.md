# Implementation Tasks: Real-Time Time Format Checking

## Phase 1: Validation Logic

### Task 1.1: Add Time Validation Function to date_utils.rs
- [ ] Create `validate_time_input(input: &str) -> Result<(), String>` function
- [ ] Implement pattern matching for HH:MM format (e.g., "14:30")
- [ ] Implement pattern matching for HH format (e.g., "14")
- [ ] Implement pattern matching for H format (e.g., "9")
- [ ] Add error message: "Invalid time format. Use HH:MM"
- [ ] Add unit tests for all time format scenarios
- [ ] Review implementation with @code-review

### Task 1.2: Verify Existing Date Validation Pattern
- [ ] Review `validate_date_input()` function in date_utils.rs
- [ ] Document validation pattern to follow
- [ ] Ensure consistent error message style

## Phase 2: State Management

### Task 2.1: Add Time Input Error State to App Struct
- [ ] Add `time_input_error: Option<String>` field to App struct in app.rs
- [ ] Add `end_time_input_error: Option<String>` field to App struct in app.rs
- [ ] Initialize fields to `None` in app initialization
- [ ] Review state changes with @code-review

### Task 2.2: Clear Time Errors on Field Navigation
- [ ] Clear `time_input_error` when navigating to time field (like `date_input_error` in event_handling.rs:526)
- [ ] Clear `end_time_input_error` when navigating to end time field
- [ ] Clear error states when opening event popup (in event_handling.rs 'a' and 'e' handlers)

## Phase 3: Event Handling

### Task 3.1: Add Real-Time Validation for Time Field
- [ ] Modify input handling in event_handling.rs
- [ ] Add validation trigger when popup_event_time is active
- [ ] Add validation trigger on backspace for time field (handle in event_handling.rs KeyCode::Backspace match arm)
- [ ] Call `validate_time_input()` on each character input
- [ ] Update `time_input_error` state based on validation result
- [ ] Review implementation with @code-review

### Task 3.2: Add Real-Time Validation for End Time Field
- [ ] Modify input handling in event_handling.rs
- [ ] Add validation trigger when popup_event_end_time is active
- [ ] Add validation trigger on backspace for end time field (handle in event_handling.rs KeyCode::Backspace match arm)
- [ ] Call `validate_time_input()` on each character input
- [ ] Update `end_time_input_error` state based on validation result
- [ ] Review implementation with @code-review

## Phase 4: User Interface

### Task 4.1: Add Time Error Display to Popup UI
- [ ] Locate popup layout in ui.rs
- [ ] Add error display area for time field
- [ ] Bind error display to `time_input_error` state
- [ ] Implement conditional rendering (show only when error present)
- [ ] Review UI changes with @code-review

### Task 4.2: Add End Time Error Display to Popup UI
- [ ] Locate popup layout in ui.rs
- [ ] Add error display area for end time field
- [ ] Bind error display to `end_time_input_error` state
- [ ] Implement conditional rendering (show only when error present)
- [ ] Review UI changes with @code-review

## Phase 5: Testing

### Task 5.1: Unit Tests for Time Validation
- [ ] Test valid HH:MM formats ("00:00", "12:30", "23:59")
- [ ] Test valid HH formats ("0", "9", "12", "23")  // Including "0"
- [ ] Test valid H formats ("0", "1", "9")  // Including "0"
- [ ] Test invalid formats ("24:00", "12:60", "abc", "12:")
- [ ] Test single "0" hour format (critical test case)

### Task 5.2: Integration Tests for Real-Time Validation
- [ ] Test error display on invalid time input
- [ ] Test error clearing on valid time input
- [ ] Test error display on invalid end time input
- [ ] Test error clearing on valid end time input
- [ ] Test error clears when navigating away from time field and back
- [ ] Test error clears when navigating away from end time field and back
- [ ] Test empty field behavior (no error)

### Task 5.3: Manual Testing
- [ ] Test time field real-time validation
- [ ] Test end time field real-time validation
- [ ] Verify all existing time formats still work
- [ ] Verify error messages display immediately
- [ ] Verify errors clear immediately on correction
- [ ] Document test results

## Phase 6: Build and Verification

### Task 6.1: Build Project
- [ ] Run full project build
- [ ] Verify no compilation errors
- [ ] Fix any warnings

### Task 6.2: Run Test Suite
- [ ] Run all unit tests
- [ ] Run integration tests
- [ ] Verify all tests pass
- [ ] Fix any failing tests

## Phase 7: Documentation

### Task 7.1: Update CHANGELOG.md
- [ ] Add new feature entry under [Unreleased] section with "### Added"
- [ ] Document real-time time format validation feature
- [ ] Use @docs-writer subagent for proper formatting per keepachangelog.com

### Task 7.2: Update README.md
- [ ] Document real-time validation feature
- [ ] Update input format documentation if needed
