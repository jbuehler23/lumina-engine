//! Simple application wrapper for easy UI development
//! 
//! This module provides a high-level API that hides all the WGPU complexity
//! and provides a simple way to create UI applications.

use crate::{UiBuilder, InputEvent, MouseButton, Modifiers};
use lumina_render::UiRenderer;
use winit::{
    event::{Event, WindowEvent, ElementState},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
    dpi::{LogicalSize, PhysicalSize},
};
use std::sync::Arc;
use glam::Vec2;

/// Configuration for creating a UI application
#[derive(Debug, Clone)]
pub struct UiAppConfig {
    /// Application window title
    pub title: String,
    /// Initial window size
    pub size: (u32, u32),
    /// Whether the window is resizable
    pub resizable: bool,
    /// Whether to show window decorations (title bar, etc.)
    pub decorations: bool,
}

impl Default for UiAppConfig {
    fn default() -> Self {
        Self {
            title: "Lumina UI Application".to_string(),
            size: (1000, 700),
            resizable: true,
            decorations: true,
        }
    }
}

/// Trait that UI applications must implement
pub trait UiApplication {
    /// Build the UI - called once during initialization
    fn build_ui(&mut self, ui: &mut UiBuilder);
    
    /// Update the application state - called every frame
    fn update(&mut self, ui: &mut UiBuilder) {
        // Default implementation does nothing
        let _ = ui;
    }
    
    /// Handle custom input events (optional)
    fn handle_input(&mut self, event: &InputEvent, ui: &mut UiBuilder) -> bool {
        // Default implementation does nothing and lets the UI handle it
        let _ = (event, ui);
        false
    }
}

/// Simple UI application that handles all the rendering complexity
pub struct UiApp<'a> {
    ui_builder: UiBuilder,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    window: Arc<Window>,
    mouse_position: Vec2,
}

impl<'a> UiApp<'a> {
    /// Create a new UI application with the given configuration
    pub async fn new(window: Arc<Window>) -> Result<Self, Box<dyn std::error::Error>> {
        let size = window.inner_size();
        
        // Initialize WGPU with sensible defaults
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        let surface = instance.create_surface(window.clone())?;
        
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or("Failed to find suitable adapter")?;
        
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;
        
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
        
        // Create UI renderer and framework
        let ui_renderer = UiRenderer::new(&device, &queue, config.clone()).await?;
        let mut ui_builder = UiBuilder::dark();
        ui_builder.framework_mut().set_renderer(ui_renderer);
        
        Ok(Self {
            ui_builder,
            surface,
            device,
            queue,
            config,
            size,
            window,
            mouse_position: Vec2::ZERO,
        })
    }
    
    /// Get mutable access to the UI builder (for advanced use)
    pub fn ui_builder_mut(&mut self) -> &mut UiBuilder {
        &mut self.ui_builder
    }
    
    /// Get immutable access to the UI builder
    pub fn ui_builder(&self) -> &UiBuilder {
        &self.ui_builder
    }
    
    /// Handle window resize
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            
            // Update UI layout
            self.ui_builder.update_layout(Vec2::new(new_size.width as f32, new_size.height as f32));
        }
    }
    
    /// Handle input events
    pub fn handle_input(&mut self, event: &WindowEvent, app: &mut dyn UiApplication) -> bool {
        // Convert winit events to UI events
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = Vec2::new(position.x as f32, position.y as f32);
                
                let ui_event = InputEvent::MouseMove {
                    position: self.mouse_position,
                    delta: Vec2::ZERO, // Could be improved with delta tracking
                };
                
                // Let the app handle it first, then the UI
                if !app.handle_input(&ui_event, &mut self.ui_builder) {
                    self.ui_builder.handle_input(ui_event);
                }
                true
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let mouse_button = match button {
                    winit::event::MouseButton::Left => MouseButton::Left,
                    winit::event::MouseButton::Right => MouseButton::Right,
                    winit::event::MouseButton::Middle => MouseButton::Middle,
                    _ => return false,
                };
                
                if let ElementState::Pressed = state {
                    let ui_event = InputEvent::MouseClick {
                        button: mouse_button,
                        position: self.mouse_position,
                        modifiers: Modifiers::default(),
                    };
                    
                    // Let the app handle it first, then the UI
                    if !app.handle_input(&ui_event, &mut self.ui_builder) {
                        self.ui_builder.handle_input(ui_event);
                    }
                }
                true
            }
            _ => false,
        }
    }
    
    /// Update the application
    pub fn update(&mut self, app: &mut dyn UiApplication) {
        // Let the app update itself
        app.update(&mut self.ui_builder);
        
        // Update UI layout
        self.ui_builder.update_layout(Vec2::new(self.size.width as f32, self.size.height as f32));
    }
    
    /// Render the application
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.08, // Dark theme background
                            g: 0.08,
                            b: 0.10,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Render the UI
            self.ui_builder.render(&mut render_pass, &self.device, &self.queue);
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
}

/// Simple function to run a UI application
/// This hides all the event loop complexity
pub fn run_ui_app<T>(mut app: T, config: UiAppConfig) -> Result<(), Box<dyn std::error::Error>>
where
    T: UiApplication + 'static,
{
    // Note: Logging can be initialized by the application if needed
    
    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title(&config.title)
            .with_inner_size(LogicalSize::new(config.size.0, config.size.1))
            .with_resizable(config.resizable)
            .with_decorations(config.decorations)
            .build(&event_loop)?
    );
    
    let mut ui_app = pollster::block_on(UiApp::new(window.clone()))?;
    
    // Build the initial UI
    app.build_ui(ui_app.ui_builder_mut());
    
    println!("🎮 {} Started!", config.title);
    println!("💡 UI created with Lumina Engine's easy API");
    
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !ui_app.handle_input(event, &mut app) {
                    match event {
                        WindowEvent::CloseRequested => {
                            println!("👋 Closing application...");
                            elwt.exit();
                        }
                        WindowEvent::Resized(physical_size) => {
                            ui_app.resize(*physical_size);
                        }
                        WindowEvent::RedrawRequested => {
                            ui_app.update(&mut app);
                            match ui_app.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost) => ui_app.resize(ui_app.size),
                                Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                                Err(e) => eprintln!("Render error: {:?}", e),
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    })?;
    
    Ok(())
}