# 🌟 Lumina Engine

An AI-native game engine built in Rust, designed to empower creators to generate and modify games using natural language prompts.

## ✨ Features

Lumina Engine is being transformed into an AI-native platform with the following features:

### 🤖 AI-Native Core
-   **Text-to-Game:** Generate entire games from high-level text descriptions.
-   **AI-Assisted Asset Generation:** Create sprites, textures, and sounds using AI.
-   **Dynamic Logic Generation:** Define game mechanics and behaviors with natural language.

### 🚀 Core Engine
-   **High-Performance ECS**: Custom Entity Component System for optimal performance.
-   **Cross-Platform**: Runs on Windows, macOS, Linux, and Web (WASM).
-   **Memory Safe**: Built in Rust for guaranteed memory safety.

### 🛠️ Development Tools
-   **Interactive Editor:** A visual editor to view and refine the AI-generated game world.
-   **Real-time Feedback:** See your changes in the game as you make them.

## 📚 Documentation & Architecture

For a deep dive into the Lumina Engine's architecture, development roadmap, and detailed guides, please refer to the `docs/` directory:

-   **[AI-Native Plan](docs/AI_NATIVE_PLAN.md)**: The detailed plan for transforming Lumina into an AI-native engine.

### Crate-Specific Documentation

-   **[🎯 Complete AI Training Guide](docs/COMPLETE_AI_TRAINING_GUIDE.md)**: **START HERE** - Complete guide to training specialized AI models
-   **[AI-Native Implementation Guide](docs/AI_NATIVE_IMPLEMENTATION.md)**: Technical details of text-to-game generation
-   **[Self-Hosted AI Setup](docs/SELF_HOSTED_AI_SETUP.md)**: Detailed Ollama and local model setup
-   **[Lumina Editor README](crates/lumina-editor/README.md)**: Detailed status and plans for the native editor
-   **[Lumina UI README](crates/lumina-ui/README.md)**: Guide to the easy-to-use UI API

## 🚀 Getting Started

### Quick Start with AI Game Generation

Generate your first game with a simple text prompt:

```bash
# Set up your AI provider (OpenAI recommended)
export OPENAI_API_KEY="your-api-key-here"

# Run the AI game generator
cargo run --example ai_game_generator
```

Example prompts that work great:
- "Create a platformer where the player collects coins and avoids enemies"
- "Make a space shooter with asteroids and power-ups"  
- "Design a puzzle game where the player pushes blocks to solve levels"

### Traditional Development

You can still create games using the traditional ECS approach:

```bash
# Run example games
cargo run --example pong
cargo run --example basic_game
```

For detailed AI-native documentation, see [AI-Native Implementation Guide](docs/AI_NATIVE_IMPLEMENTATION.md).

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## 📄 License

Licensed under either of:
-   Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
-   MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
