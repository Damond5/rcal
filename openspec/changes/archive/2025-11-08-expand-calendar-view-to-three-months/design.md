# Design Considerations for Three-Month Calendar View

## Current Architecture
The application uses a single `app.date` field to track the currently displayed month. Navigation keys (H/L) increment/decrement this date by one month, and the UI renders the calendar for that specific month.

## Proposed Changes
- **State Management**: Keep `app.date` as the starting month of the three-month view
- **Navigation Logic**: Modify H/L key handlers to add/subtract 3 months instead of 1 month
- **UI Rendering**: Generate three calendar tables for consecutive months and stack them within a single bordered area
- **Layout**: Use vertical sub-layouts to position months close together without individual borders

## Trade-offs Considered
- **Simplicity vs Flexibility**: Maintaining a single date field keeps the state simple but limits flexibility for arbitrary month ranges
- **Performance**: Rendering three months instead of one increases computation slightly but remains negligible
- **User Experience**: Three-month view provides better overview in a compact layout

## Implementation Details
- Months will be rendered as separate `Table` widgets without individual borders
- A single border will enclose all three months with an overall "RCal" title
- Month headers will be displayed as full-width `Paragraph` widgets above each calendar table
- Event indicators will be calculated for each month independently
- Selected date highlighting will work across all three months
- Week numbers will be displayed for each month in a narrow column
- Spacing is added between months for visual separation

## Backward Compatibility
- No breaking changes to data structures or persistence
- Event management functionality remains unchanged
- Only UI display and navigation behavior are modified</content>
<parameter name="filePath">/home/nikv/workspace/rcal/openspec/changes/expand-calendar-view-to-three-months/design.md