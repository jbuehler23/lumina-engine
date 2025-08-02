# Lumina Engine Examples

This directory contains examples demonstrating the proper use of Lumina Engine's modular architecture.

## 🏓 Pong Game (`pong/`)

**Recommended starting point** - A complete Pong game that demonstrates the modern ECS-based architecture:

```bash
cargo run -p pong
```

**Controls:**
- Player 1: W/S keys
- Player 2: Arrow keys
- ESC to quit

**Architecture highlights:**
- **lumina-ecs**: Entity-Component-System for game state management
- **lumina-input**: Bevy-inspired input handling with `ButtonInput<T>` resources
- **lumina-render**: WGPU-based rendering pipeline
- **lumina-ui**: Immediate mode UI framework
- **lumina-core**: `EcsAppRunner` for integrated systems management

## 🎮 Basic Game (`basic-game/`)

**Legacy example** - Uses the deprecated `Engine` and `App` architecture:

```bash
cargo run -p basic-game
```

⚠️ **Note**: This example uses deprecated APIs. Use the Pong example for new projects.

## 🏃 Platformer (`platformer/`)

**Placeholder** - Currently contains only a "Hello World" stub.

## Modern Architecture Overview

### Recommended Pattern (EcsAppRunner)

```rust
use lumina_core::{ecs_app::{EcsAppRunner, EcsApp}, Result};
use lumina_ecs::World;
use lumina_input::ButtonInput;

struct MyApp;

impl EcsApp for MyApp {
    fn setup(&mut self, world: &mut World) -> Result<()> {
        // Initialize your game entities and resources
        Ok(())
    }
    
    fn update(&mut self, world: &mut World) -> Result<()> {
        // Game logic here
        Ok(())
    }
    
    fn handle_event(&mut self, world: &mut World, event: &WindowEvent) -> Result<bool> {
        // Handle input events
        Ok(true) // Continue running
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = MyApp;
    let runner = EcsAppRunner::new(app);
    runner.run().await
}
```

### Key Components

1. **lumina-ecs**: Pure ECS implementation with `World`, `Entity`, `Component` abstractions
2. **lumina-input**: Bevy-style input handling with resource-based state management
3. **lumina-render**: WGPU-based rendering with `RenderContext` resource
4. **lumina-ui**: Immediate mode UI with `UiFramework` integration
5. **lumina-core**: `EcsAppRunner` ties everything together

### Benefits

- **Modular**: Each crate has a single responsibility
- **Testable**: ECS components and systems are easy to test in isolation  
- **Performant**: Bevy-inspired architecture with efficient resource management
- **Type-safe**: Strong typing throughout the input and rendering pipelines
- **Future-proof**: Modern async support with proper event handling

### Migration from Legacy

If you have code using the old `Engine`/`App` pattern:

1. Replace `App` trait with `EcsApp` trait
2. Replace `AppRunner` with `EcsAppRunner`
3. Use `lumina-input::ButtonInput<T>` instead of the old input system
4. Make main function `async` and use `.await` on `runner.run()`

See the Pong example for a complete working implementation.