use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Rect},
    prelude::Backend,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    Frame, Terminal,
};
use std::io;

use crate::game::Game;

use super::ui;

enum CurrentScreen {
    Main,
    Playing,
    Exiting,
}

pub struct State<'a> {
    game: Game<'a>,
    current_screen: CurrentScreen,
}

impl<'a> State<'a> {
    pub fn new(game: Game<'a>) -> Self {
        Self {
            game,
            current_screen: CurrentScreen::Main,
        }
    }

    pub fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<bool> {
        loop {
            terminal.draw(|f| self.render_frame(f))?;
            let exit_result = self.handle_events();

            match exit_result {
                Ok(true) => {
                    return Ok(true);
                }
                Err(err) => {
                    return Err(err);
                }
                _ => {}
            }
        }
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        ui::draw_main_screen(frame);
    }

    fn handle_events(&mut self) -> io::Result<bool> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                return Ok(true);
            }

            // insert logic to handle key events here
            if let KeyCode::Char('q') = key.code {
                self.current_screen = CurrentScreen::Exiting;
                return Ok(true);
            } 
        }
        Ok(false)
    }
}
