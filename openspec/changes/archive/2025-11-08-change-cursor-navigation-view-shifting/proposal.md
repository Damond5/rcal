# Change Cursor Navigation View Shifting

## Why
The current calendar navigation causes the 3-month view to shift every time the cursor changes months, making it difficult to browse the calendar smoothly. Users expect more stable views when navigating within a reasonable range.

## What Changes
- Modify h/j/k/l navigation to only shift the view when the cursor moves beyond the current 3-month boundaries
- Add view boundary state to track the displayed month range independently from cursor position
- Update UI rendering to use fixed view boundaries instead of centering on cursor month
- Maintain H/L paging behavior unchanged

## Impact
- Affected specs: ui-rendering
- Affected code: app.rs, ui.rs, event_handling.rs</content>
<parameter name="filePath">openspec/changes/change-cursor-navigation-view-shifting/proposal.md