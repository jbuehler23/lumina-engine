//! Pong Game Example - Lumina Engine
//! 
//! This example demonstrates the proper use of Lumina Engine's modular architecture:
//! - lumina-ecs: Entity-Component-System for game state
//! - lumina-input: Bevy-inspired input handling
//! - lumina-render: WGPU-based rendering
//! - lumina-ui: Immediate mode UI framework
//! - lumina-core: ECS app runner and systems integration

use lumina_core::{ecs_app::{EcsAppRunner, EcsApp, WindowConfig}, Result, Renderable};
use lumina_ecs::World;
use lumina_input::ButtonInput;
use lumina_render::RenderConfig;
use lumina_ui::Theme;
use glam::{Vec2, Vec4};
use winit::{event::WindowEvent, dpi::LogicalSize};

// Game Components
#[derive(Debug, Clone)]
struct Position(Vec2);

#[derive(Debug, Clone)]
struct Velocity(Vec2);

#[derive(Debug, Clone)]
struct Size(Vec2);

#[derive(Debug, Clone)]
struct Player {
    player_id: u8, // 1 or 2
}

#[derive(Debug, Clone)]
struct Ball;

#[derive(Debug, Clone)]
struct Score {
    player1: u32,
    player2: u32,
}

// Game constants
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_SPEED: f32 = 400.0;
const BALL_SIZE: f32 = 20.0;
const BALL_SPEED: f32 = 300.0;

struct PongApp {
    frame_count: u64,
}

impl PongApp {
    fn new() -> Self {
        Self { frame_count: 0 }
    }
    
    fn handle_input(&self, world: &mut World, _dt: f32) -> Result<()> {
        // Use lumina-input for keyboard input
        world.with_resource::<ButtonInput<winit::keyboard::PhysicalKey>, _>(|keyboard_input_opt| {
            if let Some(keyboard) = keyboard_input_opt {
                // Player 1 controls (W/S keys)
                for (entity, player) in world.query::<Player>() {
                    if player.player_id == 1 {
                        world.with_component_mut::<Velocity, ()>(entity, |velocity_opt| {
                            if let Some(velocity) = velocity_opt {
                                let mut vel_y = 0.0;
                                
                                if keyboard.pressed(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyW)) {
                                    vel_y = -PADDLE_SPEED;
                                }
                                if keyboard.pressed(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyS)) {
                                    vel_y = PADDLE_SPEED;
                                }
                                
                                velocity.0.y = vel_y;
                            }
                        });
                    } else if player.player_id == 2 {
                        world.with_component_mut::<Velocity, ()>(entity, |velocity_opt| {
                            if let Some(velocity) = velocity_opt {
                                let mut vel_y = 0.0;
                                
                                if keyboard.pressed(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowUp)) {
                                    vel_y = -PADDLE_SPEED;
                                }
                                if keyboard.pressed(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowDown)) {
                                    vel_y = PADDLE_SPEED;
                                }
                                
                                velocity.0.y = vel_y;
                            }
                        });
                    }
                }
            }
        });
        
        Ok(())
    }
    
    fn update_physics(&self, world: &mut World, dt: f32) -> Result<()> {
        // Update positions based on velocities
        for (entity, velocity) in world.query::<Velocity>() {
            world.with_component_mut::<Position, ()>(entity, |position_opt| {
                if let Some(position) = position_opt {
                    position.0 += velocity.0 * dt;
                }
            });
        }
        
        // Keep paddles within screen bounds
        for (entity, _player) in world.query::<Player>() {
            world.with_component_mut::<Position, ()>(entity, |position_opt| {
                if let Some(position) = position_opt {
                    if position.0.y < 0.0 {
                        position.0.y = 0.0;
                    }
                    if position.0.y > WINDOW_HEIGHT - PADDLE_HEIGHT {
                        position.0.y = WINDOW_HEIGHT - PADDLE_HEIGHT;
                    }
                }
            });
        }
        
        Ok(())
    }
    
    fn handle_collisions(&self, world: &mut World) -> Result<()> {
        // Ball collision with walls (top/bottom)
        for (ball_entity, _) in world.query::<Ball>() {
            let mut should_reverse_y = false;
            
            world.with_component::<Position, ()>(ball_entity, |position_opt| {
                if let Some(position) = position_opt {
                    if position.0.y <= 0.0 || position.0.y >= WINDOW_HEIGHT - BALL_SIZE {
                        should_reverse_y = true;
                    }
                }
            });
            
            if should_reverse_y {
                world.with_component_mut::<Velocity, ()>(ball_entity, |velocity_opt| {
                    if let Some(velocity) = velocity_opt {
                        velocity.0.y = -velocity.0.y;
                    }
                });
            }
            
            // Ball collision with paddles
            world.with_component::<Position, ()>(ball_entity, |ball_pos_opt| {
                if let Some(ball_pos) = ball_pos_opt {
                    for (paddle_entity, _) in world.query::<Player>() {
                        world.with_component::<Position, ()>(paddle_entity, |paddle_pos_opt| {
                            if let Some(paddle_pos) = paddle_pos_opt {
                                // Simple AABB collision detection
                                if ball_pos.0.x < paddle_pos.0.x + PADDLE_WIDTH &&
                                   ball_pos.0.x + BALL_SIZE > paddle_pos.0.x &&
                                   ball_pos.0.y < paddle_pos.0.y + PADDLE_HEIGHT &&
                                   ball_pos.0.y + BALL_SIZE > paddle_pos.0.y {
                                    
                                    world.with_component_mut::<Velocity, ()>(ball_entity, |velocity_opt| {
                                        if let Some(velocity) = velocity_opt {
                                            velocity.0.x = -velocity.0.x;
                                        }
                                    });
                                }
                            }
                        });
                    }
                }
            });
            
            // Ball off screen (scoring) - check position first
            let mut should_score = None;
            world.with_component::<Position, ()>(ball_entity, |position_opt| {
                if let Some(position) = position_opt {
                    if position.0.x < 0.0 {
                        should_score = Some(2); // Player 2 scores
                    } else if position.0.x > WINDOW_WIDTH {
                        should_score = Some(1); // Player 1 scores
                    }
                }
            });
            
            // Handle scoring outside of the position closure
            if let Some(scoring_player) = should_score {
                world.with_resource_mut::<Score, _>(|mut score_opt| {
                    if let Some(score) = score_opt.as_mut() {
                        if scoring_player == 1 {
                            score.player1 += 1;
                            log::info!("🏓 Player 1 scores! Score: {} - {}", score.player1, score.player2);
                        } else {
                            score.player2 += 1;
                            log::info!("🏓 Player 2 scores! Score: {} - {}", score.player1, score.player2);
                        }
                    }
                });
                self.reset_ball(world, ball_entity);
            }
        }
        
        Ok(())
    }
    
    fn reset_ball(&self, world: &mut World, ball_entity: lumina_ecs::Entity) {
        let new_pos = Vec2::new(WINDOW_WIDTH / 2.0 - BALL_SIZE / 2.0, WINDOW_HEIGHT / 2.0 - BALL_SIZE / 2.0);
        
        world.with_component_mut::<Position, ()>(ball_entity, |position_opt| {
            if let Some(position) = position_opt {
                position.0 = new_pos;
            }
        });
        
        // Update the renderable position too
        world.with_component_mut::<Renderable, ()>(ball_entity, |renderable_opt| {
            if let Some(renderable) = renderable_opt {
                renderable.position = new_pos;
            }
        });
        
        world.with_component_mut::<Velocity, ()>(ball_entity, |velocity_opt| {
            if let Some(velocity) = velocity_opt {
                // Random direction
                let dir_x = if simple_random() { 1.0 } else { -1.0 };
                let dir_y = (simple_random_f32() - 0.5) * 2.0;
                velocity.0 = Vec2::new(BALL_SPEED * dir_x, BALL_SPEED * dir_y);
            }
        });
    }
    
    fn sync_renderables(&self, world: &mut World) -> Result<()> {
        // Sync Position components with Renderable components
        for (entity, position) in world.query::<Position>() {
            world.with_component_mut::<Renderable, ()>(entity, |renderable_opt| {
                if let Some(renderable) = renderable_opt {
                    renderable.position = position.0;
                }
            });
        }
        Ok(())
    }
}

impl EcsApp for PongApp {
    fn window_config(&self) -> WindowConfig {
        WindowConfig {
            title: "Pong - Lumina Engine".to_string(),
            size: LogicalSize::new(800, 600),
            resizable: false,
        }
    }

    fn render_config(&self) -> RenderConfig {
        RenderConfig::default()
    }

    fn theme(&self) -> Theme {
        Theme::dark()
    }

    fn setup(&mut self, world: &mut World) -> Result<()> {
        log::info!("🏓 Setting up Pong game");
        
        // Define colors
        let paddle_color = Vec4::new(0.9, 0.9, 0.9, 1.0); // White paddles
        let ball_color = Vec4::new(1.0, 0.5, 0.2, 1.0);   // Orange ball
        
        // Create paddles
        let paddle1_pos = Vec2::new(50.0, WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0);
        world.spawn()
            .with(Position(paddle1_pos))
            .with(Velocity(Vec2::ZERO))
            .with(Size(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)))
            .with(Player { player_id: 1 })
            .with(Renderable {
                position: paddle1_pos,
                size: Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
                color: paddle_color,
            })
            .build(&world);
            
        let paddle2_pos = Vec2::new(WINDOW_WIDTH - 50.0 - PADDLE_WIDTH, WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0);
        world.spawn()
            .with(Position(paddle2_pos))
            .with(Velocity(Vec2::ZERO))
            .with(Size(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)))
            .with(Player { player_id: 2 })
            .with(Renderable {
                position: paddle2_pos,
                size: Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
                color: paddle_color,
            })
            .build(&world);
        
        // Create ball
        let ball_pos = Vec2::new(WINDOW_WIDTH / 2.0 - BALL_SIZE / 2.0, WINDOW_HEIGHT / 2.0 - BALL_SIZE / 2.0);
        world.spawn()
            .with(Position(ball_pos))
            .with(Velocity(Vec2::new(BALL_SPEED, BALL_SPEED * 0.5)))
            .with(Size(Vec2::new(BALL_SIZE, BALL_SIZE)))
            .with(Ball)
            .with(Renderable {
                position: ball_pos,
                size: Vec2::new(BALL_SIZE, BALL_SIZE),
                color: ball_color,
            })
            .build(&world);
        
        // Create score resource
        world.add_resource(Score { player1: 0, player2: 0 });
        
        // Add keyboard input resource
        if !world.has_resource::<ButtonInput<winit::keyboard::PhysicalKey>>() {
            world.add_resource(ButtonInput::<winit::keyboard::PhysicalKey>::default());
        }
        
        log::info!("✅ Pong setup complete!");
        Ok(())
    }

    fn update(&mut self, world: &mut World) -> Result<()> {
        self.frame_count += 1;
        
        // Get delta time (simplified for this example)
        let dt = 1.0 / 60.0; // Assume 60 FPS
        
        // Game systems
        self.handle_input(world, dt)?;
        self.update_physics(world, dt)?;
        self.handle_collisions(world)?;
        self.sync_renderables(world)?;
        
        // Debug output every 60 frames
        if self.frame_count % 60 == 0 {
            world.with_resource::<Score, _>(|score_opt| {
                if let Some(score) = score_opt {
                    log::debug!("🏓 Current score: {} - {}", score.player1, score.player2);
                }
            });
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
                        log::info!("🏓 Exiting Pong game");
                        return Ok(false); // Exit game
                    }
                }
            }
            WindowEvent::CloseRequested => {
                log::info!("🏓 Window close requested");
                return Ok(false); // Exit game
            }
            _ => {}
        }
        
        Ok(true) // Continue running
    }

    fn shutdown(&mut self, _world: &mut World) -> Result<()> {
        log::info!("🏓 Pong game shutting down");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    log::info!("🏓 Starting Pong Game - Lumina Engine Demo");
    log::info!("Controls: Player 1 (W/S), Player 2 (Arrow Keys), ESC to quit");
    
    // Create and run Pong app
    let pong_app = PongApp::new();
    let runner = EcsAppRunner::new(pong_app);
    
    runner.run().await
}

// Simple random number generation for ball reset
fn simple_random() -> bool {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
    hasher.finish() % 2 == 0
}

fn simple_random_f32() -> f32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
    (hasher.finish() % 1000) as f32 / 1000.0
}