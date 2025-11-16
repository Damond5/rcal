# Change Proposal: Add Suggestions Overlay
# Change ID: add-suggestions-overlay
# Date: 2025-11-16
# Purpose: Implement a dedicated overlay popup for date suggestions in the add event popup to prevent overlap with input fields.

## Summary
The current implementation renders date suggestions directly below the end date field, causing them to overlap with the end time field. This change introduces a small overlay popup that appears near the end date field when suggestions are active, using the Clear widget for proper rendering without overlapping other UI elements.

## Impact
- Modifies the UI rendering logic in `src/ui.rs` to conditionally display a suggestions overlay.
- Ensures suggestions are visible without disrupting the fixed layout of input fields.
- Maintains compliance with existing UI specs for popup rendering and field heights.

## Dependencies
- None

## Related Changes
- None