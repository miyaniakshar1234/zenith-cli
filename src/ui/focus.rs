use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(area);

    // Title
    let title = Paragraph::new("FOCUS MODE")
        .style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(title, chunks[0]);

    // Timer Logic
    let remaining = app.focus_state.remaining_sec;
    let minutes = remaining / 60;
    let seconds = remaining % 60;
    let timer_text = format!("{:02}:{:02}", minutes, seconds);

    // We could use tui-big-text here if imported, but standard text is safe for now
    let timer_display = Paragraph::new(timer_text)
        .style(
            Style::default()
                .fg(if app.focus_state.is_running {
                    Color::Green
                } else {
                    Color::Yellow
                })
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(timer_display, chunks[1]);

    // Progress Bar
    let total = app.focus_state.duration_sec as f64;
    let current = remaining as f64;
    let ratio = if total > 0.0 { current / total } else { 0.0 };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Session Progress"),
        )
        .gauge_style(Style::default().fg(Color::Cyan))
        .ratio(ratio.clamp(0.0, 1.0));

    f.render_widget(gauge, chunks[2]);
}
