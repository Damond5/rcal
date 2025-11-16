# Spec Delta: Add Suggestions Overlay
# Modifies: openspec/specs/ui-rendering/spec.md
# Date: 2025-11-16
# Purpose: Add a dedicated overlay for date suggestions to prevent overlap with input fields

## ADDED Requirements

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