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

#[derive(PartialEq)]
pub enum CurrentScreen {
    Main,
    NewGame,
    Exiting,
}

#[derive(PartialEq)]
pub enum NewGamePopups {
    NumberOfPlayers,
    PlayerNames,
}

#[derive(PartialEq)]
pub enum Popups {
    None,
    NewGamePopups(NewGamePopups),
}

pub struct State<'a> {
    pub game: Game<'a>,
    pub current_screen: CurrentScreen,
    pub current_popup: Popups,
    pub total_players: u8,
    pub input_buffer: String,
}

impl<'a> State<'a> {
    pub fn new(game: Game<'a>) -> Self {
        Self {
            game,
            current_screen: CurrentScreen::Main,
            current_popup: Popups::None,
            input_buffer: String::new(),
            total_players: 0,
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
        ui::draw_main_screen(frame, self);
    }

    fn handle_events(&mut self) -> io::Result<bool> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                return Ok(true);
            }
            match self.current_screen {
                CurrentScreen::NewGame => match self.current_popup {
                    Popups::NewGamePopups(NewGamePopups::NumberOfPlayers) => match key.code {
                        KeyCode::Delete => {
                            self.input_buffer.pop();
                        }
                        KeyCode::Backspace => {
                            self.input_buffer.pop();
                        }

                        KeyCode::Enter => {
                            if let Popups::NewGamePopups(NewGamePopups::NumberOfPlayers) =
                                self.current_popup
                            {
                                let total_players = self.input_buffer.parse().unwrap();
                                if total_players < 4 {
                                    self.set_popup_state(Popups::NewGamePopups(
                                        NewGamePopups::PlayerNames,
                                    ));
                                    self.total_players = total_players;
                                    self.input_buffer.clear();
                                } else {
                                    // error component
                                }
                            }
                        }

                        KeyCode::Char('q') => {
                            self.current_screen = CurrentScreen::Exiting;
                        }

                        _ => {
                            // check if the input is an number or not
                            match key.code.to_string().parse::<i32>() {
                                Ok(m) => {
                                    self.input_buffer.push_str(&key.code.to_string());
                                }
                                _ => {}
                            }
                        }
                    },
                    Popups::NewGamePopups(NewGamePopups::PlayerNames) => match key.code {
                        KeyCode::Delete => {
                            self.input_buffer.pop();
                        }
                        KeyCode::Backspace => {
                            self.input_buffer.pop();
                        }

                        KeyCode::Enter => match self.current_popup {
                            Popups::NewGamePopups(NewGamePopups::NumberOfPlayers) => {
                                self.set_popup_state(Popups::NewGamePopups(
                                    NewGamePopups::PlayerNames,
                                ));
                                self.total_players =
                                    self.input_buffer.parse().expect("Please enter");
                                self.input_buffer.clear();
                            }
                            Popups::NewGamePopups(NewGamePopups::PlayerNames) => {
                                match self.current_popup {
                                    Popups::NewGamePopups(NewGamePopups::PlayerNames) => {
                                        // self.game.add_player(&self.input_buffer);
                                        self.input_buffer.clear();
                                    }
                                    _ => {}
                                }
                            }

                            _ => {
                                self.input_buffer.push_str(&key.code.to_string());
                            }
                        },

                        _ => {}
                    },
                    _ => {}
                },

                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') => {
                        self.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },
                _ => {}
            }

            // insert logic to handle key events here
            match key.code {
                KeyCode::Char('q') => {
                    self.current_screen = CurrentScreen::Exiting;
                }
                KeyCode::Char('n') => {
                    self.set_current_screen_new_game();
                }
                _ => {}
            }
        }
        Ok(false)
    }

    pub fn set_current_screen_new_game(&mut self) {
        self.current_screen = CurrentScreen::NewGame;
        self.current_popup = Popups::NewGamePopups(NewGamePopups::NumberOfPlayers);
    }

    pub fn set_current_screen(&mut self, screen: CurrentScreen) {
        self.current_screen = screen;
    }

    pub fn set_popup_state(&mut self, popup: Popups) {
        self.current_popup = popup;
    }
}
