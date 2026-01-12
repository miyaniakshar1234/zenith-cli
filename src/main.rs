use color_eyre::eyre::Result;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

mod app;
mod db;
mod inputs;
mod ui;
mod utils;

use crate::app::{App, FormField, InputMode};
use crate::db::models::TaskPriority;

fn main() -> Result<()> {
    // 1. Setup Error Handling
    color_eyre::install()?;

    // 2. Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 3. Initialize App State
    let mut app = App::new()?;

    // 4. Run Main Loop
    let res = run_app(&mut terminal, &mut app);

    // 5. Restore Terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        // Handle Timer Tick
        app.on_tick();

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if (key.code == KeyCode::Char('c') || key.code == KeyCode::Char('q'))
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    return Ok(());
                }

                if app.current_view == crate::app::CurrentView::Splash {
                    if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                    app.current_view = crate::app::CurrentView::Dashboard;
                    continue;
                }

                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('?') => app.show_help = !app.show_help,
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Tab => app.cycle_view(),
                        KeyCode::Char('n') => {
                            app.editing_task_id = None;
                            app.task_form = crate::app::TaskForm::default();
                            app.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('e') => app.start_editing(),
                        KeyCode::Char('j') | KeyCode::Down => app.next_item(),
                        KeyCode::Char('k') | KeyCode::Up => app.previous_item(),
                        KeyCode::Char('l') | KeyCode::Right => app.next_kanban_col(),
                        KeyCode::Char('h') | KeyCode::Left => app.prev_kanban_col(),
                        KeyCode::Char('t') => app.toggle_timer(),
                        KeyCode::Char('r') => app.reset_timer(),
                        KeyCode::Char(' ') => {
                            if let Err(e) = app.toggle_status() {
                                eprintln!("Error: {}", e);
                            }
                        }
                        KeyCode::Char('d') | KeyCode::Delete => {
                            if let Err(e) = app.delete_current_task() {
                                eprintln!("Error: {}", e);
                            }
                        }
                        KeyCode::Char('/') => app.input_mode = InputMode::Search,
                        KeyCode::Enter => app.toggle_inspector(),
                        KeyCode::Esc => {
                            if app.is_inspecting {
                                app.toggle_inspector();
                            }
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Esc => app.input_mode = InputMode::Normal,
                        KeyCode::Tab => {
                            // Cycle Fields
                            app.task_form.active_field = match app.task_form.active_field {
                                FormField::Title => FormField::Priority,
                                FormField::Priority => FormField::XP,
                                FormField::XP => FormField::DueDate,
                                FormField::DueDate => FormField::Description,
                                FormField::Description => FormField::Title,
                            };
                        }
                        KeyCode::BackTab => {
                            app.task_form.active_field = match app.task_form.active_field {
                                FormField::Title => FormField::Description,
                                FormField::Priority => FormField::Title,
                                FormField::XP => FormField::Priority,
                                FormField::DueDate => FormField::XP,
                                FormField::Description => FormField::DueDate,
                            };
                        }
                        // Priority Handling
                        KeyCode::Left | KeyCode::Char('h')
                            if app.task_form.active_field == FormField::Priority =>
                        {
                            app.task_form.priority = match app.task_form.priority {
                                TaskPriority::High => TaskPriority::Medium,
                                TaskPriority::Medium => TaskPriority::Low,
                                TaskPriority::Low => TaskPriority::High, // Cycle
                            };
                        }
                        KeyCode::Right | KeyCode::Char('l')
                            if app.task_form.active_field == FormField::Priority =>
                        {
                            app.task_form.priority = match app.task_form.priority {
                                TaskPriority::High => TaskPriority::Low,
                                TaskPriority::Medium => TaskPriority::High,
                                TaskPriority::Low => TaskPriority::Medium,
                            };
                        }
                        // Save
                        KeyCode::Enter if app.task_form.active_field != FormField::Description => {
                            // If in description, enter allows newlines.
                            // If in other fields, it saves (or we can force Ctrl+S)
                            // Let's make Enter save unless in Description?
                            // Or better: Tab to navigate, Ctrl+S or global Enter to save.
                            // User requested "friendly".
                            // Standard form behavior: Enter submits.
                            if let Err(e) = app.save_task() {
                                eprintln!("Error: {}", e);
                            }
                            app.input_mode = InputMode::Normal;
                        }
                        // Text Input
                        _ => {
                            match app.task_form.active_field {
                                FormField::Title => {
                                    app.task_form.title.input(key);
                                }
                                FormField::Description => {
                                    app.task_form.description.input(key);
                                }
                                FormField::XP => {
                                    // Restrict to numbers
                                    if let KeyCode::Char(c) = key.code {
                                        if c.is_numeric() {
                                            app.task_form.xp.input(key);
                                        }
                                    } else if key.code == KeyCode::Backspace
                                        || key.code == KeyCode::Delete
                                    {
                                        app.task_form.xp.input(key);
                                    }
                                }
                                FormField::DueDate => {
                                    app.task_form.due_date.input(key);
                                }
                                FormField::Priority => {} // Handled above
                            }
                        }
                    },
                    InputMode::Search => match key.code {
                        KeyCode::Enter | KeyCode::Esc => app.input_mode = InputMode::Normal,
                        KeyCode::Backspace => {
                            app.search_query.pop();
                            app.refresh_state().unwrap();
                        }
                        KeyCode::Char(c) => {
                            app.search_query.push(c);
                            app.refresh_state().unwrap();
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}
