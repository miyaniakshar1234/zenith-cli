use crate::app::App;
use crate::db::models::TaskStatus;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
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

    draw_column(f, app, chunks[0], TaskStatus::Todo, "TODO", 0);
    draw_column(f, app, chunks[1], TaskStatus::Doing, "DOING", 1);
    draw_column(f, app, chunks[2], TaskStatus::Done, "DONE", 2);
}

fn draw_column(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    status: TaskStatus,
    title: &str,
    col_index: usize,
) {
    let tasks: Vec<ListItem> = app
        .tasks
        .iter()
        .filter(|t| t.status == status)
        .map(|task| ListItem::new(Line::from(vec![Span::raw(&task.title)])))
        .collect();

    let is_focused = app.kanban_state.focused_col == col_index;
    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let list = List::new(tasks)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Cyan)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    // We need to pass the correct ListState based on column
    match col_index {
        0 => f.render_stateful_widget(list, area, &mut app.kanban_state.todo_state),
        1 => f.render_stateful_widget(list, area, &mut app.kanban_state.doing_state),
        2 => f.render_stateful_widget(list, area, &mut app.kanban_state.done_state),
        _ => {}
    }
}
