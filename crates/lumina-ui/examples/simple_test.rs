//! Simple UI test for debugging text positioning and theming issues

use lumina_ui::{
    UiFramework, Theme, 
    Button, Panel, Text, 
    InputEvent, KeyCode, MouseButton, Modifiers,
};
use lumina_ui::widgets::button::ButtonVariant;
use lumina_render::UiRenderer;
use winit::{
    event::{Event, WindowEvent, ElementState},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use std::sync::Arc;
use glam::Vec2;

struct SimpleTestApp<'a> {
    ui_framework: UiFramework,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Arc<winit::window::Window>,
}

impl<'a> SimpleTestApp<'a> {
    async fn new(window: Arc<winit::window::Window>) -> Self {
        let size = window.inner_size();
        
        // Initialize WGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        let surface = instance.create_surface(window.clone()).unwrap();
        
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        
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
        let ui_renderer = UiRenderer::new(&device, &queue, config.clone()).await.unwrap();
        let theme = Theme::dark();
        let mut ui_framework = UiFramework::new(theme);
        ui_framework.set_renderer(ui_renderer);
        
        // Simple test: just a few buttons to test positioning and theming
        let test_text = Text::new("Test Text - Baseline Check")
            .font_size(24.0)
            .color([1.0, 1.0, 1.0, 1.0].into());
        ui_framework.add_root_widget(Box::new(test_text));
        
        let primary_btn = Button::new("Primary Button")
            .variant(ButtonVariant::Primary);
        ui_framework.add_root_widget(Box::new(primary_btn));
        
        let secondary_btn = Button::new("Secondary Button")
            .variant(ButtonVariant::Secondary);
        ui_framework.add_root_widget(Box::new(secondary_btn));
        
        // Test panel with explicit theme color
        let test_panel = Panel::new();
        ui_framework.add_root_widget(Box::new(test_panel));
        
        Self {
            ui_framework,
            surface,
            device,
            queue,
            config,
            size,
            window,
        }
    }
    
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            
            if let Some(renderer) = &mut self.ui_framework.renderer {
                renderer.resize([new_size.width as f32, new_size.height as f32].into());
            }
        }
    }
    
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { event: key_event, .. } => {
                if let winit::keyboard::PhysicalKey::Code(keycode) = key_event.physical_key {
                    if keycode == winit::keyboard::KeyCode::Escape && key_event.state == ElementState::Pressed {
                        return false; // Exit
                    }
                }
                true
            }
            _ => false,
        }
    }
    
    fn update(&mut self) {
        // Simple layout - just stack widgets vertically
        let available_space = Vec2::new(self.size.width as f32, self.size.height as f32);
        
        // Override the complex layout with a simple vertical stack
        self.ui_framework.state.layout_cache.clear();
        let root_widgets = self.ui_framework.state.root_widgets.clone();
        
        let mut y_offset = 50.0;
        let widget_height = 50.0;
        let padding = 20.0;
        
        for (index, &widget_id) in root_widgets.iter().enumerate() {
            let bounds = lumina_render::Rect::new(
                50.0,                    // x
                y_offset,               // y
                available_space.x - 100.0, // width
                widget_height           // height
            );
            
            let layout_result = lumina_ui::layout::LayoutResult {
                bounds,
                overflow: false,
                content_size: bounds.size,
            };
            
            self.ui_framework.state.layout_cache.insert(widget_id, layout_result);
            y_offset += widget_height + padding;
        }
        
        self.ui_framework.state.needs_render = true;
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
                            r: 0.11, // Dark theme background (same as theme.colors.background.primary)
                            g: 0.11,
                            b: 0.13,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.ui_framework.render(&mut render_pass, &self.queue);
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
}

fn main() {
    env_logger::init();
    
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new()
        .with_title("Lumina UI - Simple Test")
        .with_inner_size(winit::dpi::LogicalSize::new(600, 400))
        .build(&event_loop)
        .unwrap());
    
    let mut app = pollster::block_on(SimpleTestApp::new(window.clone()));
    
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == app.window.id() => {
                if !app.input(event) {
                    elwt.exit();
                    return;
                }
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(physical_size) => {
                        app.resize(*physical_size);
                    }
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
            _ => {}
        }
    }).unwrap();
}