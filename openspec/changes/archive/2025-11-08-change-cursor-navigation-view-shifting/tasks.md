# Tasks for Cursor Navigation View Shifting

1. **[x] Add view boundary state to App struct** - Add `view_start_month: u32` and `view_start_year: i32` fields to track the starting month/year of the current 3-month view
2. **[x] Initialize view boundaries on app startup** - Set initial `view_start_month` and `view_start_year` based on `app.date` in `App::new()`
3. **[x] Update UI rendering logic** - Modify `ui.rs` to display months based on view boundaries instead of centering on `app.date.month()`
4. **[x] Implement view boundary adjustment in navigation** - Update h/j/k/l event handlers in `event_handling.rs` to check cursor position against view boundaries and shift view when necessary
5. **[x] Handle year boundary wrapping in view calculations** - Ensure view month calculations properly wrap across years
6. **[x] Update event highlighting logic** - Verify that selected date highlighting works correctly within the fixed view range
7. **[x] Add integration tests** - Test navigation behavior at month and year boundaries to ensure view shifts correctly, including:
   - Navigating from the last day of the last displayed month to the next month (forward shift)
   - Navigating from the first day of the first displayed month to the previous month (backward shift)
   - Year boundary cases (e.g., Dec 31 to Jan 1 of next year)
8. **[x] Run existing tests** - Ensure all current tests still pass with the modified view logic</content>
<parameter name="filePath">openspec/changes/change-cursor-navigation-view-shifting/tasks.md