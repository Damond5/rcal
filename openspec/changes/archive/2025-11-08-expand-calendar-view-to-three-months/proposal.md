# Expand Calendar View to Three Months

## Summary
Modify the calendar display to show the current month plus the next two months stacked vertically within a single bordered view, providing users with a broader view of upcoming events and dates.

## Motivation
Currently, rcal displays only a single month at a time. Users need to navigate month by month to see upcoming events. Displaying three months simultaneously in a compact stacked layout will improve usability by allowing users to see their schedule across a longer timeframe without additional navigation.

## Impact
- **UI Changes**: The calendar view will display three months stacked close together within a single border, with adjusted month headers
- **Navigation**: Existing month navigation (H/L keys) will page through three-month periods
- **Performance**: Minimal impact as the calendar rendering is already efficient
- **User Experience**: Improved overview of upcoming events and dates in a space-efficient layout

## Implementation Approach
- Modify the UI rendering logic to generate three consecutive months and stack them vertically within a single bordered area
- Adjust layout to position months close together without individual borders
- Update month headers to work within the combined view
- Update navigation logic to handle three-month paging
- Ensure event indicators (*) are correctly displayed across all three months</content>
<parameter name="filePath">/home/nikv/workspace/rcal/openspec/changes/expand-calendar-view-to-three-months/proposal.md