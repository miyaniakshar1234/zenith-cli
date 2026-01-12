use crate::app::{App, CurrentView, InputMode};
use crate::ui::theme::HORIZON;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Clear, Row, Table},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    if app.input_mode == InputMode::Normal {
        let area = centered_rect(60, 60, f.area());
        f.render_widget(Clear, area);

        let block = Block::default()
            .title(" COMMAND PALETTE ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(HORIZON.surface).fg(HORIZON.fg))
            .border_style(Style::default().fg(HORIZON.accent));

        f.render_widget(block.clone(), area);

        let inner = block.inner(area);

        // Context-aware help
        let mut rows = vec![
            Row::new(vec!["Global", "TAB", "Switch View"]),
            Row::new(vec!["Global", "?", "Toggle Help"]),
            Row::new(vec!["Global", "q", "Quit"]),
        ];

        match app.current_view {
            CurrentView::Dashboard => {
                rows.extend(vec![
                    Row::new(vec!["Dashboard", "n", "New Task (!h/!m/!l for priority)"]),
                    Row::new(vec!["Dashboard", "e", "Edit Selected Task"]),
                    Row::new(vec!["Dashboard", "d", "Delete Selected Task"]),
                    Row::new(vec!["Dashboard", "SPACE", "Toggle Status"]),
                    Row::new(vec!["Dashboard", "/", "Search Mode"]),
                    Row::new(vec!["Dashboard", "j/k", "Navigate List"]),
                ]);
            }
            CurrentView::Kanban => {
                rows.extend(vec![
                    Row::new(vec!["Kanban", "h/l", "Switch Column"]),
                    Row::new(vec!["Kanban", "j/k", "Navigate Tasks"]),
                ]);
            }
            CurrentView::Focus => {
                rows.extend(vec![
                    Row::new(vec!["Focus", "t", "Start/Pause Timer"]),
                    Row::new(vec!["Focus", "r", "Reset Timer"]),
                ]);
            }
            CurrentView::Analytics | CurrentView::Splash => {}
        }

        let table = Table::new(
            rows,
            [
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(60),
            ],
        )
        .header(
            Row::new(vec!["CONTEXT", "KEY", "ACTION"]).style(
                Style::default()
                    .fg(HORIZON.accent)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .column_spacing(1);

        f.render_widget(table, inner);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
