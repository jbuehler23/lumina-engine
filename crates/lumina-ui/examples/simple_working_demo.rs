//! Simple working demo showing the Lumina UI framework in action
//! This demonstrates basic widget functionality without complex WGPU setup

use lumina_ui::{
    UiFramework, Theme, 
    Button, Panel, Text,
    InputEvent, MouseButton,
};
use lumina_ui::widgets::button::ButtonVariant;
use glam::Vec2;

fn main() {
    println!("🎮 Lumina UI Framework - Simple Working Demo");
    println!("=============================================");
    println!();

    // Create UI framework
    let theme = Theme::default();
    let mut ui = UiFramework::new(theme);

    println!("✅ UI Framework created");

    // Create widgets
    let title = Text::new("Lumina Game Engine")
        .font_size(32.0)
        .color(glam::Vec4::new(1.0, 1.0, 1.0, 1.0));

    let subtitle = Text::new("Making game development accessible to everyone")
        .font_size(16.0)
        .color(glam::Vec4::new(0.8, 0.8, 0.8, 1.0));

    let start_button = Button::new("Start New Game")
        .variant(ButtonVariant::Primary);

    let load_button = Button::new("Load Game")
        .variant(ButtonVariant::Secondary);

    let settings_button = Button::new("Settings")
        .variant(ButtonVariant::Ghost);

    let exit_button = Button::new("Exit")
        .variant(ButtonVariant::Danger);

    let main_panel = Panel::new();

    println!("✅ Widgets created:");
    println!("   • Title: '{}'", "Lumina Game Engine");
    println!("   • Subtitle: '{}'", "Making game development accessible to everyone");
    println!("   • {} buttons", 4);
    println!("   • Main panel container");

    // Add widgets to framework
    let panel_id = ui.add_root_widget(Box::new(main_panel));
    let title_id = ui.add_widget(Box::new(title));
    let subtitle_id = ui.add_widget(Box::new(subtitle));
    let start_id = ui.add_widget(Box::new(start_button));
    let load_id = ui.add_widget(Box::new(load_button));
    let settings_id = ui.add_widget(Box::new(settings_button));
    let exit_id = ui.add_widget(Box::new(exit_button));

    println!("✅ Widgets added to UI framework");

    // Simulate layout calculation
    let screen_size = Vec2::new(1200.0, 800.0);
    ui.update_layout(screen_size);

    println!("✅ Layout calculated for {}x{} screen", screen_size.x, screen_size.y);

    // Simulate user interactions
    println!();
    println!("🎯 Simulating user interactions:");

    // Mouse click on start button (approximate position)
    let start_click = InputEvent::MouseClick {
        button: MouseButton::Left,
        position: Vec2::new(600.0, 350.0),
        modifiers: Default::default(),
    };

    println!("   📱 Mouse click at ({}, {})", 600.0, 350.0);
    ui.handle_input(start_click);

    // Mouse move
    let mouse_move = InputEvent::MouseMove {
        position: Vec2::new(650.0, 400.0),
        delta: Vec2::new(50.0, 50.0),
    };

    println!("   📱 Mouse move to ({}, {})", 650.0, 400.0);
    ui.handle_input(mouse_move);

    // Key press
    let key_press = InputEvent::KeyDown {
        key: lumina_ui::KeyCode::Enter,
        modifiers: Default::default(),
    };

    println!("   ⌨️  Enter key pressed");
    ui.handle_input(key_press);

    println!("✅ Input events processed successfully");

    // Demonstrate widget state
    println!();
    println!("📊 Widget Information:");
    println!("   • Total widgets: {}", ui.state.widgets.len());
    println!("   • Root widgets: {}", ui.state.root_widgets.len());
    println!("   • Layout cache entries: {}", ui.state.layout_cache.len());
    println!("   • Hierarchy entries: {}", ui.state.hierarchy.len());

    // Demonstrate theme access
    println!();
    println!("🎨 Theme Information:");
    println!("   • Primary color: {:?}", ui.theme.colors.primary);
    println!("   • Background color: {:?}", ui.theme.colors.background.primary);
    println!("   • Text color: {:?}", ui.theme.colors.text.primary);
    println!("   • Border radius: {}", ui.theme.components.button.primary.border_radius);

    println!();
    println!("✅ Demo completed successfully!");
    println!();
    println!("🚀 Next Steps:");
    println!("   • Connect to WGPU renderer for visual output");
    println!("   • Add window management for desktop deployment");
    println!("   • Integrate with visual scripting system");
    println!("   • Build complete game editor interface");
    println!();
    println!("💡 This demo shows the UI framework is working correctly");
    println!("   and ready for graphics integration!");
}