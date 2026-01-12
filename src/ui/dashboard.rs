use crate::app::App;
use crate::db::models::{TaskPriority, TaskStatus};
use crate::ui::theme::HORIZON;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    // Master-Detail Layout
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(60), // Master (List)
                Constraint::Percentage(40), // Detail (Preview)
            ]
            .as_ref(),
        )
        .split(area);

    draw_table(f, app, chunks[0]);
    draw_preview(f, app, chunks[1]);
}

fn draw_table(f: &mut Frame, app: &mut App, area: Rect) {
    if app.tasks.is_empty() {
        let p = Paragraph::new("No tasks found.\nPress 'n' to create one.")
            .style(Style::default().fg(HORIZON.dimmed))
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
                TaskStatus::Todo => ("○", HORIZON.fg),
                TaskStatus::Doing => ("◉", HORIZON.warning),
                TaskStatus::Done => ("●", HORIZON.success),
            };

            let title_style = if task.status == TaskStatus::Done {
                Style::default()
                    .fg(HORIZON.dimmed)
                    .add_modifier(Modifier::CROSSED_OUT)
            } else {
                Style::default().fg(HORIZON.fg).add_modifier(Modifier::BOLD)
            };

            // Priority Indicator
            let priority_marker = match task.priority {
                TaskPriority::High => " 🔴",
                TaskPriority::Medium => "", // Clean for default
                TaskPriority::Low => " 🔵",
            };

            Row::new(vec![
                Cell::from(format!("  {} ", icon)).style(Style::default().fg(color)),
                Cell::from(format!("{}{}", task.title, priority_marker)).style(title_style),
                Cell::from(format!("{} XP", task.xp_reward))
                    .style(Style::default().fg(HORIZON.secondary)),
            ])
            .height(1)
            .style(Style::default().bg(HORIZON.bg))
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(5),      // Icon
            Constraint::Percentage(75), // Title
            Constraint::Length(10),     // XP
        ],
    )
    .block(
        Block::default()
            .borders(Borders::RIGHT)
            .border_style(Style::default().fg(HORIZON.border)),
    )
    .header(
        Row::new(vec!["", "TASK", "REWARD"])
            .style(
                Style::default()
                    .fg(HORIZON.dimmed)
                    .add_modifier(Modifier::BOLD),
            )
            .bottom_margin(1),
    )
    .row_highlight_style(
        Style::default()
            .bg(HORIZON.selection_bg)
            .fg(HORIZON.selection_fg)
            .add_modifier(Modifier::BOLD),
    );

    f.render_stateful_widget(table, area, &mut app.table_state);
}

fn draw_preview(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(HORIZON.surface));
    f.render_widget(block, area);

    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)].as_ref())
        .margin(2)
        .split(area)[0];

    let task = if let Some(i) = app.table_state.selected() {
        if let Some(t) = app.tasks.get(i) {
            t
        } else {
            return;
        }
    } else {
        return;
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(2), // Status Tag
                Constraint::Length(2), // Title
                Constraint::Length(1), // Meta
                Constraint::Length(2), // Spacer
                Constraint::Min(0),    // Description
            ]
            .as_ref(),
        )
        .split(inner_area);

    // 1. Status Tag
    let (status_text, status_color) = match task.status {
        TaskStatus::Todo => ("  TODO  ", HORIZON.dimmed),
        TaskStatus::Doing => ("  IN PROGRESS  ", HORIZON.warning),
        TaskStatus::Done => ("  COMPLETED  ", HORIZON.success),
    };

    let status_badge = Paragraph::new(Span::styled(
        status_text,
        Style::default()
            .bg(status_color)
            .fg(HORIZON.bg)
            .add_modifier(Modifier::BOLD),
    ));
    f.render_widget(status_badge, chunks[0]);

    // 2. Title + Priority
    let priority_text = match task.priority {
        TaskPriority::High => "[HIGH] ",
        TaskPriority::Medium => "",
        TaskPriority::Low => "[LOW] ",
    };

    let title_line = Line::from(vec![
        Span::styled(
            priority_text,
            Style::default()
                .fg(HORIZON.error)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            &task.title,
            Style::default()
                .fg(HORIZON.accent)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    let title = Paragraph::new(title_line).wrap(Wrap { trim: true });
    f.render_widget(title, chunks[1]);

    // 3. Metadata
    let meta_text = format!(
        "Reward: {} XP  •  Created: {}",
        task.xp_reward,
        task.created_at.format("%b %d")
    );
    let meta = Paragraph::new(meta_text).style(Style::default().fg(HORIZON.dimmed));
    f.render_widget(meta, chunks[2]);

    // 4. Description
    let desc = if task.description.is_empty() {
        "No details provided."
    } else {
        &task.description
    };
    let description = Paragraph::new(desc)
        .style(Style::default().fg(HORIZON.fg))
        .wrap(Wrap { trim: true });
    f.render_widget(description, chunks[4]);
}
