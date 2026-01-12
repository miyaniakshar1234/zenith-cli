use crate::app::App;
use crate::db::models::TaskStatus;
use crate::ui::theme::NORD_PRO;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(area);

    draw_col(f, app, chunks[0], TaskStatus::Todo, "TODO", 0);
    draw_col(f, app, chunks[1], TaskStatus::Doing, "IN PROGRESS", 1);
    draw_col(f, app, chunks[2], TaskStatus::Done, "DONE", 2);
}

fn draw_col(f: &mut Frame, app: &mut App, area: Rect, status: TaskStatus, title: &str, idx: usize) {
    let tasks: Vec<ListItem> = app
        .tasks
        .iter()
        .filter(|t| t.status == status)
        .map(|t| ListItem::new(Line::from(t.title.clone())))
        .collect();

    let is_focused = app.kanban_state.focused_col == idx;
    let border_color = if is_focused {
        NORD_PRO.accent
    } else {
        NORD_PRO.border
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .border_style(Style::default().fg(border_color));

    let list = List::new(tasks)
        .block(block)
        .highlight_style(Style::default().bg(NORD_PRO.selection_bg));

    match idx {
        0 => f.render_stateful_widget(list, area, &mut app.kanban_state.todo_state),
        1 => f.render_stateful_widget(list, area, &mut app.kanban_state.doing_state),
        2 => f.render_stateful_widget(list, area, &mut app.kanban_state.done_state),
        _ => {}
    }
}
