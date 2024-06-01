use std::io::stdout;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    ExecutableCommand,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};

pub struct CLI {}

impl CLI {
    pub fn new_cli() {
        stdout().execute(EnterAlternateScreen).expect("Alternative Screen");
        enable_raw_mode().expect("Enabling raw mode");
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).expect("Expected terminal");
        terminal.clear().expect("df");

        loop {
            terminal.draw(|frame| {
                let area = frame.size();
                frame.render_widget(
                    Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                        .white()
                        .on_blue(),
                    area,
                );
            }).expect("expecting draw");

            if event::poll(std::time::Duration::from_millis(16)).expect("poll durattion") {
                if let event::Event::Key(key) = event::read().expect("Added key event") {
                    if key.kind == KeyEventKind::Press
                        && key.code == KeyCode::Char('q')
                    {
                        break;
                    }
                }
            }
        }

        stdout().execute(LeaveAlternateScreen);
        disable_raw_mode();
    }
}