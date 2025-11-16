# ui-rendering Specification

## Purpose
TBD - created by archiving change add-agent-guidelines-specs. Update Purpose after archive.
## Requirements
### Requirement: Popup Rendering Technique
Popups MUST use `ratatui::widgets::Clear` to render over calendar background.

#### Scenario: Background Clearing
Given popup display,
When using Clear widget,
Then background is properly obscured.

### Requirement: Input Field Presentation
Input fields MUST be wrapped in `Block` widgets with titles for identification.

#### Scenario: Field Labeling
Given input fields,
When wrapped in Block with titles,
Then fields are clearly identified.

### Requirement: Cursor Positioning Logic
Cursor coordinates MUST be calculated relative to input field positions using character-based indexing.

#### Scenario: Accurate Cursor Placement
Given text input,
When calculating cursor position,
Then it aligns with character position.

### Requirement: Style Management System
Styling MUST be conditional based on `selected_input_field` for visual feedback.

#### Scenario: Field Highlighting
Given multiple input fields,
When one is selected,
Then it shows visual distinction.

### Requirement: Event Selection Display
Selected events in view popup MUST be highlighted with black text on light blue background.

#### Scenario: Event Highlighting
Given selected event,
When displaying in popup,
Then it uses specified colors for readability.

### Requirement: Text Field Sizing
Text field heights MUST be fixed in event creation/editing popup for consistent visibility.

#### Scenario: Consistent Heights
Given event popup,
When displaying text fields,
Then heights remain fixed.

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

</content>
<parameter name="filePath">/home/nikv/workspace/rcal/openspec/changes/expand-calendar-view-to-three-months/specs/ui-rendering/spec.md

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

### Requirement: Sync Status Message Display
The sync popup MUST NOT display redundant "Up to date" messages when the sync status is up to date. Only the status line "Status: Up to date" should be shown.

#### Scenario: No Duplicate Up to Date Message
Given the sync status is UpToDate,
When displaying the sync popup,
Then only "Status: Up to date" is shown without additional "Up to date" message.

### Requirement: Suggestions Overlay Rendering
Date suggestions in the add event popup MUST be displayed in a dedicated overlay popup positioned near the end date field, using `ratatui::widgets::Clear` for background obscuring, to avoid overlapping with other input fields.

#### Scenario: Overlay Display
Given date suggestions are available and `show_date_suggestions` is true,
When rendering the add event popup,
Then displays suggestions in a small overlay near the end date field without overlapping input fields.

#### Scenario: Overlay Positioning
Given the end date field position,
When positioning the suggestions overlay,
Then places it directly under the field, aligned to the left edge.

#### Scenario: Overlay Styling
Given the suggestions overlay,
When rendered,
Then uses a bordered block with subtle styling to distinguish it as auxiliary content.

#### Scenario: Conditional Visibility
Given no date suggestions or `show_date_suggestions` is false,
When rendering the add event popup,
Then does not display the suggestions overlay.

#### Scenario: Overlay Boundary Adjustment
Given the calculated overlay position,
When it would exceed terminal bounds,
Then repositions or resizes the overlay to stay within bounds.

