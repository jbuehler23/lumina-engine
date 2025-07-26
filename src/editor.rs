use lumina_core::{app::AppRunner, engine::EngineConfig, BasicApp};

fn main() -> lumina_core::Result<()> {
    println!("🎮 Lumina Engine Editor v0.1.0");
    println!("=============================");
    
    let config = EngineConfig {
        window_title: "Lumina Engine Editor".to_string(),
        window_width: 1920,
        window_height: 1080,
        vsync: true,
        max_fps: Some(120),
        enable_audio: true,
        enable_physics: true,
        enable_scripting: true,
    };

    println!("Starting editor with configuration:");
    println!("  Window: {}x{}", config.window_width, config.window_height);
    println!("  VSync: {}", config.vsync);
    println!("  Max FPS: {:?}", config.max_fps);
    
    let app = BasicApp::new();
    let runner = AppRunner::with_config(app, config);
    
    runner.run()
}