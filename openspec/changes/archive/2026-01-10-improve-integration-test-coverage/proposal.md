# Change: Improve Integration Test Coverage

## Why

rcal has 100 integration tests covering basic CRUD operations and UI interactions, but lacks comprehensive testing for critical workflows including synchronization, notification daemon, recurring event series management, multi-day events, error handling, and edge cases. The project currently has no coverage tooling or established coverage standards, making it difficult to measure or maintain test quality over time.

## What Changes

- Add new integration tests for sync workflows (init, pull, push, status transitions)
- Add integration tests for daemon notification delivery workflow
- Add integration tests for recurring event series operations (edit/delete entire series)
- Add integration tests for multi-day events with start/end dates and times
- Add integration tests for cache invalidation after event modifications
- Add integration tests for view boundary adjustment during navigation
- Add integration tests for CLI argument parsing (--sync-init, --sync-pull, --sync-push, --sync-status, --daemon)
- Add integration tests for configuration file creation and parsing
- Add integration tests for error handling scenarios (sync failures, permission errors, corrupted files)
- Add integration tests for edge cases (leap years, month boundaries, year transitions, large event counts)
- Set up coverage measurement tooling using cargo-llvm-cov
- Establish coverage standards and documentation for the project

## Impact

- **Affected specs**: test-coverage, testing-framework
- **Affected code**:
  - tests/integration_test.rs (significant expansion)
  - src/sync.rs (test coverage improvements)
  - src/daemon.rs (test coverage improvements)
  - src/app.rs (test coverage improvements for state management)
  - src/event_handling.rs (test coverage improvements for complex workflows)
  - src/persistence.rs (test coverage for edge cases)
  - src/date_utils.rs (test coverage for date boundaries)
  - CI/CD configuration (new GitHub Actions workflow)
- **Breaking changes**: None
