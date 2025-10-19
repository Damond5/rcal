use std::{error::Error, io};

use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use rcal::app::App;
use rcal::daemon;
use rcal::event_handling::run_app;
use rcal::persistence;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run as daemon for notifications
    #[arg(long)]
    daemon: bool,
}



fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

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
    app.events = persistence::load_events();
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
