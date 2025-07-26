use lumina_core::{
    app::{App, AppRunner},
    engine::{Engine, EngineConfig},
    input::{Key, MouseButton},
    math::{Transform2D, Vec2},
    Result,
};
use lumina_ecs::{Component, EcsSystemRunner, World, make_system};

#[derive(Debug, Clone)]
struct Position(Vec2);
impl Component for Position {}

#[derive(Debug, Clone)]
struct Velocity(Vec2);
impl Component for Velocity {}

#[derive(Debug, Clone)]
struct Player {
    speed: f32,
}
impl Component for Player {}

struct BasicGameApp {
    ecs: Option<EcsSystemRunner>,
}

impl BasicGameApp {
    fn new() -> Self {
        Self { ecs: None }
    }
}

impl App for BasicGameApp {
    fn initialize(&mut self, engine: &mut Engine) -> Result<()> {
        println!("🎮 Initializing Basic Game");
        
        let mut ecs = EcsSystemRunner::new();
        
        let world = ecs.world().clone();
        world.spawn()
            .with(Position(Vec2::new(100.0, 100.0)))
            .with(Velocity(Vec2::ZERO))
            .with(Player { speed: 200.0 })
            .build(&world);
        
        ecs.add_system(make_system(player_movement_system));
        ecs.add_system(make_system(movement_system));
        ecs.add_system(make_system(debug_system));
        
        engine.add_system(ecs)?;
        
        println!("✅ Basic Game initialized successfully!");
        Ok(())
    }

    fn update(&mut self, engine: &mut Engine) -> Result<()> {
        let input = &engine.context().input;
        
        if input.is_key_just_pressed(&Key::Escape) {
            println!("👋 Goodbye!");
            engine.stop()?;
        }
        
        Ok(())
    }

    fn shutdown(&mut self, _engine: &mut Engine) -> Result<()> {
        println!("🔚 Basic Game shutdown");
        Ok(())
    }
}

fn player_movement_system(world: &World, context: &lumina_core::engine::SystemContext) -> Result<()> {
    let input = &context.input;
    let time = context.time.read();
    let dt = time.delta_seconds();
    
    for (entity, (player, velocity)) in world.query::<Player>()
        .zip(world.query_mut::<Velocity>().iter_mut())
    {
        let mut vel = Vec2::ZERO;
        
        if input.is_key_pressed(&Key::W) || input.is_key_pressed(&Key::ArrowUp) {
            vel.y -= 1.0;
        }
        if input.is_key_pressed(&Key::S) || input.is_key_pressed(&Key::ArrowDown) {
            vel.y += 1.0;
        }
        if input.is_key_pressed(&Key::A) || input.is_key_pressed(&Key::ArrowLeft) {
            vel.x -= 1.0;
        }
        if input.is_key_pressed(&Key::D) || input.is_key_pressed(&Key::ArrowRight) {
            vel.x += 1.0;
        }
        
        if vel.length() > 0.0 {
            vel = vel.normalize() * player.speed;
        }
        
        velocity.0 = vel;
    }
    
    Ok(())
}

fn movement_system(world: &World, context: &lumina_core::engine::SystemContext) -> Result<()> {
    let time = context.time.read();
    let dt = time.delta_seconds();
    
    for (entity, (position, velocity)) in world.query_mut::<Position>().iter_mut()
        .zip(world.query::<Velocity>())
    {
        position.0 += velocity.0 * dt;
    }
    
    Ok(())
}

fn debug_system(world: &World, context: &lumina_core::engine::SystemContext) -> Result<()> {
    let time = context.time.read();
    
    if time.frame_count() % 60 == 0 {
        for (entity, position) in world.query::<Position>() {
            if world.has_component::<Player>(entity) {
                println!("Player position: ({:.1}, {:.1})", position.0.x, position.0.y);
            }
        }
    }
    
    Ok(())
}

fn main() -> Result<()> {
    let config = EngineConfig {
        window_title: "Basic Game - Lumina Engine".to_string(),
        window_width: 800,
        window_height: 600,
        vsync: true,
        max_fps: Some(60),
        enable_audio: true,
        enable_physics: false,
        enable_scripting: false,
    };

    let app = BasicGameApp::new();
    let runner = AppRunner::with_config(app, config);
    
    println!("🚀 Starting Basic Game");
    println!("Controls: WASD or Arrow Keys to move, ESC to quit");
    
    runner.run()
}
