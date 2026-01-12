use crate::app::App;
use crate::ui::theme::HORIZON;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 60, f.area());
    f.render_widget(Clear, area);

    // Get selected task
    let task = if let Some(i) = app.table_state.selected() {
        if let Some(t) = app.tasks.get(i) {
            t
        } else {
            return;
        }
    } else {
        return;
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(" INSPECTOR ")
        .style(Style::default().bg(HORIZON.bg).fg(HORIZON.fg))
        .border_style(Style::default().fg(HORIZON.accent));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1), // Title
                Constraint::Length(1), // Separator
                Constraint::Length(1), // Metadata
                Constraint::Length(1), // Separator
                Constraint::Min(0),    // Description
            ]
            .as_ref(),
        )
        .split(inner_area);

    // 1. Title
    let title = Paragraph::new(Span::styled(
        &task.title,
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(HORIZON.accent),
    ));
    f.render_widget(title, chunks[0]);

    // 2. Metadata
    let status_str = format!("{}", task.status);
    let meta = format!(
        "Status: {} | XP Reward: {} | Created: {}",
        status_str,
        task.xp_reward,
        task.created_at.format("%Y-%m-%d %H:%M")
    );
    let metadata = Paragraph::new(Span::styled(meta, Style::default().fg(HORIZON.dimmed)));
    f.render_widget(metadata, chunks[2]);

    // 3. Description
    let desc_text = if task.description.is_empty() {
        "No description provided."
    } else {
        &task.description
    };

    let description = Paragraph::new(desc_text)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(HORIZON.fg));

    f.render_widget(description, chunks[4]);
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
