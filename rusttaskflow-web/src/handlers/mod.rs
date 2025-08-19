pub mod auth;
pub mod tasks;
pub mod users;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use rusttaskflow_core::TaskFlowError;
use serde_json::json;

// Error response helper
pub struct AppError(pub TaskFlowError);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self.0 {
            TaskFlowError::TaskNotFound { .. } => (StatusCode::NOT_FOUND, self.0.to_string()),
            TaskFlowError::UserNotFound { .. } => (StatusCode::NOT_FOUND, self.0.to_string()),
            TaskFlowError::Authentication { .. } => (StatusCode::UNAUTHORIZED, self.0.to_string()),
            TaskFlowError::Authorization { .. } => (StatusCode::FORBIDDEN, self.0.to_string()),
            TaskFlowError::Validation { .. } => (StatusCode::BAD_REQUEST, self.0.to_string()),
            TaskFlowError::InvalidStatusTransition { .. } => (StatusCode::BAD_REQUEST, self.0.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<TaskFlowError>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub type AppResult<T> = Result<T, AppError>;