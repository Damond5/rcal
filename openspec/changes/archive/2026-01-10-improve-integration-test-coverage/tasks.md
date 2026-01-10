## 1. Setup and Tooling

- [ ] 1.1 Install cargo-llvm-cov for coverage measurement
- [ ] 1.2 Add coverage dependencies to Cargo.toml dev-dependencies
- [ ] 1.3 Create initial coverage baseline measurement
- [ ] 1.4 Document coverage targets (80% for core logic, 60% for TUI code)
- [ ] 1.5 Create coverage measurement script or makefile target
- [ ] 1.6 Add coverage badges to README.md

## 2. Synchronization Workflow Tests

- [ ] 3.1 Add test for full sync workflow (init → pull → modify → push)
- [ ] 3.2 Add test for automatic pull on application startup
- [ ] 3.3 Add test for sync status transitions (UpToDate → Ahead → Behind)
- [ ] 3.4 Add test for Git repository initialization with remote
- [ ] 3.5 Add test for async sync operations in background threads
- [ ] 3.6 Add test for sync error handling (network failures, auth errors)
- [ ] 3.7 Add test for multiple concurrent sync operations
- [ ] 3.8 Add test for sync popup workflow (status display → pull/push)

## 3. Daemon Notification Tests

- [ ] 3.1 Add test for daemon startup and initialization
- [ ] 3.2 Add test for real-time notification delivery with upcoming events
- [ ] 3.3 Add test for all-day event notification timing (midday before)
- [ ] 3.4 Add test for timed event notification timing (30 minutes before)
- [ ] 3.5 Add test for notification deduplication across sessions
- [ ] 3.6 Add test for file watching triggering event reloads
- [ ] 3.7 Add test for event file modification handling
- [ ] 3.8 Add test for daemon recovery from errors
- [ ] 3.9 Add test for D-Bus unavailable scenario

## 4. Recurring Event Series Tests

- [ ] 4.1 Add test for editing base event updates all instances
- [ ] 4.2 Add test for deleting recurring series from instance
- [ ] 4.3 Add test for recurring event with multi-day duration
- [ ] 4.4 Add test for Feb 29th yearly recurrence in leap years
- [ ] 4.5 Add test for Feb 29th yearly recurrence fallback in non-leap years
- [ ] 4.6 Add test for Jan 31st monthly recurrence into February
- [ ] 4.7 Add test for cross-year recurring events

## 5. Multi-Day Event Tests

- [ ] 5.1 Add test for multi-day event creation with start/end dates and times
- [ ] 5.2 Add test for multi-day event persistence round-trip (save → load → verify)
- [ ] 5.3 Add test for multi-day event with same start/end time (all-day multi-day)
- [ ] 5.4 Add test for multi-day event spanning month boundary
- [ ] 5.5 Add test for multi-day event spanning year boundary
- [ ] 5.6 Add test for displaying multi-day events in calendar view

## 6. Cache and State Management Tests

- [ ] 6.1 Add test for cache invalidation after event addition
- [ ] 6.2 Add test for cache invalidation after event editing
- [ ] 6.3 Add test for cache invalidation after event deletion
- [ ] 6.4 Add test for selective cache invalidation (per-event vs full)
- [ ] 6.5 Add test for instance cache refresh after modifications
- [ ] 6.6 Add test for sync state transitions after operations

## 7. View Boundary Adjustment Tests

- [ ] 7.1 Add test for view boundary forward shift when navigating beyond current 3-month view
- [ ] 7.2 Add test for view boundary backward shift when navigating backwards
- [ ] 7.3 Add test for view adjustment across month boundaries
- [ ] 7.4 Add test for view adjustment across year boundaries
- [ ] 7.5 Add test for date range calculations for three-month view

## 8. CLI and Configuration Tests

- [ ] 8.1 Add test for --sync-init CLI argument parsing
- [ ] 8.2 Add test for --sync-pull CLI argument parsing
- [ ] 8.3 Add test for --sync-push CLI argument parsing
- [ ] 8.4 Add test for --sync-status CLI argument parsing
- [ ] 8.5 Add test for --daemon CLI argument parsing
- [ ] 8.6 Add test for configuration file creation on first run
- [ ] 8.7 Add test for configuration file parsing and validation
- [ ] 8.8 Add test for auto-cleanup old events on startup

## 9. Error Handling Tests

- [ ] 9.1 Add test for sync network error handling
- [ ] 9.2 Add test for sync authentication failure handling
- [ ] 9.3 Add test for repository not found error handling
- [ ] 9.4 Add test for merge conflict error handling
- [ ] 9.5 Add test for file system permission denied error handling
- [ ] 9.6 Add test for corrupted markdown file handling in persistence
- [ ] 9.7 Add test for duplicate event file handling
- [ ] 9.8 Add test for large numbers of events performance

## 10. Date and Time Edge Cases

- [ ] 10.1 Add test for leap year handling (Feb 29 validation)
- [ ] 10.2 Add test for non-leap year Feb 29 rejection
- [ ] 10.3 Add test for month boundary transitions (Jan 31 → Feb, etc.)
- [ ] 10.4 Add test for year boundary handling (Dec 31 → Jan)
- [ ] 10.5 Add test for invalid date inputs (Feb 30, April 31)
- [ ] 10.6 Add test for empty input handling in date suggestions
- [ ] 10.7 Add test for multiple date suggestions matching same prefix

## 11. Complex Workflow Integration Tests

- [ ] 11.1 Add test for complete user workflow: view → add → edit → delete
- [ ] 11.2 Add test for recurring event lifecycle: create series → edit → delete instance → verify series deleted
- [ ] 11.3 Add test for sync workflow: init repo → pull → modify → push → verify
- [ ] 11.4 Add test for multi-day recurring event workflow
- [ ] 11.5 Add test for conflict resolution scenario

## 12. Documentation and Validation

- [ ] 12.1 Document coverage standards in AGENTS.md
- [ ] 12.2 Update test-coverage spec with new requirements
- [ ] 12.3 Update testing-framework spec with coverage tooling requirements
- [ ] 12.4 Run full test suite and ensure all new tests pass
- [ ] 12.5 Generate final coverage report and verify targets met
- [ ] 12.6 Update README with coverage badge and testing documentation
