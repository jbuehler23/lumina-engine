# 🌟 Lumina Engine

A comprehensive, high-performance game engine built in Rust, designed to empower artists and developers to quickly create games using their own assets. Taking inspiration from Game Maker Studio, RPG Maker, and Godot's editor, Lumina Engine combines ease of use with the performance and reliability of Rust.

## ✨ Features

Lumina Engine provides a robust set of features for game development:

### 🚀 Core Engine
-   **High-Performance ECS**: Custom Entity Component System for optimal performance.
-   **Cross-Platform**: Runs on Windows, macOS, Linux, and Web (WASM).
-   **Memory Safe**: Built in Rust for guaranteed memory safety.
-   **Multithreaded**: Parallel system execution using Rayon.

### 🎨 Graphics & Rendering
-   **Modern Graphics API**: Built on wgpu for cross-platform GPU acceleration.
-   **2D & 3D Support**: Flexible rendering pipeline supporting both 2D and 3D games.
-   **Asset Pipeline**: Efficient loading and management of textures, meshes, and animations.
-   **Shader System**: Custom shader support with hot-reloading.

### 🎵 Audio System
-   **Spatial Audio**: 3D positional audio support.
-   **Multiple Formats**: Support for common audio formats (MP3, OGG, WAV, FLAC).
-   **Real-time Effects**: Audio processing and effects pipeline.

### ⚡ Physics Integration
-   **2D Physics**: Rapier2D integration for 2D games.
-   **3D Physics**: Rapier3D integration for 3D games.
-   **Collision Detection**: Efficient broad and narrow phase collision detection.

### 📝 Scripting Support
-   **Lua Integration**: Embedded Lua scripting for game logic.
-   **WebAssembly**: WASM support for performance-critical scripts.
-   **Hot Reloading**: Live script editing without restarts.

### 🛠️ Development Tools
-   **Visual Editor**: Comprehensive editor with multiple panels (Project, Scene, Properties, Console, Visual Scripting).
-   **UI Framework**: Custom immediate-mode UI with responsive layout system.
-   **Font Rendering**: TTF font support with fontdue integration and glyph caching.
-   **Asset Management**: Asset directory structure with font loading system.
-   **Visual Scripting**: Node-based programming with example scripts and drag-drop nodes.
-   **Debug Tools**: Comprehensive logging system with debug levels and timestamps.
-   **Web Editor**: Browser-based editor for accessibility (Work in Progress).

## 📚 Documentation & Architecture

For a deep dive into the Lumina Engine's architecture, development roadmap, and detailed guides, please refer to the `docs/` directory:

-   **[Architecture Overview](docs/ARCHITECTURE.md)**: Understand the UI-first design, modular crate structure, and ECS-driven rendering.
-   **[Development Roadmap](docs/ROADMAP.md)**: See the current status, future phases, and strategic priorities for the engine and editor.
-   **[Demo Guide](docs/DEMO_GUIDE.md)**: Learn how to run existing demos and understand key features.
-   **[Editor Implementation Plan](docs/IMPLEMENTATION_PLAN.md)**: Detailed steps for implementing interactive editor features.

### Crate-Specific Documentation

-   **[Lumina Editor README](crates/lumina-editor/README.md)**: Detailed status and plans for the native editor.
-   **[Lumina UI README](crates/lumina-ui/README.md)**: Guide to the easy-to-use UI API.

## 🚀 Quick Start

### Prerequisites
-   Rust 1.70+ (latest stable recommended)
-   Git

### Building the Engine

```bash
# Clone the repository
git clone https://github.com/yourusername/lumina-engine.git
cd lumina-engine

# Build the entire workspace
cargo build --release

# Run the basic demo
cargo run --release

# Run the editor
cargo run --bin lumina-editor --release

# Run a game project
cargo run --bin lumina-runtime --release -- path/to/game.lumina
```

### Running Examples

```bash
# Basic game example
cd examples/basic-game
cargo run --release

# Platformer example
cd examples/platformer
cargo run --release
```

## 🎮 Creating Your First Game

Here's a minimal example of a Lumina Engine game:

```rust
use lumina_core::{
    app::{App, AppRunner},
    engine::{Engine, EngineConfig},
    input::Key,
    math::Vec2,
    Result,
};
use lumina_ecs::{Component, EcsSystemRunner, World, make_system};

#[derive(Debug, Clone)]
struct Position(Vec2);
impl Component for Position {}

#[derive(Debug, Clone)]
struct Player;
impl Component for Player {}

struct MyGame;

impl App for MyGame {
    fn initialize(&mut self, engine: &mut Engine) -> Result<()> {
        let mut ecs = EcsSystemRunner::new();
        
        // Spawn a player entity
        let world = ecs.world();
        world.spawn()
            .with(Position(Vec2::new(400.0, 300.0)))
            .with(Player)
            .build(&world);
        
        // Add movement system
        ecs.add_system(make_system(|world: &World, context| {
            // Game logic here
            Ok(())
        }));
        
        engine.add_system(ecs)?;
        Ok(())
    }

    fn update(&mut self, engine: &mut Engine) -> Result<()> {
        if engine.context().input.is_key_just_pressed(&Key::Escape) {
            engine.stop()?;
        }
        Ok(())
    }

    fn shutdown(&mut self, _engine: &mut Engine) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    let config = EngineConfig {
        window_title: "My Game".to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    };

    AppRunner::with_config(MyGame, config).run()
}
```

## 📊 Performance

Lumina Engine is designed for high performance:

-   **Zero-cost abstractions**: Rust's zero-cost abstractions ensure minimal runtime overhead.
-   **Memory efficient**: Custom allocators and memory pools for optimal memory usage.
-   **Parallel execution**: Systems run in parallel where possible.
-   **Cache-friendly**: Data structures optimized for CPU cache performance.

### Benchmarks

| Feature | Performance |
|---------|-------------|
| Entity spawn/despawn | 1M+ entities/second |
| Component queries | Sub-microsecond iteration |
| Memory usage | <50MB base footprint |
| Startup time | <100ms cold start |

## 🌐 Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| Windows | ✅ Full | DirectX 12, Vulkan |
| macOS | ✅ Full | Metal, Vulkan (MoltenVK) |
| Linux | ✅ Full | Vulkan, OpenGL |
| Web (WASM) | ✅ Full | WebGL 2.0, WebGPU |
| iOS | 🚧 WIP | Metal |
| Android | 🚧 WIP | Vulkan, OpenGL ES |

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Install development dependencies
cargo install cargo-watch cargo-expand

# Run tests
cargo test --all

# Run with hot reloading during development
cargo watch -x "run --example basic-game"

# Check formatting and lints
cargo fmt --all
cargo clippy --all-targets --all-features
```

## 📄 License

Licensed under either of:
-   Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
-   MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Acknowledgments

-   [wgpu](https://github.com/gfx-rs/wgpu) - Modern graphics API
-   [winit](https://github.com/rust-windowing/winit) - Cross-platform windowing
-   [rapier](https://github.com/dimforge/rapier) - Physics simulation
-   [rodio](https://github.com/RustAudio/rodio) - Audio playback
-   [egui](https://github.com/emilk/egui) - Immediate mode GUI

---

**Lumina Engine** - *Illuminate your game development journey* ✨
