use axum::{
    extract::{Extension, State},
    response::Json,
};
use rusttaskflow_core::{
    AuthResponse, LoginRequest, RegisterRequest, User, UserResponse, TaskFlowError,
};
use uuid::Uuid;
use chrono::Utc;

use crate::handlers::{AppError, AppResult};
use crate::AppState;
use crate::auth::AuthUser;

pub async fn register(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    // Validate input
    if payload.username.trim().is_empty() {
        return Err(AppError(rusttaskflow_core::TaskFlowError::Validation {
            message: "Username cannot be empty".to_string(),
        }));
    }

    if payload.email.trim().is_empty() {
        return Err(AppError(rusttaskflow_core::TaskFlowError::Validation {
            message: "Email cannot be empty".to_string(),
        }));
    }

    if payload.password.len() < 6 {
        return Err(AppError(rusttaskflow_core::TaskFlowError::Validation {
            message: "Password must be at least 6 characters long".to_string(),
        }));
    }

    // Check if user already exists
    if app_state.db.get_user_by_email(&payload.email).await.is_ok() {
        return Err(AppError(TaskFlowError::Validation {
            message: "User already exists".to_string(),
        }));
    }

    // Hash password
    let password_hash = crate::auth::AuthService::hash_password(&payload.password)?;

    // Create user
    let user = User {
        id: Uuid::new_v4(),
        username: payload.username.clone(),
        email: payload.email.clone(),
        password_hash,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        is_active: true,
    };
    
    app_state.db.create_user(&user).await?;

    // Generate token
    let token = app_state.auth_service.generate_token(user.id, &user.username, &user.email)?;

    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    // Get user by email
    let user = app_state.db.get_user_by_email(&payload.email).await
        .map_err(|_| AppError(TaskFlowError::Authentication { message: "Invalid email or password".to_string() }))?;

    // Verify password
    if !crate::auth::AuthService::verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError(TaskFlowError::Authentication { message: "Invalid email or password".to_string() }));
    }

    // Check if user is active
    if !user.is_active {
        return Err(AppError(TaskFlowError::Authentication { message: "Account is deactivated".to_string() }));
    }

    // Generate token
    let token = app_state.auth_service.generate_token(user.id, &user.username, &user.email)?;

    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

pub async fn me(
    State(app_state): State<AppState>,
    Extension(user): Extension<AuthUser>,
) -> AppResult<Json<UserResponse>> {
    let user = app_state.db.get_user_by_id(user.user_id).await?;
    Ok(Json(user.into()))
}