//! Hello World UI Demo
//! 
//! The absolute simplest UI application possible with Lumina Engine.

use lumina_ui::{
    UiApplication, UiBuilder, Color, ButtonStyle,
    UiAppConfig, run_ui_app
};

struct HelloWorldApp;

impl UiApplication for HelloWorldApp {
    fn build_ui(&mut self, ui: &mut UiBuilder) {
        // Create a simple "Hello World" UI
        let _title = ui.text("👋 Hello, Lumina UI!")
            .size(32.0)
            .color(Color::WHITE)
            .build();
        
        let _subtitle = ui.text("Building UIs has never been easier!")
            .size(16.0)
            .color(Color::rgb(0.8, 0.8, 0.8))
            .build();
        
        let _click_me_button = ui.button("Click Me!")
            .style(ButtonStyle::Primary)
            .on_click(|| println!("🎉 Hello from Lumina Engine!"))
            .build();
        
        let _quit_button = ui.button("Quit")
            .style(ButtonStyle::Danger)
            .on_click(|| std::process::exit(0))
            .build();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = UiAppConfig {
        title: "Hello Lumina UI".to_string(),
        size: (600, 400),
        ..Default::default()
    };
    
    run_ui_app(HelloWorldApp, config)
}