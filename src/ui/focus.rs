use crate::app::App;
use crate::ui::theme::get_theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let theme = get_theme(app.current_theme);

    let center = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ]
            .as_ref(),
        )
        .split(area)[1];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(4), // Timer
                Constraint::Length(1), // Spacer
                Constraint::Length(1), // Bar
            ]
            .as_ref(),
        )
        .split(center);

    // Timer
    let remaining = app.focus_state.remaining_sec;
    let time_str = format!("{:02}:{:02}", remaining / 60, remaining % 60);

    let timer = Paragraph::new(time_str)
        .style(Style::default().fg(theme.fg).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.accent)),
        );

    f.render_widget(timer, chunks[0]);

    let total = app.focus_state.duration_sec as f64;
    let ratio = if total > 0.0 {
        remaining as f64 / total
    } else {
        0.0
    };

    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(theme.success))
        .ratio(ratio)
        .use_unicode(true);

    f.render_widget(gauge, chunks[2]);
}
