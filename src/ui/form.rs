use crate::app::{App, FormField};
use crate::db::models::TaskPriority;
use crate::ui::theme::HORIZON;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

pub fn draw_form_modal(f: &mut Frame, app: &mut App) {
    let area = centered_rect(60, 60, f.area());
    f.render_widget(Clear, area);

    let title = if app.editing_task_id.is_some() {
        " EDIT TASK "
    } else {
        " NEW TASK "
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(HORIZON.accent))
        .title(title)
        .title_style(
            Style::default()
                .fg(HORIZON.accent)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().bg(HORIZON.surface));

    f.render_widget(block.clone(), area);

    let inner = block.inner(area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Title
                Constraint::Length(3), // Priority
                Constraint::Length(3), // XP | Due Date
                Constraint::Min(0),    // Description
            ]
            .as_ref(),
        )
        .split(inner);

    // 1. Title Input
    let title_border = if app.task_form.active_field == FormField::Title {
        HORIZON.accent
    } else {
        HORIZON.dimmed
    };
    app.task_form.title.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Title")
            .border_style(Style::default().fg(title_border)),
    );
    app.task_form
        .title
        .set_style(Style::default().fg(HORIZON.fg));
    f.render_widget(&app.task_form.title, chunks[0]);

    // 2. Priority Input
    let prio_border = if app.task_form.active_field == FormField::Priority {
        HORIZON.accent
    } else {
        HORIZON.dimmed
    };
    let prio_text = match app.task_form.priority {
        TaskPriority::High => "🔴 HIGH",
        TaskPriority::Medium => "🟡 MEDIUM",
        TaskPriority::Low => "🔵 LOW",
    };
    let prio_widget = Paragraph::new(prio_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Priority (< >)")
                .border_style(Style::default().fg(prio_border)),
        )
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(HORIZON.fg).add_modifier(Modifier::BOLD));
    f.render_widget(prio_widget, chunks[1]);

    // 3. XP & Due Date
    let row3 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunks[2]);

    // XP
    let xp_border = if app.task_form.active_field == FormField::XP {
        HORIZON.accent
    } else {
        HORIZON.dimmed
    };
    app.task_form.xp.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("XP")
            .border_style(Style::default().fg(xp_border)),
    );
    app.task_form.xp.set_style(Style::default().fg(HORIZON.fg));
    f.render_widget(&app.task_form.xp, row3[0]);

    // Due Date
    let due_border = if app.task_form.active_field == FormField::DueDate {
        HORIZON.accent
    } else {
        HORIZON.dimmed
    };
    app.task_form.due_date.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Due (YYYY-MM-DD)")
            .border_style(Style::default().fg(due_border)),
    );
    app.task_form
        .due_date
        .set_style(Style::default().fg(HORIZON.fg));
    f.render_widget(&app.task_form.due_date, row3[1]);

    // 4. Description Input
    let desc_border = if app.task_form.active_field == FormField::Description {
        HORIZON.accent
    } else {
        HORIZON.dimmed
    };
    app.task_form.description.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Description")
            .border_style(Style::default().fg(desc_border)),
    );
    app.task_form
        .description
        .set_style(Style::default().fg(HORIZON.fg));
    f.render_widget(&app.task_form.description, chunks[3]);
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
