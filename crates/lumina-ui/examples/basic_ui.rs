//! Basic UI framework example demonstrating buttons, panels, and layouts

use lumina_ui::{
    UiFramework, Theme, 
    Button, Panel, Text, 
    InputEvent, InputHandler, KeyCode, MouseButton, Modifiers,
};
use lumina_ui::button::ButtonVariant;
use lumina_render::{UiRenderer, Renderer, RenderConfig};
use winit::{
    event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wgpu::util::DeviceExt;
use std::sync::Arc;

struct BasicUiApp {
    ui_framework: UiFramework,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    // UI state
    counter: i32,
    button_clicked: bool,
}

impl BasicUiApp {
    async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();
        
        // Initialize WGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        
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
        let theme = Theme::default();
        let mut ui_framework = UiFramework::new(theme);
        ui_framework.set_renderer(ui_renderer);
        
        // Create UI elements
        let counter_text = Text::new("Counter: 0")
            .font_size(24.0)
            .color([1.0, 1.0, 1.0, 1.0].into());
        
        let increment_button = Button::new("Increment")
            .variant(ButtonVariant::Primary);
        
        let decrement_button = Button::new("Decrement")
            .variant(ButtonVariant::Secondary);
        
        let reset_button = Button::new("Reset")
            .variant(ButtonVariant::Danger);
        
        let info_panel = Panel::new();
        
        // Add widgets to framework
        ui_framework.add_root_widget(Box::new(counter_text));
        ui_framework.add_root_widget(Box::new(increment_button));
        ui_framework.add_root_widget(Box::new(decrement_button));
        ui_framework.add_root_widget(Box::new(reset_button));
        ui_framework.add_root_widget(Box::new(info_panel));
        
        Self {
            ui_framework,
            surface,
            device,
            queue,
            config,
            size,
            counter: 0,
            button_clicked: false,
        }
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
                    delta: [0.0, 0.0].into(), // TODO: calculate actual delta
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
                        position: [0.0, 0.0].into(), // TODO: get actual position
                        modifiers: Modifiers::default(),
                    },
                    ElementState::Released => InputEvent::MouseUp {
                        button: mouse_button,
                        position: [0.0, 0.0].into(), // TODO: get actual position
                        modifiers: Modifiers::default(),
                    },
                };
                
                self.ui_framework.handle_input(input_event);
                true
            }
            WindowEvent::KeyboardInput { 
                input: KeyboardInput { 
                    state, 
                    virtual_keycode: Some(keycode), 
                    .. 
                }, 
                .. 
            } => {
                let key = match keycode {
                    VirtualKeyCode::Space => KeyCode::Space,
                    VirtualKeyCode::Return => KeyCode::Enter,
                    VirtualKeyCode::Escape => KeyCode::Escape,
                    VirtualKeyCode::A => KeyCode::A,
                    VirtualKeyCode::D => KeyCode::D,
                    VirtualKeyCode::W => KeyCode::W,
                    VirtualKeyCode::S => KeyCode::S,
                    _ => return false,
                };
                
                let input_event = match state {
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
            }
            _ => false,
        }
    }
    
    fn update(&mut self) {
        // Update UI layout
        self.ui_framework.update_layout([self.size.width as f32, self.size.height as f32].into());
        
        // Update UI state (no input frame management needed)
    }
    
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.06,
                            g: 0.06,
                            b: 0.14,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }
        
        // Render UI
        self.ui_framework.render(&self.queue);
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Lumina UI Framework - Basic Example")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();
    
    let mut app = BasicUiApp::new(&window).await;
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !app.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            app.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            app.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                app.update();
                match app.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => app.resize(app.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // window.request_redraw(); // Not available in current winit version
            }
            _ => {}
        }
    });
}