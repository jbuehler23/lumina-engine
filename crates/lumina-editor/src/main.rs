use lumina_core::{
    app::{App, AppRunner},
    engine::{Engine, EngineConfig},
    input::Key,
    Result,
};
use lumina_ui::{
    Widget,
    editor::EditorApp,
    widgets::{Button, Text, Panel},
};

/// Editor application that demonstrates text rendering
struct LuminaEditor {
    editor_app: EditorApp,
    test_button: Button,
    test_text: Text,
    test_panel: Panel,
}

impl LuminaEditor {
    fn new() -> Self {
        Self {
            editor_app: EditorApp::new(),
            test_button: Button::new("Test Button"),
            test_text: Text::new("Hello, Lumina Engine!"),
            test_panel: Panel::new(),
        }
    }
}

impl App for LuminaEditor {
    fn initialize(&mut self, _engine: &mut Engine) -> Result<()> {
        let _ = env_logger::try_init();
        log::info!("🎨 Initializing Lumina Editor");
        
        log::info!("✅ Lumina Editor initialized successfully!");
        Ok(())
    }

    fn update(&mut self, engine: &mut Engine) -> Result<()> {
        let input = &engine.context().input;
        
        if input.is_key_just_pressed(&Key::Escape) {
            log::info!("👋 Goodbye from Lumina Editor!");
            engine.stop()?;
        }
        
        // Simple text rendering test 
        log::info!("Editor running - text rendering system ready for testing");
        log::info!("Button widget: {:?}", self.test_button.id());
        log::info!("Text widget: {:?}", self.test_text.id());
        log::info!("Panel widget: {:?}", self.test_panel.id());
        
        Ok(())
    }

    fn shutdown(&mut self, _engine: &mut Engine) -> Result<()> {
        log::info!("🔚 Lumina Editor shutdown");
        Ok(())
    }
}

fn main() -> Result<()> {
    let config = EngineConfig {
        window_title: "Lumina Editor - Text Rendering Test".to_string(),
        window_width: 1200,
        window_height: 800,
        vsync: true,
        max_fps: Some(60),
        enable_audio: false,
        enable_physics: false,
        enable_scripting: false,
    };

    let app = LuminaEditor::new();
    let runner = AppRunner::with_config(app, config);
    
    println!("🚀 Starting Lumina Editor");
    println!("This will test the TTF font rendering system");
    println!("ESC to quit");
    
    runner.run()
}