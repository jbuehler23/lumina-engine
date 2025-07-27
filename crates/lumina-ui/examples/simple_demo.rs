//! Simple demo showing Lumina UI core functionality
//! This demonstrates the widget system without graphics dependencies

use lumina_ui::{
    Button, Panel, Text, Widget,
    button::ButtonVariant,
    InputEvent, MouseButton,
    input::Modifiers,
};
use glam::Vec2;

fn main() {
    println!("🎮 Lumina UI Framework Core Demo");
    println!("=================================");
    println!();

    // Create some widgets to demonstrate the system
    println!("📦 Creating UI widgets...");
    
    // Create a panel (container)
    let panel = Panel::new();
    println!("  ✓ Created Panel widget");
    
    // Create some buttons
    let primary_btn = Button::new("Start Game")
        .variant(ButtonVariant::Primary);
    println!("  ✓ Created Primary Button: '{}'", primary_btn.get_text());
    
    let secondary_btn = Button::new("Settings")
        .variant(ButtonVariant::Secondary);
    println!("  ✓ Created Secondary Button: '{}'", secondary_btn.get_text());
    
    let danger_btn = Button::new("Exit")
        .variant(ButtonVariant::Danger);
    println!("  ✓ Created Danger Button: '{}'", danger_btn.get_text());
    
    // Create text widgets
    let title = Text::new("Lumina Game Engine")
        .font_size(24.0)
        .color(glam::Vec4::new(1.0, 1.0, 1.0, 1.0));
    println!("  ✓ Created Title Text");
    
    let subtitle = Text::new("Easy Game Creation Platform")
        .font_size(16.0)
        .color(glam::Vec4::new(0.8, 0.8, 0.8, 1.0));
    println!("  ✓ Created Subtitle Text");
    
    println!();
    
    // Demonstrate input handling
    println!("🎯 Testing input event handling...");
    
    // Create mutable button for testing
    let mut test_button = Button::new("Test Button")
        .variant(ButtonVariant::Primary);
    
    // Simulate mouse events
    let mouse_enter = InputEvent::MouseEnter;
    let mouse_click = InputEvent::MouseClick {
        button: MouseButton::Left,
        position: Vec2::new(100.0, 50.0),
        modifiers: Modifiers::default(),
    };
    let mouse_exit = InputEvent::MouseExit;
    
    // Test input responses
    println!("  📱 Mouse Enter Event:");
    let response = test_button.handle_input(&mouse_enter);
    println!("    Response: {:?}", response);
    println!("    Button state - Hovered: {}, Pressed: {}", 
             test_button.is_hovered(), test_button.is_pressed());
    
    println!("  📱 Mouse Click Event:");
    let response = test_button.handle_input(&mouse_click);
    println!("    Response: {:?}", response);
    
    println!("  📱 Mouse Exit Event:");
    let response = test_button.handle_input(&mouse_exit);
    println!("    Response: {:?}", response);
    println!("    Button state - Hovered: {}, Pressed: {}", 
             test_button.is_hovered(), test_button.is_pressed());
    
    println!();
    
    // Demonstrate layout system
    println!("📐 Testing layout system...");
    
    let available_space = Vec2::new(800.0, 600.0);
    println!("  Available space: {}x{}", available_space.x, available_space.y);
    
    // Test button layout
    let mut layout_button = Button::new("Layout Test");
    let layout_result = layout_button.layout(available_space);
    println!("  Button layout result:");
    println!("    Bounds: {:?}", layout_result.bounds);
    println!("    Content size: {:?}", layout_result.content_size);
    println!("    Overflow: {}", layout_result.overflow);
    
    println!();
    
    // Demonstrate widget customization
    println!("🎨 Testing widget customization...");
    
    let custom_button = Button::new("Custom Button")
        .variant(ButtonVariant::Ghost)
        .enabled(true);
    
    println!("  Custom button variant: {:?}", ButtonVariant::Ghost);
    println!("  Button text: '{}'", custom_button.get_text());
    
    // Test text customization
    let custom_text = Text::new("Customized Text")
        .font_size(18.0)
        .color(glam::Vec4::new(0.2, 0.8, 0.2, 1.0));
    
    println!("  Custom text created with green color");
    
    println!();
    
    // Show widget hierarchy concepts
    println!("🌳 Widget hierarchy concepts:");
    println!("  • Panel (root container)");
    println!("    ├── Title Text");
    println!("    ├── Subtitle Text");
    println!("    ├── Start Game Button (Primary)");
    println!("    ├── Settings Button (Secondary)");
    println!("    └── Exit Button (Danger)");
    
    println!();
    
    // Demonstrate visual scripting integration potential
    println!("🔧 Visual scripting integration points:");
    println!("  • Button onClick events can trigger script nodes");
    println!("  • Input events can be routed to visual script system");
    println!("  • UI state changes can update game logic");
    println!("  • Real-time property updates from script values");
    
    println!();
    
    println!("✅ Core UI framework demonstration completed!");
    println!();
    println!("🎯 Demonstrated features:");
    println!("  ✓ Widget creation and configuration");
    println!("  ✓ Input event handling and responses");
    println!("  ✓ Layout calculation system");
    println!("  ✓ Multiple widget types (Button, Panel, Text)");
    println!("  ✓ Widget state management");
    println!("  ✓ Customizable properties (colors, sizes, variants)");
    println!("  ✓ Event-driven architecture");
    println!();
    println!("🚀 Ready for integration with:");
    println!("  • Graphics rendering (WGPU)");
    println!("  • Visual scripting system");
    println!("  • Game engine core");
    println!("  • Web platform (WASM)");
    println!();
    println!("Next steps: Connect to renderer and test in browser!");
}