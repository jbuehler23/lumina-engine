//! Lumina Visual Editor
//! 
//! A comprehensive visual editor for game development built with the Lumina Engine's
//! own UI framework. Demonstrates the "dogfooding" approach where the engine's tools
//! are built using the engine itself.

use anyhow::Result;
use lumina_core::EcsAppRunner;

pub mod app;
pub mod assets;
pub mod panels;
pub mod project;
pub mod scene;
pub mod layout;
pub mod dockable_scene_panel;
pub mod toolbar;

pub use app::EditorApp;
pub use assets::{GameAsset, AssetType, AssetBrowser, AssetDatabase};
pub use project::EditorProject;
pub use panels::{SceneObject, ObjectType};
pub use scene::{Scene, SceneManager, ObjectProperty};
pub use layout::DockingManager;
pub use dockable_scene_panel::DockableScenePanel;
pub use toolbar::{EditorToolbar, ToolType, ToolbarAction};

/// Main editor runner that handles ECS architecture setup
pub struct EditorRunner;

impl EditorRunner {
    /// Create a new editor runner
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Run the editor using ECS architecture
    pub async fn run(self) -> Result<()> {
        log::info!("Starting Lumina Editor with ECS architecture");
        
        // Create the editor application
        let editor_app = EditorApp::new()?;
        
        // Create ECS app runner
        let runner = EcsAppRunner::new(editor_app);
        
        // Run the application
        runner.run().await
    }
}

impl Default for EditorRunner {
    fn default() -> Self {
        Self::new().expect("Failed to create editor runner")
    }
}
