## Context
The current end date input provides autocomplete suggestions only after the user starts typing. This change aims to improve user experience by showing common date suggestions immediately when the user enters the end date field, providing better discoverability and reducing the learning curve for date input options.

## Goals / Non-Goals
- **Goals**: Show top 5 most common relative date suggestions immediately on field entry, maintain existing typing-based suggestion updates, improve user discoverability of available date options
- **Non-Goals**: Change the suggestion algorithm for typed input, modify overlay positioning or navigation, show all available suggestions (limited to top 5 for UX)

## Decisions
- **Top 5 Suggestions Priority**: Ordered by expected frequency of use - Tomorrow (most common), Next week, End of month, Next month, Same day (for single-day events)
- **Empty Input Handling**: When input is empty, show curated top suggestions instead of all possible matches to avoid overwhelming the user
- **Maintain Existing Behavior**: Typing suggestions continue to work as before, with the new feature being additive
- **No Breaking Changes**: Existing functionality remains unchanged, new feature only enhances the experience

## Risks / Trade-offs
- **Risk**: Showing suggestions immediately might feel intrusive to power users who prefer to type without suggestions; **Mitigation**: Suggestions can still be dismissed with Esc or by continuing to type
- **Risk**: Limited to 5 suggestions might not cover all use cases; **Mitigation**: Users can still type to see more specific suggestions, and the top 5 cover 80% of common use cases
- **Risk**: Additional UI complexity; **Mitigation**: Uses existing overlay system, no new UI components needed

## Migration Plan
No migration needed - this is a pure enhancement that adds functionality without changing existing behavior. Users will see suggestions immediately when entering the field, but can continue using the application exactly as before.</content>
<parameter name="filePath">openspec/changes/2026-01-10-show-end-date-suggestions-on-field-entry/design.md