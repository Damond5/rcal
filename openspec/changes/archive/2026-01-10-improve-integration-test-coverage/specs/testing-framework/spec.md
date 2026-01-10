## ADDED Requirements

### Requirement: Coverage Measurement Integration
Test suite MUST integrate coverage measurement tooling for tracking test quality.

#### Scenario: Coverage Tool Installation
- **GIVEN** rcal development environment
- **WHEN** running `cargo install cargo-llvm-cov`
- **THEN** coverage measurement tool is available

#### Scenario: Coverage Report Generation
- **GIVEN** test suite exists
- **WHEN** running `cargo llvm-cov --html --open`
- **THEN** HTML coverage report is generated and displayed

### Requirement: Coverage Standards and Enforcement
Test coverage MUST meet documented standards for different code types.

#### Scenario: Core Logic Coverage Target
- **GIVEN** core business logic modules (persistence, date_utils, sync, app, event_handling)
- **WHEN** measuring coverage
- **THEN** target is 80% line coverage

#### Scenario: TUI Code Coverage Target
- **GIVEN** TUI/CLI modules (ui.rs, main.rs, daemon.rs)
- **WHEN** measuring coverage
- **THEN** target is 60% line coverage

#### Scenario: Coverage Baseline Establishment
- **GIVEN** existing test suite
- **WHEN** implementing new tests
- **THEN** coverage baseline is documented and improvements are tracked

### Requirement: Workflow Testing Pattern
Complex user workflows MUST be tested end-to-end using setup-action-assert pattern.

#### Scenario: Complete User Workflow Test
- **GIVEN** isolated test environment with temp directory
- **WHEN** executing sequence of operations (view → add → edit → delete)
- **THEN** each step completes successfully and final state is verified

#### Scenario: Workflow State Verification
- **GIVEN** workflow test is executing
- **WHEN** each operation completes
- **THEN** application state is verified before proceeding to next step

### Requirement: External Dependency Testing Strategy
Tests involving external dependencies (Git, D-Bus, file system) MUST use appropriate testing strategies.

#### Scenario: Git Operations Testing
- **GIVEN** test needs Git functionality
- **WHEN** testing sync operations
- **THEN** use tempfile for isolated Git repositories or `file://` protocol for local remotes

#### Scenario: Notification System Testing
- **GIVEN** test needs notification functionality
- **WHEN** testing daemon notifications
- **THEN** use trait abstraction to mock NotificationService or skip D-Bus tests in CI

#### Scenario: File System Operations Testing
- **GIVEN** test needs file system access
- **WHEN** performing file operations
- **THEN** use tempfile crate for isolated, auto-cleaning test environments

### Requirement: Complex State Integration Testing
State management across components MUST be tested in integration tests.

#### Scenario: Cross-Component State Transitions
- **GIVEN** multiple components (app state, persistence, sync)
- **WHEN** performing operation affecting multiple components
- **THEN** all components reflect correct state after operation

#### Scenario: Cache Consistency Verification
- **GIVEN** application uses instance caching
- **WHEN** events are modified
- **THEN** cache is invalidated and re-generated correctly
