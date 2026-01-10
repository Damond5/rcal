use std::{error::Error, fs, io, path::PathBuf, sync::mpsc, thread};

use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use rcal::app::App;
use rcal::daemon;
use rcal::event_handling::run_app;
use rcal::persistence;
use rcal::sync::{GitSyncProvider, SyncProvider};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run as daemon for notifications
    #[arg(long)]
    daemon: bool,

    /// Initialize sync with remote URL
    #[arg(long, value_name = "URL")]
    sync_init: Option<String>,

    /// Pull from remote
    #[arg(long)]
    sync_pull: bool,

    /// Push to remote
    #[arg(long)]
    sync_push: bool,

    /// Check sync status
    #[arg(long)]
    sync_status: bool,
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| dirs::home_dir().unwrap());
    path.push("rcal");
    fs::create_dir_all(&path).unwrap();
    path.push("config.toml");
    path
}

fn load_config() -> toml::Value {
    let config_path = get_config_path();
    if config_path.exists() {
        match fs::read_to_string(&config_path) {
            Ok(content) => match toml::from_str(&content) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Warning: Failed to parse config.toml: {e}. Using defaults.");
                    toml::Value::Table(Default::default())
                }
            },
            Err(e) => {
                eprintln!("Warning: Failed to read config.toml: {e}. Using defaults.");
                toml::Value::Table(Default::default())
            }
        }
    } else {
        toml::Value::Table(Default::default())
    }
}

fn load_remote_url() -> Option<String> {
    let config = load_config();
    config
        .get("sync")?
        .get("remote")?
        .as_str()
        .map(|s| s.to_string())
}

fn should_auto_cleanup_old_events() -> bool {
    let config = load_config();
    config
        .get("auto_cleanup_old_events")
        .and_then(|v| v.as_bool())
        .unwrap_or(true)
}

fn save_remote_url(url: &str) -> Result<(), Box<dyn Error>> {
    let mut config = load_config();
    if let Some(table) = config.as_table_mut() {
        table
            .entry("sync")
            .or_insert_with(|| toml::Value::Table(Default::default()));
        if let Some(sync_table) = table.get_mut("sync").and_then(|v| v.as_table_mut()) {
            sync_table.insert("remote".to_string(), toml::Value::String(url.to_string()));
        }
    }
    let config_str = toml::to_string(&config)?;
    let config_path = get_config_path();
    fs::write(config_path, config_str)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Handle sync commands
    if let Some(url) = args.sync_init {
        let provider = GitSyncProvider::new(url.clone());
        let home = dirs::home_dir().expect("Could not find home directory");
        let calendar_dir = home.join("calendar");
        provider.init(&calendar_dir)?;
        save_remote_url(&url)?;
        println!("Sync initialized with remote: {url}");
        return Ok(());
    }

    if args.sync_pull || args.sync_push || args.sync_status {
        if let Some(url) = load_remote_url() {
            let provider = GitSyncProvider::new(url);
            let home = dirs::home_dir().expect("Could not find home directory");
            let calendar_dir = home.join("calendar");
            if args.sync_pull {
                provider.pull(&calendar_dir)?;
                println!("Pulled from remote");
            } else if args.sync_push {
                provider.push(&calendar_dir)?;
                println!("Pushed to remote");
            } else if args.sync_status {
                let status = provider.status(&calendar_dir)?;
                println!("Status: {status:?}");
            }
        } else {
            eprintln!("No sync remote configured. Use --sync-init <url> first.");
        }
        return Ok(());
    }

    if args.daemon {
        daemon::run_daemon()?;
        return Ok(());
    }

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    app.events = persistence::load_events_from_path(&app.calendar_dir).unwrap_or_else(|e| {
        eprintln!("Failed to load events: {e}");
        Vec::new()
    });

    // Auto cleanup old events if enabled
    if should_auto_cleanup_old_events() {
        match persistence::cleanup_old_events(&app.calendar_dir, app.sync_provider.as_deref()) {
            Ok(_) => {
                // Reload events to reflect deletions
                app.events =
                    persistence::load_events_from_path(&app.calendar_dir).unwrap_or_else(|e| {
                        eprintln!("Failed to reload events after cleanup: {e}");
                        Vec::new()
                    });
            }
            Err(e) => eprintln!("Auto cleanup failed: {e}"),
        }
    }
    let (tx, rx) = mpsc::channel::<Result<(), String>>();
    app.reload_receiver = Some(rx);
    if let Some(url) = load_remote_url() {
        app.sync_provider = Some(Box::new(GitSyncProvider::new(url.clone())));
        // Spawn async pull on launch
        let provider = GitSyncProvider::new(url);
        let calendar_dir = app.calendar_dir.clone();
        let tx_clone = tx.clone();
        thread::spawn(move || {
            match provider.pull(&calendar_dir) {
                Ok(_) => {
                    // Send reload signal
                    let _ = tx_clone.send(Ok(()));
                }
                Err(e) => {
                    // Send error signal
                    let _ = tx_clone.send(Err(e.to_string()));
                }
            }
        });
    }
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}")
    }

    Ok(())
}
