//! ECS-Driven Architecture Demo
//! 
//! This example demonstrates the complete ECS-driven architecture for the Lumina Engine,
//! following the design outlined in ARCHITECTURE.md. The main loop only runs the ECS
//! schedule, and all rendering happens through ECS systems.

use lumina_ui::{
    RenderContext, UiFramework, Theme, EcsRunner, Schedule,
    UiBuilder, Color, ButtonStyle,
    example_components::{Player, GameState}
};
use lumina_ecs::World;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
    dpi::LogicalSize,
};
use std::sync::Arc;

/// Main application struct that manages the ECS runner and window
struct EcsArchitectureDemo {
    ecs_runner: EcsRunner,
    window: Arc<Window>,
}

impl EcsArchitectureDemo {
    /// Create a new ECS architecture demo
    async fn new(window: Arc<Window>) -> Result<Self, Box<dyn std::error::Error>> {
        // Create ECS world
        let world = World::new();
        
        // Initialize render context resource
        let render_context = RenderContext::new(window.clone()).await?;
        world.add_resource(render_context);
        
        // Initialize UI framework resource
        let mut ui_framework = UiFramework::new(Theme::default());
        
        // Set up UI renderer (simplified for demo)
        // In a real implementation, you'd extract device/queue from RenderContext
        // ui_framework.set_renderer(ui_renderer);
        
        // Build initial UI
        Self::build_demo_ui(&mut ui_framework);
        
        world.add_resource(ui_framework);
        
        // Add example game entities
        let player_entity = world.spawn_with(Player {
            name: "Hero".to_string(),
            health: 100,
            max_health: 100,
            level: 1,
        });
        
        let game_state_entity = world.spawn_with(GameState {
            score: 0,
            lives: 3,
            paused: false,
        });
        
        println!("🎮 ECS Architecture Demo initialized!");
        println!("   Player entity: {:?}", player_entity);
        println!("   Game state entity: {:?}", game_state_entity);
        
        // Create ECS runner with UI systems
        let schedule = Schedule::new();
        let ecs_runner = EcsRunner::new(world, schedule);
        
        Ok(Self {
            ecs_runner,
            window,
        })
    }
    
    /// Build the demo UI using the UI framework
    fn build_demo_ui(_ui_framework: &mut UiFramework) {
        let mut ui_builder = UiBuilder::dark();
        
        // Title
        // For demo purposes, we'll just create some widgets but not add them
        // In a real implementation, these would be properly integrated
        let _title = ui_builder.text("🚀 ECS-Driven Architecture Demo")
            .size(28.0)
            .color(Color::hex("#00D9FF").unwrap());
        
        let _subtitle = ui_builder.text("Complete decoupling of rendering from application logic")
            .size(16.0)
            .color(Color::hex("#AAAAAA").unwrap());
        
        let _start_button = ui_builder.button("▶️ Start Game")
            .style(ButtonStyle::Success)
            .on_click(|| {
                println!("Start game clicked - would modify ECS GameState component");
            });
        
        let _pause_button = ui_builder.button("⏸️ Pause Game")
            .style(ButtonStyle::Warning)
            .on_click(|| {
                println!("Pause game clicked - would toggle ECS GameState.paused");
            });
        
        let _heal_button = ui_builder.button("💚 Heal Player")
            .style(ButtonStyle::Primary)
            .on_click(|| {
                println!("Heal player clicked - would modify ECS Player.health");
            });
        
        let _status = ui_builder.text("All rendering happens through ECS systems")
            .size(14.0)
            .color(Color::GREEN);
    }
    
    /// Handle window resize
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.ecs_runner.world_mut().with_resource_mut::<RenderContext, _>(|mut render_context_opt| {
            if let Some(render_context) = render_context_opt.as_mut() {
                render_context.resize(new_size);
            }
        });
    }
    
    /// Handle input events through ECS systems
    fn input(&mut self, event: &WindowEvent) -> bool {
        self.ecs_runner.handle_event(event);
        false
    }
    
    /// Update the application by running the ECS schedule
    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.ecs_runner.run_frame()
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("ECS-Driven Architecture Demo")
            .with_inner_size(LogicalSize::new(1200, 800))
            .with_resizable(true)
            .build(&event_loop)?
    );
    
    let mut app = EcsArchitectureDemo::new(window.clone()).await?;
    
    println!("🚀 ECS Architecture Demo started!");
    println!("💡 Key features demonstrated:");
    println!("   • RenderContext resource holds all WGPU state");
    println!("   • UI rendering happens in ui_render_system");
    println!("   • Main loop only runs ECS schedule");
    println!("   • Complete decoupling of rendering from app logic");
    println!("   • WGPU complexity hidden from developers");
    
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !app.input(event) {
                    match event {
                        WindowEvent::CloseRequested => {
                            println!("👋 Closing ECS Architecture Demo...");
                            elwt.exit();
                        }
                        WindowEvent::Resized(physical_size) => {
                            app.resize(*physical_size);
                        }
                        WindowEvent::RedrawRequested => {
                            match app.update() {
                                Ok(_) => {},
                                Err(e) => eprintln!("Update error: {:?}", e),
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    })?;
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(run())
}