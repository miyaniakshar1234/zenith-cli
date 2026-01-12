use crate::app::{App, CurrentView, InputMode};
use crate::ui::theme::get_theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

mod analytics;
mod dashboard;
mod focus;
mod form;
mod help;
mod inspector;
mod kanban;
mod quit;
mod splash;
pub mod theme;

pub fn draw(f: &mut Frame, app: &mut App) {
    let theme = get_theme(app.current_theme);

    // 1. Background
    let bg_block = Block::default().style(Style::default().bg(theme.bg));
    f.render_widget(bg_block, f.area());

    // 2. Main Layout (Header | Content | Footer)
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Header + Tabs
                Constraint::Min(0),    // Main Content
                Constraint::Length(1), // Footer / Status
            ]
            .as_ref(),
        )
        .split(f.area());

    draw_header_tabs(f, app, layout[0]);

    // Dispatch View
    let content_area = layout[1];
    match app.current_view {
        CurrentView::Dashboard => dashboard::draw(f, app, content_area),
        CurrentView::Kanban => kanban::draw(f, app, content_area),
        CurrentView::Focus => focus::draw(f, app, content_area),
        CurrentView::Analytics => analytics::draw(f, app, content_area),
        CurrentView::Splash => splash::draw(f, app, f.area()),
    }

    draw_status_bar(f, app, layout[2]);

    // 3. Modals (Overlays)
    if app.input_mode == InputMode::Editing {
        form::draw_form_modal(f, app);
    }

    // Help Overlay
    if app.show_help {
        help::draw(f, app);
    }

    // Quit Modal (Highest Priority)
    if app.show_quit_modal {
        quit::draw_quit_modal(f, app);
    }
}

fn draw_header_tabs(f: &mut Frame, app: &App, area: Rect) {
    let theme = get_theme(app.current_theme);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(20),
                Constraint::Min(0),
                Constraint::Length(30),
            ]
            .as_ref(),
        )
        .split(area);

    // Logo
    let logo = Paragraph::new(Span::styled(
        " ZENITH ",
        Style::default()
            .fg(theme.bg)
            .bg(theme.accent)
            .add_modifier(Modifier::BOLD),
    ))
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(theme.border)),
    );
    f.render_widget(logo, chunks[0]);

    // Tabs
    let titles = vec![" DASHBOARD ", " KANBAN ", " FOCUS ", " ANALYTICS "];
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(theme.border)),
        )
        .highlight_style(
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().fg(theme.dimmed))
        .select(match app.current_view {
            CurrentView::Dashboard => 0,
            CurrentView::Kanban => 1,
            CurrentView::Focus => 2,
            CurrentView::Analytics => 3,
            CurrentView::Splash => 0,
        });
    f.render_widget(tabs, chunks[1]);

    // HUD Stats
    let profile = &app.user_profile;
    let stats_text = format!(
        " 🔥 {} | Today: {} | LVL {} ",
        app.streak, app.tasks_today, profile.level
    );
    let stats = Paragraph::new(Span::styled(
        stats_text,
        Style::default().fg(theme.secondary),
    ))
    .alignment(ratatui::layout::Alignment::Right)
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(theme.border)),
    );
    f.render_widget(stats, chunks[2]);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let theme = get_theme(app.current_theme);

    let (mode_str, mode_color) = match app.input_mode {
        InputMode::Normal => (" NORMAL ", theme.accent),
        InputMode::Editing => (" INSERT ", theme.success),
        InputMode::Search => (" SEARCH ", theme.warning),
    };

    let hints = match app.input_mode {
        InputMode::Editing => "TAB: Next • Enter: Save • Esc: Cancel",
        _ => match app.current_view {
            CurrentView::Dashboard => {
                "n: New • e: Edit • d: Delete • SPC: Status • /: Search • T: Theme"
            }
            CurrentView::Kanban => "h/l: Col • j/k: Task • T: Theme",
            CurrentView::Focus => "t: Timer • r: Reset • T: Theme",
            CurrentView::Splash => "Press Any Key",
            CurrentView::Analytics => "T: Theme",
        },
    };

    let status = Paragraph::new(Line::from(vec![
        Span::styled(
            mode_str,
            Style::default()
                .bg(mode_color)
                .fg(theme.bg)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(hints, Style::default().fg(theme.dimmed)),
        Span::raw(" "),
        if !app.search_query.is_empty() {
            Span::styled(
                format!(" {}", app.search_query),
                Style::default().fg(theme.warning),
            )
        } else {
            Span::raw("")
        },
    ]))
    .style(Style::default().bg(theme.surface));

    f.render_widget(status, area);
}
