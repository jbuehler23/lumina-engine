//! Super Simple UI App Demo
//! 
//! This example shows how easy it is to create a UI application
//! with zero WGPU boilerplate code.

use lumina_ui::{
    UiApplication, UiBuilder, Color, ButtonStyle, EasyAlignment as Alignment,
    UiAppConfig, run_ui_app, InputEvent
};

/// Our simple UI application
struct MyGameMenu {
    counter: i32,
    last_action: String,
}

impl MyGameMenu {
    fn new() -> Self {
        Self {
            counter: 0,
            last_action: "Welcome to the game!".to_string(),
        }
    }
}

impl UiApplication for MyGameMenu {
    fn build_ui(&mut self, ui: &mut UiBuilder) {
        // Create a simple game menu UI
        
        // Title
        let title = ui.text("🎮 My Awesome Game")
            .size(36.0)
            .color(Color::hex("#00D9FF").unwrap())
            .name("title")
            .build();
        
        // Subtitle  
        let subtitle = ui.text("The easiest game engine ever!")
            .size(18.0)
            .color(Color::rgb(0.8, 0.8, 0.8))
            .name("subtitle")
            .build();
        
        // Game buttons
        let start_button = ui.button("🚀 Start Adventure")
            .style(ButtonStyle::Primary)
            .name("start_button")
            .on_click({
                || println!("🎯 Starting new adventure!")
            })
            .build();
        
        let load_button = ui.button("📁 Load Game")
            .style(ButtonStyle::Secondary)
            .name("load_button")
            .on_click(|| println!("💾 Loading saved game..."))
            .build();
        
        let settings_button = ui.button("⚙️ Settings")
            .style(ButtonStyle::Secondary)
            .name("settings_button")
            .on_click(|| println!("🔧 Opening game settings..."))
            .build();
        
        // Counter demo
        let counter_text = ui.text(&format!("Score: {}", self.counter))
            .size(20.0)
            .color(Color::hex("#FFD700").unwrap())
            .name("counter")
            .build();
        
        let increment_button = ui.button("➕ Add Point")
            .style(ButtonStyle::Success)
            .name("increment")
            .on_click(|| println!("📈 Point added!"))
            .build();
        
        let reset_button = ui.button("🔄 Reset Score")
            .style(ButtonStyle::Warning)
            .name("reset")
            .on_click(|| println!("🔄 Score reset!"))
            .build();
        
        // Status text
        let status_text = ui.text(&self.last_action)
            .size(16.0)
            .color(Color::rgb(0.7, 0.7, 0.7))
            .name("status")
            .build();
        
        let quit_button = ui.button("❌ Quit Game")
            .style(ButtonStyle::Danger)
            .name("quit")
            .on_click(|| {
                println!("👋 Thanks for playing!");
                std::process::exit(0);
            })
            .build();
            
        // Arrange everything in a nice layout
        ui.column()
            .main_alignment(Alignment::Center)
            .cross_alignment(Alignment::Center)
            .gap(16.0)
            .padding(32.0)
            .child(title)
            .child(subtitle)
            .child(start_button)
            .child(load_button)
            .child(settings_button)
            .child(counter_text)
            .child(increment_button)
            .child(reset_button)
            .child(status_text)
            .child(quit_button)
            .name("main_layout")
            .build();
    }
    
    fn update(&mut self, ui: &mut UiBuilder) {
        // Update the counter display if it changed
        // In a real game, this is where you'd update game state
        
        // For demo purposes, let's just update our status occasionally
        // (In a real app, you'd have actual game logic here)
    }
    
    fn handle_input(&mut self, event: &InputEvent, ui: &mut UiBuilder) -> bool {
        // Handle custom input events
        // For example, keyboard shortcuts
        match event {
            InputEvent::MouseClick { .. } => {
                // We could check which widget was clicked and update state
                self.last_action = "Button clicked!".to_string();
                
                // Return false to let the UI system handle the click
                false
            }
            _ => false,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create our application
    let app = MyGameMenu::new();
    
    // Configure the window
    let config = UiAppConfig {
        title: "My Awesome Game - Main Menu".to_string(),
        size: (900, 600),
        resizable: true,
        decorations: true,
    };
    
    // Run the app - that's it! No WGPU code needed!
    run_ui_app(app, config)?;
    
    Ok(())
}