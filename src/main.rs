use lumina_core::{app::AppRunner, engine::EngineConfig, BasicApp};

fn main() -> lumina_core::Result<()> {
    let config = EngineConfig {
        window_title: "Lumina Engine Demo".to_string(),
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
