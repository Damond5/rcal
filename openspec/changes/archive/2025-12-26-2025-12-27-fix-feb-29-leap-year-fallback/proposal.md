# Fix February 29th Leap Year Fallback for Yearly Recurring Events

## Summary
Fix yearly recurring event generation for February 29th base dates to continue in non-leap years by falling back to February 28th, resolving current bug where such events stop generating after the initial leap year.

## Motivation
Currently, when users create a yearly recurring event on February 29th (e.g., a birthday), the event stops generating in non-leap years because `NaiveDate::with_year()` returns `None` for invalid dates. This breaks the intended behavior of yearly recurring events, causing users to miss important annual events in non-leap years.

## Impact
- Fixes bug where yearly recurring events on Feb 29th stop generating after first leap year
- Ensures users don't miss important events like birthdays or anniversaries in non-leap years
- Automatically fixes existing broken February 29th recurring events in user calendars
- Maintains predictable behavior aligned with industry standard practices (conservative approach)
- No breaking changes to existing functionality
- Improves reliability of yearly recurring events for edge case dates
