//! Minimal working demo of the Lumina UI framework
//! This example demonstrates the basic functionality without external dependencies

use lumina_ui::{
    UiFramework, Widget, WidgetId, InputEvent, MouseButton,
    Button, Panel, Text,
    button::ButtonVariant,
};
use glam::Vec2;

/// Simple demo application state
struct DemoApp {
    ui: UiManager,
    counter: i32,
    message: String,
}

impl DemoApp {
    fn new() -> Self {
        let mut ui = UiManager::new();
        
        // Create a simple UI layout
        let root_panel = Panel::new();
        let root_id = ui.add_widget(Box::new(root_panel));
        ui.set_root_widget(root_id);
        
        // Add a title
        let title = Text::new("Lumina UI Demo")
            .font_size(24.0)
            .color(glam::Vec4::new(1.0, 1.0, 1.0, 1.0));
        let title_id = ui.add_widget(Box::new(title));
        ui.add_child(root_id, title_id);
        
        // Add a counter display
        let counter_text = Text::new("Counter: 0");
        let counter_id = ui.add_widget(Box::new(counter_text));
        ui.add_child(root_id, counter_id);
        
        // Add increment button
        let increment_btn = Button::new("Increment")
            .variant(ButtonVariant::Primary);
        let increment_id = ui.add_widget(Box::new(increment_btn));
        ui.add_child(root_id, increment_id);
        
        // Add decrement button
        let decrement_btn = Button::new("Decrement")
            .variant(ButtonVariant::Secondary);
        let decrement_id = ui.add_widget(Box::new(decrement_btn));
        ui.add_child(root_id, decrement_id);
        
        Self {
            ui,
            counter: 0,
            message: "Welcome to Lumina UI!".to_string(),
        }
    }
    
    fn handle_input(&mut self, event: InputEvent) {
        match event {
            InputEvent::MouseClick { button: MouseButton::Left, position, .. } => {
                // Simple click handling - in a real app this would be more sophisticated
                println!("Mouse clicked at {:?}", position);
                
                // Simulate button clicks based on position
                if position.y > 100.0 && position.y < 140.0 {
                    if position.x > 50.0 && position.x < 150.0 {
                        // Increment button area
                        self.counter += 1;
                        self.message = format!("Counter incremented to {}", self.counter);
                        println!("{}", self.message);
                    } else if position.x > 200.0 && position.x < 300.0 {
                        // Decrement button area  
                        self.counter -= 1;
                        self.message = format!("Counter decremented to {}", self.counter);
                        println!("{}", self.message);
                    }
                }
            }
            _ => {}
        }
        
        // Pass the event to the UI system
        self.ui.handle_input(event);
    }
    
    fn update(&mut self) {
        // Update UI state
        self.ui.update();
        
        // Force a layout update
        self.ui.request_layout();
    }
    
    fn render(&mut self) {
        // In a real application, this would render to a graphics surface
        println!("=== Lumina UI Demo Frame ===");
        println!("Counter: {}", self.counter);
        println!("Message: {}", self.message);
        println!("UI Status: {} widgets", self.ui.widget_count());
        println!("============================");
        
        // Trigger UI rendering (this is a stub since we don't have a real renderer)
        self.ui.render();
    }
}

fn main() {
    println!("🎮 Lumina UI Framework Demo");
    println!("============================");
    println!();
    
    // Create the demo application
    let mut app = DemoApp::new();
    
    // Simulate some user interactions
    println!("📱 Simulating user interactions...");
    println!();
    
    // Initial render
    app.update();
    app.render();
    println!();
    
    // Simulate clicking the increment button
    app.handle_input(InputEvent::MouseClick {
        button: MouseButton::Left,
        position: Vec2::new(100.0, 120.0), // Increment button position
        modifiers: Default::default(),
    });
    app.update();
    app.render();
    println!();
    
    // Simulate clicking the increment button again
    app.handle_input(InputEvent::MouseClick {
        button: MouseButton::Left,
        position: Vec2::new(100.0, 120.0),
        modifiers: Default::default(),
    });
    app.update();
    app.render();
    println!();
    
    // Simulate clicking the decrement button
    app.handle_input(InputEvent::MouseClick {
        button: MouseButton::Left,
        position: Vec2::new(250.0, 120.0), // Decrement button position
        modifiers: Default::default(),
    });
    app.update();
    app.render();
    println!();
    
    println!("✅ Demo completed successfully!");
    println!();
    println!("🎯 This demonstrates:");
    println!("  • UI widget creation and management");
    println!("  • Input event handling");
    println!("  • Layout system integration");
    println!("  • Application state management");
    println!("  • Basic UI framework functionality");
    println!();
    println!("🚀 Ready for integration with graphics renderer!");
}