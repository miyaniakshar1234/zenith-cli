use crate::db::models::{Task, TaskStatus, UserProfile};
use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use directories::ProjectDirs;
use rusqlite::{params, Connection};
use std::fs;
use std::path::PathBuf;

pub mod models;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn init() -> Result<Self> {
        let db_path = Self::get_db_path()?;
        let conn = Connection::open(db_path)?;

        // Create Tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT DEFAULT 'TODO',
                xp_reward INTEGER DEFAULT 10,
                created_at TEXT NOT NULL,
                completed_at TEXT
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS user_profile (
                id INTEGER PRIMARY KEY DEFAULT 1,
                level INTEGER DEFAULT 1,
                current_xp INTEGER DEFAULT 0,
                next_level_xp INTEGER DEFAULT 100
            )",
            [],
        )?;

        // Initialize default user if not exists
        conn.execute(
            "INSERT OR IGNORE INTO user_profile (id, level, current_xp, next_level_xp) VALUES (1, 1, 0, 100)",
            [],
        )?;

        Ok(Self { conn })
    }

    fn get_db_path() -> Result<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "miyani", "zenith-cli") {
            let data_dir = proj_dirs.data_dir();
            if !data_dir.exists() {
                fs::create_dir_all(data_dir)?;
            }
            Ok(data_dir.join("zenith.db"))
        } else {
            // Fallback to local directory
            Ok(PathBuf::from("zenith.db"))
        }
    }

    pub fn create_task(&self, task: &Task) -> Result<()> {
        self.conn.execute(
            "INSERT INTO tasks (id, title, description, status, xp_reward, created_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                task.id,
                task.title,
                task.description,
                task.status,
                task.xp_reward,
                task.created_at.to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn get_all_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, status, xp_reward, created_at, completed_at FROM tasks ORDER BY created_at DESC"
        )?;

        let task_iter = stmt.query_map([], |row| {
            let created_at_str: String = row.get(5)?;
            let completed_at_str: Option<String> = row.get(6)?;

            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                status: row.get(3)?,
                xp_reward: row.get(4)?,
                created_at: DateTime::parse_from_rfc3339(&created_at_str)
                    .unwrap()
                    .with_timezone(&Utc),
                completed_at: completed_at_str.map(|s| {
                    DateTime::parse_from_rfc3339(&s)
                        .unwrap()
                        .with_timezone(&Utc)
                }),
            })
        })?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    pub fn update_task_status(&self, id: &str, status: TaskStatus) -> Result<()> {
        let completed_at = if status == TaskStatus::Done {
            Some(Utc::now().to_rfc3339())
        } else {
            None
        };

        self.conn.execute(
            "UPDATE tasks SET status = ?1, completed_at = ?2 WHERE id = ?3",
            params![status, completed_at, id],
        )?;
        Ok(())
    }

    pub fn delete_task(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_user_profile(&self) -> Result<UserProfile> {
        let mut stmt = self.conn.prepare(
            "SELECT id, level, current_xp, next_level_xp FROM user_profile WHERE id = 1",
        )?;

        let profile = stmt.query_row([], |row| {
            Ok(UserProfile {
                id: row.get(0)?,
                level: row.get(1)?,
                current_xp: row.get(2)?,
                next_level_xp: row.get(3)?,
            })
        })?;

        Ok(profile)
    }

    pub fn add_xp(&self, xp: i32) -> Result<()> {
        let mut profile = self.get_user_profile()?;
        profile.current_xp += xp;

        // Level Up Logic
        if profile.current_xp >= profile.next_level_xp {
            profile.current_xp -= profile.next_level_xp;
            profile.level += 1;
            profile.next_level_xp = (profile.next_level_xp as f64 * 1.5) as i32;
            // Increase difficulty
        }

        self.conn.execute(
            "UPDATE user_profile SET level = ?1, current_xp = ?2, next_level_xp = ?3 WHERE id = 1",
            params![profile.level, profile.current_xp, profile.next_level_xp],
        )?;
        Ok(())
    }
}
