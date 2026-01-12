use crate::app::App;
use crate::ui::theme::get_theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

pub fn draw_quit_modal(f: &mut Frame, app: &App) {
    let theme = get_theme(app.current_theme);

    let area = centered_rect(40, 20, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" CONFIRM EXIT ")
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .style(Style::default().bg(theme.surface).fg(theme.fg))
        .border_style(Style::default().fg(theme.error));

    f.render_widget(block.clone(), area);

    let inner = block.inner(area);

    let text = [
        Span::raw("\n"),
        Span::styled(
            "Are you sure you want to quit?",
            Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
        ),
        Span::raw("\n\n"),
        Span::styled(
            "(y) Confirm    (n) Cancel",
            Style::default().fg(theme.dimmed),
        ),
    ];

    let p = Paragraph::new(
        text.iter()
            .cloned()
            .map(ratatui::text::Line::from)
            .collect::<Vec<_>>(),
    )
    .alignment(Alignment::Center);

    f.render_widget(p, inner);
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
