use crate::app::{App, CurrentView, InputMode};
use crate::ui::theme::NEON_CYBERPUNK;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Gauge, Paragraph, Tabs},
    Frame,
};

mod dashboard;
mod focus;
mod kanban;
pub mod theme;

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), // Header
                Constraint::Length(3), // Tabs
                Constraint::Min(0),    // Content
                Constraint::Length(1), // Footer (Minimal)
            ]
            .as_ref(),
        )
        .split(f.area());

    // Background coloring (optional, if terminal supports it)
    let bg_block = Block::default().style(Style::default().bg(NEON_CYBERPUNK.background));
    f.render_widget(bg_block, f.area());

    draw_header(f, app, chunks[0]);
    draw_tabs(f, app, chunks[1]);

    // Dispatch to views
    match app.current_view {
        CurrentView::Dashboard => dashboard::draw(f, app, chunks[2]),
        CurrentView::Kanban => kanban::draw(f, app, chunks[2]),
        CurrentView::Focus => focus::draw(f, app, chunks[2]),
    }

    draw_footer(f, app, chunks[3]);

    // DRAW MODAL IF EDITING
    if app.input_mode == InputMode::Editing {
        draw_input_modal(f, app);
    }
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
                .fg(NEON_CYBERPUNK.primary)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::styled(
            "Cyberpunk Task Manager",
            Style::default().fg(NEON_CYBERPUNK.secondary),
        ),
    ])])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(NEON_CYBERPUNK.text_dim)),
    );

    f.render_widget(title, chunks[0]);

    // XP Bar
    let profile = &app.user_profile;
    let label = format!(
        "Lvl {} | XP: {}/{}",
        profile.level, profile.current_xp, profile.next_level_xp
    );
    let ratio = profile.current_xp as f64 / profile.next_level_xp as f64;

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("User Stats")
                .border_style(Style::default().fg(NEON_CYBERPUNK.text_dim)),
        )
        .gauge_style(Style::default().fg(NEON_CYBERPUNK.success))
        .ratio(ratio.clamp(0.0, 1.0))
        .label(label);

    f.render_widget(gauge, chunks[1]);
}

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles = vec!["Dashboard", "Kanban", "Focus"];
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(NEON_CYBERPUNK.text_dim)),
        )
        .highlight_style(
            Style::default()
                .fg(NEON_CYBERPUNK.accent)
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
    let help_text = match app.current_view {
        CurrentView::Kanban => "TAB: Switch View | h/l: Change Col | j/k: Nav | n: New Task",
        CurrentView::Focus => "TAB: Switch View | t: Toggle Timer | r: Reset",
        _ => "TAB: Switch View | n: New Task | SPC: Toggle | d: Delete | j/k: Nav",
    };

    let p = Paragraph::new(help_text)
        .style(Style::default().fg(NEON_CYBERPUNK.text_dim))
        .centered();
    f.render_widget(p, area);
}

fn draw_input_modal(f: &mut Frame, app: &mut App) {
    let area = centered_rect(60, 25, f.area()); // 60% width, 25% height

    // Clear area so it sits on top
    f.render_widget(Clear, area);

    // Modal styling
    app.textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double) // Fancy double border for modal
            .border_style(Style::default().fg(NEON_CYBERPUNK.accent))
            .title(" NEW TASK ")
            .title_style(
                Style::default()
                    .fg(NEON_CYBERPUNK.primary)
                    .add_modifier(Modifier::BOLD),
            ),
    );

    app.textarea
        .set_style(Style::default().fg(NEON_CYBERPUNK.text_main));
    app.textarea
        .set_cursor_style(Style::default().bg(NEON_CYBERPUNK.secondary));

    f.render_widget(&app.textarea, area);
}

/// Helper function to center a rect in another rect
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
