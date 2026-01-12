use crate::ui::theme::HORIZON;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Length(10), // Banner
                Constraint::Length(2),  // Subtitle
                Constraint::Length(4),  // Info
                Constraint::Min(0),     // Footer
            ]
            .as_ref(),
        )
        .split(area);

    // Banner (ZENITH ASCII)
    // Using simple text for stability, but styled heavily
    let banner_text = vec![
        "███████╗███████╗███╗   ██╗██╗████████╗██╗  ██╗",
        "╚══███╔╝██╔════╝████╗  ██║██║╚══██╔══╝██║  ██║",
        "  ███╔╝ █████╗  ██╔██╗ ██║██║   ██║   ███████║",
        " ███╔╝  ██╔══╝  ██║╚██╗██║██║   ██║   ██╔══██║",
        "███████╗███████╗██║ ╚████║██║   ██║   ██║  ██║",
        "╚══════╝╚══════╝╚═╝  ╚═══╝╚═╝   ╚═╝   ╚═╝  ╚═╝",
    ];

    let banner = Paragraph::new(banner_text.join("\n"))
        .style(
            Style::default()
                .fg(HORIZON.accent)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);

    f.render_widget(banner, chunks[1]);

    // Subtitle
    let subtitle = Paragraph::new("INDUSTRIAL GRADE TASK MANAGEMENT SYSTEM")
        .style(
            Style::default()
                .fg(HORIZON.secondary)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(subtitle, chunks[2]);

    // Info / Credits
    let info_text = vec![
        Line::from(vec![
            Span::styled("Version: ", Style::default().fg(HORIZON.dimmed)),
            Span::styled("1.1.0 (Power User)", Style::default().fg(HORIZON.fg)),
        ]),
        Line::from(vec![
            Span::styled("Engineer: ", Style::default().fg(HORIZON.dimmed)),
            Span::styled(
                "Miyani Akshar",
                Style::default()
                    .fg(HORIZON.success)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(HORIZON.dimmed)),
            Span::styled("OPERATIONAL", Style::default().fg(HORIZON.accent)),
        ]),
    ];

    let info = Paragraph::new(info_text).alignment(Alignment::Center);
    f.render_widget(info, chunks[3]);

    // Prompt
    let prompt = Paragraph::new("PRESS ANY KEY TO INITIALIZE...")
        .style(
            Style::default()
                .fg(HORIZON.fg)
                .add_modifier(Modifier::SLOW_BLINK),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(HORIZON.border)),
        );
    f.render_widget(prompt, chunks[4]);
}
