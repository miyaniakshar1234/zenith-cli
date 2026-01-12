use crate::app::App;
use crate::ui::theme::NORD_PRO;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
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
        .constraints([Constraint::Length(4), Constraint::Length(2)].as_ref())
        .split(center);

    let remaining = app.focus_state.remaining_sec;
    let time_str = format!("{:02}:{:02}", remaining / 60, remaining % 60);

    let timer = Paragraph::new(time_str)
        .style(
            Style::default()
                .fg(NORD_PRO.fg)
                .add_modifier(Modifier::BOLD),
        ) // Removed font size logic for now, keeping it clean
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(NORD_PRO.accent)),
        );

    f.render_widget(timer, chunks[0]);

    let total = app.focus_state.duration_sec as f64;
    let ratio = if total > 0.0 {
        remaining as f64 / total
    } else {
        0.0
    };

    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(NORD_PRO.accent))
        .ratio(ratio);

    f.render_widget(gauge, chunks[1]);
}
