use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::auth::{AuthService, AuthUser};

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract Authorization header
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    let token = match auth_header {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Create auth service (in a real app, this would be injected)
    let auth_service = AuthService::new();
    
    // Verify token
    let claims = match auth_service.verify_token(token) {
        Ok(claims) => claims,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    // Create AuthUser from claims
    let auth_user = AuthUser {
        user_id: uuid::Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?,
        username: claims.username,
    };

    // Insert user into request extensions
    req.extensions_mut().insert(auth_user);

    Ok(next.run(req).await)
}