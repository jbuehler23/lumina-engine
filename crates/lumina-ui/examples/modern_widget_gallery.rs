//! Modern Widget Gallery using proper layout containers
//!
//! This example demonstrates the new Flex and Grid layout containers
//! inspired by modern UI frameworks like Iced and CSS Flexbox.

use std::collections::HashMap;
use winit::{
    event::{Event, WindowEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
    dpi::PhysicalSize,
};
use lumina_ui::{
    UiFramework, Theme, Text, Button, Panel,
    row, column, grid, Flex, Grid, 
    Spacing, Padding, MainAxisAlignment, CrossAxisAlignment,
    FlexDirection,
};
use lumina_ui::widgets::button::{ButtonVariant, button, secondary_button, ghost_button, danger_button};
use lumina_render::UiRenderer;

struct ModernWidgetGalleryApp {
    ui_framework: UiFramework,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,  
    window: Window,
    last_action: String,
}

impl ModernWidgetGalleryApp {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();
        
        // Initialize WGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
            flags: wgpu::InstanceFlags::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
        });
        
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();
        
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();
        
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
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
        };
        surface.configure(&device, &config);
        
        // Create UI renderer and framework
        let ui_renderer = UiRenderer::new(&device, &queue, config.clone()).await.unwrap();
        let theme = Theme::dark();
        let mut ui_framework = UiFramework::new(theme);
        ui_framework.set_renderer(ui_renderer);
        
        // Create modern UI layout
        Self::setup_modern_ui(&mut ui_framework);
        
        Self {
            ui_framework,
            surface,
            device,
            queue,
            config,
            size,
            window,
            last_action: "Welcome to Modern Widget Gallery!".to_string(),
        }
    }
    
    fn setup_modern_ui(ui_framework: &mut UiFramework) {
        // Create main layout - a column container
        let mut main_column = column()
            .spacing(Spacing {
                gap: 20.0,
                padding: Padding::uniform(24.0),
            })
            .main_axis_alignment(MainAxisAlignment::Start)
            .cross_axis_alignment(CrossAxisAlignment::Stretch);
        
        // Add main column as root widget
        ui_framework.add_root_widget(Box::new(main_column));
        
        // Header section - row with title and subtitle
        let mut header_row = row()
            .spacing(Spacing {
                gap: 16.0,
                padding: Padding::uniform(0.0),
            })
            .main_axis_alignment(MainAxisAlignment::SpaceBetween)
            .cross_axis_alignment(CrossAxisAlignment::Center);
        
        // Title and subtitle column
        let mut title_column = column()
            .spacing(Spacing {
                gap: 8.0,
                padding: Padding::uniform(0.0),
            });
        
        let title_text = Text::new("Modern Widget Gallery")
            .font_size(36.0)
            .color([0.95, 0.95, 0.95, 1.0].into());
        
        let subtitle_text = Text::new("Demonstrating modern layout containers and responsive design")
            .font_size(18.0)
            .color([0.75, 0.75, 0.75, 1.0].into());
        
        // Create version info
        let version_text = Text::new("v1.0 - Powered by glyphon")
            .font_size(14.0)
            .color([0.6, 0.6, 0.6, 1.0].into());
        
        ui_framework.add_root_widget(Box::new(title_text));
        ui_framework.add_root_widget(Box::new(subtitle_text));
        ui_framework.add_root_widget(Box::new(version_text));
        
        // Button showcase section
        let section_title = Text::new("Button Variants")
            .font_size(24.0)
            .color([0.9, 0.9, 0.9, 1.0].into());
        ui_framework.add_root_widget(Box::new(section_title));
        
        // Button grid - 2x2 layout
        let mut button_grid = grid(2)
            .spacing(Spacing {
                gap: 12.0,
                padding: Padding::uniform(16.0),
            });
        
        // Create buttons with actions
        let primary_btn = button("Primary Action")
            .variant(ButtonVariant::Primary)
            .on_click(|| println!("✅ Primary button clicked!"))
            .build();
        
        let secondary_btn = secondary_button("Secondary Action")
            .on_click(|| println!("⚙️ Secondary button clicked!"))
            .build();
        
        let ghost_btn = ghost_button("Subtle Action")
            .on_click(|| println!("👻 Ghost button clicked!"))
            .build();
        
        let danger_btn = danger_button("Destructive Action")
            .on_click(|| println!("⚠️ Danger button clicked!"))
            .build();
        
        ui_framework.add_root_widget(Box::new(primary_btn));
        ui_framework.add_root_widget(Box::new(secondary_btn));
        ui_framework.add_root_widget(Box::new(ghost_btn));
        ui_framework.add_root_widget(Box::new(danger_btn));
        
        // Interactive section
        let interactive_title = Text::new("Interactive Elements")
            .font_size(24.0)
            .color([0.9, 0.9, 0.9, 1.0].into());
        ui_framework.add_root_widget(Box::new(interactive_title));
        
        // Status display
        let status_text = Text::new("Status: Ready for interaction")
            .font_size(16.0)
            .color([0.4, 0.8, 0.4, 1.0].into());
        ui_framework.add_root_widget(Box::new(status_text));
        
        // Footer
        let footer_text = Text::new("Modern UI Layout • Responsive Design • Accessible")
            .font_size(14.0)
            .color([0.5, 0.5, 0.5, 1.0].into());
        ui_framework.add_root_widget(Box::new(footer_text));
    }
    
    fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Resized(physical_size) => {
                self.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.resize(**new_inner_size);
            }
            WindowEvent::CursorMoved { position, .. } => {
                // Handle cursor movement for hover effects
                if let Some(renderer) = &mut self.ui_framework.renderer {
                    // Update UI framework with cursor position
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                // Handle mouse clicks
                match (state, button) {
                    (ElementState::Pressed, winit::event::MouseButton::Left) => {
                        self.last_action = "Left mouse button pressed".to_string();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    
    fn update(&mut self) {
        // Update animations, states, etc.
        // For now, just mark as needing re-render
        self.ui_framework.mark_needs_render();
    }
    
    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            
            // Update UI framework size
            if let Some(renderer) = &mut self.ui_framework.renderer {
                renderer.resize([new_size.width as f32, new_size.height as f32].into());
            }
        }
    }
    
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
                            r: 0.08, // Darker modern background
                            g: 0.08,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // Render modern UI with proper layout
            self.ui_framework.render(&mut render_pass, &self.device, &self.queue);
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
}

fn main() {
    env_logger::init();
    
    let event_loop = EventLoop::new().unwrap();
    
    let window = winit::window::WindowBuilder::new()
        .with_title("Modern Lumina UI Widget Gallery")
        .with_inner_size(PhysicalSize::new(1200, 800))
        .with_min_inner_size(PhysicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();
    
    let mut app = pollster::block_on(ModernWidgetGalleryApp::new(window));
    
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == app.window.id() => {
                app.handle_window_event(event);
                
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::RedrawRequested => {
                        app.update();
                        match app.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => app.resize(app.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                    _ => {}
                }
            }
            Event::AboutToWait => {
                app.window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}