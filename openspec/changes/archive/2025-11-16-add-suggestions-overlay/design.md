# Design: Suggestions Overlay
# Change ID: add-suggestions-overlay

## Overview
To resolve the overlap issue with date suggestions in the add event popup, we implement a dedicated overlay popup that renders suggestions in a small, bordered area positioned directly under the end date field. This uses the existing `Clear` widget technique for proper background obscuring, ensuring the overlay doesn't interfere with other UI elements.

## Architectural Decisions
- **Overlay Positioning**: Position the overlay directly under the end date field, aligned to the left edge of the field, with height capped at 5 lines to avoid overlap with subsequent fields while keeping it contextually relevant.
- **Conditional Rendering**: Only render the overlay when `app.show_date_suggestions` is true and suggestions are not empty, maintaining performance and UI cleanliness.
- **Size Constraints**: Limit the overlay to a small fixed size (e.g., width matching the input field, height based on number of suggestions) to prevent it from dominating the popup.
- **Styling**: Use a subtle border and background color to distinguish the overlay as auxiliary content, consistent with other popups.

## Trade-offs
- **Complexity vs. Simplicity**: An overlay adds rendering logic but avoids modifying the fixed input field layout, preserving spec compliance.
- **Visibility**: Positioning below may be more intuitive than inline, and ensures no overlap.
- **Responsiveness**: Fixed size may truncate long suggestions; consider dynamic sizing if needed in future.

## Boundary Handling
If the calculated position would place the overlay outside the terminal bounds, reposition it above the field or reduce its height dynamically.

## Implementation Notes
- Leverage existing `ratatui::widgets::Clear` and `Block` for rendering.
- Calculate overlay position relative to the end date field's `Rect`.
- Ensure the overlay is rendered after the main popup to appear on top.