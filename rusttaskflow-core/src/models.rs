use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Completed,
    Cancelled,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "À faire"),
            TaskStatus::InProgress => write!(f, "En cours"),
            TaskStatus::Completed => write!(f, "Terminé"),
            TaskStatus::Cancelled => write!(f, "Annulé"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "Faible"),
            Priority::Medium => write!(f, "Moyenne"),
            Priority::High => write!(f, "Élevée"),
            Priority::Critical => write!(f, "Critique"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
    pub assigned_to: Option<Uuid>, // User ID for collaboration
    pub created_by: Uuid, // User ID who created the task
}

impl Task {
    pub fn new(
        title: String,
        description: Option<String>,
        priority: Priority,
        tags: Vec<String>,
        created_by: Uuid,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            status: TaskStatus::Todo,
            priority,
            tags,
            created_at: now,
            updated_at: now,
            started_at: None,
            completed_at: None,
            due_date: None,
            assigned_to: None,
            created_by,
        }
    }

    pub fn start(&mut self) -> crate::Result<()> {
        match self.status {
            TaskStatus::Todo => {
                self.status = TaskStatus::InProgress;
                self.started_at = Some(Utc::now());
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(crate::TaskFlowError::InvalidStatusTransition {
                from: self.status.to_string(),
                to: TaskStatus::InProgress.to_string(),
            }),
        }
    }

    pub fn complete(&mut self) -> crate::Result<()> {
        match self.status {
            TaskStatus::InProgress | TaskStatus::Todo => {
                self.status = TaskStatus::Completed;
                self.completed_at = Some(Utc::now());
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(crate::TaskFlowError::InvalidStatusTransition {
                from: self.status.to_string(),
                to: TaskStatus::Completed.to_string(),
            }),
        }
    }

    pub fn cancel(&mut self) -> crate::Result<()> {
        match self.status {
            TaskStatus::Todo | TaskStatus::InProgress => {
                self.status = TaskStatus::Cancelled;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(crate::TaskFlowError::InvalidStatusTransition {
                from: self.status.to_string(),
                to: TaskStatus::Cancelled.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

impl User {
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            created_at: now,
            updated_at: now,
            is_active: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub assigned_to: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<Priority>,
    pub tags: Option<Vec<String>>,
    pub due_date: Option<DateTime<Utc>>,
    pub assigned_to: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskFilter {
    pub status: Option<TaskStatus>,
    pub priority: Option<Priority>,
    pub assigned_to: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub tags: Option<Vec<String>>,
    pub due_before: Option<DateTime<Utc>>,
    pub due_after: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            is_active: user.is_active,
        }
    }
}

// WebSocket message types for real-time collaboration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebSocketMessage {
    TaskCreated(Task),
    TaskUpdated(Task),
    TaskDeleted(Uuid),
    TaskStatusChanged { task_id: Uuid, status: TaskStatus },
    UserJoined(UserResponse),
    UserLeft(Uuid),
    Ping,
    Pong,
}