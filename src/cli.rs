use std::io;

mod state;
mod tui;

use state::State;

pub struct CLI {}

use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};

impl CLI {
    pub fn new_cli() -> io::Result<()> {
        color_eyre::install().expect("Error Unwrapping color eyre");
        let mut terminal = tui::init()?;
        let app_result = State::default().run(&mut terminal);
        if let Err(err) = tui::restore() {
            eprintln!(
                "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
                err
            );
        }
        tui::restore()?;
        app_result
    }
}
