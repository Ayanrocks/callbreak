use std::io::{self, stdout, Stdout};

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::Backend,
    Terminal,
};

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init() -> io::Result<Tui> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
    // restore terminal
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
