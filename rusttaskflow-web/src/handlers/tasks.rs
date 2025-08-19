use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::Json,
};
use rusttaskflow_core::{
    CreateTaskRequest, Task, TaskFilter, UpdateTaskRequest,
};
use serde::Deserialize;

use uuid::Uuid;
use chrono::Utc;

use crate::handlers::{AppError, AppResult};
use crate::auth::AuthUser;
use crate::AppState;

#[derive(Deserialize)]
pub struct TaskQuery {
    status: Option<String>,
    priority: Option<String>,
    assigned_to: Option<Uuid>,
    tags: Option<String>,
}

pub async fn create_task(
    State(app_state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Json(payload): Json<CreateTaskRequest>,
) -> AppResult<Json<Task>> {
    // Validate input
    if payload.title.trim().is_empty() {
        return Err(AppError(rusttaskflow_core::TaskFlowError::Validation {
            message: "Title cannot be empty".to_string(),
        }));
    }

    // Create task
    let mut task = Task::new(
        payload.title,
        payload.description,
        payload.priority,
        payload.tags,
        user.user_id,
    );

    // Set optional fields
    task.due_date = payload.due_date;
    task.assigned_to = payload.assigned_to;

    // Save to database
    app_state.db.create_task(&task).await?;

    // Send WebSocket notification
    let ws_message = crate::websocket::WebSocketMessage::TaskCreated {
        task: task.clone(),
        user_id: user.user_id,
    };
    let _ = app_state.websocket_tx.send(ws_message);

    Ok(Json(task))
}

pub async fn list_tasks(
    State(app_state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Query(query): Query<TaskQuery>,
) -> AppResult<Json<Vec<Task>>> {
    // Build filter
    let filter = TaskFilter {
        status: query.status.and_then(|s| serde_json::from_str(&format!("\"{}\"", s)).ok()),
        priority: query.priority.and_then(|p| serde_json::from_str(&format!("\"{}\"", p)).ok()),
        assigned_to: query.assigned_to,
        created_by: Some(user.user_id), // Only show user's tasks for now
        tags: query.tags.map(|t| t.split(',').map(|s| s.trim().to_string()).collect()),
        due_before: None,
        due_after: None,
    };

    let tasks = app_state.db.list_tasks(Some(filter)).await?;
    Ok(Json(tasks))
}

pub async fn get_task(
    State(app_state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Task>> {
    let task = app_state.db.get_task_by_id(id).await?;
    
    // Check if user has access to this task
    if task.created_by != user.user_id && task.assigned_to != Some(user.user_id) {
        return Err(AppError(rusttaskflow_core::TaskFlowError::Authorization {
            message: "You don't have access to this task".to_string(),
        }));
    }

    Ok(Json(task))
}

pub async fn update_task(
    State(app_state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTaskRequest>,
) -> AppResult<Json<Task>> {
    let mut task = app_state.db.get_task_by_id(id).await?;
    
    // Check if user has access to this task
    if task.created_by != user.user_id && task.assigned_to != Some(user.user_id) {
        return Err(AppError(rusttaskflow_core::TaskFlowError::Authorization {
            message: "You don't have access to this task".to_string(),
        }));
    }

    // Update fields
    if let Some(title) = payload.title {
        if title.trim().is_empty() {
            return Err(AppError(rusttaskflow_core::TaskFlowError::Validation {
                message: "Title cannot be empty".to_string(),
            }));
        }
        task.title = title;
    }
    
    if let Some(description) = payload.description {
        task.description = Some(description);
    }
    
    if let Some(priority) = payload.priority {
        task.priority = priority;
    }
    
    if let Some(tags) = payload.tags {
        task.tags = tags;
    }
    
    if let Some(due_date) = payload.due_date {
        task.due_date = Some(due_date);
    }
    
    if let Some(assigned_to) = payload.assigned_to {
        task.assigned_to = Some(assigned_to);
    }

    task.updated_at = Utc::now();
    
    // Save to database
    app_state.db.update_task(&task).await?;

    // Send WebSocket notification
    let ws_message = crate::websocket::WebSocketMessage::TaskUpdated {
        task: task.clone(),
        user_id: user.user_id,
    };
    let _ = app_state.websocket_tx.send(ws_message);

    Ok(Json(task))
}

pub async fn delete_task(
    State(app_state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let task = app_state.db.get_task_by_id(id).await?;
    
    // Check if user has access to this task
    if task.created_by != user.user_id {
        return Err(AppError(rusttaskflow_core::TaskFlowError::Authorization {
            message: "You can only delete tasks you created".to_string(),
        }));
    }

    app_state.db.delete_task(id).await?;
    
    // Send WebSocket notification
    let ws_message = crate::websocket::WebSocketMessage::TaskDeleted {
        task_id: id,
        user_id: user.user_id,
    };
    let _ = app_state.websocket_tx.send(ws_message);
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn start_task(
    State(app_state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Task>> {
    let mut task = app_state.db.get_task_by_id(id).await?;
    
    // Check if user has access to this task
    if task.created_by != user.user_id && task.assigned_to != Some(user.user_id) {
        return Err(AppError(rusttaskflow_core::TaskFlowError::Authorization {
            message: "You don't have access to this task".to_string(),
        }));
    }

    task.start()?;
    app_state.db.update_task(&task).await?;

    // Send WebSocket notification
    let ws_message = crate::websocket::WebSocketMessage::TaskUpdated {
        task: task.clone(),
        user_id: user.user_id,
    };
    let _ = app_state.websocket_tx.send(ws_message);

    Ok(Json(task))
}

pub async fn complete_task(
    State(app_state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Task>> {
    let mut task = app_state.db.get_task_by_id(id).await?;
    
    // Check if user has access to this task
    if task.created_by != user.user_id && task.assigned_to != Some(user.user_id) {
        return Err(AppError(rusttaskflow_core::TaskFlowError::Authorization {
            message: "You don't have access to this task".to_string(),
        }));
    }

    task.complete()?;
    app_state.db.update_task(&task).await?;

    // Send WebSocket notification
    let ws_message = crate::websocket::WebSocketMessage::TaskUpdated {
        task: task.clone(),
        user_id: user.user_id,
    };
    let _ = app_state.websocket_tx.send(ws_message);

    Ok(Json(task))
}

pub async fn cancel_task(
    State(app_state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Task>> {
    let mut task = app_state.db.get_task_by_id(id).await?;
    
    // Check if user has access to this task
    if task.created_by != user.user_id && task.assigned_to != Some(user.user_id) {
        return Err(AppError(rusttaskflow_core::TaskFlowError::Authorization {
            message: "You don't have access to this task".to_string(),
        }));
    }

    task.cancel()?;
    app_state.db.update_task(&task).await?;

    // Send WebSocket notification
    let ws_message = crate::websocket::WebSocketMessage::TaskUpdated {
        task: task.clone(),
        user_id: user.user_id,
    };
    let _ = app_state.websocket_tx.send(ws_message);

    Ok(Json(task))
}