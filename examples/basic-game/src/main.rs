//! Basic Game Example - Lumina Engine (Updated Architecture)
//! 
//! This example demonstrates the updated ECS architecture with proper
//! separation of concerns using individual crates:
//! - lumina-ecs: Entity-Component-System 
//! - lumina-input: Input handling
//! - lumina-render: Rendering pipeline
//! - lumina-core: ECS app runner

use lumina_core::{ecs_app::{EcsAppRunner, EcsApp, WindowConfig}, Result};
use lumina_ecs::World;
use lumina_input::ButtonInput;
use lumina_render::RenderConfig;
use lumina_ui::Theme;
use glam::Vec2;
use winit::{event::WindowEvent, dpi::LogicalSize};

// Game Components using proper ECS patterns
#[derive(Debug, Clone)]
struct Position(Vec2);

#[derive(Debug, Clone)]
struct Velocity(Vec2);

#[derive(Debug, Clone)]
struct Player {
    speed: f32,
}

struct BasicGameApp {
    frame_count: u64,
}

impl BasicGameApp {
    fn new() -> Self {
        Self { frame_count: 0 }
    }
}

impl EcsApp for BasicGameApp {
    fn window_config(&self) -> WindowConfig {
        WindowConfig {
            title: "Basic Game - Lumina Engine (Modern ECS)".to_string(),
            size: LogicalSize::new(800, 600),
            resizable: true,
        }
    }

    fn render_config(&self) -> RenderConfig {
        RenderConfig::default()
    }

    fn theme(&self) -> Theme {
        Theme::default()
    }

    fn setup(&mut self, world: &mut World) -> Result<()> {
        log::info!("🎮 Setting up Basic Game with ECS architecture");
        
        // Create a player entity with Position, Velocity, and Player components
        world.spawn()
            .with(Position(Vec2::new(100.0, 100.0)))
            .with(Velocity(Vec2::ZERO))
            .with(Player { speed: 200.0 })
            .build(&world);
        
        // Add keyboard input resource for WASD/Arrow key controls
        if !world.has_resource::<ButtonInput<winit::keyboard::PhysicalKey>>() {
            world.add_resource(ButtonInput::<winit::keyboard::PhysicalKey>::default());
        }
        
        log::info!("✅ Basic Game initialized successfully with modern ECS!");
        Ok(())
    }

    fn update(&mut self, world: &mut World) -> Result<()> {
        self.frame_count += 1;
        
        // Get delta time (simplified)
        let dt = 1.0 / 60.0; // Assume 60 FPS
        
        // Player movement system using lumina-input
        self.player_movement_system(world, dt)?;
        
        // Movement system
        self.movement_system(world, dt)?;
        
        // Debug system (every 60 frames)
        if self.frame_count % 60 == 0 {
            self.debug_system(world)?;
        }
        
        Ok(())
    }

    fn handle_event(&mut self, world: &mut World, event: &WindowEvent) -> Result<bool> {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                // Update keyboard input state using lumina-input
                world.with_resource_mut::<ButtonInput<winit::keyboard::PhysicalKey>, _>(|mut keyboard_input_opt| {
                    if let Some(keyboard) = keyboard_input_opt.as_mut() {
                        match event.state {
                            winit::event::ElementState::Pressed => {
                                keyboard.press(event.physical_key);
                            }
                            winit::event::ElementState::Released => {
                                keyboard.release(event.physical_key);
                            }
                        }
                    }
                });
                
                // Handle ESC to quit
                if let winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape) = event.physical_key {
                    if event.state == winit::event::ElementState::Pressed {
                        log::info!("👋 Goodbye! (ESC pressed)");
                        return Ok(false); // Exit game
                    }
                }
            }
            WindowEvent::CloseRequested => {
                log::info!("🔚 Window close requested");
                return Ok(false); // Exit game
            }
            _ => {}
        }
        
        Ok(true) // Continue running
    }

    fn shutdown(&mut self, _world: &mut World) -> Result<()> {
        log::info!("🔚 Basic Game shutdown with modern ECS");
        Ok(())
    }
}

impl BasicGameApp {
    fn player_movement_system(&self, world: &mut World, dt: f32) -> Result<()> {
        world.with_resource::<ButtonInput<winit::keyboard::PhysicalKey>, _>(|keyboard_input_opt| {
            if let Some(keyboard) = keyboard_input_opt {
                // Find entities with both Player and Velocity components
                for (entity, player) in world.query::<Player>() {
                    world.with_component_mut::<Velocity, ()>(entity, |velocity_opt| {
                        if let Some(velocity) = velocity_opt {
                            let mut vel = Vec2::ZERO;
                            
                            // WASD controls
                            if keyboard.pressed(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyW)) {
                                vel.y -= 1.0;
                            }
                            if keyboard.pressed(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyS)) {
                                vel.y += 1.0;
                            }
                            if keyboard.pressed(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyA)) {
                                vel.x -= 1.0;
                            }
                            if keyboard.pressed(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyD)) {
                                vel.x += 1.0;
                            }
                            
                            // Arrow key controls (alternative)
                            if keyboard.pressed(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowUp)) {
                                vel.y -= 1.0;
                            }
                            if keyboard.pressed(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowDown)) {
                                vel.y += 1.0;
                            }
                            if keyboard.pressed(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowLeft)) {
                                vel.x -= 1.0;
                            }
                            if keyboard.pressed(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowRight)) {
                                vel.x += 1.0;
                            }
                            
                            if vel.length() > 0.0 {
                                vel = vel.normalize() * player.speed;
                            }
                            
                            velocity.0 = vel;
                        }
                    });
                }
            }
        });
        
        Ok(())
    }

    fn movement_system(&self, world: &mut World, dt: f32) -> Result<()> {
        // Update positions based on velocities
        for (entity, velocity) in world.query::<Velocity>() {
            world.with_component_mut::<Position, ()>(entity, |position_opt| {
                if let Some(position) = position_opt {
                    position.0 += velocity.0 * dt;
                }
            });
        }
        
        Ok(())
    }

    fn debug_system(&self, world: &mut World) -> Result<()> {
        for (entity, position) in world.query::<Position>() {
            if world.has_component::<Player>(entity) {
                log::info!("🎮 Player position: ({:.1}, {:.1})", position.0.x, position.0.y);
            }
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    log::info!("🚀 Starting Basic Game with Modern ECS Architecture");
    log::info!("Controls: WASD or Arrow Keys to move, ESC to quit");
    log::info!("Architecture: lumina-ecs + lumina-input + lumina-render + lumina-ui + lumina-core");
    
    // Create and run the game
    let app = BasicGameApp::new();
    let runner = EcsAppRunner::new(app);
    
    runner.run().await
}