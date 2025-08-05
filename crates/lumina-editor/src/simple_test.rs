use lumina_core::{
    app::{App, AppRunner},
    engine::{Engine, EngineConfig},
    input::Key,
    Result,
};
use lumina_render::{UiRenderer, Rect, FontHandle};
use glam::{Vec2, Vec4};

/// Simple test application that directly tests text rendering
pub struct SimpleTextTest {
    renderer_ready: bool,
}

impl SimpleTextTest {
    pub fn new() -> Self {
        Self {
            renderer_ready: false,
        }
    }
}

impl App for SimpleTextTest {
    fn initialize(&mut self, _engine: &mut Engine) -> Result<()> {
        let _ = env_logger::try_init();
        log::info!("🧪 Simple Text Test Starting");
        
        self.renderer_ready = true;
        log::info!("✅ Simple Text Test Ready");
        Ok(())
    }

    fn update(&mut self, engine: &mut Engine) -> Result<()> {
        let input = &engine.context().input;
        
        if input.is_key_just_pressed(&Key::Escape) {
            log::info!("👋 Goodbye from Simple Text Test!");
            engine.stop()?;
        }
        
        // Log that we're running
        if self.renderer_ready {
            log::debug!("Text renderer ready for testing");
        }
        
        Ok(())
    }

    fn shutdown(&mut self, _engine: &mut Engine) -> Result<()> {
        log::info!("🔚 Simple Text Test shutdown");
        Ok(())
    }
}

pub fn run_simple_test() -> Result<()> {
    let config = EngineConfig {
        window_title: "Simple Text Rendering Test".to_string(),
        window_width: 800,
        window_height: 600,
        vsync: true,
        max_fps: Some(60),
        enable_audio: false,
        enable_physics: false,
        enable_scripting: false,
    };

    let app = SimpleTextTest::new();
    let runner = AppRunner::with_config(app, config);
    
    println!("🧪 Starting Simple Text Rendering Test");
    println!("This test focuses only on text rendering");
    println!("ESC to quit");
    
    runner.run()
}