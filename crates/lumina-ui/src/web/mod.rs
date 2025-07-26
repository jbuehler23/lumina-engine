//! Web-specific functionality for Lumina UI framework
//! Handles WASM compilation and browser integration

use crate::{UiFramework, UiRenderer, Theme};
use wasm_bindgen::prelude::*;
use web_sys::{console, HtmlCanvasElement, Window, Document};
use std::cell::RefCell;
use std::rc::Rc;

/// Web application wrapper for the UI framework
#[wasm_bindgen]
pub struct WebApp {
    ui_framework: Option<UiFramework>,
    canvas: HtmlCanvasElement,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    surface: Option<wgpu::Surface>,
    config: Option<wgpu::SurfaceConfiguration>,
}

#[wasm_bindgen]
impl WebApp {
    /// Create a new web application
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<WebApp, JsValue> {
        // Set up panic handler for better error messages
        console_error_panic_hook::set_once();
        
        // Get the canvas element
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        
        console::log_1(&"Lumina UI Framework initializing for web...".into());
        
        Ok(WebApp {
            ui_framework: None,
            canvas,
            device: None,
            queue: None,
            surface: None,
            config: None,
        })
    }
    
    /// Initialize the WGPU context and UI framework
    #[wasm_bindgen]
    pub async fn initialize(&mut self) -> Result<(), JsValue> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL,
            dx12_shader_compiler: Default::default(),
        });
        
        let surface = instance.create_surface_from_canvas(&self.canvas)
            .map_err(|e| JsValue::from_str(&format!("Failed to create surface: {:?}", e)))?;
        
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or("Failed to get adapter")?;
        
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    label: None,
                },
                None,
            )
            .await
            .map_err(|e| JsValue::from_str(&format!("Failed to get device: {:?}", e)))?;
        
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        
        let size = self.get_canvas_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        
        // Create UI framework
        let ui_renderer = UiRenderer::new(device.clone(), queue.clone(), config.clone()).await
            .map_err(|e| JsValue::from_str(&format!("Failed to create UI renderer: {:?}", e)))?;
        let theme = Theme::dark();
        let ui_framework = UiFramework::new(ui_renderer, theme);
        
        self.device = Some(device);
        self.queue = Some(queue);
        self.surface = Some(surface);
        self.config = Some(config);
        self.ui_framework = Some(ui_framework);
        
        console::log_1(&"Lumina UI Framework initialized successfully!".into());
        Ok(())
    }
    
    /// Get the current canvas size
    fn get_canvas_size(&self) -> (u32, u32) {
        (
            self.canvas.client_width() as u32,
            self.canvas.client_height() as u32,
        )
    }
    
    /// Resize the framework when canvas size changes
    #[wasm_bindgen]
    pub fn resize(&mut self, width: u32, height: u32) {
        if let (Some(config), Some(surface), Some(ui_framework)) = (
            &mut self.config,
            &self.surface,
            &mut self.ui_framework,
        ) {
            config.width = width;
            config.height = height;
            surface.configure(&self.device.as_ref().unwrap(), config);
            ui_framework.renderer.resize([width as f32, height as f32].into());
        }
    }
    
    /// Handle mouse events from JavaScript
    #[wasm_bindgen]
    pub fn handle_mouse_move(&mut self, x: f32, y: f32) {
        if let Some(ui_framework) = &mut self.ui_framework {
            let raw_event = crate::input::RawInputEvent::MouseMove {
                position: [x, y].into(),
            };
            let input_events = ui_framework.input_handler.process_input(&raw_event);
            for input_event in input_events {
                ui_framework.handle_input(input_event);
            }
        }
    }
    
    /// Handle mouse click events
    #[wasm_bindgen]
    pub fn handle_mouse_click(&mut self, x: f32, y: f32, button: u32) {
        if let Some(ui_framework) = &mut self.ui_framework {
            let mouse_button = match button {
                0 => crate::input::MouseButton::Left,
                1 => crate::input::MouseButton::Middle,
                2 => crate::input::MouseButton::Right,
                _ => return,
            };
            
            let raw_event = crate::input::RawInputEvent::MouseDown {
                button: mouse_button,
                position: [x, y].into(),
                modifiers: crate::input::Modifiers::empty(),
            };
            let input_events = ui_framework.input_handler.process_input(&raw_event);
            for input_event in input_events {
                ui_framework.handle_input(input_event);
            }
        }
    }
    
    /// Handle key events
    #[wasm_bindgen]
    pub fn handle_key_down(&mut self, key_code: u32) {
        if let Some(ui_framework) = &mut self.ui_framework {
            if let Some(key) = self.js_key_to_lumina_key(key_code) {
                let raw_event = crate::input::RawInputEvent::KeyDown {
                    key,
                    modifiers: crate::input::Modifiers::empty(),
                };
                let input_events = ui_framework.input_handler.process_input(&raw_event);
                for input_event in input_events {
                    ui_framework.handle_input(input_event);
                }
            }
        }
    }
    
    /// Convert JavaScript key codes to Lumina key codes
    fn js_key_to_lumina_key(&self, key_code: u32) -> Option<crate::input::KeyCode> {
        match key_code {
            32 => Some(crate::input::KeyCode::Space),
            13 => Some(crate::input::KeyCode::Enter),
            27 => Some(crate::input::KeyCode::Escape),
            8 => Some(crate::input::KeyCode::Backspace),
            9 => Some(crate::input::KeyCode::Tab),
            65 => Some(crate::input::KeyCode::A),
            68 => Some(crate::input::KeyCode::D),
            83 => Some(crate::input::KeyCode::S),
            87 => Some(crate::input::KeyCode::W),
            37 => Some(crate::input::KeyCode::ArrowLeft),
            38 => Some(crate::input::KeyCode::ArrowUp),
            39 => Some(crate::input::KeyCode::ArrowRight),
            40 => Some(crate::input::KeyCode::ArrowDown),
            _ => None,
        }
    }
    
    /// Update the UI framework
    #[wasm_bindgen]
    pub fn update(&mut self) {
        if let Some(ui_framework) = &mut self.ui_framework {
            let size = self.get_canvas_size();
            ui_framework.update_layout([size.0 as f32, size.1 as f32].into());
            ui_framework.input_handler.begin_frame();
        }
    }
    
    /// Render the UI
    #[wasm_bindgen]
    pub fn render(&mut self) -> Result<(), JsValue> {
        if let (Some(ui_framework), Some(surface), Some(queue)) = (
            &mut self.ui_framework,
            &self.surface,
            &self.queue,
        ) {
            match surface.get_current_texture() {
                Ok(output) => {
                    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
                    
                    let mut encoder = self.device.as_ref().unwrap().create_command_encoder(
                        &wgpu::CommandEncoderDescriptor {
                            label: Some("Render Encoder"),
                        }
                    );
                    
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
                        });
                    }
                    
                    // Render UI
                    ui_framework.render();
                    
                    queue.submit(std::iter::once(encoder.finish()));
                    output.present();
                }
                Err(wgpu::SurfaceError::Lost) => {
                    let size = self.get_canvas_size();
                    self.resize(size.0, size.1);
                }
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    return Err(JsValue::from_str("GPU out of memory"));
                }
                Err(e) => {
                    console::log_1(&format!("Render error: {:?}", e).into());
                }
            }
        }
        Ok(())
    }
    
    /// Add a button to the UI (example API)
    #[wasm_bindgen]
    pub fn add_button(&mut self, text: &str, x: f32, y: f32) {
        if let Some(ui_framework) = &mut self.ui_framework {
            let button = crate::widgets::Button::new(text)
                .variant(crate::widgets::ButtonVariant::Primary)
                .on_click(move || {
                    console::log_1(&format!("Button '{}' clicked!", text).into());
                });
            
            ui_framework.add_root_widget(Box::new(button));
        }
    }
    
    /// Clear all UI elements
    #[wasm_bindgen]
    pub fn clear_ui(&mut self) {
        if let Some(ui_framework) = &mut self.ui_framework {
            // Clear all widgets
            for widget_id in ui_framework.state.root_widgets.clone() {
                ui_framework.remove_widget(widget_id);
            }
        }
    }
    
    /// Get the current number of UI elements
    #[wasm_bindgen]
    pub fn get_widget_count(&self) -> u32 {
        if let Some(ui_framework) = &self.ui_framework {
            ui_framework.state.widgets.len() as u32
        } else {
            0
        }
    }
}

/// Initialize the web application with error handling
#[wasm_bindgen]
pub async fn init_web_app(canvas_id: &str) -> Result<WebApp, JsValue> {
    let mut app = WebApp::new(canvas_id)?;
    app.initialize().await?;
    Ok(app)
}

/// JavaScript utility functions
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

/// Macro for easier console logging
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub(crate) use console_log;