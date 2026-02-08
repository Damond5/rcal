# Implementation Tasks: Rename "Start Time" to "Time"

## Overview
Rename all user-facing "Start Time" labels and enum variants to "Time" while keeping internal `start_time` field names unchanged. This is a pure nomenclature change with no functional modifications.

---

## Documentation Updates

- [x] **EVENT_FORMAT.md**: Replace all 6 occurrences of "Start Time" with "Time"
  - Change format documentation from `- **Start Time**:` to `- **Time**:`
  - Verify consistency with src/rcal/src/persistence.rs changes

- [x] **README.md**: Replace all 7 occurrences of "Start Time" with "Time"
  - Update usage examples, descriptions, and format specifications
  - Ensure examples match new UI labels

- [x] **CHANGELOG.md**: Remove existing breaking change note related to "Start Time"
  - Remove outdated note since this change standardizes the label without affecting file format compatibility

---

## Source Code Updates

### src/app.rs

- [x] **Line 44: Enum PopupInputField variant**
  - Rename `StartTime` to `Time`

- [x] **Lines 61, 110, 155: State field names**
  - Rename `popup_event_start_time` to `popup_event_time`

- [x] **Lines 17-29: CalendarEvent struct (NO CHANGE)**
  - Keep internal field `start_time` unchanged (semantically correct - it IS the start time)

### src/ui.rs (Lines 530, 536, 539)

- [x] **UI Label Text**
  - Change "Start Time" to "Time" (3 occurrences)
  - Update displayed text and placeholder values

### src/event_handling.rs

- [x] **Enum Variant References**
  - Update all references to `StartTime` enum variant to use `Time`
  - Update pattern matching, comparisons, and variant handling

### src/rcal/src/persistence.rs

- [x] **File Format Labels**
  - Update parsing and serialization labels for consistency with EVENT_FORMAT.md
  - Ensure user-facing labels match new "Time" terminology

---

## Test Updates

### tests/integration_test.rs

- [x] **Test Assertions**
  - Update all 8+ test assertions referencing "Start Time" to use "Time"
  - Update any enum variant references from `StartTime` to `Time`

---

## Verification

- [x] **Build the Project**
  - Run `cargo build` or `cargo check`
  - Verify compilation succeeds without errors

- [x] **Run Tests**
  - Execute `cargo test`
  - Ensure all tests pass with updated labels

- [x] **Verify UI Displays Correctly**
  - Run the application
  - Confirm "Time" label appears instead of "Start Time"
  - Check all popup states and input modes for consistency

---

## Technical Notes

- **Internal vs External Naming**: Only change user-facing labels and enum variants. Internal `start_time` field in CalendarEvent struct remains unchanged since it semantically represents the start time.

- **File Format Compatibility**: No structural changes to event data storage. Only label strings in human-readable format sections change.

- **No Breaking Changes**: This is a pure refactoring of internal naming and labels with no functional impact on capabilities.

