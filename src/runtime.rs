use lumina_core::{app::AppRunner, engine::EngineConfig, BasicApp};
use std::env;

fn main() -> lumina_core::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: lumina-runtime <project-file>");
        eprintln!("Example: lumina-runtime game.lumina");
        return Ok(());
    }
    
    let project_file = &args[1];
    println!("🚀 Lumina Engine Runtime v0.1.0");
    println!("===============================");
    println!("Loading project: {}", project_file);
    
    let config = EngineConfig {
        window_title: format!("Lumina Game - {}", project_file),
        window_width: 1280,
        window_height: 720,
        vsync: true,
        max_fps: Some(60),
        enable_audio: true,
        enable_physics: true,
        enable_scripting: true,
    };
    
    let app = BasicApp::new();
    let runner = AppRunner::with_config(app, config);
    
    runner.run()
}