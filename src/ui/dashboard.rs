use crate::app::App;
use crate::db::models::TaskStatus;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let tasks: Vec<ListItem> = app
        .tasks
        .iter()
        .map(|task| {
            let (status_icon, color) = match task.status {
                TaskStatus::Todo => ("TODO ", Color::Red),
                TaskStatus::Doing => ("DOING", Color::Yellow),
                TaskStatus::Done => ("DONE ", Color::Green),
            };

            let content = Line::from(vec![
                Span::styled(
                    format!("[{}] ", status_icon),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{:<30}", &task.title),
                    Style::default().fg(Color::White),
                ),
                Span::styled(
                    format!(" ({} XP)", task.xp_reward),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);

            ListItem::new(content)
        })
        .collect();

    let tasks_list = List::new(tasks)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Tasks (Dashboard)"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(tasks_list, area, &mut app.list_state);
}
