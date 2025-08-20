use rusttaskflow_core::{Task, User, TaskFilter, Result, TaskFlowError};
use sqlx::{PgPool, Row};
use std::env;
use uuid::Uuid;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> anyhow::Result<Self> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|e| anyhow::anyhow!("DATABASE_URL must be set for PostgreSQL connection: {:?}", e))?;
        
        tracing::info!("Connecting to PostgreSQL with URL: {}", 
                      database_url.chars().take(20).collect::<String>() + "...");
        
        let pool = PgPool::connect(&database_url).await
            .map_err(|e| anyhow::anyhow!("Failed to connect to PostgreSQL: {}", e))?;
        
        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> anyhow::Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    // User operations
    pub async fn create_user(&self, user: &User) -> Result<()> {
        sqlx::query("INSERT INTO users (id, username, email, password_hash, created_at, updated_at, is_active) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(user.id.to_string())
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(user.created_at)
            .bind(user.updated_at)
            .bind(user.is_active)
            .execute(&self.pool)
            .await
            .map_err(TaskFlowError::Database)?;
        
        Ok(())
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<User> {
        let row = sqlx::query("SELECT * FROM users WHERE id = $1")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(TaskFlowError::Database)?;

        match row {
            Some(row) => Ok(User {
                id: Uuid::parse_str(row.get("id")).map_err(|e| TaskFlowError::Internal(anyhow::anyhow!("Invalid UUID: {}", e)))?,
                username: row.get("username"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                is_active: row.get("is_active"),
            }),
            None => Err(TaskFlowError::UserNotFound { id: id.to_string() }),
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let row = sqlx::query("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(TaskFlowError::Database)?;

        match row {
            Some(row) => Ok(User {
                id: Uuid::parse_str(row.get("id")).map_err(|e| TaskFlowError::Internal(anyhow::anyhow!("Invalid UUID: {}", e)))?,
                username: row.get("username"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                is_active: row.get("is_active"),
            }),
            None => Err(TaskFlowError::UserNotFound { id: email.to_string() }),
        }
    }

    pub async fn list_users(&self) -> Result<Vec<User>> {
        let rows = sqlx::query("SELECT * FROM users WHERE is_active = true ORDER BY username")
            .fetch_all(&self.pool)
            .await
            .map_err(TaskFlowError::Database)?;

        let users = rows
            .into_iter()
            .map(|row| -> Result<User> {
                Ok(User {
                    id: Uuid::parse_str(row.get("id")).map_err(|e| TaskFlowError::Internal(anyhow::anyhow!("Invalid UUID: {}", e)))?,
                    username: row.get("username"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    is_active: row.get("is_active"),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(users)
    }

    // Task operations
    pub async fn create_task(&self, task: &Task) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO tasks (
                id, title, description, status, priority, tags, 
                created_at, updated_at, started_at, completed_at, 
                due_date, assigned_to, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#
        )
        .bind(task.id.to_string())
        .bind(&task.title)
        .bind(&task.description)
        .bind(serde_json::to_string(&task.status).map_err(TaskFlowError::Serialization)?)
        .bind(serde_json::to_string(&task.priority).map_err(TaskFlowError::Serialization)?)
        .bind(serde_json::to_string(&task.tags).map_err(TaskFlowError::Serialization)?)
        .bind(task.created_at)
        .bind(task.updated_at)
        .bind(task.started_at)
        .bind(task.completed_at)
        .bind(task.due_date)
        .bind(task.assigned_to.map(|id| id.to_string()))
        .bind(task.created_by.to_string())
        .execute(&self.pool)
        .await
        .map_err(TaskFlowError::Database)?;
        
        Ok(())
    }

    pub async fn get_task_by_id(&self, id: Uuid) -> Result<Task> {
        let row = sqlx::query("SELECT * FROM tasks WHERE id = $1")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(TaskFlowError::Database)?;

        match row {
            Some(row) => Ok(Task {
                id: Uuid::parse_str(row.get("id")).map_err(|e| TaskFlowError::Internal(anyhow::anyhow!("Invalid UUID: {}", e)))?,
                title: row.get("title"),
                description: row.get("description"),
                status: serde_json::from_str(&row.get::<String, _>("status")).map_err(TaskFlowError::Serialization)?,
                priority: serde_json::from_str(&row.get::<String, _>("priority")).map_err(TaskFlowError::Serialization)?,
                tags: serde_json::from_str(&row.get::<String, _>("tags")).map_err(TaskFlowError::Serialization)?,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                started_at: row.get("started_at"),
                completed_at: row.get("completed_at"),
                due_date: row.get("due_date"),
                assigned_to: row.get::<Option<String>, _>("assigned_to").map(|s| Uuid::parse_str(&s).ok()).flatten(),
                created_by: Uuid::parse_str(row.get("created_by")).map_err(|e| TaskFlowError::Internal(anyhow::anyhow!("Invalid UUID: {}", e)))?,
            }),
            None => Err(TaskFlowError::TaskNotFound { id: id.to_string() }),
        }
    }

    pub async fn update_task(&self, task: &Task) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE tasks SET 
                title = $2, description = $3, status = $4, priority = $5, 
                tags = $6, updated_at = $7, started_at = $8, completed_at = $9,
                due_date = $10, assigned_to = $11
            WHERE id = $1
            "#
        )
        .bind(task.id.to_string())
        .bind(&task.title)
        .bind(&task.description)
        .bind(serde_json::to_string(&task.status).map_err(TaskFlowError::Serialization)?)
        .bind(serde_json::to_string(&task.priority).map_err(TaskFlowError::Serialization)?)
        .bind(serde_json::to_string(&task.tags).map_err(TaskFlowError::Serialization)?)
        .bind(task.updated_at)
        .bind(task.started_at)
        .bind(task.completed_at)
        .bind(task.due_date)
        .bind(task.assigned_to.map(|id| id.to_string()))
        .execute(&self.pool)
        .await
        .map_err(TaskFlowError::Database)?;
        
        Ok(())
    }

    pub async fn delete_task(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("DELETE FROM tasks WHERE id = $1")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(TaskFlowError::Database)?;

        if result.rows_affected() == 0 {
            return Err(TaskFlowError::TaskNotFound { id: id.to_string() });
        }

        Ok(())
    }

    pub async fn list_tasks(&self, filter: Option<TaskFilter>) -> Result<Vec<Task>> {
        let mut query = "SELECT * FROM tasks WHERE 1=1".to_string();
        let mut bind_count = 0;

        if let Some(filter) = filter {
            if filter.status.is_some() {
                bind_count += 1;
                query.push_str(&format!(" AND status = ${}", bind_count));
            }
            
            if filter.priority.is_some() {
                bind_count += 1;
                query.push_str(&format!(" AND priority = ${}", bind_count));
            }
            
            if filter.assigned_to.is_some() {
                bind_count += 1;
                query.push_str(&format!(" AND assigned_to = ${}", bind_count));
            }
            
            if filter.created_by.is_some() {
                bind_count += 1;
                query.push_str(&format!(" AND created_by = ${}", bind_count));
            }
        }

        query.push_str(" ORDER BY created_at DESC");

        // Pour simplifier, on utilise une requête basique pour l'instant
        let rows = sqlx::query("SELECT * FROM tasks ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(TaskFlowError::Database)?;

        let tasks = rows
            .into_iter()
            .map(|row| -> Result<Task> {
                Ok(Task {
                    id: Uuid::parse_str(row.get("id")).map_err(|e| TaskFlowError::Internal(anyhow::anyhow!("Invalid UUID: {}", e)))?,
                    title: row.get("title"),
                    description: row.get("description"),
                    status: serde_json::from_str(&row.get::<String, _>("status")).map_err(TaskFlowError::Serialization)?,
                    priority: serde_json::from_str(&row.get::<String, _>("priority")).map_err(TaskFlowError::Serialization)?,
                    tags: serde_json::from_str(&row.get::<String, _>("tags")).map_err(TaskFlowError::Serialization)?,
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    started_at: row.get("started_at"),
                    completed_at: row.get("completed_at"),
                    due_date: row.get("due_date"),
                    assigned_to: row.get::<Option<String>, _>("assigned_to").map(|s| Uuid::parse_str(&s).ok()).flatten(),
                    created_by: Uuid::parse_str(row.get("created_by")).map_err(|e| TaskFlowError::Internal(anyhow::anyhow!("Invalid UUID: {}", e)))?,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(tasks)
    }
}