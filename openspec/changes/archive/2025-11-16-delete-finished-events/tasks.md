# Tasks for Delete Finished Events

## 1. Implementation
- [x] 1.1 Add configuration option `auto_cleanup_old_events` in TOML config file to enable automatic cleanup on every launch
- [x] 1.2 Implement `is_finished_before` function in persistence module to check if event ended before given date
- [x] 1.3 Add `cleanup_old_events` function that loads events, filters finished ones older than 2 months, and deletes them
- [x] 1.4 Integrate cleanup call in main application startup when config option is enabled
- [x] 1.5 Add error handling for cleanup failures (log errors, continue processing)
- [x] 1.6 Ensure cleanup only deletes valid event files (check .md extension and event format)
- [x] 1.7 Add logging for cleanup operations (number of events deleted, any errors)

## 2. Testing
- [x] 2.1 Add integration test for cleanup functionality (including edge cases)
- [x] 2.2 Test sync behavior with cleanup to ensure consistency across devices

## 3. Documentation
- [x] 3.1 Update event-management spec with new requirement for automatic cleanup
- [x] 3.2 Update README with documentation for the new config option and warnings</content>
<parameter name="filePath">openspec/changes/delete-finished-events/tasks.md