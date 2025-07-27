//! Main editor application that integrates UI framework with WGPU rendering

use anyhow::Result;
use glam::Vec2;
use lumina_render::UiRenderer;
use lumina_ui::{
    UiFramework, Theme, InputEvent, MouseButton, KeyCode, Modifiers,
};
use winit::{
    event::{WindowEvent, ElementState},
    window::Window,
    keyboard::{Key, NamedKey},
    raw_window_handle::{HasRawWindowHandle, HasRawDisplayHandle},
};

use std::sync::Arc;

use crate::panels::{MenuBar, ProjectPanel, ScenePanel, PropertiesPanel, ConsolePanel};
use crate::project::EditorProject;

/// Main editor application that manages UI and rendering
pub struct EditorApp {
    // Window
    window: Arc<Window>,
    // Rendering components
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    
    // UI Framework
    ui_framework: UiFramework,
    
    // Editor state
    current_project: Option<EditorProject>,
    panels: EditorPanels,
    
    // Input tracking
    mouse_position: Vec2,
}

/// Container for all editor panels
pub struct EditorPanels {
    pub menu_bar: MenuBar,
    pub project_panel: ProjectPanel,
    pub scene_panel: ScenePanel,
    pub properties_panel: PropertiesPanel,
    pub console_panel: ConsolePanel,
}

impl EditorApp {
    /// Create a new editor application
    pub async fn new(window: Window) -> Result<Self> {
        let window = Arc::new(window);
        let size = window.inner_size();
        
        // Initialize WGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        let surface = instance.create_surface(Arc::clone(&window))?;
        
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("Failed to find suitable adapter"))?;
        
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Editor Device"),
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
        let theme = Theme::dark();
        let mut ui_framework = UiFramework::new(theme);
        ui_framework.set_renderer(ui_renderer);
        
        // Initialize editor panels
        let panels = EditorPanels::new(&mut ui_framework)?;
        
        log::info!("Editor app initialized with {}x{} window", size.width, size.height);
        
        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            ui_framework,
            current_project: None,
            panels,
            mouse_position: Vec2::ZERO,
        })
    }
    
    /// Handle window events, returns true if handled
    pub fn handle_window_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = Vec2::new(position.x as f32, position.y as f32);
                let input_event = InputEvent::MouseMove {
                    position: self.mouse_position,
                    delta: Vec2::ZERO, // TODO: Calculate actual delta
                };
                self.ui_framework.handle_input(input_event);
                true
            },
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
                        position: self.mouse_position,
                        modifiers: Modifiers::default(),
                    },
                    ElementState::Released => InputEvent::MouseUp {
                        button: mouse_button,
                        position: self.mouse_position,
                        modifiers: Modifiers::default(),
                    },
                };
                
                self.ui_framework.handle_input(input_event);
                true
            },
            WindowEvent::KeyboardInput { 
                event, .. 
            } => {
                let key_code = match event.logical_key {
                    Key::Named(NamedKey::Space) => KeyCode::Space,
                    Key::Named(NamedKey::Enter) => KeyCode::Enter,
                    Key::Named(NamedKey::Escape) => KeyCode::Escape,
                    Key::Named(NamedKey::Tab) => KeyCode::Tab,
                    Key::Character(ref s) if s == "a" => KeyCode::A,
                    Key::Character(ref s) if s == "s" => KeyCode::S,
                    Key::Character(ref s) if s == "d" => KeyCode::D,
                    Key::Character(ref s) if s == "w" => KeyCode::W,
                    _ => return false,
                };
                
                let input_event = match event.state {
                    ElementState::Pressed => InputEvent::KeyDown {
                        key: key_code,
                        modifiers: Modifiers::default(),
                    },
                    ElementState::Released => InputEvent::KeyUp {
                        key: key_code,
                        modifiers: Modifiers::default(),
                    },
                };
                
                self.ui_framework.handle_input(input_event);
                true
            },
            _ => false,
        }
    }
    
    /// Resize the application
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            
            // Update UI framework size
            if let Some(renderer) = &mut self.ui_framework.renderer {
                renderer.resize(Vec2::new(new_size.width as f32, new_size.height as f32));
            }
            
            log::debug!("Editor resized to {}x{}", new_size.width, new_size.height);
        }
    }
    
    /// Update the editor
    pub fn update(&mut self) {
        // Update UI layout
        let screen_size = Vec2::new(self.size.width as f32, self.size.height as f32);
        self.ui_framework.update_layout(screen_size);
        
        // Update panels
        self.panels.update(&mut self.ui_framework);
    }
    
    /// Render the editor
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Editor Render Encoder"),
        });
        
        // Clear the screen with editor background color
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Editor Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.06, // Dark background matching theme
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
    
    /// Get the current window size
    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }
    
    /// Request a redraw from the window
    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
    
    /// Create a new project
    pub fn new_project(&mut self, name: String, path: String) -> Result<()> {
        let project = EditorProject::new(name, path)?;
        self.current_project = Some(project);
        log::info!("Created new project");
        Ok(())
    }
    
    /// Load an existing project
    pub fn load_project(&mut self, path: String) -> Result<()> {
        let project = EditorProject::load(path)?;
        self.current_project = Some(project);
        log::info!("Loaded project");
        Ok(())
    }
    
    /// Get the current project
    pub fn current_project(&self) -> Option<&EditorProject> {
        self.current_project.as_ref()
    }
}

impl EditorPanels {
    /// Create new editor panels
    pub fn new(ui: &mut UiFramework) -> Result<Self> {
        let menu_bar = MenuBar::new(ui)?;
        let project_panel = ProjectPanel::new(ui)?;
        let scene_panel = ScenePanel::new(ui)?;
        let properties_panel = PropertiesPanel::new(ui)?;
        let console_panel = ConsolePanel::new(ui)?;
        
        Ok(Self {
            menu_bar,
            project_panel,
            scene_panel,
            properties_panel,
            console_panel,
        })
    }
    
    /// Update all panels
    pub fn update(&mut self, ui: &mut UiFramework) {
        self.menu_bar.update(ui);
        self.project_panel.update(ui);
        self.scene_panel.update(ui);
        self.properties_panel.update(ui);
        self.console_panel.update(ui);
    }
}