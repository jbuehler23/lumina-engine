use std::sync::Arc;
use tokio::sync::RwLock;

pub mod project;
pub mod websocket;
pub mod api;

pub use project::*;
pub use websocket::*;

/// Main application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub project_manager: Arc<RwLock<project::ProjectManager>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            project_manager: Arc::new(RwLock::new(project::ProjectManager::new())),
        }
    }
}