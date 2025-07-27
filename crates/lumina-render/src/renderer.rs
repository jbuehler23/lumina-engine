//! Core rendering system implementation
//!
//! This module provides the main `Renderer` struct that manages WGPU device,
//! surface, and core rendering resources.

use crate::{RenderConfig, RenderError, RenderResult, UiRenderer};
use glam::Vec2;
use std::sync::Arc;

/// Main rendering system
pub struct Renderer {
    /// WGPU instance
    pub instance: wgpu::Instance,
    /// WGPU adapter
    pub adapter: wgpu::Adapter,
    /// WGPU device
    pub device: wgpu::Device,
    /// WGPU queue
    pub queue: wgpu::Queue,
    /// Window surface
    pub surface: Option<wgpu::Surface<'static>>,
    /// Surface configuration
    pub surface_config: Option<wgpu::SurfaceConfiguration>,
    /// UI renderer
    pub ui_renderer: Option<UiRenderer>,
    /// Current screen size
    pub screen_size: Vec2,
}

impl Renderer {
    /// Create a new renderer with the given configuration
    pub async fn new(config: RenderConfig) -> RenderResult<Self> {
        // Create WGPU instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: Self::backends_from_preference(config.backend),
            ..Default::default()
        });

        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or(RenderError::AdapterNotFound)?;

        // Request device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Lumina Render Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;

        let screen_size = Vec2::new(config.window.size.0 as f32, config.window.size.1 as f32);

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
            surface: None,
            surface_config: None,
            ui_renderer: None,
            screen_size,
        })
    }

    /// Create a surface for rendering to a window
    pub fn create_surface(&mut self, window: Arc<winit::window::Window>) -> RenderResult<()> {
        let surface = self.instance.create_surface(window)?;
        
        let surface_caps = surface.get_capabilities(&self.adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: self.screen_size.x as u32,
            height: self.screen_size.y as u32,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&self.device, &config);

        self.surface = Some(surface);
        self.surface_config = Some(config);

        Ok(())
    }

    /// Initialize UI renderer
    pub async fn init_ui_renderer(&mut self) -> RenderResult<()> {
        if let Some(config) = &self.surface_config {
            let ui_renderer = UiRenderer::new(
                &self.device,
                &self.queue,
                config.clone(),
            ).await?;
            self.ui_renderer = Some(ui_renderer);
        }
        Ok(())
    }

    /// Resize the rendering surface
    pub fn resize(&mut self, new_size: Vec2) {
        self.screen_size = new_size;
        
        if let (Some(surface), Some(config)) = (&self.surface, &mut self.surface_config) {
            config.width = new_size.x as u32;
            config.height = new_size.y as u32;
            surface.configure(&self.device, config);
            
            if let Some(ui_renderer) = &mut self.ui_renderer {
                ui_renderer.resize(new_size);
            }
        }
    }

    /// Begin a new frame
    pub fn begin_frame(&mut self) -> RenderResult<Option<wgpu::SurfaceTexture>> {
        if let Some(surface) = &self.surface {
            let output = surface.get_current_texture().map_err(|e| {
                RenderError::InvalidOperation(format!("Failed to get surface texture: {}", e))
            })?;
            
            if let Some(ui_renderer) = &mut self.ui_renderer {
                ui_renderer.begin_frame(&self.queue);
            }
            
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }

    /// End the current frame
    pub fn end_frame(&mut self, output: Option<wgpu::SurfaceTexture>) {
        if let Some(ui_renderer) = &mut self.ui_renderer {
            ui_renderer.end_frame(&self.queue);
        }
        
        if let Some(output) = output {
            output.present();
        }
    }

    /// Get a reference to the UI renderer
    pub fn ui_renderer(&self) -> Option<&UiRenderer> {
        self.ui_renderer.as_ref()
    }

    /// Get a mutable reference to the UI renderer
    pub fn ui_renderer_mut(&mut self) -> Option<&mut UiRenderer> {
        self.ui_renderer.as_mut()
    }

    fn backends_from_preference(preference: crate::BackendPreference) -> wgpu::Backends {
        use crate::BackendPreference;
        match preference {
            BackendPreference::Vulkan => wgpu::Backends::VULKAN,
            BackendPreference::Metal => wgpu::Backends::METAL,
            BackendPreference::Dx12 => wgpu::Backends::DX12,
            BackendPreference::WebGl => wgpu::Backends::GL,
            BackendPreference::Auto => wgpu::Backends::all(),
        }
    }
}