# Design Decisions for February 29th Leap Year Fallback

## Architectural Overview
This change modifies the yearly recurrence generation logic in `persistence.rs` to handle the special case of February 29th base dates by falling back to February 28th in non-leap years.

## Key Decisions

### Fallback Strategy
- **Conservative approach**: Use February 28th as the fallback date for February 29th events in non-leap years
- **Reasoning**: Ensures annual events like birthdays/anniversaries are not missed, with "better late than never" philosophy
- **Alternative considered**: March 1st was rejected as it could cause events to occur in wrong month

### Implementation Location
Modify `generate_recurring_instances_in_range()` function in `src/persistence.rs`
- Specifically handle the `Recurrence::Yearly` case when base date is February 29th
- Changes affect both the "skip to first date" loop (lines 249-269) and the generation loop (lines 272-316)

### Algorithm
When generating yearly instances in the skip loop and generation loop:
1. Check if base date is February 29th (month == 2 && day == 29)
2. If yes, attempt to generate next year's date with `with_year(year + 1)`
3. If result is `None` (non-leap year), use February 28th of that year instead
4. Continue generation as normal

### Multi-Day Events
For multi-day events starting on February 29th:
- Apply the same fallback logic to both start_date and end_date
- Calculate duration before applying fallback: `duration = end_date - start_date`
- If start_date falls back to February 28th, end_date becomes `fallback_start_date + duration`
- This ensures event duration is preserved

### Data Consistency
- `base_date` field remains as February 29th (original intention)
- `start_date` field for instances becomes February 28th (actual occurrence)
- This distinction allows proper instance tracking while preserving user's original intent

### Notification Behavior
For all-day February 29th events falling back to February 28th:
- Notification timing: "midday of day before" the actual occurrence date
- If event occurs on February 28th, notification triggers on February 27th
- This aligns with existing notification logic that uses instance's start_date

### Testing Strategy
- Unit tests to verify fallback behavior across leap year boundaries
- Tests for century years (1899→1900, 1999→2000, 2099→2100)
- Tests for multi-day February 29th events
- Tests for February 28th base events (should NOT fall forward)
- Integration tests to ensure yearly Feb 29th events display correctly in UI
- Cache invalidation tests for Feb 29th events

## Considerations
- **Performance**: Minimal overhead - only affects February 29th base dates, simple month/day check
- **Edge Cases**: 
  - Century years: Handled by chrono library (2000 was leap, 1900 was not, 2100 will not)
  - February 28th base events: Must NOT fall forward to February 29th in leap years
  - Multi-day events: Duration preservation is critical for spanning multiple days
- **Consistency**: Aligns with industry practice of prioritizing occurrence over exact date
- **Backward Compatibility**: No breaking changes to existing functionality
- **User Experience**: Users may notice date change; consider future enhancement with visual indicator
- **Data Migration**: Existing broken Feb 29th events will automatically start working after this fix
