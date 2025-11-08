# Tasks for Expanding Calendar View to Three Months

1. **[x] Modify UI rendering logic** - Update `ui.rs` to generate calendar tables for current month, next month, and month after next, stacked within a single bordered area
2. **[x] Adjust layout constraints** - Modify the layout to position three calendar tables close together inside one border without individual borders
3. **[x] Update month headers** - Integrate month headers into the combined view (e.g., month name in week number column)
4. **[x] Update month navigation** - Change H/L key handlers to page through three-month periods instead of single months
5. **[x] Ensure event indicators work across months** - Verify that event markers (*) appear correctly for events in all three displayed months
6. **[x] Update hints text** - Modify the bottom hints to reflect the new navigation behavior (three-month paging)
7. **[x] Test navigation edge cases** - Verify correct behavior when navigating near year boundaries and leap years
8. **[x] Run existing tests** - Ensure all integration tests still pass with the expanded view</content>
<parameter name="filePath">/home/nikv/workspace/rcal/openspec/changes/expand-calendar-view-to-three-months/tasks.md