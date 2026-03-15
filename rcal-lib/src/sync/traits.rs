//! Sync provider traits for calendar synchronization.
//!
//! Defines the SyncProvider trait for implementing various sync backends.

use std::any::Any;
use std::error::Error;
use std::path::Path;

use crate::models::SyncStatus;

/// Trait for implementing synchronization providers.
///
/// Different implementations can provide synchronization with various
/// backends (Git, cloud storage, etc.)
pub trait SyncProvider: Send + Sync {
    /// Initializes the sync provider for a given path.
    fn init(&self, path: &Path) -> Result<(), Box<dyn Error>>;

    /// Pulls changes from the remote repository.
    fn pull(&self, path: &Path) -> Result<SyncStatus, Box<dyn Error>>;

    /// Pushes changes to the remote repository.
    fn push(&self, path: &Path) -> Result<SyncStatus, Box<dyn Error>>;

    /// Gets the current sync status.
    fn status(&self, path: &Path) -> Result<SyncStatus, Box<dyn Error>>;

    /// Returns self as Any for downcasting.
    fn as_any(&self) -> &dyn Any;
}
