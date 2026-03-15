//! SyncStatus enum representing the state of calendar synchronization.

use serde::{Deserialize, Serialize};

/// Represents the synchronization status with a remote calendar repository.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SyncStatus {
    /// Local and remote are up to date.
    UpToDate,
    /// Local has changes not yet pushed to remote.
    Ahead,
    /// Remote has changes not yet pulled to local.
    Behind,
    /// Both local and remote have conflicting changes.
    Conflicts,
    /// An error occurred during synchronization.
    Error(String),
}

impl SyncStatus {
    /// Returns true if the status represents an error condition.
    pub fn is_error(&self) -> bool {
        matches!(self, SyncStatus::Error(_))
    }

    /// Returns a human-readable description of the status.
    pub fn description(&self) -> String {
        match self {
            SyncStatus::UpToDate => "Up to date".to_string(),
            SyncStatus::Ahead => "Ahead of remote".to_string(),
            SyncStatus::Behind => "Behind remote".to_string(),
            SyncStatus::Conflicts => "Conflicting changes".to_string(),
            SyncStatus::Error(msg) => format!("Error: {msg}"),
        }
    }

    /// Returns true if synchronization is possible (no conflicts or errors).
    pub fn can_sync(&self) -> bool {
        !matches!(self, SyncStatus::Conflicts | SyncStatus::Error(_))
    }
}

impl std::fmt::Display for SyncStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl std::error::Error for SyncStatus {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_status_is_error() {
        assert!(!SyncStatus::UpToDate.is_error());
        assert!(!SyncStatus::Ahead.is_error());
        assert!(!SyncStatus::Behind.is_error());
        assert!(!SyncStatus::Conflicts.is_error());
        assert!(SyncStatus::Error("test".to_string()).is_error());
    }

    #[test]
    fn test_sync_status_can_sync() {
        assert!(SyncStatus::UpToDate.can_sync());
        assert!(SyncStatus::Ahead.can_sync());
        assert!(SyncStatus::Behind.can_sync());
        assert!(!SyncStatus::Conflicts.can_sync());
        assert!(!SyncStatus::Error("test".to_string()).can_sync());
    }

    #[test]
    fn test_sync_status_display() {
        assert_eq!(SyncStatus::UpToDate.to_string(), "Up to date");
        assert_eq!(SyncStatus::Ahead.to_string(), "Ahead of remote");
        assert_eq!(
            SyncStatus::Error("connection failed".to_string()).to_string(),
            "Error: connection failed"
        );
    }
}
