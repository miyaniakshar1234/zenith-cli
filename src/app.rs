use crate::db::{
    models::{Task, TaskPriority, TaskStatus, UserProfile},
    Database,
};
use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use ratatui::widgets::{ListState, TableState};
use tui_textarea::TextArea;

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
    Search,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CurrentView {
    Splash,
    Dashboard,
    Kanban,
    Focus,
    Analytics,
}

#[derive(PartialEq, Clone, Copy)]
pub enum FormField {
    Title,
    Priority,
    XP,
    DueDate,
    Description,
}

pub struct TaskForm<'a> {
    pub title: TextArea<'a>,
    pub description: TextArea<'a>,
    pub priority: TaskPriority,
    pub xp: TextArea<'a>,
    pub due_date: TextArea<'a>,
    pub active_field: FormField,
}

impl<'a> Default for TaskForm<'a> {
    fn default() -> Self {
        let mut title = TextArea::default();
        title.set_placeholder_text("Task Title...");

        let mut description = TextArea::default();
        description.set_placeholder_text("Detailed description...");

        let mut xp = TextArea::default();
        xp.set_placeholder_text("10");
        xp.insert_str("10");

        let mut due_date = TextArea::default();
        due_date.set_placeholder_text("YYYY-MM-DD");

        Self {
            title,
            description,
            priority: TaskPriority::Medium,
            xp,
            due_date,
            active_field: FormField::Title,
        }
    }
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
            duration_sec: 25 * 60,
            remaining_sec: 25 * 60,
            last_tick: None,
        }
    }
}

pub struct KanbanState {
    pub todo_state: ListState,
    pub doing_state: ListState,
    pub done_state: ListState,
    pub focused_col: usize,
}

impl Default for KanbanState {
    fn default() -> Self {
        let mut s = Self {
            todo_state: ListState::default(),
            doing_state: ListState::default(),
            done_state: ListState::default(),
            focused_col: 0,
        };
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
    pub task_form: TaskForm<'a>,
    pub table_state: TableState,
    pub current_view: CurrentView,
    pub focus_state: FocusState,
    pub kanban_state: KanbanState,
    pub is_inspecting: bool,
    pub editing_task_id: Option<String>,
    pub search_query: String,
    pub stats: Vec<(String, u64)>,
    pub show_help: bool,
}

impl<'a> App<'a> {
    pub fn new() -> Result<Self> {
        let db = Database::init()?;
        let tasks = db.get_all_tasks()?;
        let user_profile = db.get_user_profile()?;
        let stats = db.get_weekly_stats()?;

        let mut table_state = TableState::default();
        if !tasks.is_empty() {
            table_state.select(Some(0));
        }

        Ok(Self {
            db,
            tasks,
            user_profile,
            input_mode: InputMode::Normal,
            task_form: TaskForm::default(),
            table_state,
            current_view: CurrentView::Splash,
            focus_state: FocusState::default(),
            kanban_state: KanbanState::default(),
            is_inspecting: false,
            editing_task_id: None,
            search_query: String::new(),
            stats,
            show_help: false,
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

        if self.table_state.selected().is_none() && !self.tasks.is_empty() {
            self.table_state.select(Some(0));
        } else if self.table_state.selected().unwrap_or(0) >= self.tasks.len() {
            self.table_state
                .select(Some(self.tasks.len().saturating_sub(1)));
        }

        Ok(())
    }

    pub fn save_task(&mut self) -> Result<()> {
        if self.task_form.title.lines().join("").trim().is_empty() {
            return Ok(());
        }

        let title = self.task_form.title.lines().join(" ").trim().to_string();
        let description = self
            .task_form
            .description
            .lines()
            .join("\n")
            .trim()
            .to_string();
        let priority = self.task_form.priority;
        let xp_str = self.task_form.xp.lines().join("").trim().to_string();
        let xp_reward = xp_str.parse::<i32>().unwrap_or(10);

        let due_date_str = self.task_form.due_date.lines().join("").trim().to_string();
        let due_date = if due_date_str.is_empty() {
            None
        } else {
            if let Ok(naive) = chrono::NaiveDate::parse_from_str(&due_date_str, "%Y-%m-%d") {
                Some(DateTime::from_naive_utc_and_offset(
                    naive.and_hms_opt(23, 59, 59).unwrap(),
                    Utc,
                ))
            } else {
                None
            }
        };

        if let Some(id) = &self.editing_task_id {
            self.db
                .update_task_content(id, &title, &description, priority, due_date)?;
        } else {
            let task = Task::new(title, description, priority, xp_reward, due_date);
            self.db.create_task(&task)?;
        }

        self.editing_task_id = None;
        self.task_form = TaskForm::default();
        self.refresh_state()?;
        Ok(())
    }

    pub fn start_editing(&mut self) {
        if self.current_view != CurrentView::Dashboard {
            return;
        }

        if let Some(i) = self.table_state.selected() {
            if let Some(task) = self.tasks.get(i) {
                self.editing_task_id = Some(task.id.clone());

                self.task_form.title = TextArea::new(vec![task.title.clone()]);
                self.task_form.description =
                    TextArea::new(task.description.lines().map(|s| s.to_string()).collect());
                self.task_form.priority = task.priority;
                self.task_form.xp = TextArea::new(vec![task.xp_reward.to_string()]);

                let due_str = task
                    .due_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_default();
                self.task_form.due_date = TextArea::new(vec![due_str]);

                self.task_form.active_field = FormField::Title;
                self.input_mode = InputMode::Editing;
            }
        }
    }

    pub fn toggle_inspector(&mut self) {
        if self.current_view == CurrentView::Dashboard && !self.tasks.is_empty() {
            self.is_inspecting = !self.is_inspecting;
        }
    }

    pub fn next_item(&mut self) {
        match self.current_view {
            CurrentView::Dashboard => self.next_dashboard_task(),
            CurrentView::Kanban => self.next_kanban_item(),
            CurrentView::Focus | CurrentView::Analytics | CurrentView::Splash => {}
        }
    }

    pub fn previous_item(&mut self) {
        match self.current_view {
            CurrentView::Dashboard => self.previous_dashboard_task(),
            CurrentView::Kanban => self.previous_kanban_item(),
            CurrentView::Focus | CurrentView::Analytics | CurrentView::Splash => {}
        }
    }

    fn next_dashboard_task(&mut self) {
        if self.tasks.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.tasks.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn previous_dashboard_task(&mut self) {
        if self.tasks.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tasks.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

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
            CurrentView::Splash => CurrentView::Dashboard,
            CurrentView::Dashboard => CurrentView::Kanban,
            CurrentView::Kanban => CurrentView::Focus,
            CurrentView::Focus => CurrentView::Analytics,
            CurrentView::Analytics => CurrentView::Dashboard,
        };
    }

    pub fn toggle_status(&mut self) -> Result<()> {
        if self.current_view != CurrentView::Dashboard {
            return Ok(());
        }

        if let Some(i) = self.table_state.selected() {
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
                    self.table_state.select(Some(i));
                }
            }
        }
        Ok(())
    }

    pub fn delete_current_task(&mut self) -> Result<()> {
        if self.current_view != CurrentView::Dashboard {
            return Ok(());
        }

        if let Some(i) = self.table_state.selected() {
            if let Some(task) = self.tasks.get(i) {
                self.db.delete_task(&task.id)?;
                self.refresh_state()?;

                if self.tasks.is_empty() {
                    self.table_state.select(None);
                } else if i >= self.tasks.len() {
                    self.table_state.select(Some(self.tasks.len() - 1));
                } else {
                    self.table_state.select(Some(i));
                }
            }
        }
        Ok(())
    }

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
                    }
                    self.focus_state.last_tick = Some(now);
                }
            } else {
                self.focus_state.last_tick = Some(now);
            }
        }
    }
}
