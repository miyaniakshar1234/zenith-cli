use crate::db::{
    models::{Task, TaskStatus, UserProfile},
    Database,
};
use color_eyre::eyre::Result;
use ratatui::widgets::ListState;

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub running: bool,
    pub db: Database,
    pub tasks: Vec<Task>,
    pub user_profile: UserProfile,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub list_state: ListState,
}

impl App {
    pub fn new() -> Result<Self> {
        let db = Database::init()?;
        let tasks = db.get_all_tasks()?;
        let user_profile = db.get_user_profile()?;
        let mut list_state = ListState::default();
        if !tasks.is_empty() {
            list_state.select(Some(0));
        }

        Ok(Self {
            running: true,
            db,
            tasks,
            user_profile,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            list_state,
        })
    }

    pub fn refresh_state(&mut self) -> Result<()> {
        self.tasks = self.db.get_all_tasks()?;
        self.user_profile = self.db.get_user_profile()?;
        if self.list_state.selected().is_none() && !self.tasks.is_empty() {
            self.list_state.select(Some(0));
        }
        Ok(())
    }

    pub fn add_task(&mut self) -> Result<()> {
        if self.input_buffer.trim().is_empty() {
            return Ok(());
        }
        let task = Task::new(self.input_buffer.clone(), String::new(), 10);
        self.db.create_task(&task)?;
        self.input_buffer.clear();
        self.refresh_state()?;
        Ok(())
    }

    pub fn next_task(&mut self) {
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

    pub fn previous_task(&mut self) {
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

    pub fn toggle_status(&mut self) -> Result<()> {
        if let Some(i) = self.list_state.selected() {
            if let Some(task) = self.tasks.get(i) {
                let new_status = match task.status {
                    TaskStatus::Todo => TaskStatus::Doing,
                    TaskStatus::Doing => TaskStatus::Done,
                    TaskStatus::Done => TaskStatus::Todo,
                };

                // Award XP if completing a task
                if new_status == TaskStatus::Done && task.status != TaskStatus::Done {
                    self.db.add_xp(task.xp_reward)?;
                }

                self.db.update_task_status(&task.id, new_status)?;
                self.refresh_state()?;
                // Restore selection after refresh
                if i < self.tasks.len() {
                    self.list_state.select(Some(i));
                }
            }
        }
        Ok(())
    }
}
