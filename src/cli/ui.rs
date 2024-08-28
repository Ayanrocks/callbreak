use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw_main_screen(frame: &mut Frame) {
    let layouts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(frame.size());

    let title_block = Block::default().style(Style::default());

    let title = Paragraph::new(Text::styled("Callbreak", Style::default().fg(Color::Green)))
        .block(title_block);

    frame.render_widget(title, layouts[0]);
}
