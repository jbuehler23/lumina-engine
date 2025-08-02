# Lumina UI: Easy API Guide

This document provides a guide to the Lumina UI Easy API, designed to make UI creation accessible to non-technical game developers. It offers a simple, declarative way to build user interfaces without requiring deep knowledge of the underlying rendering system.

## Quick Start

```rust
use lumina_ui::{UiBuilder, Color, ButtonStyle, Direction, EasyAlignment as Alignment};

// Create a new UI with dark theme
let mut ui = UiBuilder::dark();

// Add some text
ui.text("Welcome to My Game!")
    .size(24.0)
    .color(Color::WHITE)
    .name("title")
    .build();

// Add a button
ui.button("Start Game")
    .style(ButtonStyle::Primary)
    .name("start_button")
    .build();

// Get the UI framework for rendering
let framework = ui.build();
```

## Core Concepts

### 1. UiBuilder
The `UiBuilder` is your main interface for creating UIs. It provides methods for creating different types of widgets and layouts.

```rust
// Create builders with different themes
let ui_dark = UiBuilder::dark();
let ui_light = UiBuilder::light();

// Or create with a custom theme
let ui_custom = UiBuilder::new(my_theme);
```

### 2. Colors
The `Color` struct provides an easy way to specify colors:

```rust
// Predefined colors
Color::WHITE
Color::BLACK
Color::RED
Color::GREEN
Color::BLUE
Color::TRANSPARENT

// RGB colors (values 0.0 to 1.0)
Color::rgb(1.0, 0.5, 0.0) // Orange

// RGBA colors with transparency
Color::rgba(1.0, 1.0, 1.0, 0.5) // Semi-transparent white

// Hex colors
Color::hex("#FF5500").unwrap() // Orange
Color::hex("#FF5500AA").unwrap() // Orange with transparency
```

### 3. Widget Names
All widgets can be given names for easy reference later:

```rust
let button_id = ui.button("Click Me")
    .name("my_button")
    .build();

// Later, get the widget by name
if let Some(widget_id) = ui.get_widget("my_button") {
    // Do something with the widget
}
```

## Widget Types

### Text Widgets
```rust
ui.text("Hello World!")
    .size(18.0)
    .color(Color::WHITE)
    .name("greeting")
    .build();
```

### Buttons
```rust
// Primary button (main action)
ui.button("Start Game")
    .style(ButtonStyle::Primary)
    .build();

// Secondary button (less important action)
ui.button("Settings")
    .style(ButtonStyle::Secondary)
    .build();

// Danger button (destructive action)
ui.button("Delete Save")
    .style(ButtonStyle::Danger)
    .build();

// Ghost button (subtle action)
ui.button("Cancel")
    .style(ButtonStyle::Ghost)
    .build();
```

### Containers
```rust
ui.container()
    .background(Color::rgba(0.2, 0.2, 0.2, 0.8))
    .padding(16.0)
    .name("main_panel")
    .build();
```

## Layouts

### Row Layout (Horizontal)
```rust
let button1 = ui.button("One").build();
let button2 = ui.button("Two").build();
let button3 = ui.button("Three").build();

ui.row()
    .main_alignment(Alignment::Center)
    .gap(8.0)
    .child(button1)
    .child(button2)
    .child(button3)
    .build();
```

### Column Layout (Vertical)
```rust
let title = ui.text("Menu").size(24.0).build();
let start = ui.button("Start").build();
let quit = ui.button("Quit").build();

ui.column()
    .main_alignment(Alignment::Center)
    .cross_alignment(Alignment::Center)
    .gap(16.0)
    .child(title)
    .child(start)
    .child(quit)
    .build();
```

## Complete Example

Here's a complete example of creating a game menu:

```rust
use lumina_ui::{UiBuilder, Color, ButtonStyle, Alignment};

fn create_main_menu() -> UiFramework {
    let mut ui = UiBuilder::dark();
    
    // Create widgets
    let title = ui.text("🎮 My Awesome Game")
        .size(36.0)
        .color(Color::hex("#00D9FF").unwrap())
        .build();
    
    let subtitle = ui.text("The adventure begins...")
        .size(18.0)
        .color(Color::rgb(0.8, 0.8, 0.8))
        .build();
    
    let start_button = ui.button("🚀 Start Adventure")
        .style(ButtonStyle::Primary)
        .build();
    
    let load_button = ui.button("📁 Load Game")
        .style(ButtonStyle::Secondary)
        .build();
    
    let settings_button = ui.button("⚙️ Settings")
        .style(ButtonStyle::Secondary)
        .build();
    
    let quit_button = ui.button("❌ Quit")
        .style(ButtonStyle::Danger)
        .build();
    
    // Arrange in layout
    ui.column()
        .main_alignment(Alignment::Center)
        .cross_alignment(Alignment::Center)
        .gap(20.0)
        .padding(32.0)
        .child(title)
        .child(subtitle)
        .child(start_button)
        .child(load_button)
        .child(settings_button)
        .child(quit_button)
        .name("main_menu")
        .build();
    
    ui.build()
}
```

## Integration with Game Loop

```rust
// In your game initialization
let mut ui_framework = create_main_menu();

// In your game loop
ui_framework.update_layout(screen_size);
ui_framework.render(&mut render_pass, &device, &queue);
```

## Button Click Handling

While the easy API focuses on UI creation, button clicks are typically handled in your game's input system:

```rust
// Check if a specific button was clicked
if let Some(button_id) = ui.get_widget("start_button") {
    // Check if this button was clicked in your input handler
    if was_clicked(button_id) {
        start_game();
    }
}
```

## Design Philosophy

The Easy API is designed with these principles:

1. **Simplicity**: Common tasks should be simple
2. **Discoverability**: Method names should be self-explanatory
3. **Flexibility**: Advanced users can still access the full framework
4. **Game-First**: Optimized for game development workflows
5. **Non-Technical Friendly**: Accessible to artists and designers

## Next Steps

- Check out the `easy_ui_demo.rs` example for a working implementation
- Look at the `widget_gallery.rs` example for more advanced usage
- Explore the full API documentation for advanced features
- Contribute your own UI patterns and examples!
