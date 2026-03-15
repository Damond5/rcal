//! Git-based synchronization provider.
//!
//! Implements the SyncProvider trait using Git for calendar synchronization.

use std::any::Any;
use std::error::Error;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::models::SyncStatus;
use crate::sync::traits::SyncProvider;

/// Git-based sync provider for calendar synchronization.
pub struct GitSyncProvider {
    /// URL of the remote repository.
    pub remote_url: String,
    /// Branch to sync with.
    pub branch: String,
}

impl GitSyncProvider {
    /// Creates a new GitSyncProvider with the given remote URL.
    pub fn new(remote_url: String) -> Self {
        Self {
            remote_url,
            branch: "main".to_string(),
        }
    }

    /// Creates a new GitSyncProvider with the given remote URL and branch.
    pub fn with_branch(mut self, branch: &str) -> Self {
        self.branch = branch.to_string();
        self
    }
}

impl SyncProvider for GitSyncProvider {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn init(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        // Init repo if not exists
        if !path.join(".git").exists() {
            let status = Command::new("git")
                .args(["init"])
                .current_dir(path)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()?;
            if !status.success() {
                return Err("Git init failed".into());
            }
        }

        // Check if remote exists
        let remote_check = Command::new("git")
            .args(["remote", "get-url", "origin"])
            .current_dir(path)
            .output();
        if remote_check.is_err() || !remote_check.as_ref().unwrap().status.success() {
            // Add remote
            let status = Command::new("git")
                .args(["remote", "add", "origin", &self.remote_url])
                .current_dir(path)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()?;
            if !status.success() {
                return Err("Git remote add failed".into());
            }
        }

        // Fetch
        let output = Command::new("git")
            .args(["fetch", "origin"])
            .current_dir(path)
            .stdout(Stdio::null())
            .output()?;
        if !output.status.success() {
            return Err(format!(
                "Git fetch failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        Ok(())
    }

    fn pull(&self, path: &Path) -> Result<SyncStatus, Box<dyn Error>> {
        // Use git command for pull with rebase to avoid conflicts
        let output = Command::new("git")
            .args(["pull", "--rebase", "origin", &self.branch])
            .current_dir(path)
            .stdout(Stdio::null())
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("conflict") {
                return Err("Merge conflicts detected. Please resolve manually.".into());
            }
            return Err(format!("Git pull failed: {stderr}").into());
        }

        // Return updated status after successful pull
        self.status(path)
    }

    fn push(&self, path: &Path) -> Result<SyncStatus, Box<dyn Error>> {
        // Add all changes
        let status = Command::new("git")
            .args(["add", "."])
            .current_dir(path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        if !status.success() {
            return Err("Git add failed".into());
        }

        // Check if there are changes
        let diff_output = Command::new("git")
            .args(["diff", "--cached", "--quiet"])
            .current_dir(path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        if diff_output.success() {
            // No changes to commit, return current status
            return self.status(path);
        }

        // Commit
        let output = Command::new("git")
            .args(["commit", "-m", "Sync events"])
            .current_dir(path)
            .stdout(Stdio::null())
            .output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stderr.contains("nothing to commit") {
                return Err(format!("Git commit failed: {stderr}").into());
            }
        }

        // Push
        let output = Command::new("git")
            .args(["push", "-u", "origin", &self.branch])
            .current_dir(path)
            .stdout(Stdio::null())
            .output()?;
        if !output.status.success() {
            return Err(format!(
                "Git push failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        // Return updated status after successful push
        self.status(path)
    }

    fn status(&self, path: &Path) -> Result<SyncStatus, Box<dyn Error>> {
        // Check if repo exists
        if !path.join(".git").exists() {
            return Ok(SyncStatus::Error("Not a git repository".to_string()));
        }

        // Get status
        let output = Command::new("git")
            .args(["status", "-sb"])
            .current_dir(path)
            .output()?;
        if !output.status.success() {
            return Ok(SyncStatus::Error("Failed to get git status".to_string()));
        }

        let status = String::from_utf8_lossy(&output.stdout);
        if status.contains("ahead") && status.contains("behind") {
            Ok(SyncStatus::Conflicts)
        } else if status.contains("ahead") {
            Ok(SyncStatus::Ahead)
        } else if status.contains("behind") {
            Ok(SyncStatus::Behind)
        } else if status.contains(&format!("origin/{}", self.branch)) {
            Ok(SyncStatus::UpToDate)
        } else {
            Ok(SyncStatus::Error("No remote branch found".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_sync_status_no_repo() {
        let temp_dir = TempDir::new().unwrap();
        let provider = GitSyncProvider::new("https://example.com/repo.git".to_string());
        let result = provider.status(temp_dir.path());
        assert_eq!(
            result.unwrap(),
            SyncStatus::Error("Not a git repository".to_string())
        );
    }

    #[test]
    fn test_sync_provider_new() {
        let provider = GitSyncProvider::new("https://example.com/repo.git".to_string());
        assert_eq!(provider.remote_url, "https://example.com/repo.git");
        assert_eq!(provider.branch, "main");
    }

    #[test]
    fn test_sync_provider_with_branch() {
        let provider =
            GitSyncProvider::new("https://example.com/repo.git".to_string()).with_branch("develop");
        assert_eq!(provider.branch, "develop");
    }
}
