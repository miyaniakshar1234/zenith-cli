use crate::app::App;
use crate::db::models::TaskPriority;
use crate::ui::theme::get_theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let theme = get_theme(app.current_theme);

    // Center the focus workspace nicely
    let vertical_center = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10), // Top padding
                Constraint::Percentage(80), // Main Focus Area
                Constraint::Percentage(10), // Bottom padding
            ]
            .as_ref(),
        )
        .split(area)[1];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(8), // Big Timer (5 lines text + borders)
                Constraint::Length(1), // Spacer
                Constraint::Length(2), // Progress Bar
                Constraint::Length(1), // Spacer
                Constraint::Min(0),    // Active Task Context
            ]
            .as_ref(),
        )
        .split(vertical_center);

    // --- 1. BIG TIMER ---
    let remaining = app.focus_state.remaining_sec;
    let mins = remaining / 60;
    let secs = remaining % 60;

    // Construct ASCII Art Time
    let m1 = mins / 10;
    let m2 = mins % 10;
    let s1 = secs / 10;
    let s2 = secs % 10;

    let digit_m1 = get_big_digit(m1);
    let digit_m2 = get_big_digit(m2);
    let colon = get_big_colon();
    let digit_s1 = get_big_digit(s1);
    let digit_s2 = get_big_digit(s2);

    // Combine lines
    let mut big_text_lines = vec![String::new(); 5];
    for i in 0..5 {
        big_text_lines[i] = format!(
            "{}  {}    {}    {}  {}",
            digit_m1[i], digit_m2[i], colon[i], digit_s1[i], digit_s2[i]
        );
    }

    let timer_style = if app.focus_state.is_running {
        Style::default()
            .fg(theme.success)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(theme.warning)
            .add_modifier(Modifier::DIM)
    };

    let big_timer_paragraph = Paragraph::new(big_text_lines.join("\n"))
        .alignment(Alignment::Center)
        .style(timer_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .title(" SESSION TIMER ")
                .title_alignment(Alignment::Center)
                .border_style(Style::default().fg(theme.accent)),
        );

    f.render_widget(big_timer_paragraph, chunks[0]);

    // --- 2. PROGRESS GAUGE ---
    let total = app.focus_state.duration_sec as f64;
    let ratio = if total > 0.0 {
        remaining as f64 / total
    } else {
        0.0
    };

    let gauge_color = if ratio < 0.2 {
        theme.error
    } else {
        theme.accent
    };

    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(gauge_color))
        .ratio(ratio)
        .label(format!("{}% Remaining", (ratio * 100.0) as u64))
        .use_unicode(true)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(gauge, chunks[2]);

    // --- 3. ACTIVE TASK CARD ---
    let task_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" ACTIVE OBJECTIVE ")
        .style(Style::default().bg(theme.surface))
        .border_style(Style::default().fg(theme.secondary));

    let task_inner = task_block.inner(chunks[4]);
    f.render_widget(task_block, chunks[4]);

    if let Some(index) = app.table_state.selected() {
        if let Some(task) = app.tasks.get(index) {
            let info_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(2), // Header Line
                        Constraint::Length(1), // Separator
                        Constraint::Min(0),    // Description
                    ]
                    .as_ref(),
                )
                .split(task_inner);

            // Header: [PRIORITY] Title
            let (prio_icon, prio_color) = match task.priority {
                TaskPriority::High => (" 🔴 HIGH PRIORITY ", theme.error),
                TaskPriority::Medium => (" 🟡 MEDIUM ", theme.warning),
                TaskPriority::Low => (" 🔵 LOW ", theme.success),
            };

            let header = Line::from(vec![
                Span::styled(
                    prio_icon,
                    Style::default()
                        .bg(prio_color)
                        .fg(theme.bg)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("  "),
                Span::styled(
                    &task.title,
                    Style::default()
                        .fg(theme.fg)
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::raw("  "),
                Span::styled(
                    format!("({} XP)", task.xp_reward),
                    Style::default().fg(theme.dimmed),
                ),
            ]);

            f.render_widget(
                Paragraph::new(header).alignment(Alignment::Center),
                info_layout[0],
            );

            // Description
            let desc = if task.description.is_empty() {
                "No additional directives provided for this objective."
            } else {
                &task.description
            };

            let desc_widget = Paragraph::new(desc)
                .wrap(Wrap { trim: true })
                .style(Style::default().fg(theme.fg))
                .alignment(Alignment::Center);

            f.render_widget(desc_widget, info_layout[2]);
        } else {
            f.render_widget(
                Paragraph::new("Select a task from Dashboard first.")
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(theme.error)),
                task_inner,
            );
        }
    } else {
        f.render_widget(
            Paragraph::new("NO ACTIVE TASK LOCKED.\nPress 'Tab' to return to Dashboard.")
                .alignment(Alignment::Center)
                .style(Style::default().fg(theme.dimmed)),
            task_inner,
        );
    }
}

// --- ASCII FONT HELPER ---
// 5-line high font
fn get_big_digit(n: i64) -> Vec<&'static str> {
    match n {
        0 => vec!["██████", "██  ██", "██  ██", "██  ██", "██████"],
        1 => vec!["  ██  ", "████  ", "  ██  ", "  ██  ", "██████"],
        2 => vec!["██████", "    ██", "██████", "██    ", "██████"],
        3 => vec!["██████", "    ██", "██████", "    ██", "██████"],
        4 => vec!["██  ██", "██  ██", "██████", "    ██", "    ██"],
        5 => vec!["██████", "██    ", "██████", "    ██", "██████"],
        6 => vec!["██████", "██    ", "██████", "██  ██", "██████"],
        7 => vec!["██████", "   ██ ", "  ██  ", " ██   ", "██    "],
        8 => vec!["██████", "██  ██", "██████", "██  ██", "██████"],
        9 => vec!["██████", "██  ██", "██████", "    ██", "██████"],
        _ => vec![""; 5],
    }
}

fn get_big_colon() -> Vec<&'static str> {
    vec!["      ", "  ██  ", "      ", "  ██  ", "      "]
}
