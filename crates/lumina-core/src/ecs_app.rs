//! ECS-driven application framework for Lumina Engine
//! 
//! This module provides an enhanced application framework that integrates
//! ECS, rendering, and windowing into a cohesive architecture.

use crate::{Result, LuminaError};
use lumina_ecs::World;
use lumina_render::{RenderContext, RenderConfig};
use lumina_ui::{UiFramework, Theme};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
    dpi::LogicalSize,
};
use std::sync::{Arc, Mutex};

/// Enhanced application trait for ECS-driven Lumina applications
/// 
/// This trait extends the basic App trait with ECS and windowing support.
pub trait EcsApp {
    /// Initialize the application with ECS world and systems
    fn setup(&mut self, world: &mut World) -> Result<()>;
    
    /// Update the application (called every frame)
    fn update(&mut self, _world: &mut World) -> Result<()> {
        // Default implementation does nothing
        Ok(())
    }
    
    /// Handle window events
    fn handle_event(&mut self, _world: &mut World, _event: &WindowEvent) -> Result<bool> {
        // Default implementation doesn't handle events
        // Return true if event was handled
        Ok(false)
    }
    
    /// Get window configuration
    fn window_config(&self) -> WindowConfig {
        WindowConfig::default()
    }
    
    /// Get render configuration
    fn render_config(&self) -> RenderConfig {
        RenderConfig::default()
    }
    
    /// Get UI theme
    fn theme(&self) -> Theme {
        Theme::default()
    }
    
    /// Called when the application shuts down
    fn shutdown(&mut self, _world: &mut World) -> Result<()> {
        // Default implementation does nothing
        Ok(())
    }
}

/// Window configuration for ECS applications
#[derive(Debug, Clone)]
pub struct WindowConfig {
    /// Window title
    pub title: String,
    /// Initial window size
    pub size: LogicalSize<u32>,
    /// Whether window is resizable
    pub resizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Lumina Game".to_string(),
            size: LogicalSize::new(1280, 720),
            resizable: true,
        }
    }
}

/// ECS application runner that manages the complete game loop
/// 
/// This runner integrates ECS, rendering, UI, and windowing into a single,
/// cohesive application framework following the architecture principles.
pub struct EcsAppRunner<T: EcsApp> {
    /// User's application implementation
    app: T,
    /// ECS world
    world: Arc<Mutex<World>>,
    /// Window configuration
    window_config: WindowConfig,
    /// Render configuration
    render_config: RenderConfig,
    /// UI theme
    theme: Theme,
}

impl<T: EcsApp> EcsAppRunner<T> {
    /// Create a new ECS application runner
    pub fn new(app: T) -> Self {
        let window_config = app.window_config();
        let render_config = app.render_config();
        let theme = app.theme();
        
        Self {
            app,
            world: Arc::new(Mutex::new(World::new())),
            window_config,
            render_config,
            theme,
        }
    }
    
    /// Run the application with the complete ECS-driven architecture
    pub async fn run(mut self) -> Result<()> {
        env_logger::init();
        
        // Create event loop and window
        let event_loop = EventLoop::new()
            .map_err(|e| LuminaError::InitializationError(format!("Failed to create event loop: {}", e)))?;
        
        let window = Arc::new(
            WindowBuilder::new()
                .with_title(&self.window_config.title)
                .with_inner_size(self.window_config.size)
                .with_resizable(self.window_config.resizable)
                .build(&event_loop)
                .map_err(|e| LuminaError::InitializationError(format!("Failed to create window: {}", e)))?
        );
        
        // Initialize core resources
        self.initialize_resources(window.clone()).await?;
        
        // Setup the application
        {
            let mut world = self.world.lock().unwrap();
            self.app.setup(&mut world)?;
        }
        
        println!("🚀 Lumina ECS Application started!");
        println!("💡 Architecture features:");
        println!("   • Pure ECS-driven game loop");
        println!("   • RenderContext resource in lumina-render");
        println!("   • UI rendering through lumina-core systems");
        println!("   • Complete separation of concerns");
        
        // Run the main event loop
        event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    self.handle_window_event(event, elwt);
                }
                Event::AboutToWait => {
                    window.request_redraw();
                }
                Event::LoopExiting => {
                    println!("👋 Lumina Application shutting down...");
                    let mut world = self.world.lock().unwrap();
                    let _ = self.app.shutdown(&mut world);
                }
                _ => {}
            }
        })
        .map_err(|e| LuminaError::RuntimeError(format!("Event loop error: {}", e)).into())
    }
    
    /// Initialize all core resources
    async fn initialize_resources(&mut self, window: Arc<Window>) -> Result<()> {
        let mut world = self.world.lock().unwrap();
        
        // Initialize render context resource
        let render_context = RenderContext::new(window.clone(), self.render_config.clone())
            .await
            .map_err(|e| LuminaError::InitializationError(format!("Failed to create render context: {}", e)))?;
        world.add_resource(render_context);
        
        // Initialize UI framework resource
        let ui_framework = UiFramework::new(self.theme.clone());
        world.add_resource(ui_framework);
        
        Ok(())
    }
    
    /// Handle window events
    fn handle_window_event(&mut self, event: &WindowEvent, elwt: &winit::event_loop::EventLoopWindowTarget<()>) {
        let mut world = self.world.lock().unwrap();
        
        // Let the application handle the event first
        let handled = match self.app.handle_event(&mut world, event) {
            Ok(handled) => handled,
            Err(e) => {
                eprintln!("Error handling event: {}", e);
                false
            }
        };
        
        if !handled {
            match event {
                WindowEvent::CloseRequested => {
                    elwt.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    world.with_resource_mut::<RenderContext, _>(|mut render_context_opt| {
                        if let Some(render_context) = render_context_opt.as_mut() {
                            render_context.resize(*physical_size);
                        }
                    });
                }
                WindowEvent::RedrawRequested => {
                    // Update the application
                    if let Err(e) = self.app.update(&mut world) {
                        eprintln!("Update error: {}", e);
                    }
                    
                    // Run rendering systems
                    if let Err(e) = crate::render_systems::ui_update_system(&mut world) {
                        eprintln!("UI update error: {}", e);
                    }
                    
                    if let Err(e) = crate::render_systems::ui_render_system(&mut world) {
                        eprintln!("UI render error: {}", e);
                    }
                }
                _ => {
                    // Forward other input events to the input system
                    if let Err(e) = crate::render_systems::input_system(&mut world, event) {
                        eprintln!("Input system error: {}", e);
                    }
                }
            }
        }
    }
}

/// Convenience function to run an ECS application
/// 
/// This is the main entry point for ECS-driven Lumina applications.
pub async fn run_ecs_app<T: EcsApp>(app: T) -> Result<()> {
    EcsAppRunner::new(app).run().await
}

/// Example ECS application for demonstration
pub struct ExampleEcsApp {
    pub title: String,
}

impl ExampleEcsApp {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }
}

impl EcsApp for ExampleEcsApp {
    fn setup(&mut self, world: &mut World) -> Result<()> {
        // Add some example components for demonstration
        use lumina_ui::example_components::{Player, GameState};
        
        let player_entity = world.spawn_with(Player {
            name: "Demo Player".to_string(),
            health: 100,
            max_health: 100,
            level: 1,
        });
        
        let game_state_entity = world.spawn_with(GameState {
            score: 0,
            lives: 3,
            paused: false,
        });
        
        println!("🎮 Example ECS app setup complete!");
        println!("   Player entity: {:?}", player_entity);
        println!("   Game state entity: {:?}", game_state_entity);
        
        Ok(())
    }
    
    fn window_config(&self) -> WindowConfig {
        WindowConfig {
            title: self.title.clone(),
            size: LogicalSize::new(1200, 800),
            resizable: true,
        }
    }
    
    fn theme(&self) -> Theme {
        Theme::dark()
    }
}