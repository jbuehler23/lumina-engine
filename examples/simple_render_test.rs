use lumina_core::{ecs_app::{EcsAppRunner, EcsApp, WindowConfig}, Result, Renderable};
use lumina_ecs::World;
use lumina_render::RenderConfig;
use lumina_ui::Theme;
use glam::{Vec2, Vec4};
use winit::{event::WindowEvent, dpi::LogicalSize};

struct SimpleRenderTest;

impl EcsApp for SimpleRenderTest {
    fn window_config(&self) -> WindowConfig {
        WindowConfig {
            title: "Simple Render Test - Lumina Engine".to_string(),
            size: LogicalSize::new(400, 300),
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
        log::info!("🎮 Setting up simple render test");
        
        // Create a simple red rectangle in the center
        let red_rect = Renderable {
            position: Vec2::new(150.0, 100.0),
            size: Vec2::new(100.0, 100.0),
            color: Vec4::new(1.0, 0.0, 0.0, 1.0), // Red
        };
        
        let entity = world.spawn()
            .with(red_rect)
            .build(&world);
        
        log::info!("✅ Created red rectangle entity: {:?}", entity);
        
        // Verify it was added
        let has_component = world.has_component::<Renderable>(entity);
        log::info!("📦 Entity has Renderable component: {}", has_component);
        
        // Count total Renderable components
        let mut count = 0;
        for (_entity, _renderable) in world.query::<Renderable>() {
            count += 1;
        }
        log::info!("🔢 Total Renderable components in world: {}", count);
        
        Ok(())
    }

    fn update(&mut self, _world: &mut World) -> Result<()> {
        // No game logic needed for this test
        Ok(())
    }

    fn handle_event(&mut self, _world: &mut World, event: &WindowEvent) -> Result<bool> {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if let winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape) = event.physical_key {
                    if event.state == winit::event::ElementState::Pressed {
                        log::info!("🛑 Escape pressed - exiting");
                        return Ok(false); // Exit
                    }
                }
            }
            WindowEvent::CloseRequested => {
                log::info!("🛑 Window close requested");
                return Ok(false); // Exit
            }
            _ => {}
        }
        
        Ok(true) // Continue running
    }

    fn shutdown(&mut self, _world: &mut World) -> Result<()> {
        log::info!("🛑 Simple render test shutting down");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    log::info!("🎮 Starting Simple Render Test");
    log::info!("Instructions: Press ESC to exit");
    
    // Create and run test app
    let test_app = SimpleRenderTest;
    let runner = EcsAppRunner::new(test_app);
    
    runner.run().await
}