//! Game Prototype - Demonstrates a complete game created with visual scripting
//! Shows how visual scripts can control actual game entities and behavior

use lumina_ui::{
    UiFramework, UiRenderer, Theme, 
    widgets::{Button, Panel, Text, ButtonVariant},
    input::{InputHandler, RawInputEvent, KeyCode, MouseButton, Modifiers},
};
use lumina_scripting::visual_scripting::{
    VisualScript, VisualScriptExecutor, ScriptNode, NodeType, InputType, ScriptValue,
    create_player_movement_script, create_coin_collection_script,
};
use lumina_core::{Engine, App, Time};
use lumina_ecs::{World, Entity, Component};
use winit::{
    event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use glam::{Vec2, Vec3};
use std::collections::HashMap;

// Game Components
#[derive(Component, Debug, Clone)]
struct Transform {
    position: Vec3,
    rotation: f32,
    scale: Vec3,
}

#[derive(Component, Debug, Clone)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component, Debug, Clone)]
struct Player {
    speed: f32,
    jump_force: f32,
    grounded: bool,
}

#[derive(Component, Debug, Clone)]
struct Coin {
    value: i32,
    collected: bool,
}

#[derive(Component, Debug, Clone)]
struct ScriptComponent {
    script: VisualScript,
    variables: HashMap<String, ScriptValue>,
}

#[derive(Component, Debug, Clone)]
struct Sprite {
    texture_id: String,
    color: [f32; 4],
    size: Vec2,
}

// Game Entity
#[derive(Debug)]
struct GameObject {
    entity: Entity,
    transform: Transform,
    sprite: Option<Sprite>,
    script: Option<ScriptComponent>,
}

impl GameObject {
    fn new(entity: Entity, position: Vec3) -> Self {
        Self {
            entity,
            transform: Transform {
                position,
                rotation: 0.0,
                scale: Vec3::ONE,
            },
            sprite: None,
            script: None,
        }
    }
    
    fn with_sprite(mut self, texture_id: String, size: Vec2, color: [f32; 4]) -> Self {
        self.sprite = Some(Sprite {
            texture_id,
            color,
            size,
        });
        self
    }
    
    fn with_script(mut self, script: VisualScript) -> Self {
        self.script = Some(ScriptComponent {
            script,
            variables: HashMap::new(),
        });
        self
    }
}

// Game State
struct GameState {
    score: i32,
    lives: i32,
    level: i32,
    game_objects: HashMap<Entity, GameObject>,
    player_entity: Option<Entity>,
    coins: Vec<Entity>,
    script_executor: VisualScriptExecutor,
}

impl GameState {
    fn new() -> Self {
        Self {
            score: 0,
            lives: 3,
            level: 1,
            game_objects: HashMap::new(),
            player_entity: None,
            coins: Vec::new(),
            script_executor: VisualScriptExecutor::new(),
        }
    }
    
    fn create_player(&mut self, world: &mut World) -> Entity {
        let entity = world.spawn();
        let player_script = create_player_movement_script();
        
        let player_object = GameObject::new(entity, Vec3::new(100.0, 400.0, 0.0))
            .with_sprite("player.png".to_string(), Vec2::new(32.0, 32.0), [0.2, 0.6, 1.0, 1.0])
            .with_script(player_script);
        
        // Add components to ECS
        world.insert(entity, player_object.transform.clone());
        world.insert(entity, Player {
            speed: 200.0,
            jump_force: 300.0,
            grounded: false,
        });
        world.insert(entity, Velocity { x: 0.0, y: 0.0 });
        
        if let Some(sprite) = &player_object.sprite {
            world.insert(entity, sprite.clone());
        }
        
        if let Some(script) = &player_object.script {
            world.insert(entity, script.clone());
        }
        
        self.game_objects.insert(entity, player_object);
        self.player_entity = Some(entity);
        
        println!("🎮 Created player at position ({}, {})", 100.0, 400.0);
        entity
    }
    
    fn create_coin(&mut self, world: &mut World, position: Vec3) -> Entity {
        let entity = world.spawn();
        let coin_script = create_coin_collection_script();
        
        let coin_object = GameObject::new(entity, position)
            .with_sprite("coin.png".to_string(), Vec2::new(24.0, 24.0), [1.0, 0.8, 0.2, 1.0])
            .with_script(coin_script);
        
        // Add components to ECS
        world.insert(entity, coin_object.transform.clone());
        world.insert(entity, Coin {
            value: 10,
            collected: false,
        });
        
        if let Some(sprite) = &coin_object.sprite {
            world.insert(entity, sprite.clone());
        }
        
        if let Some(script) = &coin_object.script {
            world.insert(entity, script.clone());
        }
        
        self.game_objects.insert(entity, coin_object);
        self.coins.push(entity);
        
        println!("💰 Created coin at position ({}, {})", position.x, position.y);
        entity
    }
    
    fn update(&mut self, world: &mut World, time: &Time, input_keys: &[KeyCode]) {
        // Process visual scripts for all entities
        for (entity, game_object) in &self.game_objects {
            if let Some(script_component) = &game_object.script {
                // Execute visual script
                for node in &script_component.script.nodes {
                    match &node.node_type {
                        NodeType::OnInput(input_type) => {
                            match input_type {
                                InputType::KeyHeld(key) => {
                                    let key_pressed = match key.as_str() {
                                        "A" => input_keys.contains(&KeyCode::A),
                                        "D" => input_keys.contains(&KeyCode::D),
                                        "W" => input_keys.contains(&KeyCode::W),
                                        "Space" => input_keys.contains(&KeyCode::Space),
                                        _ => false,
                                    };
                                    
                                    if key_pressed {
                                        self.execute_movement_logic(*entity, world, key, time);
                                    }
                                }
                                _ => {}
                            }
                        }
                        NodeType::OnCollision(tag) => {
                            if tag == "Player" && self.player_entity.is_some() {
                                self.check_coin_collection(*entity, world);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        
        // Update physics
        self.update_physics(world, time);
    }
    
    fn execute_movement_logic(&mut self, entity: Entity, world: &mut World, key: &str, time: &Time) {
        if let Some(mut velocity) = world.get_component_mut::<Velocity>(entity) {
            if let Some(player) = world.get_component::<Player>(entity) {
                match key {
                    "A" => {
                        velocity.x = -player.speed;
                        println!("🏃 Player moving left");
                    }
                    "D" => {
                        velocity.x = player.speed;
                        println!("🏃 Player moving right");
                    }
                    "Space" => {
                        if player.grounded {
                            velocity.y = player.jump_force;
                            println!("🦘 Player jumping!");
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    
    fn check_coin_collection(&mut self, coin_entity: Entity, world: &mut World) {
        if let (Some(coin_transform), Some(mut coin)) = (
            world.get_component::<Transform>(coin_entity),
            world.get_component_mut::<Coin>(coin_entity)
        ) {
            if let Some(player_entity) = self.player_entity {
                if let Some(player_transform) = world.get_component::<Transform>(player_entity) {
                    let distance = (coin_transform.position - player_transform.position).length();
                    
                    if distance < 40.0 && !coin.collected {
                        coin.collected = true;
                        self.score += coin.value;
                        println!("💰 Coin collected! Score: {}", self.score);
                        
                        // Mark for removal
                        self.coins.retain(|&e| e != coin_entity);
                        self.game_objects.remove(&coin_entity);
                        world.despawn(coin_entity);
                    }
                }
            }
        }
    }
    
    fn update_physics(&mut self, world: &mut World, time: &Time) {
        // Apply gravity and movement
        for entity in world.entities() {
            if let (Some(mut transform), Some(mut velocity)) = (
                world.get_component_mut::<Transform>(entity),
                world.get_component_mut::<Velocity>(entity)
            ) {
                // Apply gravity
                velocity.y -= 500.0 * time.delta_seconds();
                
                // Update position
                transform.position.x += velocity.x * time.delta_seconds();
                transform.position.y += velocity.y * time.delta_seconds();
                
                // Ground collision (simple)
                if transform.position.y <= 0.0 {
                    transform.position.y = 0.0;
                    velocity.y = 0.0;
                    
                    if let Some(mut player) = world.get_component_mut::<Player>(entity) {
                        player.grounded = true;
                    }
                } else {
                    if let Some(mut player) = world.get_component_mut::<Player>(entity) {
                        player.grounded = false;
                    }
                }
                
                // Reset horizontal velocity (friction)
                velocity.x *= 0.8;
            }
        }
    }
    
    fn render_debug_info(&self) {
        println!("📊 Game State - Score: {}, Lives: {}, Level: {}", 
                self.score, self.lives, self.level);
        println!("   Entities: {}, Coins: {}", 
                self.game_objects.len(), self.coins.len());
    }
}

struct GamePrototype {
    ui_framework: UiFramework,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    
    // Game state
    world: World,
    game_state: GameState,
    engine_time: Time,
    pressed_keys: Vec<KeyCode>,
    
    // UI state
    show_debug: bool,
    paused: bool,
}

impl GamePrototype {
    async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();
        
        // Initialize WGPU (same setup as other examples)
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();
        
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        
        // Create UI framework
        let ui_renderer = UiRenderer::new(device.clone(), queue.clone(), config.clone()).await.unwrap();
        let theme = Theme::dark();
        let mut ui_framework = UiFramework::new(ui_renderer, theme);
        
        // Setup game UI
        Self::setup_game_ui(&mut ui_framework);
        
        // Initialize game world
        let mut world = World::new();
        let mut game_state = GameState::new();
        
        // Create game entities
        game_state.create_player(&mut world);
        
        // Create coins at various positions
        let coin_positions = vec![
            Vec3::new(300.0, 100.0, 0.0),
            Vec3::new(500.0, 150.0, 0.0),
            Vec3::new(700.0, 200.0, 0.0),
            Vec3::new(900.0, 100.0, 0.0),
        ];
        
        for pos in coin_positions {
            game_state.create_coin(&mut world, pos);
        }
        
        println!("🎮 Game Prototype Initialized!");
        println!("   Use WASD or Arrow Keys to move");
        println!("   Space to jump");
        println!("   Collect all coins to win!");
        
        Self {
            ui_framework,
            surface,
            device,
            queue,
            config,
            size,
            world,
            game_state,
            engine_time: Time::new(),
            pressed_keys: Vec::new(),
            show_debug: true,
            paused: false,
        }
    }
    
    fn setup_game_ui(ui_framework: &mut UiFramework) {
        // Game HUD
        let score_text = Text::new("Score: 0")
            .font_size(20.0)
            .color([1.0, 1.0, 1.0, 1.0].into());
        
        let lives_text = Text::new("Lives: 3")
            .font_size(20.0)
            .color([1.0, 1.0, 1.0, 1.0].into());
        
        let level_text = Text::new("Level: 1")
            .font_size(20.0)
            .color([1.0, 1.0, 1.0, 1.0].into());
        
        // Control panel
        let pause_btn = Button::new("⏸ Pause")
            .variant(ButtonVariant::Secondary)
            .on_click(|| println!("Game paused"));
        
        let reset_btn = Button::new("🔄 Reset")
            .variant(ButtonVariant::Danger)
            .on_click(|| println!("Game reset"));
        
        let debug_btn = Button::new("🐛 Debug")
            .variant(ButtonVariant::Ghost)
            .on_click(|| println!("Debug info toggled"));
        
        // Add widgets
        ui_framework.add_root_widget(Box::new(score_text));
        ui_framework.add_root_widget(Box::new(lives_text));
        ui_framework.add_root_widget(Box::new(level_text));
        ui_framework.add_root_widget(Box::new(pause_btn));
        ui_framework.add_root_widget(Box::new(reset_btn));
        ui_framework.add_root_widget(Box::new(debug_btn));
    }
    
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            
            self.ui_framework.renderer.resize([new_size.width as f32, new_size.height as f32].into());
        }
    }
    
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { 
                input: KeyboardInput { 
                    state, 
                    virtual_keycode: Some(keycode), 
                    .. 
                }, 
                .. 
            } => {
                let key = match keycode {
                    VirtualKeyCode::A => KeyCode::A,
                    VirtualKeyCode::D => KeyCode::D,
                    VirtualKeyCode::W => KeyCode::W,
                    VirtualKeyCode::S => KeyCode::S,
                    VirtualKeyCode::Space => KeyCode::Space,
                    VirtualKeyCode::P => {
                        if *state == ElementState::Pressed {
                            self.paused = !self.paused;
                            println!("Game {}", if self.paused { "paused" } else { "resumed" });
                        }
                        return true;
                    }
                    VirtualKeyCode::R => {
                        if *state == ElementState::Pressed {
                            // Reset game
                            self.game_state = GameState::new();
                            self.world = World::new();
                            self.game_state.create_player(&mut self.world);
                            println!("Game reset!");
                        }
                        return true;
                    }
                    VirtualKeyCode::F3 => {
                        if *state == ElementState::Pressed {
                            self.show_debug = !self.show_debug;
                        }
                        return true;
                    }
                    _ => return false,
                };
                
                match state {
                    ElementState::Pressed => {
                        if !self.pressed_keys.contains(&key) {
                            self.pressed_keys.push(key);
                        }
                    }
                    ElementState::Released => {
                        self.pressed_keys.retain(|&k| k != key);
                    }
                }
                true
            }
            _ => false,
        }
    }
    
    fn update(&mut self) {
        if !self.paused {
            self.engine_time.update();
            self.game_state.update(&mut self.world, &self.engine_time, &self.pressed_keys);
            
            if self.show_debug {
                self.game_state.render_debug_info();
            }
        }
        
        self.ui_framework.update_layout([self.size.width as f32, self.size.height as f32].into());
        self.ui_framework.input_handler.begin_frame();
    }
    
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.3,
                            b: 0.6,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }
        
        // TODO: Render game entities here
        // For now, we're just demonstrating the visual scripting system
        
        // Render UI
        self.ui_framework.render();
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    
    println!("🎮 Lumina Game Prototype - Visual Scripting in Action!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("This demonstrates how visual scripts control real game logic:");
    println!("  • Player movement scripts respond to keyboard input");
    println!("  • Coin collection scripts trigger on collision");
    println!("  • Physics and game state are updated by visual nodes");
    println!();
    println!("Controls:");
    println!("  • WASD: Move player");
    println!("  • Space: Jump");
    println!("  • P: Pause/Resume");
    println!("  • R: Reset game");
    println!("  • F3: Toggle debug info");
    println!();
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Lumina Game Prototype - Visual Scripting Powered Game")
        .with_inner_size(winit::dpi::LogicalSize::new(1000, 700))
        .build(&event_loop)
        .unwrap();
    
    let mut game = GamePrototype::new(&window).await;
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !game.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            game.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            game.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                game.update();
                match game.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => game.resize(game.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}