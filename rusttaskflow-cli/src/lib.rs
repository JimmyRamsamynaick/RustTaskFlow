//! RustTaskFlow - A modern and high-performance command-line task manager
//!
//! This library provides a comprehensive task management system with features like:
//! - Task creation, modification, and deletion
//! - Priority levels and status tracking
//! - Tag-based organization
//! - Search and filtering capabilities
//! - Statistics and productivity tracking
//! - Multiple storage backends (JSON, SQLite)
//! - Modern CLI interface with colors and formatting
//! - Due date management

pub mod task;
pub mod task_manager;
pub mod storage;
pub mod cli;
pub mod ui;

pub use task::{Task, Priority, Status};
pub use task_manager::{TaskManager, TaskFilter, TaskStats};
pub use storage::{Storage, JsonStorage, SqliteStorage, create_storage};
pub use cli::{Cli, Commands};
pub use ui::*;

/// Result type used throughout the application
pub type Result<T> = anyhow::Result<T>;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");