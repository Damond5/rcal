//! Notifications module - desktop notification support.
//!
//! Provides the Notifier trait for sending desktop notifications.
//! Platform-specific implementations are feature-gated.

pub mod daemon;

#[cfg(feature = "desktop-notifications")]
pub mod platform {
    //! Platform-specific notification implementation (Linux with D-Bus).

    use crate::notifications::Notifier;
    use std::error::Error;

    /// Desktop notification implementation using notify-rust.
    pub struct DesktopNotifier;

    impl DesktopNotifier {
        pub fn new() -> Self {
            Self
        }
    }

    impl Notifier for DesktopNotifier {
        fn notify(&self, title: &str, body: &str) -> Result<(), Box<dyn Error>> {
            notify_rust::Notification::new()
                .summary(title)
                .body(body)
                .show()
                .map(|_| ())
                .map_err(|e| e.into())
        }
    }

    impl Default for DesktopNotifier {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(not(feature = "desktop-notifications"))]
pub mod stub {
    //! Stub notification implementation when desktop notifications are disabled.

    use crate::notifications::Notifier;
    use std::error::Error;

    /// Stub notifier that does nothing.
    pub struct StubNotifier;

    impl StubNotifier {
        pub fn new() -> Self {
            Self
        }
    }

    impl Notifier for StubNotifier {
        fn notify(&self, _title: &str, _body: &str) -> Result<(), Box<dyn Error>> {
            // No-op implementation
            Ok(())
        }
    }

    impl Default for StubNotifier {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Trait for desktop notification providers.
pub trait Notifier: Send + Sync {
    /// Sends a notification with the given title and body.
    fn notify(&self, title: &str, body: &str) -> Result<(), Box<dyn std::error::Error>>;
}

/// Re-exports the appropriate notifier based on feature flags.
#[cfg(feature = "desktop-notifications")]
pub use platform::DesktopNotifier as DefaultNotifier;

/// Re-exports the stub notifier when desktop notifications are disabled.
#[cfg(not(feature = "desktop-notifications"))]
pub use stub::StubNotifier as DefaultNotifier;

pub use daemon::NotificationDaemon;
