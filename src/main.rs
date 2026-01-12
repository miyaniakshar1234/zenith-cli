use color_eyre::eyre::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
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

use crate::app::{App, InputMode};

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
                // IMPORTANT: FIX DOUBLE TYPING BUG
                // Only process KeyPress events, ignore Release/Repeat
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Tab => app.cycle_view(),
                        KeyCode::Char('n') => app.input_mode = InputMode::Editing,

                        // Universal Navigation
                        KeyCode::Char('j') | KeyCode::Down => app.next_item(),
                        KeyCode::Char('k') | KeyCode::Up => app.previous_item(),

                        // Kanban Specific
                        KeyCode::Char('l') | KeyCode::Right => app.next_kanban_col(),
                        KeyCode::Char('h') | KeyCode::Left => app.prev_kanban_col(),

                        // Focus Specific
                        KeyCode::Char('t') => app.toggle_timer(),
                        KeyCode::Char('r') => app.reset_timer(),

                        // Actions
                        KeyCode::Char(' ') => {
                            if let Err(e) = app.toggle_status() {
                                eprintln!("Error toggling status: {}", e);
                            }
                        }
                        KeyCode::Char('d') | KeyCode::Delete => {
                            if let Err(e) = app.delete_current_task() {
                                eprintln!("Error deleting task: {}", e);
                            }
                        }
                        KeyCode::Enter => {
                            app.toggle_inspector();
                        }
                        KeyCode::Esc => {
                            if app.is_inspecting {
                                app.toggle_inspector();
                            }
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Enter => {
                            if let Err(e) = app.add_task() {
                                eprintln!("Error adding task: {}", e);
                            }
                            app.input_mode = InputMode::Normal;
                        }
                        _ => {
                            // DELEGATE TO TUI-TEXTAREA
                            app.textarea.input(key);
                        }
                    },
                }
            }
        }
    }
}
