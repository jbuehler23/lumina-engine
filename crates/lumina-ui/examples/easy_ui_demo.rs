//! Easy UI API Demo
//! 
//! This example demonstrates the new easy-to-use declarative API
//! designed to make UI creation accessible to non-technical users.

use lumina_ui::{UiBuilder, Color, ButtonStyle, EasyAlignment as Alignment, InputEvent, MouseButton, Modifiers};
use lumina_render::UiRenderer;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use std::sync::Arc;

struct EasyUiDemoApp<'a> {
    ui_builder: UiBuilder,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Arc<winit::window::Window>,
    mouse_position: glam::Vec2,
}

impl<'a> EasyUiDemoApp<'a> {
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
        
        // Create UI renderer and builder
        let ui_renderer = UiRenderer::new(&device, &queue, config.clone()).await.unwrap();
        let mut ui_builder = UiBuilder::dark();
        ui_builder.framework_mut().set_renderer(ui_renderer);
        
        // Build the UI using the easy API
        Self::build_demo_ui(&mut ui_builder);
        
        Self {
            ui_builder,
            surface,
            device,
            queue,
            config,
            size,
            window,
            mouse_position: glam::Vec2::ZERO,
        }
    }
    
    fn build_demo_ui(ui: &mut UiBuilder) {
        // Create a welcome screen with multiple sections
        
        // Header section
        let title = ui.text("🎮 Lumina Engine - Easy UI Demo")
            .size(36.0)
            .color(Color::hex("#00D9FF").unwrap()) // Bright cyan
            .name("main_title")
            .build();
        
        let subtitle = ui.text("Demonstrating the easy-to-use UI API")
            .size(18.0)
            .color(Color::rgb(0.8, 0.8, 0.8))
            .name("subtitle")
            .build();
        
        // Button section with interactive callbacks
        let play_button = ui.button("Start Playing")
            .style(ButtonStyle::Primary)
            .name("play_button")
            .on_click(|| println!("🚀 Starting the game! Loading assets..."))
            .build();
        
        let create_button = ui.button("Create Game")
            .style(ButtonStyle::Success)
            .name("create_button")
            .on_click(|| println!("🛠️ Opening game creation tools..."))
            .build();
        
        let settings_button = ui.button("Settings")
            .style(ButtonStyle::Secondary)
            .name("settings_button")
            .on_click(|| println!("⚙️ Opening settings menu..."))
            .build();
        
        let help_button = ui.button("Help & Tutorials")
            .style(ButtonStyle::Ghost)
            .name("help_button")
            .on_click(|| println!("❓ Opening help and tutorials..."))
            .build();
        
        let exit_button = ui.button("Exit")
            .style(ButtonStyle::Danger)
            .name("exit_button")
            .on_click(|| {
                println!("❌ Exiting application...");
                std::process::exit(0);
            })
            .build();
        
        // Info section
        let info_text = ui.text("This UI was created with just a few lines of code!")
            .size(16.0)
            .color(Color::hex("#FFD700").unwrap()) // Gold
            .name("info_text")
            .build();
        
        let features_text = ui.text("✨ Features: Easy API • Dark Theme • Modern Layouts • Responsive Design")
            .size(14.0)
            .color(Color::rgb(0.7, 0.7, 0.7))
            .name("features_text")
            .build();
        
        // Status section
        let status_text = ui.text("🟢 System Status: All systems operational")
            .size(14.0)
            .color(Color::GREEN)
            .name("status_text")
            .build();
        
        // Version info
        let version_text = ui.text("v1.0.0 - Built with Lumina Engine & glyphon")
            .size(12.0)
            .color(Color::rgb(0.5, 0.5, 0.5))
            .name("version_text")
            .build();
        
        // Create button row
        let button_row = ui.row()
            .main_alignment(Alignment::Center)
            .cross_alignment(Alignment::Center)
            .gap(12.0)
            .child(play_button)
            .child(create_button)
            .child(settings_button)
            .name("button_row")
            .build();
        
        // Create secondary button row
        let secondary_row = ui.row()
            .main_alignment(Alignment::Center)
            .cross_alignment(Alignment::Center)
            .gap(12.0)
            .child(help_button)
            .child(exit_button)
            .name("secondary_row")
            .build();
        
        // Main layout - arrange everything in a column
        ui.column()
            .main_alignment(Alignment::Center)
            .cross_alignment(Alignment::Center)
            .gap(24.0)
            .padding(32.0)
            .child(title)
            .child(subtitle)
            .child(button_row)
            .child(secondary_row)
            .child(info_text)
            .child(features_text)
            .child(status_text)
            .child(version_text)
            .name("main_layout")
            .build();
    }
    
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            
            // Update UI layout
            self.ui_builder.update_layout([new_size.width as f32, new_size.height as f32].into());
        }
    }
    
    fn update(&mut self) {
        // Update UI layout
        self.ui_builder.update_layout([self.size.width as f32, self.size.height as f32].into());
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
                            r: 0.08, // Dark modern background
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

            // Render the easy UI
            self.ui_builder.render(&mut render_pass, &self.device, &self.queue);
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
    
    fn handle_input(&mut self, event: &WindowEvent) -> bool {
        // Convert winit events to UI events and handle them
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                // Update tracked mouse position
                self.mouse_position = glam::Vec2::new(position.x as f32, position.y as f32);
                
                let ui_event = InputEvent::MouseMove {
                    position: self.mouse_position,
                    delta: [0.0, 0.0].into(), // We could track this for more advanced interactions
                };
                self.ui_builder.handle_input(ui_event);
                true
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let mouse_button = match button {
                    winit::event::MouseButton::Left => MouseButton::Left,
                    winit::event::MouseButton::Right => MouseButton::Right,
                    winit::event::MouseButton::Middle => MouseButton::Middle,
                    _ => return false,
                };
                
                if let winit::event::ElementState::Pressed = state {
                    // Use the tracked mouse position for accurate click detection
                    let ui_event = InputEvent::MouseClick {
                        button: mouse_button,
                        position: self.mouse_position,
                        modifiers: Modifiers::default(),
                    };
                    self.ui_builder.handle_input(ui_event);
                }
                true
            }
            _ => false,
        }
    }
}

fn main() {
    env_logger::init();
    
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new()
        .with_title("Lumina UI - Easy API Demo")
        .with_inner_size(winit::dpi::LogicalSize::new(1000, 700))
        .build(&event_loop)
        .unwrap());
    
    let mut app = pollster::block_on(EasyUiDemoApp::new(window.clone()));
    
    println!("🎮 Easy UI Demo Started!");
    println!("💡 This demo shows how easy it is to create UIs with Lumina Engine");
    println!("✨ Try clicking the buttons (they'll print to console)");
    
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == app.window.id() => {
                if !app.handle_input(event) {
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
            }
            _ => {}
        }
    }).unwrap();
}