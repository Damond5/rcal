# ui-rendering Specification Delta

## ADDED Requirements

### Requirement: Three-Month Calendar Display
The main calendar view MUST display the current month plus the next two consecutive months stacked vertically within a single bordered area.

#### Scenario: Current Month Display
Given the application is launched,
When displaying the calendar,
Then the top section shows the current month.

#### Scenario: Next Month Display
Given the application is launched,
When displaying the calendar,
Then the middle section shows the month following the current month.

#### Scenario: Month After Next Display
Given the application is launched,
When displaying the calendar,
Then the bottom section shows the month two months after the current month.

#### Scenario: Single Border Enclosure
Given the three-month view is displayed,
When rendering the calendar,
Then all three months are enclosed within a single border.

#### Scenario: Month Headers Display
Given the three-month view is displayed,
When rendering the calendar,
Then each month has a full-width header showing "Month Year" above its calendar table.

#### Scenario: Event Indicators Across Months
Given events exist in multiple months,
When displaying the three-month view,
Then event indicators (*) appear correctly in all relevant months.

#### Scenario: Selected Date Highlighting
Given a date is selected,
When the selected date falls within the displayed months,
Then it is highlighted with black text on light blue background regardless of which month it appears in.

### Requirement: Three-Month Navigation
Navigation keys (H/L) MUST page through three-month periods instead of single months.

#### Scenario: Forward Paging
Given the calendar displays months X, X+1, X+2,
When pressing L (next),
Then display months X+3, X+4, X+5.

#### Scenario: Backward Paging
Given the calendar displays months X, X+1, X+2,
When pressing H (previous),
Then display months X-3, X-2, X-1.

#### Scenario: Year Boundary Handling
Given the current view ends in December,
When paging forward,
Then correctly display January of the next year.</content>
<parameter name="filePath">/home/nikv/workspace/rcal/openspec/changes/expand-calendar-view-to-three-months/specs/ui-rendering/spec.md