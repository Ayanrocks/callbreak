use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{block::Title, Block, Borders, Paragraph},
    Frame,
};

pub fn draw_main_screen(frame: &mut Frame) {
    let layouts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(2), Constraint::Length(3)].as_ref())
        .split(frame.size());

    let title = Title::from("  Callbreak  ".bold());
    let sub_title = Title::from(" By Ayan Banerjee ");

    let header_block = Block::bordered()
        .title(title.alignment(Alignment::Center))
        .title(sub_title.alignment(Alignment::Right))
        .style(Style::default())
        .borders(Borders::ALL);

    frame.render_widget(header_block, layouts[0]);

    let footer = Paragraph::new("Press 'q' to exit")
        .style(Style::default().fg(Color::LightBlue))
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, layouts[1]);
}
