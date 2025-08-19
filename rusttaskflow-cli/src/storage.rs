use crate::task::Task;
use anyhow::{anyhow, Result};
use rusqlite::{params, Connection, Row};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub trait Storage {
    fn save_tasks(&self, tasks: &HashMap<Uuid, Task>) -> Result<()>;
    fn load_tasks(&self) -> Result<HashMap<Uuid, Task>>;
    fn backup(&self) -> Result<()>;
}

/// Stockage JSON
pub struct JsonStorage {
    file_path: String,
}

impl JsonStorage {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    pub fn default() -> Result<Self> {
        let data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow!("Impossible de trouver le répertoire de données"))?;
        let app_dir = data_dir.join("rusttaskflow");
        
        if !app_dir.exists() {
            fs::create_dir_all(&app_dir)?;
        }
        
        let file_path = app_dir.join("tasks.json").to_string_lossy().to_string();
        Ok(Self::new(file_path))
    }
}

impl Storage for JsonStorage {
    fn save_tasks(&self, tasks: &HashMap<Uuid, Task>) -> Result<()> {
        let json = serde_json::to_string_pretty(tasks)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    fn load_tasks(&self) -> Result<HashMap<Uuid, Task>> {
        if !Path::new(&self.file_path).exists() {
            return Ok(HashMap::new());
        }
        
        let content = fs::read_to_string(&self.file_path)?;
        if content.trim().is_empty() {
            return Ok(HashMap::new());
        }
        
        let tasks: HashMap<Uuid, Task> = serde_json::from_str(&content)?;
        Ok(tasks)
    }

    fn backup(&self) -> Result<()> {
        let backup_path = format!("{}.backup", self.file_path);
        if Path::new(&self.file_path).exists() {
            fs::copy(&self.file_path, backup_path)?;
        }
        Ok(())
    }
}

/// Stockage SQLite
pub struct SqliteStorage {
    db_path: String,
}

impl SqliteStorage {
    pub fn new(db_path: String) -> Result<Self> {
        let storage = Self { db_path };
        storage.init_database()?;
        Ok(storage)
    }

    pub fn default() -> Result<Self> {
        let data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow!("Impossible de trouver le répertoire de données"))?;
        let app_dir = data_dir.join("rusttaskflow");
        
        if !app_dir.exists() {
            fs::create_dir_all(&app_dir)?;
        }
        
        let db_path = app_dir.join("tasks.db").to_string_lossy().to_string();
        Self::new(db_path)
    }

    fn init_database(&self) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL,
                priority TEXT NOT NULL,
                tags TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                due_date TEXT,
                completed_at TEXT
            )
            "#,
            [],
        )?;
        
        Ok(())
    }

    fn task_from_row(row: &Row) -> Result<(Uuid, Task), rusqlite::Error> {
        let id_str: String = row.get(0)?;
        let id = Uuid::parse_str(&id_str).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(
                0,
                rusqlite::types::Type::Text,
                Box::new(e),
            )
        })?;
        
        let title: String = row.get(1)?;
        let description: Option<String> = row.get(2)?;
        let status_str: String = row.get(3)?;
        let priority_str: String = row.get(4)?;
        let tags_str: String = row.get(5)?;
        let created_at_str: String = row.get(6)?;
        let updated_at_str: String = row.get(7)?;
        let due_date_str: Option<String> = row.get(8)?;
        let completed_at_str: Option<String> = row.get(9)?;
        
        // Parse status
        let status = match status_str.as_str() {
            "Todo" => crate::task::Status::Todo,
            "InProgress" => crate::task::Status::InProgress,
            "Completed" => crate::task::Status::Completed,
            "Cancelled" => crate::task::Status::Cancelled,
            _ => crate::task::Status::Todo,
        };
        
        // Parse priority
        let priority = match priority_str.as_str() {
            "Low" => crate::task::Priority::Low,
            "Medium" => crate::task::Priority::Medium,
            "High" => crate::task::Priority::High,
            "Critical" => crate::task::Priority::Critical,
            _ => crate::task::Priority::Medium,
        };
        
        // Parse tags
        let tags: Vec<String> = if tags_str.is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&tags_str).unwrap_or_default()
        };
        
        // Parse dates
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    6,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?
            .with_timezone(&chrono::Utc);
        
        let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at_str)
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    7,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?
            .with_timezone(&chrono::Utc);
        
        let due_date = if let Some(due_str) = due_date_str {
            Some(
                chrono::DateTime::parse_from_rfc3339(&due_str)
                    .map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            8,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?
                    .with_timezone(&chrono::Utc),
            )
        } else {
            None
        };
        
        let completed_at = if let Some(completed_str) = completed_at_str {
            Some(
                chrono::DateTime::parse_from_rfc3339(&completed_str)
                    .map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            9,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?
                    .with_timezone(&chrono::Utc),
            )
        } else {
            None
        };
        
        let task = Task {
            id,
            title,
            description,
            status,
            priority,
            tags,
            created_at,
            updated_at,
            due_date,
            completed_at,
        };
        
        Ok((id, task))
    }
}

impl Storage for SqliteStorage {
    fn save_tasks(&self, tasks: &HashMap<Uuid, Task>) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        
        // Vider la table
        conn.execute("DELETE FROM tasks", [])?;
        
        // Insérer toutes les tâches
        for task in tasks.values() {
            let tags_json = serde_json::to_string(&task.tags)?;
            
            conn.execute(
                r#"
                INSERT INTO tasks (
                    id, title, description, status, priority, tags,
                    created_at, updated_at, due_date, completed_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                "#,
                params![
                    task.id.to_string(),
                    task.title,
                    task.description,
                    task.status.to_string(),
                    task.priority.to_string(),
                    tags_json,
                    task.created_at.to_rfc3339(),
                    task.updated_at.to_rfc3339(),
                    task.due_date.map(|d| d.to_rfc3339()),
                    task.completed_at.map(|d| d.to_rfc3339()),
                ],
            )?;
        }
        
        Ok(())
    }

    fn load_tasks(&self) -> Result<HashMap<Uuid, Task>> {
        let conn = Connection::open(&self.db_path)?;
        
        let mut stmt = conn.prepare(
            "SELECT id, title, description, status, priority, tags, created_at, updated_at, due_date, completed_at FROM tasks"
        )?;
        
        let task_iter = stmt.query_map([], Self::task_from_row)?;
        
        let mut tasks = HashMap::new();
        for task_result in task_iter {
            let (id, task) = task_result?;
            tasks.insert(id, task);
        }
        
        Ok(tasks)
    }

    fn backup(&self) -> Result<()> {
        let backup_path = format!("{}.backup", self.db_path);
        if Path::new(&self.db_path).exists() {
            fs::copy(&self.db_path, backup_path)?;
        }
        Ok(())
    }
}

/// Factory pour créer le bon type de stockage
pub fn create_storage(storage_type: &str) -> Result<Box<dyn Storage>> {
    match storage_type.to_lowercase().as_str() {
        "json" => Ok(Box::new(JsonStorage::default()?)),
        "sqlite" => Ok(Box::new(SqliteStorage::default()?)),
        _ => Err(anyhow!("Type de stockage non supporté: {}", storage_type)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Task;
    use tempfile::tempdir;

    #[test]
    fn test_json_storage() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_tasks.json");
        let storage = JsonStorage::new(file_path.to_string_lossy().to_string());
        
        let mut tasks = HashMap::new();
        let task = Task::new("Test task".to_string());
        let task_id = task.id;
        tasks.insert(task_id, task);
        
        // Test save
        storage.save_tasks(&tasks).unwrap();
        
        // Test load
        let loaded_tasks = storage.load_tasks().unwrap();
        assert_eq!(loaded_tasks.len(), 1);
        assert!(loaded_tasks.contains_key(&task_id));
    }

    #[test]
    fn test_sqlite_storage() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test_tasks.db");
        let storage = SqliteStorage::new(db_path.to_string_lossy().to_string()).unwrap();
        
        let mut tasks = HashMap::new();
        let task = Task::new("Test task".to_string());
        let task_id = task.id;
        tasks.insert(task_id, task);
        
        // Test save
        storage.save_tasks(&tasks).unwrap();
        
        // Test load
        let loaded_tasks = storage.load_tasks().unwrap();
        assert_eq!(loaded_tasks.len(), 1);
        assert!(loaded_tasks.contains_key(&task_id));
    }
}