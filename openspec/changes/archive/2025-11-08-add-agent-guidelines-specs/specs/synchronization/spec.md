## ADDED Requirements

### Requirement: Provider Abstraction Pattern
Sync functionality MUST use a `SyncProvider` trait for extensibility.

#### Scenario: Provider Implementation
Given new sync method,
When implementing SyncProvider,
Then integrates with existing system.

### Requirement: Git Provider Implementation
Initial sync MUST use Git for version control and cross-device sync.

#### Scenario: Git-Based Sync
Given Git repository,
When syncing,
Then uses Git for version control.

### Requirement: System Git Integration
Sync MUST use system `git` commands via `std::process::Command` for SSH support.

#### Scenario: SSH Authentication
Given SSH-configured Git,
When syncing,
Then uses user's SSH setup.

### Requirement: Asynchronous Auto-Sync
Automatic sync operations MUST run asynchronously in background threads.

#### Scenario: Background Pull
Given app launch,
When pulling changes,
Then happens in background without blocking TUI.

#### Scenario: Background Push
Given save/delete operation,
When pushing changes,
Then happens asynchronously.

### Requirement: Configuration Persistence
Sync settings MUST be stored in `~/.config/rcal/config.toml`.

#### Scenario: Config Storage
Given sync configuration,
When saving,
Then persists in TOML file.

### Requirement: Graceful Error Handling
Sync operations MUST fail gracefully with user-friendly messages.

#### Scenario: Sync Failure
Given sync error,
When operation fails,
Then shows message without crashing.

### Requirement: Manual Conflict Resolution
No automatic conflict resolution; users MUST manually edit markdown files.

#### Scenario: Conflict Handling
Given merge conflict,
When syncing,
Then requires manual resolution in files.

### Requirement: Persistence Layer Integration
Sync MUST hook into persistence layer for optional auto-push on save/delete.

#### Scenario: Auto-Push on Save
Given event save,
When auto-push enabled,
Then pushes changes in background.

### Requirement: Daemon Sync Integration
Daemon MUST reload events via file watching after sync operations.

#### Scenario: Post-Sync Reload
Given sync completion,
When daemon watching,
Then reloads events automatically.

### Requirement: Security Model
Sync MUST rely on user's Git/SSH setup with no app-level secrets.

#### Scenario: Secure Authentication
Given user's Git config,
When syncing,
Then uses existing authentication.

### Requirement: Testing Strategy
Sync MUST have unit tests for provider logic and integration tests for workflows.

#### Scenario: Provider Testing
Given sync provider,
When unit testing,
Then logic is verified.

#### Scenario: Workflow Testing
Given sync workflow,
When integration testing,
Then end-to-end sync works.

## Cross-references
- See `event-management` for persistence integration