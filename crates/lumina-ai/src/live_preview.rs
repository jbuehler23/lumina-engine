//! Live game preview system for AI-generated scenes
//! 
//! This module provides real-time preview capabilities that instantly convert
//! AI-generated SDL into playable games within the Lumina Engine.

use std::sync::Arc;
use std::time::{Duration, Instant};
use anyhow::{Result, Context};
use log::{info, warn, error};
use tokio::sync::{mpsc, RwLock};

use lumina_ecs::World;
use lumina_core::{EcsApp, EcsAppRunner, WindowConfig, run_ecs_app};
use lumina_render::RenderConfig;
use lumina_ui::Theme;
use winit::{event::WindowEvent, dpi::LogicalSize};

use crate::{
    scene_description::SceneDescription,
    scene_loader::{SceneLoader, SceneLoadResult},
    game_generator::GameGenerationResult,
};

/// Live preview system for AI-generated games
pub struct LivePreviewSystem {
    /// Scene loader for converting SDL to ECS
    scene_loader: SceneLoader,
    /// Current preview state
    preview_state: Arc<RwLock<PreviewState>>,
    /// Communication channel for preview updates
    update_sender: mpsc::UnboundedSender<PreviewUpdate>,
    update_receiver: mpsc::UnboundedReceiver<PreviewUpdate>,
    /// Preview configuration
    config: PreviewConfig,
}

/// Current state of the preview system
#[derive(Debug, Clone)]
pub struct PreviewState {
    /// Currently loaded scene
    pub current_scene: Option<SceneDescription>,
    /// Preview status
    pub status: PreviewStatus,
    /// Last generation time
    pub last_update: Option<Instant>,
    /// Performance metrics
    pub metrics: PreviewMetrics,
}

/// Preview system status
#[derive(Debug, Clone)]
pub enum PreviewStatus {
    /// No scene loaded
    Idle,
    /// Loading a scene
    Loading,
    /// Scene is active and playable
    Active,
    /// Error occurred during loading
    Error(String),
}

/// Performance metrics for the preview system
#[derive(Debug, Clone, Default)]
pub struct PreviewMetrics {
    /// Time to load scene (SDL parsing + ECS creation)
    pub load_time: Duration,
    /// Number of entities created
    pub entities_loaded: usize,
    /// Number of systems active
    pub systems_active: usize,
    /// Current FPS
    pub fps: f32,
    /// Memory usage in MB
    pub memory_usage: f32,
}

/// Configuration for the preview system
#[derive(Debug, Clone)]
pub struct PreviewConfig {
    /// Automatically start preview after generation
    pub auto_preview: bool,
    /// Show debug overlay with metrics
    pub debug_overlay: bool,
    /// Target FPS for preview
    pub target_fps: u32,
    /// Maximum entities allowed in preview
    pub max_entities: usize,
    /// Preview window size
    pub window_size: (u32, u32),
}

/// Updates sent to the preview system
#[derive(Debug)]
pub enum PreviewUpdate {
    /// Load a new scene
    LoadScene(SceneDescription),
    /// Update scene incrementally
    UpdateScene(SceneDescription),
    /// Stop current preview
    Stop,
    /// Request current metrics
    GetMetrics,
}

/// Results from preview operations
#[derive(Debug)]
pub struct PreviewResult {
    /// Success status
    pub success: bool,
    /// Load result details
    pub load_result: Option<SceneLoadResult>,
    /// Performance metrics
    pub metrics: PreviewMetrics,
    /// Any warnings or errors
    pub messages: Vec<String>,
}

/// Live preview application that runs the generated game
pub struct PreviewApp {
    /// Scene description being previewed
    scene: SceneDescription,
    /// Scene metadata
    scene_metadata: Option<crate::scene_loader::SceneMetadata>,
    /// Preview configuration
    config: PreviewConfig,
    /// Performance tracking
    frame_timer: Instant,
    frame_count: u32,
    last_fps_update: Instant,
    current_fps: f32,
}

impl LivePreviewSystem {
    /// Create a new live preview system
    pub fn new(config: PreviewConfig) -> Self {
        let (update_sender, update_receiver) = mpsc::unbounded_channel();
        
        Self {
            scene_loader: SceneLoader::new(),
            preview_state: Arc::new(RwLock::new(PreviewState {
                current_scene: None,
                status: PreviewStatus::Idle,
                last_update: None,
                metrics: PreviewMetrics::default(),
            })),
            update_sender,
            update_receiver,
            config,
        }
    }

    /// Preview an AI-generated game scene immediately
    pub async fn preview_scene(&mut self, scene: SceneDescription) -> Result<PreviewResult> {
        let start_time = Instant::now();
        
        info!("Starting live preview for scene: {}", scene.metadata.name);
        
        // Update status to loading
        {
            let mut state = self.preview_state.write().await;
            state.status = PreviewStatus::Loading;
            state.last_update = Some(start_time);
        }

        // Validate scene before loading
        let validation_result = self.validate_scene(&scene).await?;
        if !validation_result.is_valid {
            let error_msg = format!("Scene validation failed: {:?}", validation_result.errors);
            error!("{}", error_msg);
            
            let mut state = self.preview_state.write().await;
            state.status = PreviewStatus::Error(error_msg.clone());
            
            return Ok(PreviewResult {
                success: false,
                load_result: None,
                metrics: PreviewMetrics::default(),
                messages: vec![error_msg],
            });
        }

        // Load scene into ECS world
        let mut world = World::new();
        let load_result = match self.scene_loader.load_scene(&mut world, scene.clone()) {
            Ok(result) => result,
            Err(e) => {
                let error_msg = format!("Failed to load scene: {}", e);
                error!("{}", error_msg);
                
                let mut state = self.preview_state.write().await;
                state.status = PreviewStatus::Error(error_msg.clone());
                
                return Ok(PreviewResult {
                    success: false,
                    load_result: None,
                    metrics: PreviewMetrics::default(),
                    messages: vec![error_msg],
                });
            }
        };

        let load_time = start_time.elapsed();

        // Create preview app and start game in separate task
        let preview_app = PreviewApp::new(scene.clone(), self.config.clone());
        
        // Start the game in a separate task using the full ECS app framework
        let preview_handle = tokio::spawn(async move {
            run_ecs_app(preview_app).await
        });

        // Update state to active
        {
            let mut state = self.preview_state.write().await;
            state.current_scene = Some(scene);
            state.status = PreviewStatus::Active;
            state.metrics = PreviewMetrics {
                load_time,
                entities_loaded: load_result.entities_created,
                systems_active: 0, // Will be updated by the running app
                fps: 0.0,
                memory_usage: 0.0,
            };
        }

        info!("Live preview started successfully in {:.2}ms", load_time.as_millis());

        Ok(PreviewResult {
            success: true,
            load_result: Some(load_result),
            metrics: PreviewMetrics {
                load_time,
                entities_loaded: load_result.entities_created,
                systems_active: 1,
                fps: self.config.target_fps as f32,
                memory_usage: 0.0,
            },
            messages: validation_result.warnings,
        })
    }

    /// Preview a complete AI generation result
    pub async fn preview_generation(&mut self, result: GameGenerationResult) -> Result<PreviewResult> {
        info!("Previewing AI generation result: {}", result.scene.metadata.name);
        
        // Log generation statistics
        info!("Generation stats: {} entities, {} assets, {:.2}s generation time",
              result.stats.entities_created,
              result.stats.assets_generated,
              result.stats.total_time);

        // Preview the scene
        self.preview_scene(result.scene).await
    }

    /// Update the current preview with modifications
    pub async fn update_preview(&mut self, updated_scene: SceneDescription) -> Result<PreviewResult> {
        info!("Updating preview with modifications");
        
        // For now, we'll do a full reload
        // In the future, this could be optimized for incremental updates
        self.preview_scene(updated_scene).await
    }

    /// Stop the current preview
    pub async fn stop_preview(&mut self) -> Result<()> {
        info!("Stopping live preview");
        
        let mut state = self.preview_state.write().await;
        state.status = PreviewStatus::Idle;
        state.current_scene = None;
        
        Ok(())
    }

    /// Get current preview state
    pub async fn get_state(&self) -> PreviewState {
        self.preview_state.read().await.clone()
    }

    /// Get current performance metrics
    pub async fn get_metrics(&self) -> PreviewMetrics {
        self.preview_state.read().await.metrics.clone()
    }

    /// Validate a scene before loading
    async fn validate_scene(&self, scene: &SceneDescription) -> Result<ValidationResult> {
        let mut result = ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        // Check entity count
        if scene.entities.len() > self.config.max_entities {
            result.errors.push(format!(
                "Too many entities: {} (max: {})",
                scene.entities.len(),
                self.config.max_entities
            ));
            result.is_valid = false;
        }

        // Check for required components
        for entity in &scene.entities {
            if !entity.components.contains_key("Transform") {
                result.warnings.push(format!(
                    "Entity '{}' missing Transform component",
                    entity.name
                ));
            }
        }

        // Check for player entity
        let has_player = scene.entities.iter().any(|e| 
            e.components.contains_key("Player") || 
            e.tags.contains(&"player".to_string())
        );

        if !has_player {
            result.warnings.push("No player entity found in scene".to_string());
        }

        Ok(result)
    }
}

/// Scene validation result
#[derive(Debug)]
struct ValidationResult {
    is_valid: bool,
    errors: Vec<String>,
    warnings: Vec<String>,
}

impl PreviewApp {
    /// Create a new preview application
    pub fn new(scene: SceneDescription, config: PreviewConfig) -> Self {
        Self {
            scene,
            scene_metadata: None,
            config,
            frame_timer: Instant::now(),
            frame_count: 0,
            last_fps_update: Instant::now(),
            current_fps: 0.0,
        }
    }

    /// Update FPS calculation
    fn update_fps(&mut self) {
        self.frame_count += 1;
        
        if self.last_fps_update.elapsed() >= Duration::from_secs(1) {
            self.current_fps = self.frame_count as f32 / self.last_fps_update.elapsed().as_secs_f32();
            self.frame_count = 0;
            self.last_fps_update = Instant::now();
        }
    }
}

impl EcsApp for PreviewApp {
    /// Initialize the preview application with the AI-generated scene
    fn setup(&mut self, world: &mut World) -> lumina_core::Result<()> {
        info!("Setting up AI-generated game preview: {}", self.scene.metadata.name);
        
        // Load the AI-generated scene into the ECS world
        let scene_loader = SceneLoader::new();
        let load_result = scene_loader.load_scene(world, self.scene.clone())
            .map_err(|e| lumina_core::LuminaError::InitializationError(format!("Failed to load scene: {}", e)))?;
        
        info!("Preview setup complete: {} entities created", load_result.entities_created);
        
        // Store scene metadata for later use
        self.scene_metadata = Some(load_result.metadata);
        
        Ok(())
    }
    
    /// Update the game every frame
    fn update(&mut self, world: &mut World) -> lumina_core::Result<()> {
        // Update FPS tracking
        self.update_fps();
        
        // Run any custom game logic systems here
        // For AI-generated games, this could include:
        // - Physics updates
        // - AI behavior systems
        // - Collision detection
        // - Game logic updates
        
        Ok(())
    }
    
    /// Handle window events like keyboard input
    fn handle_event(&mut self, _world: &mut World, event: &WindowEvent) -> lumina_core::Result<bool> {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                use winit::keyboard::{KeyCode, PhysicalKey};
                
                if event.state.is_pressed() {
                    match event.physical_key {
                        PhysicalKey::Code(KeyCode::Escape) => {
                            info!("Escape pressed - closing preview");
                            // Return true to indicate we handled the event and want to exit
                            return Ok(true);
                        }
                        PhysicalKey::Code(KeyCode::F1) => {
                            info!("F1 pressed - toggling debug overlay");
                            self.config.debug_overlay = !self.config.debug_overlay;
                            return Ok(true);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        
        // Event not handled by us
        Ok(false)
    }
    
    /// Get window configuration for the preview
    fn window_config(&self) -> WindowConfig {
        WindowConfig {
            title: format!("Lumina AI Preview: {}", self.scene.metadata.name),
            size: LogicalSize::new(self.config.window_size.0, self.config.window_size.1),
            resizable: true,
        }
    }
    
    /// Get render configuration for the preview
    fn render_config(&self) -> RenderConfig {
        RenderConfig {
            target_fps: self.config.target_fps,
            vsync: true,
            ..RenderConfig::default()
        }
    }
    
    /// Get UI theme for the preview
    fn theme(&self) -> Theme {
        Theme::dark()
    }
    
    /// Handle cleanup when the preview shuts down
    fn shutdown(&mut self, _world: &mut World) -> lumina_core::Result<()> {
        info!("AI preview shutting down");
        Ok(())
    }
}

impl Default for PreviewConfig {
    fn default() -> Self {
        Self {
            auto_preview: true,
            debug_overlay: true,
            target_fps: 60,
            max_entities: 1000,
            window_size: (1280, 720),
        }
    }
}

/// Convenience functions for quick preview operations
pub mod preview {
    use super::*;
    
    /// Quick preview of an SDL JSON string
    pub async fn preview_sdl_string(sdl_json: &str) -> Result<PreviewResult> {
        let scene = SceneDescription::from_json(sdl_json)?;
        let mut preview_system = LivePreviewSystem::new(PreviewConfig::default());
        preview_system.preview_scene(scene).await
    }
    
    /// Quick preview of a scene file
    pub async fn preview_scene_file(file_path: &str) -> Result<PreviewResult> {
        let sdl_json = std::fs::read_to_string(file_path)?;
        preview_sdl_string(&sdl_json).await
    }
    
    /// Preview with custom configuration
    pub async fn preview_with_config(scene: SceneDescription, config: PreviewConfig) -> Result<PreviewResult> {
        let mut preview_system = LivePreviewSystem::new(config);
        preview_system.preview_scene(scene).await
    }
}