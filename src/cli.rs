use std::{io, panic::{set_hook, take_hook}};

mod state;
mod tui;
mod ui;

use state::State;

pub struct CLI {}

use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};

use crate::game::Game;

impl CLI {
    pub fn new_cli() -> io::Result<()> {
        Self::init_panic_hook();
        color_eyre::install().expect("Error Unwrapping color eyre");
        let mut terminal = tui::init()?;
        // initialize a new game
        let game = Game::new_game();
        let mut new_app_state = State::new(game);
        let app_result = new_app_state.run_app(&mut terminal);
        if let Err(err) = app_result {
            println!("{err:?}");
        }
        if let Err(err) = tui::restore() {
            eprintln!(
                "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
                err
            );
        }

        // app_result?
        Ok(())
    }

    pub fn init_panic_hook() {
        let original_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            // intentionally ignore errors here since we're already in a panic
            let _ = tui::restore();
            original_hook(panic_info);
        }));
    }
}
