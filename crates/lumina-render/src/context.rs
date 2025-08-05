//! ECS RenderContext resource for the Lumina Engine
//! 
//! This module implements the RenderContext resource that holds all WGPU state
//! and integrates with the existing Renderer infrastructure.

use crate::{Renderer, RenderConfig, RenderResult};
use glam::Vec2;
use std::sync::Arc;
use winit::window::Window;

/// RenderContext resource that holds all WGPU state for ECS systems
/// 
/// This resource provides a single, accessible point for any rendering system
/// to get the necessary GPU handles without exposing WGPU complexity to developers.
/// It wraps the existing Renderer and extends it for ECS usage.
pub struct RenderContext {
    /// Core renderer instance
    pub renderer: Renderer,
    /// Window reference for resize handling
    pub window: Arc<Window>,
}

impl RenderContext {
    /// Create a new RenderContext with the given window and config
    pub async fn new(window: Arc<Window>, config: RenderConfig) -> RenderResult<Self> {
        let renderer = Renderer::new(config, window.clone()).await?;
        
        Ok(Self {
            renderer,
            window,
        })
    }
    
    /// Create a new RenderContext with default config
    pub async fn new_with_window(window: Arc<Window>) -> RenderResult<Self> {
        Self::new(window, RenderConfig::default()).await
    }
    
    /// Handle window resize by updating the renderer
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            let size = Vec2::new(new_size.width as f32, new_size.height as f32);
            self.renderer.resize(size);
        }
    }
    
    /// Get the current window size as Vec2
    pub fn window_size(&self) -> Vec2 {
        self.renderer.screen_size
    }
    
    /// Get the surface format
    pub fn surface_format(&self) -> Option<wgpu::TextureFormat> {
        self.renderer.surface_config.as_ref().map(|config| config.format)
    }
    
    /// Get the WGPU device
    pub fn device(&self) -> &wgpu::Device {
        &self.renderer.device
    }
    
    /// Get the WGPU queue
    pub fn queue(&self) -> &wgpu::Queue {
        &self.renderer.queue
    }
    
    /// Get the WGPU surface (if available)
    pub fn surface(&self) -> Option<&wgpu::Surface<'static>> {
        self.renderer.surface.as_ref()
    }
    
    /// Get the surface configuration (if available)
    pub fn surface_config(&self) -> Option<&wgpu::SurfaceConfiguration> {
        self.renderer.surface_config.as_ref()
    }
    
    /// Begin a new frame and return the surface texture
    pub fn begin_frame(&mut self) -> RenderResult<Option<wgpu::SurfaceTexture>> {
        self.renderer.begin_frame()
    }
    
    /// End the current frame and present the surface texture
    pub fn end_frame(&mut self, output: Option<wgpu::SurfaceTexture>) {
        self.renderer.end_frame(output);
    }
    
    /// Get access to the underlying renderer
    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }
    
    /// Get mutable access to the underlying renderer
    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
}