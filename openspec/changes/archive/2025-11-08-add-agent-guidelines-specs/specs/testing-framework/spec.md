## ADDED Requirements

### Requirement: Test Addition for New Features
New functionality MUST include accompanying tests.

#### Scenario: Feature Implementation
Given a new feature is implemented,
When tests are added for the feature,
Then functionality is verified.

### Requirement: Integration Testing Approach
Tests MUST use `cargo test` with `ratatui::backend::TestBackend` for TUI testing.

#### Scenario: TUI Test Execution
Given the application TUI,
When using TestBackend for testing,
Then UI interactions can be simulated.

### Requirement: Event Simulation
User interactions MUST be tested by simulating `crossterm::event::Event` inputs.

#### Scenario: Event Input Testing
Given a user interaction,
When simulating the corresponding Event,
Then the interaction is tested.

### Requirement: State Verification
Tests MUST assert application state changes and UI behavior including mode transitions.

#### Scenario: State Change Assertion
Given an operation,
When verifying state after the operation,
Then correct state transitions occur.

### Requirement: Workflow Testing
Complete user workflows MUST be tested end-to-end.

#### Scenario: Workflow Execution
Given a user workflow (e.g., view → add → delete),
When testing the complete flow,
Then all steps work correctly.

### Requirement: External Tool Avoidance
Tests MUST not use external tools like `xdotool`; keep within Rust ecosystem.

#### Scenario: Pure Rust Testing
Given test scenarios,
When using only Rust testing tools,
Then tests remain isolated.

### Requirement: Daemon Testing Isolation
Notification logic MUST be isolated into testable functions; avoid testing infinite loops.

#### Scenario: Isolated Notification Testing
Given notification logic,
When extracted into testable functions,
Then daemon behavior can be verified.

### Requirement: Event Handler Separation
`handle_event()` function MUST be extracted for isolated testing.

#### Scenario: Handler Testing
Given event handling logic,
When separated into handle_event,
Then events can be tested independently.

### Requirement: Library and Binary Structure
Crate MUST be both library and binary (`[lib]` and `[[bin]]` in Cargo.toml) to expose modules for testing.

#### Scenario: Module Exposure
Given the crate structure,
When configured as [lib] and [[bin]],
Then internal modules are testable.

### Requirement: Debug Traits
Enums used in tests MUST have `#[derive(Debug)]`.

#### Scenario: Debug Formatting
Given enums in tests,
When deriving Debug,
Then assertion formatting works.

### Requirement: Comprehensive Coverage
All user interactions, state transitions, and edge cases MUST be tested.

#### Scenario: Edge Case Testing
Given potential edge cases,
When tests cover them,
Then robustness is ensured.

### Requirement: Test Organization
Integration tests MUST be placed in `tests/` directory with descriptive names.

#### Scenario: Test File Location
Given integration tests,
When placed in tests/ directory,
Then they are properly organized.

### Requirement: Test Isolation
Each test MUST be independent and not rely on other tests.

#### Scenario: Independent Execution
Given multiple tests,
When run in any order,
Then they pass consistently.

### Requirement: Event Simulation Pattern
Tests MUST use `Event::Key(KeyEvent::from(KeyCode::Char('a')))` pattern.

#### Scenario: Consistent Simulation
Given key input testing,
When using the standard pattern,
Then inputs are simulated correctly.

### Requirement: Workflow Coverage
Tests MUST cover complete user journeys including error cases.

#### Scenario: Error Path Testing
Given error scenarios,
When tested in workflows,
Then error handling is verified.