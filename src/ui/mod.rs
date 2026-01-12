use crate::app::{App, CurrentView, InputMode};
use crate::ui::theme::NORD_PRO;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

mod dashboard;
mod focus;
mod inspector;
mod kanban;
pub mod theme;

pub fn draw(f: &mut Frame, app: &mut App) {
    // 1. Main Layout (Sidebar | Content)
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(20), // Sidebar Width
                Constraint::Min(0),     // Content Area
            ]
            .as_ref(),
        )
        .split(f.area());

    // 2. Sidebar & Content
    draw_sidebar(f, app, main_layout[0]);

    // Content Wrapper (Header + View + Status Bar)
    let content_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Header
                Constraint::Min(0),    // View
                Constraint::Length(1), // Status Bar
            ]
            .as_ref(),
        )
        .split(main_layout[1]);

    draw_header(f, app, content_layout[0]);

    // Dispatch View
    match app.current_view {
        CurrentView::Dashboard => dashboard::draw(f, app, content_layout[1]),
        CurrentView::Kanban => kanban::draw(f, app, content_layout[1]),
        CurrentView::Focus => focus::draw(f, app, content_layout[1]),
    }

    draw_status_bar(f, app, content_layout[2]);

    // 3. Modals (Overlays)
    if app.input_mode == InputMode::Editing {
        draw_input_modal(f, app);
    }

    // 4. Inspector Modal
    if app.is_inspecting {
        inspector::draw(f, app);
    }
}

fn draw_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let items = vec![
        ListItem::new("   Dashboard"),
        ListItem::new("   Kanban Board"),
        ListItem::new("   Focus Timer"),
    ];

    let current_idx = match app.current_view {
        CurrentView::Dashboard => 0,
        CurrentView::Kanban => 1,
        CurrentView::Focus => 2,
    };

    let nav = List::new(items)
        .block(
            Block::default()
                .borders(Borders::RIGHT)
                .style(Style::default().bg(NORD_PRO.bg))
                .border_style(Style::default().fg(NORD_PRO.border)),
        )
        .highlight_style(
            Style::default()
                .bg(NORD_PRO.selection_bg)
                .fg(NORD_PRO.accent)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▎"); // Modern sidebar marker

    // We fake a ListState for the sidebar just to render the selection
    let mut state = ratatui::widgets::ListState::default();
    state.select(Some(current_idx));

    f.render_stateful_widget(nav, area, &mut state);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(25)].as_ref())
        .split(area);

    // Breadcrumb / Page Title
    let page_title = match app.current_view {
        CurrentView::Dashboard => "DASHBOARD",
        CurrentView::Kanban => "WORKFLOW / KANBAN",
        CurrentView::Focus => "DEEP WORK / FOCUS",
    };

    let title = Paragraph::new(Line::from(vec![
        Span::styled(
            " ZENITH ",
            Style::default()
                .fg(NORD_PRO.bg)
                .bg(NORD_PRO.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" ", Style::default().bg(NORD_PRO.bg)),
        Span::styled(
            page_title,
            Style::default()
                .fg(NORD_PRO.fg)
                .add_modifier(Modifier::BOLD),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(NORD_PRO.border)),
    );

    f.render_widget(title, chunks[0]);

    // Mini Stats
    let profile = &app.user_profile;
    let stats = Paragraph::new(format!(
        "Lvl {} • {} XP ",
        profile.level, profile.current_xp
    ))
    .style(Style::default().fg(NORD_PRO.inactive))
    .alignment(ratatui::layout::Alignment::Right)
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(NORD_PRO.border)),
    );

    f.render_widget(stats, chunks[1]);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let (mode_str, mode_color) = match app.input_mode {
        InputMode::Normal => (" NORMAL ", NORD_PRO.accent),
        InputMode::Editing => (" INSERT ", NORD_PRO.success),
    };

    let hints = match app.current_view {
        CurrentView::Dashboard => "n: New • d: Delete • SPC: Complete • j/k: Nav",
        CurrentView::Kanban => "h/l: Col • j/k: Task",
        CurrentView::Focus => "t: Start/Stop • r: Reset",
    };

    let status = Paragraph::new(Line::from(vec![
        Span::styled(
            mode_str,
            Style::default()
                .bg(mode_color)
                .fg(NORD_PRO.bg)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(hints, Style::default().fg(NORD_PRO.inactive)),
    ]))
    .style(Style::default().bg(NORD_PRO.bg));

    f.render_widget(status, area);
}

fn draw_input_modal(f: &mut Frame, app: &mut App) {
    let area = centered_rect(50, 20, f.area());
    f.render_widget(Clear, area);

    app.textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(NORD_PRO.accent))
            .title(" NEW TASK "),
    );
    app.textarea.set_style(Style::default().fg(NORD_PRO.fg));
    app.textarea
        .set_cursor_style(Style::default().bg(NORD_PRO.accent));

    f.render_widget(&app.textarea, area);
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
