use crate::app::App;
use crate::db::models::TaskStatus;
use crate::ui::theme::NEBULA;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    if app.tasks.is_empty() {
        let p = ratatui::widgets::Paragraph::new("No tasks found.\nPress 'n' to create one.")
            .style(Style::default().fg(NEBULA.inactive))
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default().borders(Borders::NONE));
        f.render_widget(p, area);
        return;
    }

    let rows: Vec<Row> = app
        .tasks
        .iter()
        .map(|task| {
            let (icon, color) = match task.status {
                TaskStatus::Todo => ("", NEBULA.accent_secondary),
                TaskStatus::Doing => ("", NEBULA.warning),
                TaskStatus::Done => ("", NEBULA.success),
            };

            let title_style = if task.status == TaskStatus::Done {
                Style::default()
                    .fg(NEBULA.inactive)
                    .add_modifier(Modifier::CROSSED_OUT)
            } else {
                Style::default().fg(NEBULA.fg).add_modifier(Modifier::BOLD)
            };

            use ratatui::widgets::Cell;

            Row::new(vec![
                Cell::from(format!(" {} ", icon)),
                Cell::from(task.title.clone()).style(title_style),
                Cell::from(format!("{} XP", task.xp_reward)),
                Cell::from(format!("{}", task.status)),
            ])
            .style(Style::default().fg(color)) // Default row color
            .height(1)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),      // Icon
            Constraint::Percentage(70), // Title
            Constraint::Length(10),     // XP
            Constraint::Length(10),     // Status
        ],
    )
    .block(Block::default().borders(Borders::NONE))
    .header(
        Row::new(vec!["", "TITLE", "REWARD", "STATUS"])
            .style(
                Style::default()
                    .fg(NEBULA.accent_primary)
                    .add_modifier(Modifier::BOLD),
            )
            .bottom_margin(1),
    )
    .row_highlight_style(
        Style::default()
            .bg(NEBULA.selection_bg)
            .fg(NEBULA.selection_fg)
            .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol("🚀 ");

    f.render_stateful_widget(table, area, &mut app.table_state);
}
