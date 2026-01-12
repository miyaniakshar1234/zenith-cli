use crate::db::models::{Task, TaskPriority, TaskStatus, UserProfile};
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
                priority TEXT DEFAULT 'MEDIUM',
                xp_reward INTEGER DEFAULT 10,
                due_date TEXT,
                created_at TEXT NOT NULL,
                completed_at TEXT
            )",
            [],
        )?;

        // Migration: Add priority if missing
        let has_priority = {
            let mut stmt = conn.prepare("PRAGMA table_info(tasks)")?;
            let columns: Vec<String> = stmt
                .query_map([], |row| row.get(1))?
                .filter_map(|r| r.ok())
                .collect();
            columns.contains(&"priority".to_string())
        };

        if !has_priority {
            conn.execute(
                "ALTER TABLE tasks ADD COLUMN priority TEXT DEFAULT 'MEDIUM'",
                [],
            )?;
        }

        // Migration: Add due_date if missing
        let has_due_date = {
            let mut stmt = conn.prepare("PRAGMA table_info(tasks)")?;
            let columns: Vec<String> = stmt
                .query_map([], |row| row.get(1))?
                .filter_map(|r| r.ok())
                .collect();
            columns.contains(&"due_date".to_string())
        };

        if !has_due_date {
            conn.execute("ALTER TABLE tasks ADD COLUMN due_date TEXT", [])?;
        }

        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT
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
            Ok(PathBuf::from("zenith.db"))
        }
    }

    pub fn create_task(&self, task: &Task) -> Result<()> {
        self.conn.execute(
            "INSERT INTO tasks (id, title, description, status, priority, xp_reward, due_date, created_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                task.id,
                task.title,
                task.description,
                task.status,
                task.priority,
                task.xp_reward,
                task.due_date.map(|d| d.to_rfc3339()),
                task.created_at.to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn get_all_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, status, priority, xp_reward, due_date, created_at, completed_at FROM tasks ORDER BY created_at DESC"
        )?;

        let task_iter = stmt.query_map([], |row| {
            let due_date_str: Option<String> = row.get(6)?;
            let created_at_str: String = row.get(7)?;
            let completed_at_str: Option<String> = row.get(8)?;

            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                status: row.get(3)?,
                priority: row.get(4)?,
                xp_reward: row.get(5)?,
                due_date: due_date_str.map(|s| {
                    DateTime::parse_from_rfc3339(&s)
                        .unwrap()
                        .with_timezone(&Utc)
                }),
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

    pub fn update_task_content(
        &self,
        id: &str,
        title: &str,
        description: &str,
        priority: TaskPriority,
        due_date: Option<DateTime<Utc>>,
    ) -> Result<()> {
        self.conn.execute(
            "UPDATE tasks SET title = ?1, description = ?2, priority = ?3, due_date = ?4 WHERE id = ?5",
            params![title, description, priority, due_date.map(|d| d.to_rfc3339()), id],
        )?;
        Ok(())
    }

    pub fn get_weekly_stats(&self) -> Result<Vec<(String, u64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT substr(completed_at, 1, 10) as day, COUNT(*) 
             FROM tasks 
             WHERE status = 'Done' AND completed_at IS NOT NULL
             GROUP BY day 
             ORDER BY day DESC 
             LIMIT 7",
        )?;

        let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;

        let mut stats = Vec::new();
        for r in rows {
            stats.push(r?);
        }
        Ok(stats)
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

        if profile.current_xp >= profile.next_level_xp {
            profile.current_xp -= profile.next_level_xp;
            profile.level += 1;
            profile.next_level_xp = (profile.next_level_xp as f64 * 1.5) as i32;
        }

        self.conn.execute(
            "UPDATE user_profile SET level = ?1, current_xp = ?2, next_level_xp = ?3 WHERE id = 1",
            params![profile.level, profile.current_xp, profile.next_level_xp],
        )?;
        Ok(())
    }

    pub fn get_streak(&self) -> Result<u32> {
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT substr(completed_at, 1, 10) as day 
             FROM tasks 
             WHERE status = 'Done' AND completed_at IS NOT NULL
             ORDER BY day DESC",
        )?;

        let days: Vec<String> = stmt
            .query_map([], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        if days.is_empty() {
            return Ok(0);
        }

        let today = Utc::now().format("%Y-%m-%d").to_string();
        let yesterday = (Utc::now() - chrono::Duration::days(1))
            .format("%Y-%m-%d")
            .to_string();

        // If latest completed task was not today or yesterday, streak is 0
        if days[0] != today && days[0] != yesterday {
            return Ok(0);
        }

        let mut streak = 0;
        // Start checking from the day of the latest task
        let mut current_check = if days[0] == today {
            Utc::now()
        } else {
            Utc::now() - chrono::Duration::days(1)
        };

        for day_str in days {
            let expected = current_check.format("%Y-%m-%d").to_string();
            if day_str == expected {
                streak += 1;
                current_check = current_check - chrono::Duration::days(1);
            } else {
                break;
            }
        }
        Ok(streak)
    }

    pub fn get_tasks_today(&self) -> Result<u32> {
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let mut stmt = self.conn.prepare(
            "SELECT COUNT(*) FROM tasks WHERE status = 'Done' AND substr(completed_at, 1, 10) = ?1",
        )?;
        let count: u32 = stmt.query_row(params![today], |row| row.get(0))?;
        Ok(count)
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT value FROM settings WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }
}
