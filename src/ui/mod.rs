use crate::app::{App, CurrentView, FormField, InputMode};
use crate::db::models::TaskPriority;
use crate::ui::theme::HORIZON;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Tabs},
    Frame,
};

mod analytics;
mod dashboard;
mod focus;
mod help;
mod inspector;
mod kanban;
mod splash;
pub mod theme;

pub fn draw(f: &mut Frame, app: &mut App) {
    // 1. Background
    let bg_block = Block::default().style(Style::default().bg(HORIZON.bg));
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
        CurrentView::Splash => splash::draw(f, f.area()),
    }

    draw_status_bar(f, app, layout[2]);

    // 3. Modals (Overlays)
    if app.input_mode == InputMode::Editing {
        draw_form_modal(f, app);
    }

    // Help Overlay
    if app.show_help {
        help::draw(f, app);
    }
}

fn draw_header_tabs(f: &mut Frame, app: &App, area: Rect) {
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
            .fg(HORIZON.bg)
            .bg(HORIZON.accent)
            .add_modifier(Modifier::BOLD),
    ))
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(HORIZON.border)),
    );
    f.render_widget(logo, chunks[0]);

    // Tabs
    let titles = vec![" DASHBOARD ", " KANBAN ", " FOCUS ", " ANALYTICS "];
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(HORIZON.border)),
        )
        .highlight_style(
            Style::default()
                .fg(HORIZON.accent)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().fg(HORIZON.dimmed))
        .select(match app.current_view {
            CurrentView::Dashboard => 0,
            CurrentView::Kanban => 1,
            CurrentView::Focus => 2,
            CurrentView::Analytics => 3,
            CurrentView::Splash => 0, // No tab selected usually, but mapping to 0
        });
    f.render_widget(tabs, chunks[1]);

    // HUD Stats
    let profile = &app.user_profile;
    let stats_text = format!(" LVL {} | XP {} ", profile.level, profile.current_xp);
    let stats = Paragraph::new(Span::styled(
        stats_text,
        Style::default().fg(HORIZON.secondary),
    ))
    .alignment(ratatui::layout::Alignment::Right)
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(HORIZON.border)),
    );
    f.render_widget(stats, chunks[2]);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let (mode_str, mode_color) = match app.input_mode {
        InputMode::Normal => (" NORMAL ", HORIZON.accent),
        InputMode::Editing => (" INSERT ", HORIZON.success),
        InputMode::Search => (" SEARCH ", HORIZON.warning),
    };

    let hints = match app.input_mode {
        InputMode::Editing => "TAB: Next Field • Enter: Save • Esc: Cancel",
        _ => match app.current_view {
            CurrentView::Dashboard => "n: New • e: Edit • d: Delete • SPC: Status • /: Search",
            CurrentView::Kanban => "h/l: Col • j/k: Task",
            CurrentView::Focus => "t: Timer • r: Reset",
            CurrentView::Splash => "Press Any Key",
            CurrentView::Analytics => "",
        },
    };

    let status = Paragraph::new(Line::from(vec![
        Span::styled(
            mode_str,
            Style::default()
                .bg(mode_color)
                .fg(HORIZON.bg)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(hints, Style::default().fg(HORIZON.dimmed)),
        Span::raw(" "),
        if !app.search_query.is_empty() {
            Span::styled(
                format!(" {}", app.search_query),
                Style::default().fg(HORIZON.warning),
            )
        } else {
            Span::raw("")
        },
    ]))
    .style(Style::default().bg(HORIZON.surface));

    f.render_widget(status, area);
}

fn draw_form_modal(f: &mut Frame, app: &mut App) {
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
                Constraint::Length(3), // XP
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

    // 2. Priority Input (Selector)
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
                .title("Priority (< > to change)")
                .border_style(Style::default().fg(prio_border)),
        )
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(HORIZON.fg).add_modifier(Modifier::BOLD));
    f.render_widget(prio_widget, chunks[1]);

    // 3. XP Input
    let xp_border = if app.task_form.active_field == FormField::XP {
        HORIZON.accent
    } else {
        HORIZON.dimmed
    };
    app.task_form.xp.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Reward (XP)")
            .border_style(Style::default().fg(xp_border)),
    );
    app.task_form.xp.set_style(Style::default().fg(HORIZON.fg));
    f.render_widget(&app.task_form.xp, chunks[2]);

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
