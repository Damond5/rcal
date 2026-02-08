## Context

This design document outlines the approach for renaming UI labels and internal identifiers from "Start Time" to "Time" throughout the rcal application. The change affects user-facing text in the UI, documentation, and internal identifiers while preserving backward compatibility where necessary.

The CalendarEvent struct internally represents a start time, but the user-facing label "Start Time" in the UI and documentation should simply be "Time" for brevity and clarity. This is a user experience improvement that simplifies the terminology without changing the underlying semantics.

## Goals / Non-Goals

**Goals:**
- Rename all UI labels from "Start Time" to "Time"
- Rename the `PopupInputField` enum variant from `StartTime` to `Time`
- Rename state management fields from `popup_event_start_time` to `popup_event_time`
- Update all documentation references from "Start Time" to "Time"
- Ensure test assertions reflect the new terminology
- Maintain backward compatibility with existing event file format where possible

**Non-Goals:**
- Changing the internal CalendarEvent struct field name `start_time` (it remains semantically correct as the start time)
- Modifying the file format structure or breaking existing event data
- Changing any date/time handling logic or functionality
- Renaming other popup input fields beyond StartTime

## Decisions

### 1. Rename PopupInputField Enum Variant

**Decision:** Rename the enum variant from `StartTime` to `Time` in `src/app.rs` at line 44.

**Rationale:** The enum represents a field type in the popup form. The shorter "Time" variant name improves code readability and aligns with the new UI label. Enum variant names should be concise and meaningful within the enum's domain.

**Alternatives considered:**
- Keep `StartTime` variant name: Rejected because it would create inconsistency between the enum name and the displayed UI label
- Rename to `EventTime`: Rejected because it adds unnecessary verbosity; "Time" is sufficient context within the PopupInputField enum

### 2. Rename State Management Fields

**Decision:** Rename `popup_event_start_time` to `popup_event_time` in `src/app.rs` at lines 61, 110, and 155.

**Rationale:** State field names should match the enum variant they represent. Since the enum is now `Time`, the corresponding state field should be `popup_event_time`. This consistency makes the code more maintainable and easier to understand.

**Alternatives considered:**
- Keep `popup_event_start_time`: Rejected because it would create confusion with the renamed enum variant and UI label
- Rename to `popup_start_time`: Rejected because the "event" prefix clarifies this is popup event state, not the event's actual start time

### 3. Update UI Labels

**Decision:** Change all UI label text from "Start Time" to "Time" in `src/ui.rs` at lines 530, 536, and 539.

**Rationale:** User-facing text should be concise. "Time" is clearer and consistent with the renamed internal identifiers. This improves the user experience by reducing visual clutter without losing meaning.

**Alternatives considered:**
- Keep "Start Time" label: Rejected because it creates inconsistency with the renamed internal identifiers
- Use "Event Time": Rejected because "Time" is sufficient and less verbose; the context makes it clear

### 4. Preserve Internal CalendarEvent Field Name

**Decision:** Keep the `start_time` field name in the CalendarEvent struct (lines 17-29 of `src/app.rs`).

**Rationale:** The field represents the actual start time of the event, which is semantically correct and should not change. Only user-facing labels are being simplified, not the internal data model. Changing this field name would be a breaking change to the internal API and event file format.

**Alternatives considered:**
- Rename to `time`: Rejected because it loses the semantic meaning that this is the start time specifically
- Rename to `event_time`: Rejected because it's more verbose without adding clarity; "start_time" is precise

### 5. Update Event Handling Code

**Decision:** Update all references to the `StartTime` enum variant in `src/event_handling.rs` to use `Time` instead.

**Rationale:** All code that references the enum variant must be updated to maintain compilation correctness. This includes match arms, pattern matching, and any conditional logic based on the variant.

**Alternatives considered:**
- Create a type alias: Overengineered for this simple rename; direct renaming is cleaner

### 6. Update File Format Documentation

**Decision:** Update the file format parsing and serialization labels in `src/rcal/src/persistence.rs` to use "Time" instead of "Start Time".

**Rationale:** Documentation strings and user-facing error messages should use consistent terminology. The file format itself should be updated to use the new label for consistency.

**Alternatives considered:**
- Keep "Start Time" in file format: Rejected because it creates confusion between documentation and implementation

### 7. Update Test Assertions

**Decision:** Update all test assertions in `tests/integration_test.rs` (8+ occurrences) to expect "Time" instead of "Start Time".

**Rationale:** Tests must reflect the new behavior and terminology. This ensures that the changes are correctly implemented and prevents regression.

**Alternatives considered:**
- Skip test updates: Rejected because tests would fail, making it impossible to verify correctness

### 8. Update Documentation

**Decision:** Update all documentation files:
- `EVENT_FORMAT.md`: 6 occurrences of "Start Time" label
- `README.md`: 7 occurrences of "Start Time"
- `CHANGELOG.md`: Remove old breaking change note related to this rename

**Rationale:** Documentation must match the current implementation. The CHANGELOG should not contain outdated notes about planned breaking changes that are now being implemented.

**Alternatives considered:**
- Keep outdated documentation: Rejected because it creates confusion for users and contributors

## Risks / Trade-offs

### Risks
1. **Test Failures**: Tests that check for "Start Time" labels will fail until updated. Mitigation: Update all test assertions in a single commit.
2. **Documentation Inconsistency**: Outdated documentation could confuse users during the transition. Mitigation: Update all documentation in the same commit as code changes.
3. **External Tools**: Any external tools or scripts that depend on the "Start Time" label in event files may break. Mitigation: Document the change in release notes; the change is purely cosmetic in user-facing text.

### Trade-offs
1. **Completeness vs. Compatibility**: The change is comprehensive across all user-facing text but preserves internal field names. This is the right trade-off because:
   - Internal field names (`start_time`) are implementation details and don't affect users
   - User-facing text is what users actually see and interact with
   - Breaking internal field names would require complex migration logic

2. **Simplicity vs. Flexibility**: A simpler rename approach was chosen over creating abstraction layers or type aliases. This is the right trade-off because:
   - The rename is straightforward and localized
   - No additional complexity is introduced
   - Future changes (if any) can be made incrementally

## Implementation Order

1. Update source code identifiers (enum variant, state fields)
2. Update UI labels in `src/ui.rs`
3. Update event handling code in `src/event_handling.rs`
4. Update file format parsing in `src/rcal/src/persistence.rs`
5. Update tests in `tests/integration_test.rs`
6. Update documentation files (EVENT_FORMAT.md, README.md, CHANGELOG.md)
7. Verify all tests pass
8. Update CHANGELOG.md with the completed change
