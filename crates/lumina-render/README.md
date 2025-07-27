# Lumina Render

High-performance, cross-platform rendering system for the Lumina game engine, built on WGPU for modern graphics APIs (Vulkan, Metal, DirectX 12, WebGL2).

## Overview

Lumina Render provides the graphics foundation for the Lumina Engine, offering:

- **Cross-Platform**: Works seamlessly across desktop (Windows, macOS, Linux) and web (WebAssembly)
- **Modern Graphics**: Built on WGPU, supporting Vulkan, Metal, DirectX 12, and WebGL2
- **UI-First**: Optimized for immediate-mode UI rendering with batching and clipping
- **Type-Safe**: Full Rust type safety with comprehensive error handling
- **Performance**: Efficient rendering with minimal overhead

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│    Renderer     │    │   UiRenderer    │    │  TextRenderer   │
│                 │    │                 │    │                 │
│ • Core WGPU     │◄──►│ • UI Batching   │◄──►│ • Font Loading  │
│ • Surface Mgmt  │    │ • Clipping      │    │ • Glyph Cache   │
│ • Config        │    │ • Primitives    │    │ • Text Layout   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## Core Components

### Renderer
Main rendering context that manages WGPU device, surface, and core resources.

```rust
use lumina_render::{Renderer, RenderConfig};

async fn setup_renderer() -> Result<Renderer, Box<dyn std::error::Error>> {
    let config = RenderConfig::default();
    let renderer = Renderer::new(config).await?;
    Ok(renderer)
}
```

### UiRenderer
Specialized renderer for immediate-mode UI interfaces with efficient batching.

```rust
use lumina_render::{UiRenderer, Rect};
use glam::{Vec2, Vec4};

// Draw UI elements
renderer.draw_rect(Rect::new(10.0, 10.0, 100.0, 50.0), Vec4::new(1.0, 0.0, 0.0, 1.0));
renderer.draw_text("Hello World", Vec2::new(20.0, 25.0), font_handle, 16.0, Vec4::ONE);
```

### TextRenderer
Font management and text rendering with glyph caching.

```rust
use lumina_render::{TextRenderer, FontHandle};

let mut text_renderer = TextRenderer::new(&device, &queue, surface_format)?;
let font = text_renderer.load_default_font()?;
```

## Features

### Graphics Pipelines
- **Solid Color Pipeline**: Efficient colored rectangle rendering
- **Texture Pipeline**: Textured quad rendering with blend modes
- **Text Pipeline**: Optimized text rendering with SDF fonts

### Buffer Management
- **Vertex Buffers**: Dynamic vertex data with efficient updates
- **Index Buffers**: Indexed rendering for reduced memory usage
- **Uniform Buffers**: Shader constants and matrices

### Resource Management
- **Texture Loading**: PNG, JPG support with automatic format conversion
- **Font Loading**: TTF font support with glyph atlasing
- **Memory Management**: Efficient GPU memory allocation

## Platform Support

| Platform | Graphics API | Status |
|----------|--------------|--------|
| Windows  | DirectX 12   | ✅ Supported |
| Windows  | Vulkan       | ✅ Supported |
| macOS    | Metal        | ✅ Supported |
| Linux    | Vulkan       | ✅ Supported |
| Web      | WebGL2       | ✅ Supported |

## Performance

- **Batched Rendering**: UI elements are batched for minimal draw calls
- **Clipping Optimization**: Hierarchical clipping reduces overdraw
- **Memory Efficiency**: Smart buffer reuse and pooling
- **60+ FPS**: Optimized for smooth real-time rendering

## Integration with Lumina Engine

Lumina Render seamlessly integrates with other Lumina Engine components:

- **lumina-ui**: Provides rendering backend for UI framework
- **lumina-core**: Uses core math and utility types
- **lumina-ecs**: Renders ECS component data
- **lumina-editor**: Powers the visual editor interface

## Configuration

```rust
use lumina_render::{RenderConfig, BackendPreference, WindowConfig};

let config = RenderConfig {
    target_fps: 60,
    vsync: true,
    msaa_samples: 4,
    backend: BackendPreference::Auto,
    window: WindowConfig {
        title: "My Game".to_string(),
        size: (1280, 720),
        resizable: true,
        decorations: true,
        fullscreen: false,
    },
};
```

## Error Handling

Comprehensive error types with detailed context:

```rust
use lumina_render::{RenderError, RenderResult};

match renderer.init_ui_renderer().await {
    Ok(_) => println!("UI renderer initialized"),
    Err(RenderError::GraphicsInit(msg)) => eprintln!("Graphics init failed: {}", msg),
    Err(RenderError::AdapterNotFound) => eprintln!("No suitable graphics adapter found"),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Development Status

✅ **Core Rendering**: Complete WGPU integration with shader pipelines
✅ **UI Rendering**: Full immediate-mode UI rendering with batching and clipping
✅ **Text Rendering**: Complete TTF font loading with fontdue and glyph atlas system
✅ **Font Management**: Asset-based font loading with Inter font support
✅ **Buffer Management**: Optimized vertex/index buffers with 100K+ capacity
✅ **WGPU Integration**: Proper pipeline validation and surface configuration
🚧 **3D Rendering**: Planned for future versions  
🚧 **Post-Processing**: Planned for advanced effects  

## Contributing

This crate is part of the larger Lumina Engine project. See the main project README for contribution guidelines.

## License

MIT OR Apache-2.0