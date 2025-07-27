//! Lumina Render
//!
//! High-performance, cross-platform rendering system for the Lumina game engine.
//! Built on WGPU for modern graphics APIs (Vulkan, Metal, DirectX 12, WebGL2).
//!
//! This crate provides:
//! - Core rendering infrastructure and pipeline management
//! - UI rendering capabilities for immediate-mode interfaces
//! - Text rendering with font management
//! - Texture and resource management
//! - Cross-platform window and surface management
//!
//! # Architecture
//!
//! The rendering system is designed with modularity in mind:
//! - `Renderer`: Core rendering context and resource management
//! - `UiRenderer`: Specialized UI rendering with batching and clipping
//! - `TextRenderer`: Font-based text rendering
//! - `Pipeline`: Shader pipeline management
//! - `Buffer`: GPU buffer management and utilities
//!
//! # Usage
//!
//! ```rust,no_run
//! use lumina_render::{Renderer, RenderConfig};
//!
//! async fn setup_renderer() -> Result<Renderer, Box<dyn std::error::Error>> {
//!     let config = RenderConfig::default();
//!     let renderer = Renderer::new(config).await?;
//!     Ok(renderer)
//! }
//! ```

#![warn(missing_docs)]

pub mod renderer;
pub mod ui;
pub mod text;
pub mod pipeline;
pub mod buffer;
pub mod texture;
pub mod error;
pub mod window;

// Re-export commonly used types
pub use renderer::*;
pub use ui::{UiRenderer, UiVertex, UiUniforms, DrawCommand, FontHandle};
pub use text::*;
pub use pipeline::*;
pub use buffer::*;
pub use texture::{TextureManager, Texture};
pub use error::*;
pub use window::*;

// Re-export texture handle from texture module to avoid conflicts
pub use texture::TextureHandle;

// Re-export important external types
pub use wgpu;
pub use winit;
pub use glam::{Vec2, Vec3, Vec4, Mat4};

use serde::{Deserialize, Serialize};

/// Configuration for the rendering system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderConfig {
    /// Target framerate (0 = unlimited)
    pub target_fps: u32,
    /// Enable VSync
    pub vsync: bool,
    /// MSAA sample count (1 = disabled)
    pub msaa_samples: u32,
    /// Preferred graphics backend
    pub backend: BackendPreference,
    /// Window configuration
    pub window: WindowConfig,
}

/// Graphics backend preference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackendPreference {
    /// Prefer Vulkan (best performance)
    Vulkan,
    /// Prefer Metal (macOS/iOS)
    Metal,
    /// Prefer DirectX 12 (Windows)
    Dx12,
    /// Prefer WebGL2 (web)
    WebGl,
    /// Let WGPU choose the best backend
    Auto,
}

/// Window configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    /// Window title
    pub title: String,
    /// Initial window size
    pub size: (u32, u32),
    /// Whether window is resizable
    pub resizable: bool,
    /// Window decorations (title bar, etc.)
    pub decorations: bool,
    /// Fullscreen mode
    pub fullscreen: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            target_fps: 60,
            vsync: true,
            msaa_samples: 1,
            backend: BackendPreference::Auto,
            window: WindowConfig::default(),
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Lumina Application".to_string(),
            size: (1280, 720),
            resizable: true,
            decorations: true,
            fullscreen: false,
        }
    }
}

/// Rectangle representing screen coordinates or texture regions
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    /// Position of the rectangle
    pub position: Vec2,
    /// Size of the rectangle
    pub size: Vec2,
}

impl Rect {
    /// Create a new rectangle
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            size: Vec2::new(width, height),
        }
    }
    
    /// Check if a point is inside this rectangle
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.position.x
            && point.x <= self.position.x + self.size.x
            && point.y >= self.position.y
            && point.y <= self.position.y + self.size.y
    }
    
    /// Get the center point of the rectangle
    pub fn center(&self) -> Vec2 {
        self.position + self.size * 0.5
    }
    
    /// Get the minimum bounds (top-left corner)
    pub fn min(&self) -> Vec2 {
        self.position
    }
    
    /// Get the maximum bounds (bottom-right corner)
    pub fn max(&self) -> Vec2 {
        self.position + self.size
    }
    
    /// Create a rectangle from min/max coordinates
    pub fn from_min_max(min: Vec2, max: Vec2) -> Self {
        Self {
            position: min,
            size: max - min,
        }
    }
    
    /// Expand the rectangle by the given amount in all directions
    pub fn expand(&self, amount: f32) -> Self {
        Self {
            position: self.position - Vec2::splat(amount),
            size: self.size + Vec2::splat(amount * 2.0),
        }
    }
    
    /// Shrink the rectangle by the given amount in all directions
    pub fn shrink(&self, amount: f32) -> Self {
        self.expand(-amount)
    }
}
