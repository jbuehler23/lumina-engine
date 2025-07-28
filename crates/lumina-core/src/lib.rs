pub mod app;
pub mod engine;
pub mod event;
pub mod input;
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
pub use input::*;
pub use math::*;
pub use memory::*;
pub use time::*;
pub use utils::*;
pub use render_systems::*;
pub use ecs_app::*;
// Re-export visual scripting from lumina-scripting
pub use lumina_scripting::*;

// Re-export MouseButton from event module for convenience
pub use event::MouseButton;

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
