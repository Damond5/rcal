# Change: Update Sync Status on Push/Pull

## Why
After performing push or pull operations, the sync status displayed in the UI may become stale, as the operations modify the repository state but the status is not refreshed automatically. This can lead to confusion for users who expect the status to reflect the latest sync state.

## What Changes
- **BREAKING**: Modify the `SyncProvider` trait to return `SyncStatus` from `push` and `pull` methods (requires updates to any custom implementations)
- Update the `GitSyncProvider` implementation to fetch and return the updated status after operations
- Ensure UI components refresh the displayed sync status after push/pull operations

## Impact
- Affected specs: synchronization
- Affected code: src/sync.rs, src/ui.rs (event handling and status display)