pub mod app;
pub mod engine;
pub mod event;
pub mod math;
pub mod memory;
pub mod time;
pub mod utils;
pub mod render_systems;
pub mod ecs_app;
// Visual scripting now lives in lumina-scripting crate

pub use app::*;
pub use engine::*;
pub use event::*;
pub use math::*;
pub use memory::*;
pub use time::*;
pub use utils::*;
pub use render_systems::*;
pub use ecs_app::*;

// Essential components for game development
pub use render_systems::Renderable;

// Re-export input types from lumina-input for convenience
pub use lumina_input as input;
// Re-export visual scripting from lumina-scripting
pub use lumina_scripting::*;

pub type Result<T> = anyhow::Result<T>;

#[derive(Debug, thiserror::Error)]
pub enum LuminaError {
    #[error("Initialization failed: {0}")]
    InitializationError(String),
    #[error("Runtime error: {0}")]
    RuntimeError(String),
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
}

pub fn init_logging() {
    #[cfg(feature = "logging")]
    {
        env_logger::init();
        log::info!("Lumina Engine logging initialized");
    }
}
