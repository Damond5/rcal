//! Daemon module - runs the background notification daemon.

use std::error::Error;
use std::thread;
use std::time::Duration;

use rcal_lib::storage::FileEventRepository;
use rcal_lib::{
    notifications::DefaultNotifier, notifications::NotificationDaemon, EventRepository,
};

pub fn run_daemon() -> Result<(), Box<dyn Error>> {
    let repository = FileEventRepository::with_default_path()?;
    let notifier = Box::new(DefaultNotifier::new());
    let mut daemon = NotificationDaemon::new(notifier);

    // Load initial events
    daemon.set_events(repository.load().unwrap_or_else(|e| {
        eprintln!("Failed to load initial events: {e}");
        Vec::new()
    }));

    loop {
        daemon.check_and_notify();

        // Reload events
        daemon.set_events(repository.load().unwrap_or_else(|e| {
            eprintln!("Failed to load events: {e}");
            Vec::new()
        }));

        thread::sleep(Duration::from_secs(60));
    }
}
