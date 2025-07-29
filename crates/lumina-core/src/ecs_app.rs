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
    
    /// Handle UI actions from button clicks and other UI interactions
    fn handle_ui_action(&mut self, _world: &mut World, _action: String) -> Result<()> {
        // Default implementation logs the action
        log::info!("🖱️ UI action received: {}", _action);
        Ok(())
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
        println!("🔧 [DEBUG] Starting EcsAppRunner::run()");
        
        // Create event loop and window
        println!("🔧 [DEBUG] Creating event loop...");
        let event_loop = EventLoop::new()
            .map_err(|e| LuminaError::InitializationError(format!("Failed to create event loop: {}", e)))?;
        println!("🔧 [DEBUG] Event loop created successfully");
        
        println!("🔧 [DEBUG] Creating window...");
        let window = Arc::new(
            WindowBuilder::new()
                .with_title(&self.window_config.title)
                .with_inner_size(self.window_config.size)
                .with_resizable(self.window_config.resizable)
                .build(&event_loop)
                .map_err(|e| LuminaError::InitializationError(format!("Failed to create window: {}", e)))?
        );
        println!("🔧 [DEBUG] Window created successfully");
        
        // Initialize core resources
        println!("🔧 [DEBUG] Initializing core resources...");
        self.initialize_resources(window.clone()).await?;
        println!("🔧 [DEBUG] Core resources initialized");
        
        // Setup the application
        println!("🔧 [DEBUG] Setting up application...");
        {
            let mut world = self.world.lock().unwrap();
            self.app.setup(&mut world)?;
        }
        println!("🔧 [DEBUG] Application setup complete");
        
        println!("🚀 Lumina ECS Application started!");
        println!("💡 Architecture features:");
        println!("   • Pure ECS-driven game loop");
        println!("   • RenderContext resource in lumina-render");
        println!("   • UI rendering through lumina-core systems");
        println!("   • Complete separation of concerns");
        
        println!("🔧 [DEBUG] Starting event loop...");
        println!("✨ Window should now be visible with dark blue background");
        println!("💡 Press Ctrl+C or close the window to exit");
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
                    // Request a redraw for continuous rendering
                    // This should be fine now that we properly present frames
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
        println!("🔧 [DEBUG] Locking world for resource initialization...");
        let mut world = self.world.lock().unwrap();
        
        // Initialize render context resource
        println!("🔧 [DEBUG] Creating render context...");
        let render_context = RenderContext::new(window.clone(), self.render_config.clone())
            .await
            .map_err(|e| LuminaError::InitializationError(format!("Failed to create render context: {}", e)))?;
        println!("🔧 [DEBUG] Render context created, adding to world...");
        world.add_resource(render_context);
        
        // Initialize UI framework resource
        println!("🔧 [DEBUG] Creating UI framework...");
        let ui_framework = UiFramework::new(self.theme.clone());
        println!("🔧 [DEBUG] UI framework created, adding to world...");
        world.add_resource(ui_framework);
        
        // Initialize input events resource
        println!("🔧 [DEBUG] Creating input events resource...");
        world.add_resource(crate::render_systems::InputEvents::default());
        
        println!("🔧 [DEBUG] All resources added to world successfully");
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
                    
                    // Process input events after UI is rendered
                    match crate::render_systems::process_input_events(&mut world) {
                        Ok(actions) => {
                            for action in actions {
                                if let Err(e) = self.app.handle_ui_action(&mut world, action) {
                                    eprintln!("UI action error: {}", e);
                                }
                            }
                        }
                        Err(e) => eprintln!("Input processing error: {}", e),
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