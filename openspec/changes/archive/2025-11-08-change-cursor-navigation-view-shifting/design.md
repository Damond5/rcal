# Design Considerations for Cursor Navigation View Shifting

## Current Architecture
The application currently displays a 3-month calendar view that is dynamically centered on the month containing `app.date` (the selected date). When `app.date` changes to a different month, the view automatically shifts to center on the new month.

## Proposed Changes
- **State Management**: Add a `view_start_month` field to the `App` struct to track the starting month of the current 3-month view
- **View Calculation**: Modify UI rendering to display `view_start_month`, `view_start_month+1`, `view_start_month+2` instead of centering on `app.date.month()`
- **Navigation Logic**: Update h/j/k/l event handlers to check if `app.date` is outside the current view boundaries and adjust `view_start_month` accordingly
- **Boundary Handling**: When `app.date.month() < view_start_month`, set `view_start_month = app.date.month()`; when `app.date.month() > view_start_month + 2`, set `view_start_month = app.date.month() - 2`
- **Year Boundaries**: Handle month wrapping across years in view calculations

## Trade-offs Considered
- **User Experience vs Complexity**: Fixed view boundaries provide more stable navigation but require additional state management
- **Performance**: Minimal impact as view calculations remain simple arithmetic
- **Backward Compatibility**: H/L paging behavior remains unchanged, only cursor navigation is affected

## Implementation Details
- Add `view_start_month: u32` and `view_start_year: i32` to `App` struct
- Initialize `view_start_month` and `view_start_year` based on current `app.date` on app startup
- Update UI rendering in `ui.rs` to use view boundaries instead of centering on cursor
- Modify event handlers in `event_handling.rs` to adjust view boundaries when cursor moves outside
- Ensure event highlighting works correctly across the fixed view range

## Testing Considerations
- Test navigation at month boundaries to ensure view shifts correctly
- Test year boundary wrapping in view calculations
- Verify event display works across the fixed 3-month range
- Ensure H/L paging still works independently and does not interfere with the new view boundary state
- Test backward compatibility: H/L paging should continue to page through 3-month periods without affecting cursor navigation view stability</content>
<parameter name="filePath">openspec/changes/change-cursor-navigation-view-shifting/design.md