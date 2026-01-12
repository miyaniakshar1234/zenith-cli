use crate::app::App;
use crate::db::models::TaskStatus;
use crate::ui::theme::NORD_PRO;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    // Empty State
    if app.tasks.is_empty() {
        let p = ratatui::widgets::Paragraph::new("No tasks found.\nPress 'n' to create one.")
            .style(Style::default().fg(NORD_PRO.inactive))
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default().borders(Borders::NONE));
        f.render_widget(p, area);
        return;
    }

    let tasks: Vec<ListItem> = app
        .tasks
        .iter()
        .map(|task| {
            let (icon, color) = match task.status {
                TaskStatus::Todo => ("○", NORD_PRO.fg),
                TaskStatus::Doing => ("◉", NORD_PRO.warning),
                TaskStatus::Done => ("●", NORD_PRO.success),
            };

            let title_style = if task.status == TaskStatus::Done {
                Style::default()
                    .fg(NORD_PRO.inactive)
                    .add_modifier(Modifier::CROSSED_OUT)
            } else {
                Style::default()
                    .fg(NORD_PRO.fg)
                    .add_modifier(Modifier::BOLD)
            };

            let content = Line::from(vec![
                Span::styled(format!(" {} ", icon), Style::default().fg(color)),
                Span::styled(&task.title, title_style),
            ]);

            ListItem::new(content)
        })
        .collect();

    let list = List::new(tasks)
        .block(Block::default().borders(Borders::NONE))
        .highlight_style(Style::default().bg(NORD_PRO.selection_bg))
        .highlight_symbol("│");

    f.render_stateful_widget(list, area, &mut app.list_state);
}
