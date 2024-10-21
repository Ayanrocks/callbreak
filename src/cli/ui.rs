use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use super::state::{CurrentScreen, NewGamePopups, PlayerNamePopups, Popups, State};

pub fn draw_main_screen(frame: &mut Frame, state: &mut State) {
    let layouts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(2), Constraint::Length(3)].as_ref())
        .split(frame.area());

    let title = Title::from("  Callbreak  ".bold());
    let sub_title = Title::from(" By Ayan Banerjee ");

    let header_block = Block::bordered()
        .title(title.alignment(Alignment::Center))
        .title(sub_title.alignment(Alignment::Right))
        .style(Style::default())
        .borders(Borders::ALL);

    frame.render_widget(header_block, layouts[0]);

    // if new game then show a popup to add new players
    // if game is running then show the game screen
    if state.current_screen == CurrentScreen::NewGame {
        match state.current_popup {
            Popups::NewGamePopups(NewGamePopups::NumberOfPlayers) => {
                draw_new_game_popup(frame, state)
            }
            Popups::NewGamePopups(NewGamePopups::PlayerNames(PlayerNamePopups::PlayerName))
            | Popups::NewGamePopups(NewGamePopups::PlayerNames(PlayerNamePopups::PlayerPin))
            | Popups::NewGamePopups(NewGamePopups::PlayerNames(PlayerNamePopups::PlayerCall)) => {
                draw_new_game_popup_with_player(frame, state)
            }
            _ => {}
        }
    }

    // footer section with options
    let footer_options = vec![
        match state.current_screen {
            CurrentScreen::NewGame => Span::styled(
                format!("New Game: {}", state.current_popup),
                Style::default().fg(Color::Green),
            ),
            CurrentScreen::Main => Span::styled("Main Screen", Style::default().fg(Color::Yellow)),
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // options
        {
            match state.current_screen {
                CurrentScreen::NewGame => Span::styled(
                    "(q) to quit / [Enter] to Select",
                    Style::default().fg(Color::White),
                ),
                CurrentScreen::Main => Span::styled(
                    "(q) to quit / (n) to start new game",
                    Style::default().fg(Color::White),
                ),
                CurrentScreen::Exiting => Span::styled(
                    "(y) for Yes / (n) for No",
                    Style::default().fg(Color::White),
                ),
            }
        },
    ];

    let footer = Paragraph::new(Line::from(footer_options))
        .style(Style::default().fg(Color::LightBlue))
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, layouts[1]);

    if !state.error.is_empty() {
        draw_error_modal(frame, state, layouts[1]);
    }
}

fn draw_new_game_popup(frame: &mut Frame, state: &mut State) {
    // frame.render_widget(Clear, frame.area());
    let popup_layout = centered_rect(50, 40, frame.area());
    let popup = Block::default()
        .title(Title::from(" New Game ").alignment(Alignment::Center))
        .borders(Borders::NONE)
        .style(Style::default().fg(Color::White).bg(Color::Blue));

    frame.render_widget(popup, popup_layout);

    let popup_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(4), Constraint::Length(3)].as_ref())
        .split(popup_layout);

    let header = Title::from("Enter the number of players to be playing: ");

    let header_block = Block::default()
        .title(header)
        .style(Style::default().bg(Color::Yellow).fg(Color::White));

    let header_body = Paragraph::new(state.input_buffer.clone())
        .style(Style::new().bold().fg(Color::Black))
        .block(header_block)
        .wrap(Wrap { trim: false });

    frame.render_widget(header_body, popup_chunks[0]);

    let footer = Paragraph::new(Line::from("Press 'ESC' to exit"))
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    frame.render_widget(footer, popup_chunks[1]);
}

fn draw_error_modal(frame: &mut Frame, state: &State, rect: Rect) {
    frame.render_widget(Clear, rect);
    let error_modal = Paragraph::new(state.error.to_owned())
        .block(Block::default().borders(Borders::ALL))
        .bg(Color::Red);

    frame.render_widget(error_modal, rect);
}

fn draw_new_game_popup_with_player(frame: &mut Frame, state: &mut State) {
    // frame.render_widget(Clear, frame.area());
    let popup_layout = centered_rect(50, 40, frame.area());
    let popup = Block::default()
        .title(Title::from(" New Game ").alignment(Alignment::Center))
        .borders(Borders::NONE)
        .style(Style::default().fg(Color::White).bg(Color::Blue));

    frame.render_widget(popup, popup_layout);

    let popup_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(4),
                Constraint::Length(4),
                Constraint::Length(4),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(popup_layout);

    let header = vec![match state.current_popup {
        Popups::NewGamePopups(NewGamePopups::PlayerNames(PlayerNamePopups::PlayerName)) => {
            Span::styled(
                "Enter the name of the player: ",
                Style::default().fg(Color::White),
            )
        }
        Popups::NewGamePopups(NewGamePopups::PlayerNames(PlayerNamePopups::PlayerPin)) => {
            Span::styled(
                "Enter the pin of the player: ",
                Style::default().fg(Color::White),
            )
        }
        Popups::NewGamePopups(NewGamePopups::PlayerNames(PlayerNamePopups::PlayerCall)) => {
            Span::styled(
                "Enter the call of the player: ",
                Style::default().fg(Color::White),
            )
        }
        _ => Span::from(""),
    }
    .to_owned()];

    let header_block = Block::new()
        .title(Line::from(header))
        .style(Style::default().bg(Color::Yellow).fg(Color::White));

    let player_name_header = Title::from("Enter the name of the player: ");

    let player_name_block = Block::default()
        .title(player_name_header)
        .style(Style::default().bg(Color::Yellow).fg(Color::White));

    let player_name_body = Paragraph::new(state.input_buffer.clone())
        .style(Style::new().bold().fg(Color::Black))
        .block(player_name_block)
        .wrap(Wrap { trim: true });

    let player_pin_header = Title::from("Enter the pin of the player: ");

    let player_pin_block = Block::default()
        .title(player_pin_header)
        .style(Style::default().bg(Color::Yellow).fg(Color::White));

    let player_pin_body = Paragraph::new(state.input_buffer.clone()).block(player_pin_block);

    let player_call_header = Title::from("Enter the call of the player: ");

    let player_call_block = Block::default()
        .title(player_call_header)
        .style(Style::default().bg(Color::Yellow).fg(Color::White));

    let player_call_body = Paragraph::new(state.input_buffer.clone()).block(player_call_block);

    // let header_body = Paragraph::new(state.input_buffer.clone())
    //     .style(Style::new().bold().fg(Color::Black))
    //     .block(header_block)
    //     .wrap(Wrap { trim: true });

    frame.render_widget(player_name_body, popup_chunks[0]);
    frame.render_widget(player_pin_body, popup_chunks[1]);
    frame.render_widget(player_call_body, popup_chunks[2]);

    let footer = Paragraph::new(Line::from("Press 'ESC' to exit"))
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    frame.render_widget(footer, popup_chunks[3]);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
