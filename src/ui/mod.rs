use crate::app::{App, CurrentView, InputMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Tabs},
    Frame,
};

mod dashboard;
mod focus;
mod kanban;

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), // Header
                Constraint::Length(3), // Tabs
                Constraint::Min(0),    // Content
                Constraint::Length(3), // Footer
            ]
            .as_ref(),
        )
        .split(f.area());

    draw_header(f, app, chunks[0]);
    draw_tabs(f, app, chunks[1]);

    // Dispatch to views
    match app.current_view {
        CurrentView::Dashboard => dashboard::draw(f, app, chunks[2]),
        CurrentView::Kanban => kanban::draw(f, app, chunks[2]),
        CurrentView::Focus => focus::draw(f, app, chunks[2]),
    }

    draw_footer(f, app, chunks[3]);
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

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles = vec!["Dashboard", "Kanban", "Focus"];
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Views"))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .select(match app.current_view {
            CurrentView::Dashboard => 0,
            CurrentView::Kanban => 1,
            CurrentView::Focus => 2,
        });
    f.render_widget(tabs, area);
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let input_block = Block::default().borders(Borders::ALL).title("Input");

    match app.input_mode {
        InputMode::Normal => {
            let help_text = match app.current_view {
                CurrentView::Kanban => {
                    "TAB: Switch View | h/l: Change Col | j/k: Nav | n: New Task"
                }
                CurrentView::Focus => "TAB: Switch View | t: Toggle Timer | r: Reset",
                _ => "TAB: Switch View | n: New Task | SPC: Toggle | d: Delete | j/k: Nav",
            };

            let p = Paragraph::new(help_text)
                .style(Style::default().fg(Color::Gray))
                .block(input_block);
            f.render_widget(p, area);
        }
        InputMode::Editing => {
            let input = Paragraph::new(app.input_buffer.as_str())
                .style(Style::default().fg(Color::Yellow))
                .block(input_block.title("Create New Task (Enter to Save, Esc to Cancel)"));
            f.render_widget(input, area);
        }
    }
}
