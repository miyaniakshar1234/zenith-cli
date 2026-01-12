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
    Search,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CurrentView {
    Dashboard,
    Kanban,
    Focus,
    Analytics,
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
    pub is_inspecting: bool,
    pub editing_task_id: Option<String>,
    pub search_query: String,
    pub stats: Vec<(String, u64)>,
}

impl<'a> App<'a> {
    pub fn new() -> Result<Self> {
        let db = Database::init()?;
        let tasks = db.get_all_tasks()?;
        let user_profile = db.get_user_profile()?;
        let stats = db.get_weekly_stats()?;

        let mut list_state = ListState::default();
        if !tasks.is_empty() {
            list_state.select(Some(0));
        }

        let mut textarea = TextArea::default();
        textarea.set_placeholder_text("Title (Line 1)\nDescription (Line 2+)...");

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
            is_inspecting: false,
            editing_task_id: None,
            search_query: String::new(),
            stats,
        })
    }

    pub fn refresh_state(&mut self) -> Result<()> {
        let all_tasks = self.db.get_all_tasks()?;

        if self.search_query.is_empty() {
            self.tasks = all_tasks;
        } else {
            let q = self.search_query.to_lowercase();
            self.tasks = all_tasks
                .into_iter()
                .filter(|t| {
                    t.title.to_lowercase().contains(&q) || t.description.to_lowercase().contains(&q)
                })
                .collect();
        }

        self.user_profile = self.db.get_user_profile()?;
        self.stats = self.db.get_weekly_stats()?;

        // Ensure selections remain valid
        if self.list_state.selected().is_none() && !self.tasks.is_empty() {
            self.list_state.select(Some(0));
        } else if self.list_state.selected().unwrap_or(0) >= self.tasks.len() {
            self.list_state
                .select(Some(self.tasks.len().saturating_sub(1)));
        }

        Ok(())
    }

    pub fn save_task(&mut self) -> Result<()> {
        let lines = self.textarea.lines();
        if lines.is_empty() || lines[0].trim().is_empty() {
            return Ok(());
        }

        let title = lines[0].trim().to_string();
        let description = if lines.len() > 1 {
            lines[1..].join("\n").trim().to_string()
        } else {
            String::new()
        };

        if let Some(id) = &self.editing_task_id {
            // Update existing
            self.db.update_task_content(id, &title, &description)?;
        } else {
            // Create new
            let task = Task::new(title, description, 10);
            self.db.create_task(&task)?;
        }

        // Reset
        self.editing_task_id = None;
        self.textarea = TextArea::default();
        self.textarea
            .set_placeholder_text("Title (Line 1)\nDescription (Line 2+)...");

        self.refresh_state()?;
        Ok(())
    }

    pub fn start_editing(&mut self) {
        if self.current_view != CurrentView::Dashboard {
            return;
        }

        if let Some(i) = self.list_state.selected() {
            if let Some(task) = self.tasks.get(i) {
                self.editing_task_id = Some(task.id.clone());

                // Pre-fill text area
                let mut text = task.title.clone();
                if !task.description.is_empty() {
                    text.push('\n');
                    text.push_str(&task.description);
                }

                self.textarea = TextArea::new(text.lines().map(|s| s.to_string()).collect());
                self.input_mode = InputMode::Editing;
            }
        }
    }

    pub fn toggle_inspector(&mut self) {
        if self.current_view == CurrentView::Dashboard && !self.tasks.is_empty() {
            self.is_inspecting = !self.is_inspecting;
        }
    }

    // Navigation Logic
    pub fn next_item(&mut self) {
        match self.current_view {
            CurrentView::Dashboard => self.next_dashboard_task(),
            CurrentView::Kanban => self.next_kanban_item(),
            CurrentView::Focus | CurrentView::Analytics => {}
        }
    }

    pub fn previous_item(&mut self) {
        match self.current_view {
            CurrentView::Dashboard => self.previous_dashboard_task(),
            CurrentView::Kanban => self.previous_kanban_item(),
            CurrentView::Focus | CurrentView::Analytics => {}
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
            CurrentView::Focus => CurrentView::Analytics,
            CurrentView::Analytics => CurrentView::Dashboard,
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
                    if self.focus_state.remaining_sec >= delta {
                        self.focus_state.remaining_sec -= delta;
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
