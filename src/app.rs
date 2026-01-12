use crate::db::{
    models::{Task, TaskStatus, UserProfile},
    Database,
};
use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use ratatui::widgets::ListState;
use tui_textarea::TextArea;

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CurrentView {
    Dashboard,
    Kanban,
    Focus,
}

pub struct FocusState {
    pub is_running: bool,
    pub duration_sec: i64,
    pub remaining_sec: i64,
    pub last_tick: Option<DateTime<Utc>>,
}

impl Default for FocusState {
    fn default() -> Self {
        Self {
            is_running: false,
            duration_sec: 25 * 60, // 25 minutes
            remaining_sec: 25 * 60,
            last_tick: None,
        }
    }
}

pub struct KanbanState {
    pub todo_state: ListState,
    pub doing_state: ListState,
    pub done_state: ListState,
    pub focused_col: usize, // 0=Todo, 1=Doing, 2=Done
}

impl Default for KanbanState {
    fn default() -> Self {
        let mut s = Self {
            todo_state: ListState::default(),
            doing_state: ListState::default(),
            done_state: ListState::default(),
            focused_col: 0,
        };
        // Select first item by default for better UX
        s.todo_state.select(Some(0));
        s.doing_state.select(Some(0));
        s.done_state.select(Some(0));
        s
    }
}

pub struct App<'a> {
    pub db: Database,
    pub tasks: Vec<Task>,
    pub user_profile: UserProfile,
    pub input_mode: InputMode,
    pub textarea: TextArea<'a>,
    pub list_state: ListState,
    pub current_view: CurrentView,
    pub focus_state: FocusState,
    pub kanban_state: KanbanState,
}

impl<'a> App<'a> {
    pub fn new() -> Result<Self> {
        let db = Database::init()?;
        let tasks = db.get_all_tasks()?;
        let user_profile = db.get_user_profile()?;
        let mut list_state = ListState::default();
        if !tasks.is_empty() {
            list_state.select(Some(0));
        }

        let mut textarea = TextArea::default();
        textarea.set_placeholder_text("Type task description...");

        Ok(Self {
            db,
            tasks,
            user_profile,
            input_mode: InputMode::Normal,
            textarea,
            list_state,
            current_view: CurrentView::Dashboard,
            focus_state: FocusState::default(),
            kanban_state: KanbanState::default(),
        })
    }

    pub fn refresh_state(&mut self) -> Result<()> {
        self.tasks = self.db.get_all_tasks()?;
        self.user_profile = self.db.get_user_profile()?;

        // Ensure selections remain valid
        if self.list_state.selected().is_none() && !self.tasks.is_empty() {
            self.list_state.select(Some(0));
        }

        Ok(())
    }

    pub fn add_task(&mut self) -> Result<()> {
        let content = self.textarea.lines().join("\n");
        if content.trim().is_empty() {
            return Ok(());
        }
        // Split title and description? For now, just title
        let title = content.lines().next().unwrap_or("").to_string();

        let task = Task::new(title, String::new(), 10);
        self.db.create_task(&task)?;

        // Reset textarea
        self.textarea = TextArea::default();
        self.textarea
            .set_placeholder_text("Type task description...");

        self.refresh_state()?;
        Ok(())
    }

    // Navigation Logic
    pub fn next_item(&mut self) {
        match self.current_view {
            CurrentView::Dashboard => self.next_dashboard_task(),
            CurrentView::Kanban => self.next_kanban_item(),
            CurrentView::Focus => {} // No list in Focus mode
        }
    }

    pub fn previous_item(&mut self) {
        match self.current_view {
            CurrentView::Dashboard => self.previous_dashboard_task(),
            CurrentView::Kanban => self.previous_kanban_item(),
            CurrentView::Focus => {}
        }
    }

    fn next_dashboard_task(&mut self) {
        if self.tasks.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.tasks.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn previous_dashboard_task(&mut self) {
        if self.tasks.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tasks.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    // Kanban Navigation
    fn next_kanban_item(&mut self) {
        let status = self.get_status_from_col(self.kanban_state.focused_col);
        let count = self.tasks.iter().filter(|t| t.status == status).count();
        if count == 0 {
            return;
        }

        let state = match self.kanban_state.focused_col {
            0 => &mut self.kanban_state.todo_state,
            1 => &mut self.kanban_state.doing_state,
            2 => &mut self.kanban_state.done_state,
            _ => return,
        };

        let i = match state.selected() {
            Some(i) => {
                if i >= count - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }

    fn previous_kanban_item(&mut self) {
        let status = self.get_status_from_col(self.kanban_state.focused_col);
        let count = self.tasks.iter().filter(|t| t.status == status).count();
        if count == 0 {
            return;
        }

        let state = match self.kanban_state.focused_col {
            0 => &mut self.kanban_state.todo_state,
            1 => &mut self.kanban_state.doing_state,
            2 => &mut self.kanban_state.done_state,
            _ => return,
        };

        let i = match state.selected() {
            Some(i) => {
                if i == 0 {
                    count - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }

    pub fn next_kanban_col(&mut self) {
        if self.kanban_state.focused_col < 2 {
            self.kanban_state.focused_col += 1;
        } else {
            self.kanban_state.focused_col = 0;
        }
    }

    pub fn prev_kanban_col(&mut self) {
        if self.kanban_state.focused_col > 0 {
            self.kanban_state.focused_col -= 1;
        } else {
            self.kanban_state.focused_col = 2;
        }
    }

    fn get_status_from_col(&self, col: usize) -> TaskStatus {
        match col {
            0 => TaskStatus::Todo,
            1 => TaskStatus::Doing,
            2 => TaskStatus::Done,
            _ => TaskStatus::Todo,
        }
    }

    pub fn cycle_view(&mut self) {
        self.current_view = match self.current_view {
            CurrentView::Dashboard => CurrentView::Kanban,
            CurrentView::Kanban => CurrentView::Focus,
            CurrentView::Focus => CurrentView::Dashboard,
        };
    }

    pub fn toggle_status(&mut self) -> Result<()> {
        // Simplified: Only allow toggling from Dashboard for now to ensure ID safety
        if self.current_view != CurrentView::Dashboard {
            return Ok(());
        }

        if let Some(i) = self.list_state.selected() {
            if let Some(task) = self.tasks.get(i) {
                let new_status = match task.status {
                    TaskStatus::Todo => TaskStatus::Doing,
                    TaskStatus::Doing => TaskStatus::Done,
                    TaskStatus::Done => TaskStatus::Todo,
                };

                if new_status == TaskStatus::Done && task.status != TaskStatus::Done {
                    self.db.add_xp(task.xp_reward)?;
                }

                self.db.update_task_status(&task.id, new_status)?;
                self.refresh_state()?;
                if i < self.tasks.len() {
                    self.list_state.select(Some(i));
                }
            }
        }
        Ok(())
    }

    pub fn delete_current_task(&mut self) -> Result<()> {
        if self.current_view != CurrentView::Dashboard {
            return Ok(());
        }

        if let Some(i) = self.list_state.selected() {
            if let Some(task) = self.tasks.get(i) {
                self.db.delete_task(&task.id)?;
                self.refresh_state()?;

                // Adjust selection
                if self.tasks.is_empty() {
                    self.list_state.select(None);
                } else if i >= self.tasks.len() {
                    self.list_state.select(Some(self.tasks.len() - 1));
                } else {
                    self.list_state.select(Some(i));
                }
            }
        }
        Ok(())
    }

    // Focus Mode Logic
    pub fn toggle_timer(&mut self) {
        self.focus_state.is_running = !self.focus_state.is_running;
        if self.focus_state.is_running {
            self.focus_state.last_tick = Some(Utc::now());
        } else {
            self.focus_state.last_tick = None;
        }
    }

    pub fn reset_timer(&mut self) {
        self.focus_state.is_running = false;
        self.focus_state.remaining_sec = self.focus_state.duration_sec;
        self.focus_state.last_tick = None;
    }

    pub fn on_tick(&mut self) {
        if self.focus_state.is_running {
            let now = Utc::now();
            if let Some(last_tick) = self.focus_state.last_tick {
                let delta = now.signed_duration_since(last_tick).num_seconds();
                if delta > 0 {
                    if self.focus_state.remaining_sec >= delta as i64 {
                        self.focus_state.remaining_sec -= delta as i64;
                    } else {
                        self.focus_state.remaining_sec = 0;
                        self.focus_state.is_running = false;
                        // Optionally: Auto-add XP for finishing a pomodoro
                    }
                    self.focus_state.last_tick = Some(now);
                }
            } else {
                self.focus_state.last_tick = Some(now);
            }
        }
    }
}
