use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskFlowError {
    #[error("Task not found: {id}")]
    TaskNotFound { id: String },
    
    #[error("User not found: {id}")]
    UserNotFound { id: String },
    
    #[error("Invalid task status transition from {from} to {to}")]
    InvalidStatusTransition { from: String, to: String },
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Authentication error: {message}")]
    Authentication { message: String },
    
    #[error("Authorization error: {message}")]
    Authorization { message: String },
    
    #[error("Validation error: {message}")]
    Validation { message: String },
    
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, TaskFlowError>;