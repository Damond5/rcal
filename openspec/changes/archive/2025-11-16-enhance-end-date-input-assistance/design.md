# Design for Enhance End Date Input Assistance

## Context
The current end date input in the event creation/editing popup uses plain text input with manual DD/MM parsing. Users must remember the exact format and are prone to input errors. This enhancement adds real-time validation and auto-completion to improve usability while maintaining the required DD/MM format.

## Goals / Non-Goals
- **Goals**: Provide immediate feedback for invalid dates, suggest common date patterns, maintain keyboard-centric workflow
- **Non-Goals**: Change the underlying date format, add full calendar picker widget, modify date parsing logic

## Decisions
- **Validation Approach**: Real-time validation on input changes, showing errors in red below the input field
- **Suggestion System**: Context-aware suggestions based on start date, including relative dates like "tomorrow", "next week"
- **UI Integration**: Extend existing App state and UI rendering without major restructuring

## Alternatives Considered
- **Full Calendar Widget**: Rejected due to higher complexity and screen space requirements
- **Dropdown Selection**: Considered but rejected as it limits arbitrary date entry
- **Delayed Validation**: Real-time preferred for immediate feedback

## Implementation Considerations
- **Performance**: Debounce input validation to avoid excessive computations on rapid typing. Consider a 100-200ms delay before triggering validation and suggestions to balance responsiveness with performance.
- **Edge Cases**: Ensure suggestions handle cases where the start date is in the past or future relative to "now." For example, if start date is tomorrow, "tomorrow" should suggest the day after the start date.
- **Accessibility**: Error messages and suggestions should be compatible with screen readers and keyboard navigation.

## Risks / Trade-offs
- **Minimal Risk**: Additive enhancement, backward compatible
- **Performance**: Negligible impact as validation is lightweight
- **UI Complexity**: Adds visual elements but maintains clean interface

## Migration Plan
- No migration needed - purely additive feature
- Existing date input continues to work unchanged