# Lumina Engine Examples

This directory contains examples demonstrating Lumina Engine's capabilities.

## Examples

### 🎮 Pong (`/examples/pong/`)
A complete Pong game demonstrating:
- ECS architecture with lumina-ecs
- Real-time input handling with lumina-input  
- 2D rendering with lumina-render
- Game loop and collision detection
- Entity-component patterns

**Run:** `cargo run --manifest-path examples/pong/Cargo.toml`

### 🏃 Working Platformer (`examples/working_platformer.rs`)
Physics simulation demonstrating:
- Scene Description Language (SDL) loading
- 2D physics system with gravity
- AABB collision detection and response
- Character controllers and ground detection
- Multiple entity types (player, coins, platforms)

**Run:** `cargo run --example working_platformer`

## Getting Started

1. **Basic ECS Game**: Start with the Pong example to understand ECS patterns
2. **Physics Integration**: Explore the Working Platformer for physics concepts
3. **Custom Games**: Use these examples as templates for your own games

## Next Steps

The Working Platformer example demonstrates the SDL → ECS → Physics pipeline but lacks player input. The next logical step is implementing input handling to create an interactive playable platformer demo.