use crate::app::App;
use crate::db::models::TaskStatus;
use crate::ui::theme::NEON_CYBERPUNK;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let tasks: Vec<ListItem> = app
        .tasks
        .iter()
        .map(|task| {
            let (status_icon, color) = match task.status {
                TaskStatus::Todo => (" ", NEON_CYBERPUNK.error), // Nerd Font Icon (Circle)
                TaskStatus::Doing => (" ", NEON_CYBERPUNK.warning), // Nerd Font Icon (Note)
                TaskStatus::Done => (" ", NEON_CYBERPUNK.success), // Nerd Font Icon (Check)
            };

            // Strikethrough for done tasks
            let style = if task.status == TaskStatus::Done {
                Style::default()
                    .fg(NEON_CYBERPUNK.text_dim)
                    .add_modifier(Modifier::CROSSED_OUT)
            } else {
                Style::default().fg(NEON_CYBERPUNK.text_main)
            };

            let content = Line::from(vec![
                Span::styled(
                    format!("{}", status_icon),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(format!("{:<30}", &task.title), style),
                Span::styled(
                    format!(" ({} XP)", task.xp_reward),
                    Style::default().fg(NEON_CYBERPUNK.text_dim),
                ),
            ]);

            ListItem::new(content)
        })
        .collect();

    let tasks_list = List::new(tasks)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" TASKS ")
                .border_style(Style::default().fg(NEON_CYBERPUNK.primary)),
        )
        .highlight_style(
            Style::default()
                .bg(NEON_CYBERPUNK.primary.into()) // Highlight BG
                .fg(NEON_CYBERPUNK.background.into()) // Highlight Text
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" ➤ ");

    f.render_stateful_widget(tasks_list, area, &mut app.list_state);
}
