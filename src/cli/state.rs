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
use std::{fmt::Display, io};

use crate::{
    game::{Call, Game},
    player::Player,
};

use super::ui;

#[derive(PartialEq)]
pub enum CurrentScreen {
    Main,
    NewGame,
    Exiting,
}

#[derive(PartialEq, Debug)]
pub enum PlayerNamePopups {
    PlayerName,
    PlayerPin,
    PlayerCall,
}

#[derive(PartialEq, Debug)]
pub enum NewGamePopups {
    NumberOfPlayers,
    PlayerNames(PlayerNamePopups),
}

#[derive(PartialEq, Debug)]
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
    pub temp_player_name: String,
    pub temp_player_pin: u16,
    pub temp_player_call: u8,
    pub error: String,
}

impl Display for Popups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Popups::None => write!(f, "None"),
            Popups::NewGamePopups(popup) => write!(f, "NewGame({:?})", popup),
        }
    }
}

impl<'a> State<'a> {
    pub fn new(game: Game<'a>) -> Self {
        Self {
            game,
            current_screen: CurrentScreen::Main,
            current_popup: Popups::None,
            input_buffer: String::new(),
            total_players: 0,
            temp_player_name: String::new(),
            temp_player_pin: 0,
            temp_player_call: 0,
            error: String::from(""),
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
            self.reset_error();
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                return Ok(true);
            }
            match self.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('n') => {
                        self.set_current_screen_new_game();
                    }
                    KeyCode::Char('q') => {
                        self.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
                CurrentScreen::NewGame => match self.current_popup {
                    // New Game Screen with Number of players as active popup
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
                                let total_players_result = self.input_buffer.parse();
                                match total_players_result {
                                    Ok(total_players) => {
                                        if total_players < 5 {
                                            self.set_popup_state(Popups::NewGamePopups(
                                                NewGamePopups::PlayerNames(
                                                    PlayerNamePopups::PlayerName,
                                                ),
                                            ));
                                            self.total_players = total_players;
                                            self.input_buffer.clear();
                                        } else {
                                            // error component
                                            self.set_error(String::from(
                                                "You've entered too many players (max 4)",
                                            ))
                                        }
                                    }
                                    Err(e) => self.set_error(e.to_string()),
                                }
                            }
                        }

                        KeyCode::Esc => {
                            self.set_current_screen(CurrentScreen::Exiting);
                        }

                        KeyCode::Char('q') => {
                            self.current_screen = CurrentScreen::Exiting;
                        }

                        _ => {
                            // check if the input is an number or not
                            if let Ok(_m) = key.code.to_string().parse::<i32>() {
                                self.input_buffer.push_str(&key.code.to_string());
                            }
                        }
                    },
                    // New Game Screen with Player Names as active popup
                    Popups::NewGamePopups(NewGamePopups::PlayerNames(
                        PlayerNamePopups::PlayerName,
                    ))
                    | Popups::NewGamePopups(NewGamePopups::PlayerNames(
                        PlayerNamePopups::PlayerPin,
                    ))
                    | Popups::NewGamePopups(NewGamePopups::PlayerNames(
                        PlayerNamePopups::PlayerCall,
                    )) => match key.code {
                        KeyCode::Delete => {
                            self.input_buffer.pop();
                        }
                        KeyCode::Backspace => {
                            self.input_buffer.pop();
                        }

                        KeyCode::Enter => {
                            if self.temp_player_name.is_empty() {
                                self.temp_player_name = self.input_buffer.clone();
                                self.input_buffer.clear();
                            } else if !self.temp_player_name.is_empty() && self.temp_player_pin == 0
                            {
                                let pin_result = self.input_buffer.parse();
                                match pin_result {
                                    Ok(pin) => {
                                        self.temp_player_pin = pin;
                                        self.input_buffer.clear();
                                    }
                                    Err(e) => self.set_error(e.to_string()),
                                }
                            } else if !self.temp_player_name.is_empty()
                                && self.temp_player_pin != 0
                                && self.temp_player_call == 0
                            {
                                let call_result = self.input_buffer.parse();
                                match call_result {
                                    Ok(call) => {
                                        self.temp_player_call = call;
                                        self.input_buffer.clear();
                                    }
                                    Err(e) => self.set_error(e.to_string()),
                                }

                                self.game.add_players(
                                    &self.temp_player_name,
                                    &self.temp_player_pin,
                                    self.temp_player_call,
                                );

                                if self.game.get_player_count() == self.total_players as usize {
                                    self.set_current_screen(CurrentScreen::Main);
                                    self.set_popup_state(Popups::None);
                                }

                                self.clear_temp_player();
                            }
                        }

                        KeyCode::Esc => {
                            self.set_current_screen(CurrentScreen::Exiting);
                        }

                        _ => {
                            self.input_buffer.push_str(&key.code.to_string());
                        }
                    },
                    _ => {}
                },

                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') => {
                        // TODO: Instead of reseting to main, we should reset to the previous screen by keeping a stack of screens
                        self.input_buffer.clear();
                        self.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },
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

    pub fn set_error(&mut self, error_str: String) {
        self.error = error_str
    }

    pub fn reset_error(&mut self) {
        self.error = String::from("")
    }

    pub fn set_popup_state(&mut self, popup: Popups) {
        self.current_popup = popup;
    }

    fn clear_temp_player(&mut self) {
        self.temp_player_name.clear();
        self.temp_player_pin = 0;
        self.temp_player_call = 0;
    }
}
