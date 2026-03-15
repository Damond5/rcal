//! Sync module - synchronization providers for calendar data.
//!
//! Provides traits and implementations for syncing calendar events
//! with remote repositories (e.g., Git).

pub mod git_sync;
pub mod traits;

pub use git_sync::GitSyncProvider;
pub use traits::SyncProvider;
