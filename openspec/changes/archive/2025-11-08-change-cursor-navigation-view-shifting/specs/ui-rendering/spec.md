# ui-rendering Specification Delta

## MODIFIED Requirements

### Requirement: Three-Month Calendar Display
The main calendar view MUST display three consecutive months stacked vertically within a single bordered area, with the range determined by view boundaries rather than centering on the current month.

#### Scenario: Fixed Range Display
Given the application is displaying a calendar view,
When the selected date is within the view boundaries,
Then the view remains stable and does not shift.

#### Scenario: View Shift on Cursor Exit
Given the selected date moves beyond the current view boundaries,
When navigating with h/j/k/l keys,
Then the view shifts to include the new selected date.

#### Scenario: Backward View Shift
Given the selected date moves to a month before the first displayed month,
When navigating backward,
Then the view shifts backward to show the new month range.

#### Scenario: Forward View Shift
Given the selected date moves to a month after the last displayed month,
When navigating forward,
Then the view shifts forward to show the new month range.

## ADDED Requirements

### Requirement: Cursor Navigation View Boundaries
Navigation keys (h/j/k/l) MUST maintain stable view boundaries until the selected date moves outside the current three-month range.

#### Scenario: Stable View During Navigation
Given the selected date is within the displayed months,
When pressing h/j/k/l keys,
Then the view boundaries remain unchanged.

#### Scenario: Boundary Detection
Given navigation attempts to move the selected date outside the current view,
When the date would fall outside months X, X+1, X+2,
Then adjust view boundaries to include the new date position.

#### Scenario: Year Boundary View Shift
Given the view shift crosses a year boundary,
When navigating from December to January,
Then correctly display the new year in month headers.</content>
<parameter name="filePath">openspec/changes/change-cursor-navigation-view-shifting/specs/ui-rendering/spec.md