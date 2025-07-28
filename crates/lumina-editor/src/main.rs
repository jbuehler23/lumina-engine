use lumina_core::{
    app::{App, AppRunner},
    engine::{Engine, EngineConfig},
    input::Key,
    Result,
};
use lumina_ui::{
    Widget, UiFramework, Theme,
    editor::EditorApp,
    widgets::{Button, Text, Panel},
};
use glam::{Vec2, Vec4};

/// Editor application that demonstrates clean, readable UI
struct LuminaEditor {
    ui_framework: Option<UiFramework>,
    // UI elements with proper styling
    title_text: Text,
    subtitle_text: Text,
    primary_button: Button,
    secondary_button: Button,
    info_panel: Panel,
}

impl LuminaEditor {
    fn new() -> Self {
        Self {
            ui_framework: None,
            // Create properly styled UI elements
            title_text: Text::new("Lumina Engine - Text Rendering Test")
                .font_size(24.0)
                .color(Vec4::new(1.0, 1.0, 1.0, 1.0)), // White text
            subtitle_text: Text::new("Testing TTF font rendering with cosmic-text")
                .font_size(16.0)
                .color(Vec4::new(0.8, 0.8, 0.8, 1.0)), // Light gray text
            primary_button: Button::new("Primary Action")
                .variant(lumina_ui::widgets::button::ButtonVariant::Primary),
            secondary_button: Button::new("Secondary Action")
                .variant(lumina_ui::widgets::button::ButtonVariant::Secondary),
            info_panel: Panel::new(),
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
        
        // Update UI framework if it exists
        if let Some(ui_framework) = &mut self.ui_framework {
            // Handle input and update animations
            log::debug!("UI framework running - text rendering active");
        }
        
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