use crate::app::{App, InputMode};
use crate::db::models::TaskStatus;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), // Header (Level/XP)
                Constraint::Min(0),    // Content (Tasks)
                Constraint::Length(3), // Footer / Input
            ]
            .as_ref(),
        )
        .split(f.area());

    draw_header(f, app, chunks[0]);
    draw_task_list(f, app, chunks[1]);
    draw_input_area(f, app, chunks[2]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    let title = Paragraph::new(vec![Line::from(vec![
        Span::styled(
            "ZENITH",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::styled(
            "Cyberpunk Task Manager",
            Style::default().fg(Color::Magenta),
        ),
    ])])
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(title, chunks[0]);

    // XP Bar
    let profile = &app.user_profile;
    let label = format!(
        "Lvl {} | XP: {}/{}",
        profile.level, profile.current_xp, profile.next_level_xp
    );
    let ratio = profile.current_xp as f64 / profile.next_level_xp as f64;

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("User Stats"))
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(ratio.clamp(0.0, 1.0))
        .label(label);

    f.render_widget(gauge, chunks[1]);
}

fn draw_task_list(f: &mut Frame, app: &mut App, area: Rect) {
    let tasks: Vec<ListItem> = app
        .tasks
        .iter()
        .map(|task| {
            let (status_icon, color) = match task.status {
                TaskStatus::Todo => ("TODO ", Color::Red),
                TaskStatus::Doing => ("DOING", Color::Yellow),
                TaskStatus::Done => ("DONE ", Color::Green),
            };

            let content = Line::from(vec![
                Span::styled(
                    format!("[{}] ", status_icon),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{:<30}", &task.title),
                    Style::default().fg(Color::White),
                ),
                Span::styled(
                    format!(" ({} XP)", task.xp_reward),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);

            ListItem::new(content)
        })
        .collect();

    let tasks_list = List::new(tasks)
        .block(Block::default().borders(Borders::ALL).title("Tasks"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(tasks_list, area, &mut app.list_state);
}

fn draw_input_area(f: &mut Frame, app: &App, area: Rect) {
    let input_block = Block::default().borders(Borders::ALL).title("Input");

    match app.input_mode {
        InputMode::Normal => {
            let help_text =
                Paragraph::new("n: New Task | q: Quit | SPC: Toggle Status | j/k: Navigate")
                    .style(Style::default().fg(Color::Gray))
                    .block(input_block);
            f.render_widget(help_text, area);
        }
        InputMode::Editing => {
            let input = Paragraph::new(app.input_buffer.as_str())
                .style(Style::default().fg(Color::Yellow))
                .block(input_block.title("Create New Task (Enter to Save, Esc to Cancel)"));
            f.render_widget(input, area);
        }
    }
}
