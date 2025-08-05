//! Widget Gallery Example
//! 
//! A comprehensive showcase of all Lumina UI widgets demonstrating:
//! - Proper text baseline alignment
//! - Dark theme integration
//! - Interactive widget behavior
//! - Layout organization

use lumina_ui::{
    UiFramework, Theme, 
    Button, Text, 
    InputEvent, KeyCode, MouseButton, Modifiers,
};
use lumina_ui::widgets::button::{ButtonVariant, button, secondary_button, ghost_button, danger_button};
use lumina_render::UiRenderer;
use winit::{
    event::{Event, WindowEvent, ElementState},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use std::sync::Arc;

struct WidgetGalleryApp<'a> {
    ui_framework: UiFramework,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Arc<winit::window::Window>,
    // Demo state
    counter: i32,
    last_action: String,
    show_debug: bool,
}

impl<'a> WidgetGalleryApp<'a> {
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
        
        // Create UI layout with organized sections
        Self::setup_ui(&mut ui_framework);
        
        Self {
            ui_framework,
            surface,
            device,
            queue,
            config,
            size,
            window,
            counter: 0,
            last_action: "Welcome to Widget Gallery!".to_string(),
            show_debug: false,
        }
    }
    
    fn setup_ui(ui_framework: &mut UiFramework) {
        // === HEADER SECTION ===
        let title_text = Text::new("🎨 Lumina UI Widget Gallery")
            .font_size(36.0)
            .color([0.95, 0.95, 0.95, 1.0].into());
        ui_framework.add_root_widget(Box::new(title_text));
        
        let subtitle_text = Text::new("Demonstrating modern UI components with glyphon text rendering")
            .font_size(18.0)
            .color([0.75, 0.75, 0.75, 1.0].into());
        ui_framework.add_root_widget(Box::new(subtitle_text));
        
        let version_text = Text::new("✨ Version 1.0 - Powered by WGPU & Glyphon")
            .font_size(14.0)
            .color([0.6, 0.8, 1.0, 1.0].into());
        ui_framework.add_root_widget(Box::new(version_text));
        
        // === BUTTON SHOWCASE ===
        let button_section_text = Text::new("🔘 Button Variants")
            .font_size(24.0)
            .color([0.9, 0.9, 0.9, 1.0].into());
        ui_framework.add_root_widget(Box::new(button_section_text));
        
        let button_desc = Text::new("Different button styles for various UI actions:")
            .font_size(16.0)
            .color([0.7, 0.7, 0.7, 1.0].into());
        ui_framework.add_root_widget(Box::new(button_desc));
        
        // Primary button
        let primary_btn = button("🚀 Primary Action")
            .on_click(|| println!("✅ Primary button clicked - main action executed!"))
            .build();
        ui_framework.add_root_widget(Box::new(primary_btn));
        
        // Secondary button
        let secondary_btn = secondary_button("⚙️ Secondary Action") 
            .on_click(|| println!("🔧 Secondary button clicked - configuration opened!"))
            .build();
        ui_framework.add_root_widget(Box::new(secondary_btn));
        
        // Ghost button
        let ghost_btn = ghost_button("👻 Subtle Action")
            .on_click(|| println!("💫 Ghost button clicked - subtle operation performed!"))
            .build();
        ui_framework.add_root_widget(Box::new(ghost_btn));
        
        // Danger button
        let danger_btn = danger_button("⚠️ Destructive Action")
            .on_click(|| println!("🗑️ Danger button clicked - destructive action confirmed!"))
            .build();
        ui_framework.add_root_widget(Box::new(danger_btn));
        
        // === INTERACTIVE DEMO ===
        let interactive_section = Text::new("🎮 Interactive Elements")
            .font_size(24.0)
            .color([0.9, 0.9, 0.9, 1.0].into());
        ui_framework.add_root_widget(Box::new(interactive_section));
        
        let counter_display = Text::new("📊 Counter Demo: 0")
            .font_size(20.0)
            .color([0.4, 0.8, 0.4, 1.0].into());
        ui_framework.add_root_widget(Box::new(counter_display));
        
        let increment_btn = Button::new("➕ Increment (+1)")
            .variant(ButtonVariant::Primary);
        ui_framework.add_root_widget(Box::new(increment_btn));
        
        let decrement_btn = Button::new("➖ Decrement (-1)")
            .variant(ButtonVariant::Secondary);
        ui_framework.add_root_widget(Box::new(decrement_btn));
        
        let reset_btn = Button::new("🔄 Reset to Zero")
            .variant(ButtonVariant::Danger);
        ui_framework.add_root_widget(Box::new(reset_btn));
        
        // === TEXT SHOWCASE ===
        let text_section = Text::new("📝 Typography Examples")
            .font_size(24.0)
            .color([0.9, 0.9, 0.9, 1.0].into());
        ui_framework.add_root_widget(Box::new(text_section));
        
        let large_text = Text::new("Large Header Text (32px)")
            .font_size(32.0)
            .color([1.0, 0.8, 0.4, 1.0].into());
        ui_framework.add_root_widget(Box::new(large_text));
        
        let medium_text = Text::new("Medium Body Text (18px) - Perfect for content")
            .font_size(18.0)
            .color([0.85, 0.85, 0.85, 1.0].into());
        ui_framework.add_root_widget(Box::new(medium_text));
        
        let small_text = Text::new("Small Detail Text (14px) - Great for descriptions and metadata")
            .font_size(14.0)
            .color([0.65, 0.65, 0.65, 1.0].into());
        ui_framework.add_root_widget(Box::new(small_text));
        
        // === STATUS AND INFO ===
        let status_section = Text::new("📊 Status Information")
            .font_size(20.0)
            .color([0.9, 0.9, 0.9, 1.0].into());
        ui_framework.add_root_widget(Box::new(status_section));
        
        let status_ready = Text::new("🟢 System Status: Ready & Operational")
            .font_size(16.0)
            .color([0.4, 0.9, 0.4, 1.0].into());
        ui_framework.add_root_widget(Box::new(status_ready));
        
        let performance_info = Text::new("⚡ Performance: Glyphon text rendering active")
            .font_size(16.0)
            .color([0.4, 0.8, 1.0, 1.0].into());
        ui_framework.add_root_widget(Box::new(performance_info));
        
        // === FOOTER ===
        let instructions = Text::new("💡 Instructions: Hover and click buttons to see interactions")
            .font_size(14.0)
            .color([0.6, 0.7, 0.8, 1.0].into());
        ui_framework.add_root_widget(Box::new(instructions));
        
        let footer_text = Text::new("🏆 Lumina Engine UI System • Modern • Responsive • Accessible")
            .font_size(12.0)
            .color([0.5, 0.5, 0.5, 1.0].into());
        ui_framework.add_root_widget(Box::new(footer_text));
    }
    
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
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
    
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let input_event = InputEvent::MouseMove {
                    position: [position.x as f32, position.y as f32].into(),
                    delta: [0.0, 0.0].into(),
                };
                self.ui_framework.handle_input(input_event);
                true
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let mouse_button = match button {
                    winit::event::MouseButton::Left => MouseButton::Left,
                    winit::event::MouseButton::Right => MouseButton::Right,
                    winit::event::MouseButton::Middle => MouseButton::Middle,
                    _ => return false,
                };
                
                let input_event = match state {
                    ElementState::Pressed => InputEvent::MouseDown {
                        button: mouse_button,
                        position: [0.0, 0.0].into(),
                        modifiers: Modifiers::default(),
                    },
                    ElementState::Released => InputEvent::MouseUp {
                        button: mouse_button,
                        position: [0.0, 0.0].into(),
                        modifiers: Modifiers::default(),
                    },
                };
                
                self.ui_framework.handle_input(input_event);
                true
            }
            WindowEvent::KeyboardInput { event: key_event, .. } => {
                if let winit::keyboard::PhysicalKey::Code(keycode) = key_event.physical_key {
                    let key = match keycode {
                        winit::keyboard::KeyCode::Space => KeyCode::Space,
                        winit::keyboard::KeyCode::Enter => KeyCode::Enter,
                        winit::keyboard::KeyCode::Escape => KeyCode::Escape,
                        winit::keyboard::KeyCode::KeyD => KeyCode::D,
                        winit::keyboard::KeyCode::KeyA => KeyCode::A,
                        winit::keyboard::KeyCode::KeyW => KeyCode::W,
                        winit::keyboard::KeyCode::KeyS => KeyCode::S,
                        _ => return false,
                    };
                    
                    if key_event.state == ElementState::Pressed && key == KeyCode::D {
                        self.show_debug = !self.show_debug;
                        self.last_action = format!("Debug mode: {}", if self.show_debug { "ON" } else { "OFF" });
                    }
                    
                    let input_event = match key_event.state {
                        ElementState::Pressed => InputEvent::KeyDown {
                            key,
                            modifiers: Modifiers::default(),
                        },
                        ElementState::Released => InputEvent::KeyUp {
                            key,
                            modifiers: Modifiers::default(),
                        },
                    };
                    
                    self.ui_framework.handle_input(input_event);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    
    fn update(&mut self) {
        // Update UI layout with improved spacing
        self.ui_framework.update_layout([self.size.width as f32, self.size.height as f32].into());
        
        // Handle UI interactions and update state
        self.handle_widget_interactions();
    }
    
    fn handle_widget_interactions(&mut self) {
        // This would typically be handled through callbacks in a real implementation
        // For now, we just update our demo state
        
        if self.show_debug {
            // Update debug info
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
                            r: 0.11, // Dark theme background
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

            // Render UI with proper theming
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
    let window = Arc::new(WindowBuilder::new()
        .with_title("Lumina UI - Widget Gallery")
        .with_inner_size(winit::dpi::LogicalSize::new(1200, 800))
        .build(&event_loop)
        .unwrap());
    
    let mut app = pollster::block_on(WidgetGalleryApp::new(window.clone()));
    
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == app.window.id() => {
                if !app.input(event) {
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