# Add Yearly Recurring Events

## Summary
Extend the recurring events feature to support yearly recurrence patterns, allowing events to repeat annually on the same date.

## Motivation
Users frequently need to schedule events that occur yearly, such as birthdays, anniversaries, or annual reviews. The current system supports daily, weekly, and monthly recurrence but lacks yearly support.

## Impact
- Adds "yearly" as a valid recurrence option in event creation/editing
- Extends existing recurring event generation logic to handle yearly patterns
- Updates event format documentation to include yearly recurrence
- Updates README.md to reflect yearly recurrence support in the features list
- No breaking changes to existing functionality
- Maintains backward compatibility with existing events