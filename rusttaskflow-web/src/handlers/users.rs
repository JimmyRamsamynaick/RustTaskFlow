use axum::{
    extract::{Extension, Path, State},
    response::Json,
};
use rusttaskflow_core::UserResponse;
use uuid::Uuid;

use crate::handlers::AppResult;
use crate::auth::AuthUser;
use crate::AppState;

pub async fn list_users(
    State(app_state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
) -> AppResult<Json<Vec<UserResponse>>> {
    let users = app_state.db.list_users().await?;
    let user_responses: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();
    Ok(Json(user_responses))
}

pub async fn get_user(
    State(app_state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<UserResponse>> {
    let user = app_state.db.get_user_by_id(id).await?;
    Ok(Json(user.into()))
}