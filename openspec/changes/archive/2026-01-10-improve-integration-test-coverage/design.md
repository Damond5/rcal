## Context

rcal currently has 100 integration tests and 34 unit tests, providing good coverage of basic CRUD operations and UI interactions. However, critical workflows such as synchronization, daemon notification delivery, recurring event series management, multi-day events, cache invalidation, and error handling lack comprehensive test coverage. Additionally, the project lacks coverage measurement tooling and established coverage standards, making it difficult to track and maintain test quality.

The testing strategy follows existing patterns using:
- Integration tests in `tests/` directory with TestBackend for TUI testing
- Unit tests in `#[cfg(test)]` modules within source files
- tempfile for isolated file system operations

## Goals / Non-Goals

### Goals
- Increase integration test coverage from 100 to 200+ tests targeting critical missing workflows
- Add comprehensive tests for synchronization, daemon notifications, recurring events, multi-day events, and error handling
- Establish coverage measurement tooling using cargo-llvm-cov
- Define and enforce coverage standards (80% for core logic, 60% for TUI code)
- Document testing best practices and coverage targets

### Non-Goals
- Achieving 100% coverage (not feasible for TUI/CLI code)
- Rewriting existing tests (focus on adding new tests for missing coverage)
- Adding performance benchmarking tests (out of scope)
- Adding property-based testing (future consideration, not now)

## Decisions

### Decision 1: Use cargo-llvm-cov for coverage measurement

**What**: Choose cargo-llvm-cov as the primary coverage tool instead of cargo-tarpaulin.

**Why**:
- Officially recommended by Rust Project Primer
- Uses LLVM's source-based code coverage (-C instrument-coverage)
- Actively maintained with recent releases
- Better support for doctests and multiple output formats

**Alternatives considered**:
- **cargo-tarpaulin**: Mature and Linux-focused, but may have inaccuracies with unique package/feature combinations
- **Manual coverage tracking**: Too error-prone and lacks automation

### Decision 2: Target 80% coverage for core logic, 60% for TUI code

**What**: Set tiered coverage targets based on code type rather than a single global target.

**Why**:
- Core business logic (persistence, date handling, sync) is critical and should have high coverage
- TUI/CLI code (ui.rs, main.rs) is harder to test due to terminal I/O dependencies
- Industry standard for library crates is 80-100%, binary crates is 60-80%

**Alternatives considered**:
- **100% coverage for all code**: Unrealistic for TUI applications and would require extensive mocking
- **50% coverage minimum**: Too low to provide confidence in critical paths

### Decision 3: Use tempfile for all file system tests

**What**: Continue and expand use of tempfile crate for all file system operations in tests.

**Why**:
- Standard practice in Rust projects
- Provides automatic cleanup of test artifacts
- Isolated test environments prevent test interference
- Already used in existing test suite

**Alternatives considered**:
- **assert_fs**: More feature-rich but tempfile is already a dependency and sufficient
- **In-memory file systems**: Would require mocking the entire persistence layer

### Decision 4: Test workflows end-to-end rather than mocking dependencies

**What**: Test synchronization and notification workflows using real Git operations and file watching where possible, only mocking when necessary (e.g., desktop notifications).

**Why**:
- Tests integration between components, not just individual functions
- Catches real-world issues that mocking would hide
- Existing patterns in rcal already use real operations (tempfile, git commands)

**Alternatives considered**:
- **Mock all external dependencies**: Would catch fewer integration issues
- **Full system tests with installed rcal**: Too slow for rapid development and hard to automate in CI

## Risks / Trade-offs

### Risk 1: Sync tests require Git setup and may be flaky in CI

**Mitigation**:
- Use tempfile to create isolated Git repositories
- Mock network errors for failure scenarios instead of relying on actual network failures
- Use Git's `file://` protocol for remote URLs to avoid network dependencies

### Risk 2: Daemon tests may be slow or platform-dependent

**Mitigation**:
- Extract notification logic into testable functions (already partially done)
- Skip actual D-Bus tests in CI (no desktop environment)
- Use trait abstraction for notification service to enable mocking

### Risk 3: Test suite execution time increases significantly

**Mitigation**:
- Use tokio::test for async operations to avoid actual delays
- Leverage existing test infrastructure and helpers
- Consider test grouping (unit vs integration) for faster feedback loops

### Risk 4: Coverage tools may produce inconsistent results

**Mitigation**:
- Document exact commands and versions used
- Use cargo-llvm-cov which is more reliable than alternatives
- Track trends over time rather than absolute values

### Trade-off: More tests vs development speed

Adding 100+ integration tests increases test suite execution time, but provides:
- Higher confidence in refactoring
- Fewer bugs in production
- Better documentation of intended behavior

## Migration Plan

### Phase 1: Setup and Tooling (1-2 days)
1. Install cargo-llvm-cov and add to dev-dependencies
2. Create initial coverage baseline
3. Document coverage targets and measurement process

### Phase 2: Critical Workflow Tests (1-2 weeks)
1. Implement sync workflow tests (init, pull, push, status transitions)
2. Implement daemon notification tests
3. Implement recurring event series tests
4. Implement multi-day event tests

### Phase 3: State and View Tests (3-5 days)
1. Implement cache invalidation tests
2. Implement view boundary adjustment tests
3. Implement CLI and configuration tests

### Phase 4: Error and Edge Cases (3-5 days)
1. Implement error handling tests (sync failures, file errors)
2. Implement date/time edge case tests
3. Implement complex workflow integration tests

### Phase 5: Documentation and Validation (1-2 days)
1. Update specs and documentation
2. Run full test suite and generate coverage report
3. Verify coverage targets met
4. Update README with coverage information

## Open Questions

- Should coverage thresholds be enforced in CI (blocking PRs) or informational only?
  - **Recommendation**: Start informational, consider enforcing after 3 months of data
- Should test organization be refactored (e.g., separate test files by module)?
  - **Recommendation**: Keep single integration_test.rs for now, consider splitting if it exceeds 2000 lines
- Should property-based testing (proptest) be added for date utilities?
  - **Recommendation**: Defer to future enhancement, focus on deterministic tests first
