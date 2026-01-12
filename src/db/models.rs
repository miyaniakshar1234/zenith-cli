use chrono::{DateTime, Utc};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Todo,
    Doing,
    Done,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "TODO"),
            TaskStatus::Doing => write!(f, "DOING"),
            TaskStatus::Done => write!(f, "DONE"),
        }
    }
}

// Implement conversion for SQLite
impl ToSql for TaskStatus {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for TaskStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        String::column_result(value).and_then(|s| match s.as_str() {
            "TODO" => Ok(TaskStatus::Todo),
            "DOING" => Ok(TaskStatus::Doing),
            "DONE" => Ok(TaskStatus::Done),
            _ => Err(FromSqlError::InvalidType),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub xp_reward: i32,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(title: String, description: String, xp: i32) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            status: TaskStatus::Todo,
            xp_reward: xp,
            created_at: Utc::now(),
            completed_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: i32,
    pub level: i32,
    pub current_xp: i32,
    pub next_level_xp: i32,
}

impl UserProfile {
    pub fn default() -> Self {
        Self {
            id: 1,
            level: 1,
            current_xp: 0,
            next_level_xp: 100,
        }
    }
}
