# event-management Specification Delta

## ADDED Requirements

### Requirement: February 29th Leap Year Fallback for Yearly Recurrence
Yearly recurring events with February 29th as the base date MUST automatically fall back to February 28th in non-leap years, ensuring events continue occurring annually.

#### Scenario: February 29th Fallback to February 28th
Given a yearly recurring event with February 29th as the base date,
When generating instances for a non-leap year,
Then the instance occurs on February 28th instead of being skipped.

#### Scenario: February 29th Occurs on Leap Years
Given a yearly recurring event with February 29th as the base date,
When generating instances for a leap year,
Then the instance occurs on February 29th.

#### Scenario: Non-February 29th Yearly Events Unaffected
Given a yearly recurring event with a base date other than February 29th,
When generating instances across all years,
Then instances occur on the same day and month annually.

#### Scenario: February 29th Multi-Day Event Fallback
Given a yearly recurring multi-day event starting February 29th and spanning multiple days,
When generating instances for a non-leap year,
Then both start_date and end_date fall back to February 28th, preserving the event duration.

#### Scenario: Century Year Transition Handling
Given a yearly recurring February 29th event crossing century year boundaries (e.g., 1899→1900, 2099→2100),
When generating instances,
Then events fall back correctly on non-leap years regardless of century leap year rules.

#### Scenario: February 29th Event Notification Timing
Given an all-day yearly event on February 29th falling back to February 28th in a non-leap year,
When the notification is due (midday of day before event),
Then the notification triggers on February 27th (day before actual occurrence on February 28th).
