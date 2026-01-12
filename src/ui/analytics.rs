use crate::app::App;
use crate::ui::theme::NORD_PRO;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{BarChart, Block, Borders, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Summary text
                Constraint::Min(0),    // Chart
            ]
            .as_ref(),
        )
        .split(area);

    // Summary Logic
    let total_week: u64 = app.stats.iter().map(|(_, count)| count).sum();
    let summary_text = format!("Tasks Completed (Last 7 Days): {}", total_week);

    let summary = Paragraph::new(Span::styled(
        summary_text,
        Style::default()
            .fg(NORD_PRO.accent)
            .add_modifier(Modifier::BOLD),
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(NORD_PRO.border)),
    );

    f.render_widget(summary, chunks[0]);

    // Bar Chart
    // Convert stats to specific format for BarChart
    // Note: BarChart expects &str for label and u64 for value
    // We need to construct owned strings in App if we want dynamic labels,
    // but BarChart takes &str. We can format them on the fly if the vector is consistent.

    // We'll take the stats from App (which are Vec<(String, u64)>)
    // and format them for the chart. The string is YYYY-MM-DD.
    // We'll just show MM-DD for space.

    let bar_data: Vec<(&str, u64)> = app
        .stats
        .iter()
        .map(|(date, count)| {
            let label = if date.len() >= 10 {
                &date[5..10]
            } else {
                date.as_str()
            };
            (label, *count)
        })
        .collect();

    let barchart = BarChart::default()
        .block(
            Block::default()
                .title("Productivity Velocity")
                .borders(Borders::ALL),
        )
        .data(&bar_data)
        .bar_width(8)
        .bar_gap(3)
        .bar_style(Style::default().fg(NORD_PRO.success))
        .value_style(
            Style::default()
                .fg(NORD_PRO.fg)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(barchart, chunks[1]);
}
