use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::sync::broadcast;

mod auth;
mod database;
mod handlers;
mod middleware;
mod websocket;

use auth::AuthService;
use database::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub auth_service: Arc<AuthService>,
    pub websocket_tx: broadcast::Sender<websocket::WebSocketMessage>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rusttaskflow_web=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize database
    tracing::info!("Connecting to database...");
    let database = Database::new().await?;
    tracing::info!("Running migrations...");
    database.migrate().await?;
    tracing::info!("Database initialized successfully");

    // Build our application with routes
    let app = create_app(database).await;

    // Run it with hyper on localhost:3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server starting on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn create_app(database: Database) -> Router {
    // Initialize auth service
    let auth_service = Arc::new(AuthService::new());
    
    // Initialize WebSocket broadcast channel
    let (websocket_tx, _) = broadcast::channel(1000);
    
    let app_state = AppState {
        db: database,
        auth_service,
        websocket_tx,
    };

    // CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with routes
    Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .nest("/api/v1", api_routes())
        .route("/ws", get(websocket::websocket_handler))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors)
                .layer(DefaultBodyLimit::max(1024 * 1024)) // 1MB
        )
        .with_state(app_state)
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth_routes())
        .nest("/tasks", task_routes())
        .nest("/users", user_routes())
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::auth::register))
        .route("/login", post(handlers::auth::login))
        .route("/me", get(handlers::auth::me).layer(axum::middleware::from_fn(middleware::auth::auth_middleware)))
}

fn task_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::tasks::create_task))
        .route("/", get(handlers::tasks::list_tasks))
        .route("/:id", get(handlers::tasks::get_task))
        .route("/:id", put(handlers::tasks::update_task))
        .route("/:id", delete(handlers::tasks::delete_task))
        .route("/:id/start", post(handlers::tasks::start_task))
        .route("/:id/complete", post(handlers::tasks::complete_task))
        .route("/:id/cancel", post(handlers::tasks::cancel_task))
        .layer(axum::middleware::from_fn(middleware::auth::auth_middleware))
}

fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::users::list_users))
        .route("/:id", get(handlers::users::get_user))
        .layer(axum::middleware::from_fn(middleware::auth::auth_middleware))
}

async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "ok",
        "service": "rusttaskflow-web",
        "version": env!("CARGO_PKG_VERSION")
    })))
}