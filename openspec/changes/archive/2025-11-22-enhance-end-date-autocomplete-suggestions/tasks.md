## 1. Expand Relative Date Suggestions
- [x] 1.1 Add support for "next monday", "next friday" suggestions
- [x] 1.2 Add duration-based suggestions: "1 day", "1 week", "2 weeks", "1 month"
- [x] 1.3 Add "next month", "end of year" suggestions
- [x] 1.4 Add "same day" suggestion for events ending same day

## 2. Improve Fuzzy Matching and Prefix Handling
- [x] 2.1 Implement fuzzy matching for typos (e.g., "tomorow" → "tomorrow")
- [x] 2.2 Support partial word matching beyond prefixes
- [x] 2.3 Remove case-sensitivity requirement for matching

## 3. Enhanced Date Completion
- [x] 3.1 Support more partial inputs: "15/" → complete with current/next month
- [x] 3.2 Support " /10" → complete with appropriate day
- [x] 3.3 Add suggestions for common date patterns like "last day of month", "first of next month"
- [x] 3.4 Show full date formats when partial input matches

## 4. User Experience Enhancements
- [x] 4.1 Show suggestion descriptions alongside dates (e.g., "Tomorrow (02/11)")
- [x] 4.2 Add arrow key navigation for suggestions
- [x] 4.3 Improve overlay positioning with better boundary detection
- [x] 4.4 Prioritize suggestions based on likelihood

## 5. Validation Integration
- [x] 5.1 Show validation feedback in suggestions (gray out invalid dates)
- [x] 5.2 Suggest corrections for invalid inputs
- [x] 5.3 Highlight suggestions that would create invalid date ranges

## 6. Performance and Edge Cases
- [x] 6.1 Limit suggestions to reasonable number (maintain max 5)
- [x] 6.2 Handle edge cases like month transitions and leap years
- [x] 6.3 Add loading states if needed for complex generation

## 7. Testing
- [x] 7.1 Update existing tests in date_utils.rs for new suggestions
- [x] 7.2 Add integration tests for UI suggestion display
- [x] 7.3 Test fuzzy matching and edge cases

## 8. Documentation
- [x] 8.1 Update inline code comments for new suggestion logic
- [x] 8.2 Update any relevant README sections about date input

## 9. Changelog Update
- [x] 9.1 Update CHANGELOG.md with new end date autocomplete features using docs-writer subagent</content>
<parameter name="filePath">openspec/changes/enhance-end-date-autocomplete-suggestions/tasks.md