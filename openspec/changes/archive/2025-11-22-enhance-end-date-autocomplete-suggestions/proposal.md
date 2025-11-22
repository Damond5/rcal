# Change: Enhance End Date Autocomplete Suggestions

## Why
The current end date autocomplete suggestions are limited to basic relative dates ("tomorrow", "next week", "end of month") and simple partial date completion. Users would benefit from more comprehensive and intuitive suggestions that support natural language inputs, better fuzzy matching, enhanced date completion, improved UX, validation integration, and performance optimizations.

## What Changes
- Expand relative date suggestions to include more natural language options like "next monday", "in 3 days", "next month", "end of year", "1 day", "1 week", "2 weeks", "1 month"
- Improve fuzzy matching and prefix handling to support typos and partial word matching
- Enhance date completion for more partial inputs and common patterns
- Add user experience enhancements like suggestion descriptions, better keyboard navigation, and improved overlay positioning
- Integrate validation feedback in suggestions to show invalid dates
- Optimize performance and handle edge cases for suggestions

## Goals / Non-Goals
- **Goals**: Support common natural language date patterns, improve input accuracy, maintain DD/MM format, enhance accessibility for date entry
- **Non-Goals**: Full natural language processing, support for complex date expressions beyond common patterns, integration with external calendar APIs

## Risks / Trade-offs
- **Risk**: More suggestions may slightly increase UI complexity; **Mitigation**: Limit to 5 suggestions max and maintain clear overlay positioning
- **Risk**: Fuzzy matching could suggest incorrect dates; **Mitigation**: Prioritize exact matches and validate suggestions against date logic

## Impact
- Affected specs: event-management
- Affected code: src/date_utils.rs (get_date_suggestions function), src/ui.rs (suggestions overlay), src/event_handling.rs (suggestion triggering)</content>
<parameter name="filePath">openspec/changes/enhance-end-date-autocomplete-suggestions/proposal.md