//! Game Prototype - ECS Architecture Demo
//! 
//! This demonstrates a complete game using the new ECS-driven architecture
//! where all rendering, UI, and game logic are handled through ECS systems.

use lumina_core::{EcsApp, run_ecs_app, WindowConfig, Result};
use lumina_ui::{UiFramework, UiBuilder, Color, ButtonStyle, example_components::{Player, GameState}};
use lumina_ecs::{World, Entity};
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use glam::{Vec2, Vec3};

// Game Components
#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: f32,
    pub scale: Vec3,
}

#[derive(Debug, Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct PlayerComponent {
    pub speed: f32,
    pub jump_force: f32,
    pub grounded: bool,
}

#[derive(Debug, Clone)]
pub struct Coin {
    pub value: i32,
    pub collected: bool,
}

#[derive(Debug, Clone)]
pub struct Sprite {
    pub texture_id: String,
    pub color: [f32; 4],
    pub size: Vec2,
}

/// Main game application using the ECS architecture
pub struct GamePrototypeEcs {
    title: String,
    player_entity: Option<Entity>,
    coins: Vec<Entity>,
    paused: bool,
    score: i32,
    lives: i32,
}

impl GamePrototypeEcs {
    pub fn new() -> Self {
        Self {
            title: "Lumina Game Prototype - ECS Architecture".to_string(),
            player_entity: None,
            coins: Vec::new(),
            paused: false,
            score: 0,
            lives: 3,
        }
    }

    fn create_player(&mut self, world: &mut World) -> Entity {
        let entity = world.spawn();
        
        // Add transform component
        world.add_component(entity, Transform {
            position: Vec3::new(100.0, 400.0, 0.0),
            rotation: 0.0,
            scale: Vec3::ONE,
        });
        
        // Add player component
        world.add_component(entity, PlayerComponent {
            speed: 200.0,
            jump_force: 300.0,
            grounded: false,
        });
        
        // Add velocity component
        world.add_component(entity, Velocity { x: 0.0, y: 0.0 });
        
        // Add sprite component
        world.add_component(entity, Sprite {
            texture_id: "player.png".to_string(),
            color: [0.2, 0.6, 1.0, 1.0],
            size: Vec2::new(32.0, 32.0),
        });
        
        self.player_entity = Some(entity);
        println!("🎮 Created player at position (100, 400)");
        entity
    }

    fn create_coin(&mut self, world: &mut World, position: Vec3) -> Entity {
        let entity = world.spawn();
        
        // Add transform component
        world.add_component(entity, Transform {
            position,
            rotation: 0.0,
            scale: Vec3::ONE,
        });
        
        // Add coin component
        world.add_component(entity, Coin {
            value: 10,
            collected: false,
        });
        
        // Add sprite component
        world.add_component(entity, Sprite {
            texture_id: "coin.png".to_string(),
            color: [1.0, 0.8, 0.2, 1.0],
            size: Vec2::new(24.0, 24.0),
        });
        
        self.coins.push(entity);
        println!("💰 Created coin at position ({}, {})", position.x, position.y);
        entity
    }

    fn setup_ui(&self, ui_framework: &mut UiFramework) {
        let mut ui_builder = UiBuilder::dark();
        
        // Game title
        let title = ui_builder.text("🎮 ECS Game Prototype")
            .size(24.0)
            .color(Color::hex("#00D9FF").unwrap())
            .build();
        ui_framework.add_root_widget(title);
        
        // Game status
        let status = ui_builder.text(&format!("Score: {} | Lives: {} | Status: {}", 
            self.score, 
            self.lives, 
            if self.paused { "Paused" } else { "Playing" }))
            .size(16.0)
            .color(Color::WHITE)
            .build();
        ui_framework.add_root_widget(status);
        
        // Control buttons
        let pause_button = ui_builder.button("⏸️ Pause")
            .style(ButtonStyle::Warning)
            .on_click(|| println!("Pause clicked - would pause game"))
            .build();
        ui_framework.add_root_widget(pause_button);
        
        let reset_button = ui_builder.button("🔄 Reset")
            .style(ButtonStyle::Danger)
            .on_click(|| println!("Reset clicked - would reset game"))
            .build();
        ui_framework.add_root_widget(reset_button);
        
        // Instructions
        let instructions = ui_builder.text("WASD: Move | Space: Jump | P: Pause | R: Reset")
            .size(14.0)
            .color(Color::hex("#AAAAAA").unwrap())
            .build();
        ui_framework.add_root_widget(instructions);
    }

    fn update_game_logic(&mut self, world: &mut World) {
        if self.paused {
            return;
        }

        // Simple physics update
        for entity in world.iter_entities() {
            world.with_component_mut::<Transform>(entity, |transform_opt| {
                if let Some(transform) = transform_opt {
                    world.with_component_mut::<Velocity>(entity, |velocity_opt| {
                        if let Some(velocity) = velocity_opt {
                            // Apply gravity
                            velocity.y -= 500.0 * 0.016; // Assuming 60fps
                            
                            // Update position
                            transform.position.x += velocity.x * 0.016;
                            transform.position.y += velocity.y * 0.016;
                            
                            // Ground collision
                            if transform.position.y <= 0.0 {
                                transform.position.y = 0.0;
                                velocity.y = 0.0;
                                
                                // Set grounded for player
                                world.with_component_mut::<PlayerComponent>(entity, |player_opt| {
                                    if let Some(player) = player_opt {
                                        player.grounded = true;
                                    }
                                });
                            } else {
                                world.with_component_mut::<PlayerComponent>(entity, |player_opt| {
                                    if let Some(player) = player_opt {
                                        player.grounded = false;
                                    }
                                });
                            }
                            
                            // Friction
                            velocity.x *= 0.8;
                        }
                    });
                }
            });
        }

        // Coin collection check
        if let Some(player_entity) = self.player_entity {
            world.with_component::<Transform>(player_entity, |player_transform_opt| {
                if let Some(player_transform) = player_transform_opt {
                    for &coin_entity in &self.coins.clone() {
                        world.with_component::<Transform>(coin_entity, |coin_transform_opt| {
                            if let Some(coin_transform) = coin_transform_opt {
                                let distance = (coin_transform.position - player_transform.position).length();
                                if distance < 40.0 {
                                    world.with_component_mut::<Coin>(coin_entity, |coin_opt| {
                                        if let Some(coin) = coin_opt {
                                            if !coin.collected {
                                                coin.collected = true;
                                                self.score += coin.value;
                                                println!("💰 Coin collected! Score: {}", self.score);
                                            }
                                        }
                                    });
                                }
                            }
                        });
                    }
                }
            });
        }
    }
}

impl EcsApp for GamePrototypeEcs {
    fn setup(&mut self, world: &mut World) -> Result<()> {
        println!("🚀 Setting up ECS Game Prototype");
        
        // Create player
        self.create_player(world);
        
        // Create coins at various positions
        let coin_positions = vec![
            Vec3::new(300.0, 100.0, 0.0),
            Vec3::new(500.0, 150.0, 0.0),
            Vec3::new(700.0, 200.0, 0.0),
            Vec3::new(900.0, 100.0, 0.0),
        ];
        
        for pos in coin_positions {
            self.create_coin(world, pos);
        }
        
        // Add UI components
        let player_ui_entity = world.spawn_with(Player {
            name: "Hero".to_string(),
            health: 100,
            max_health: 100,
            level: 1,
        });
        
        let game_state_entity = world.spawn_with(GameState {
            score: self.score,
            lives: self.lives,
            paused: self.paused,
        });
        
        // Setup UI
        world.with_resource_mut::<UiFramework, _>(|mut ui_framework_opt| {
            if let Some(ui_framework) = ui_framework_opt.as_mut() {
                self.setup_ui(ui_framework);
            }
        });
        
        println!("   Player UI entity: {:?}", player_ui_entity);
        println!("   Game state entity: {:?}", game_state_entity);
        println!("   Coins created: {}", self.coins.len());
        println!();
        println!("Controls:");
        println!("  • WASD: Move player");
        println!("  • Space: Jump");
        println!("  • P: Pause/Resume");
        println!("  • R: Reset game");
        
        Ok(())
    }
    
    fn update(&mut self, world: &mut World) -> Result<()> {
        // Update game logic
        self.update_game_logic(world);
        
        // Update UI state based on game state
        world.with_resource_mut::<GameState, _>(|mut game_state_opt| {
            if let Some(game_state) = game_state_opt.as_mut() {
                game_state.score = self.score;
                game_state.lives = self.lives;
                game_state.paused = self.paused;
            }
        });
        
        Ok(())
    }
    
    fn handle_event(&mut self, world: &mut World, event: &WindowEvent) -> Result<bool> {
        match event {
            WindowEvent::KeyboardInput { 
                event: key_event,
                .. 
            } => {
                use winit::event::{ElementState, KeyEvent};
                use winit::keyboard::{KeyCode, PhysicalKey};
                
                if let KeyEvent { 
                    physical_key: PhysicalKey::Code(keycode), 
                    state: ElementState::Pressed,
                    .. 
                } = key_event {
                    match keycode {
                        KeyCode::KeyP => {
                            self.paused = !self.paused;
                            println!("Game {}", if self.paused { "paused" } else { "resumed" });
                            return Ok(true);
                        }
                        KeyCode::KeyR => {
                            // Reset game
                            self.score = 0;
                            self.lives = 3;
                            self.paused = false;
                            
                            // Reset player position
                            if let Some(player_entity) = self.player_entity {
                                world.with_component_mut::<Transform>(player_entity, |transform_opt| {
                                    if let Some(transform) = transform_opt {
                                        transform.position = Vec3::new(100.0, 400.0, 0.0);
                                    }
                                });
                                world.with_component_mut::<Velocity>(player_entity, |velocity_opt| {
                                    if let Some(velocity) = velocity_opt {
                                        velocity.x = 0.0;
                                        velocity.y = 0.0;
                                    }
                                });
                            }
                            
                            println!("Game reset!");
                            return Ok(true);
                        }
                        KeyCode::KeyA | KeyCode::KeyD | KeyCode::KeyW | KeyCode::KeyS | KeyCode::Space => {
                            if !self.paused && self.player_entity.is_some() {
                                self.handle_player_input(world, *keycode);
                            }
                            return Ok(true);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        
        Ok(false)
    }
    
    fn window_config(&self) -> WindowConfig {
        WindowConfig {
            title: self.title.clone(),
            size: LogicalSize::new(1000, 700),
            resizable: true,
        }
    }
    
    fn theme(&self) -> lumina_ui::Theme {
        lumina_ui::Theme::dark()
    }
}

impl GamePrototypeEcs {
    fn handle_player_input(&mut self, world: &mut World, keycode: winit::keyboard::KeyCode) {
        if let Some(player_entity) = self.player_entity {
            use winit::keyboard::KeyCode;
            
            world.with_component_mut::<Velocity>(player_entity, |velocity_opt| {
                if let Some(velocity) = velocity_opt {
                    world.with_component::<PlayerComponent>(player_entity, |player_opt| {
                        if let Some(player) = player_opt {
                            match keycode {
                                KeyCode::KeyA => {
                                    velocity.x = -player.speed;
                                    println!("🏃 Player moving left");
                                }
                                KeyCode::KeyD => {
                                    velocity.x = player.speed;
                                    println!("🏃 Player moving right");
                                }
                                KeyCode::Space => {
                                    if player.grounded {
                                        velocity.y = player.jump_force;
                                        println!("🦘 Player jumping!");
                                    }
                                }
                                _ => {}
                            }
                        }
                    });
                }
            });
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎮 Lumina Game Prototype - ECS Architecture Demo!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("This demonstrates the complete ECS-driven architecture:");
    println!("  • Game logic runs through ECS systems");
    println!("  • Rendering happens in lumina-core systems");
    println!("  • UI is managed through ECS resources");
    println!("  • Complete separation of concerns");
    println!();
    
    let app = GamePrototypeEcs::new();
    run_ecs_app(app).await
}