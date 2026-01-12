use crate::app::App;
use crate::ui::theme::NEON_CYBERPUNK;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
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
                .fg(NEON_CYBERPUNK.secondary)
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

    let timer_color = if app.focus_state.is_running {
        NEON_CYBERPUNK.success
    } else if remaining == 0 {
        NEON_CYBERPUNK.error
    } else {
        NEON_CYBERPUNK.accent
    };

    let timer_display = Paragraph::new(timer_text)
        .style(
            Style::default()
                .fg(timer_color)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(timer_color)),
        );

    f.render_widget(timer_display, chunks[1]);

    // Progress Bar
    let total = app.focus_state.duration_sec as f64;
    let current = remaining as f64;
    let ratio = if total > 0.0 { current / total } else { 0.0 };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Session Progress"),
        )
        .gauge_style(Style::default().fg(NEON_CYBERPUNK.primary))
        .ratio(ratio.clamp(0.0, 1.0));

    f.render_widget(gauge, chunks[2]);
}
