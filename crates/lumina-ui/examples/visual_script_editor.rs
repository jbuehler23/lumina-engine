//! Visual Script Editor - Node-based programming interface
//! Demonstrates the power of Lumina UI for creating complex editor interfaces

use lumina_ui::{
    UiFramework, UiRenderer, Theme, 
    widgets::{Button, Panel, Text, ButtonVariant, Canvas},
    input::{InputHandler, RawInputEvent, KeyCode, MouseButton, Modifiers},
    editor::{EditorApp, NodeEditor},
    layout::{LayoutConstraints, HorizontalAlign, VerticalAlign, Alignment},
};
use lumina_scripting::visual_scripting::{
    VisualScript, ScriptNode, NodeConnection, NodeType, InputType, ScriptValue,
    create_player_movement_script, create_coin_collection_script,
};
use winit::{
    event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use glam::Vec2;
use std::collections::HashMap;

#[derive(Debug)]
struct VisualNode {
    id: String,
    position: Vec2,
    size: Vec2,
    node_type: NodeType,
    selected: bool,
    connections: Vec<String>, // Connected node IDs
}

struct VisualScriptEditor {
    ui_framework: UiFramework,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    
    // Editor state
    current_script: VisualScript,
    visual_nodes: HashMap<String, VisualNode>,
    selected_node: Option<String>,
    canvas_offset: Vec2,
    canvas_zoom: f32,
    is_panning: bool,
    last_mouse_pos: Vec2,
    
    // Node palette
    available_nodes: Vec<NodeType>,
}

impl VisualScriptEditor {
    async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();
        
        // Initialize WGPU (same as basic example)
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
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
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
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
        };
        surface.configure(&device, &config);
        
        // Create UI renderer and framework
        let ui_renderer = UiRenderer::new(device.clone(), queue.clone(), config.clone()).await.unwrap();
        let theme = Theme::dark();
        let mut ui_framework = UiFramework::new(ui_renderer, theme);
        
        // Create editor UI
        Self::setup_editor_ui(&mut ui_framework);
        
        // Load a sample script
        let current_script = create_player_movement_script();
        let visual_nodes = Self::create_visual_nodes_from_script(&current_script);
        
        // Available node types for the palette
        let available_nodes = vec![
            NodeType::OnStart,
            NodeType::OnUpdate,
            NodeType::OnInput(InputType::KeyPressed("Space".to_string())),
            NodeType::MoveTowards { target: "player".to_string(), speed: 100.0 },
            NodeType::PlaySound("jump.wav".to_string()),
            NodeType::SetProperty { 
                target: "player".to_string(), 
                property: "velocity_y".to_string(), 
                value: ScriptValue::Number(300.0) 
            },
            NodeType::SpawnObject("bullet".to_string()),
            NodeType::DestroyObject("self".to_string()),
            NodeType::Print("Debug message".to_string()),
        ];
        
        Self {
            ui_framework,
            surface,
            device,
            queue,
            config,
            size,
            current_script,
            visual_nodes,
            selected_node: None,
            canvas_offset: Vec2::ZERO,
            canvas_zoom: 1.0,
            is_panning: false,
            last_mouse_pos: Vec2::ZERO,
            available_nodes,
        }
    }
    
    fn setup_editor_ui(ui_framework: &mut UiFramework) {
        // Main toolbar
        let toolbar_panel = Panel::new();
        
        let new_script_btn = Button::new("New Script")
            .variant(ButtonVariant::Primary)
            .on_click(|| println!("Creating new script"));
        
        let load_script_btn = Button::new("Load Script")
            .variant(ButtonVariant::Secondary)
            .on_click(|| println!("Loading script"));
        
        let save_script_btn = Button::new("Save Script")
            .variant(ButtonVariant::Secondary)
            .on_click(|| println!("Saving script"));
        
        let play_btn = Button::new("▶ Play")
            .variant(ButtonVariant::Primary)
            .on_click(|| println!("Running script"));
        
        let stop_btn = Button::new("⏹ Stop")
            .variant(ButtonVariant::Danger)
            .on_click(|| println!("Stopping script"));
        
        // Node palette panel
        let palette_panel = Panel::new();
        let palette_title = Text::new("Node Palette")
            .font_size(16.0)
            .color([1.0, 1.0, 1.0, 1.0].into());
        
        // Script canvas
        let canvas = Canvas::new();
        
        // Property inspector
        let properties_panel = Panel::new();
        let properties_title = Text::new("Properties")
            .font_size(16.0)
            .color([1.0, 1.0, 1.0, 1.0].into());
        
        // Add all widgets to framework
        ui_framework.add_root_widget(Box::new(toolbar_panel));
        ui_framework.add_root_widget(Box::new(new_script_btn));
        ui_framework.add_root_widget(Box::new(load_script_btn));
        ui_framework.add_root_widget(Box::new(save_script_btn));
        ui_framework.add_root_widget(Box::new(play_btn));
        ui_framework.add_root_widget(Box::new(stop_btn));
        ui_framework.add_root_widget(Box::new(palette_panel));
        ui_framework.add_root_widget(Box::new(palette_title));
        ui_framework.add_root_widget(Box::new(canvas));
        ui_framework.add_root_widget(Box::new(properties_panel));
        ui_framework.add_root_widget(Box::new(properties_title));
    }
    
    fn create_visual_nodes_from_script(script: &VisualScript) -> HashMap<String, VisualNode> {
        let mut visual_nodes = HashMap::new();
        
        for node in &script.nodes {
            let visual_node = VisualNode {
                id: node.id.clone(),
                position: Vec2::new(node.position.0, node.position.1),
                size: Vec2::new(150.0, 80.0), // Default node size
                node_type: node.node_type.clone(),
                selected: false,
                connections: script.connections
                    .iter()
                    .filter(|conn| conn.from_node == node.id)
                    .map(|conn| conn.to_node.clone())
                    .collect(),
            };
            visual_nodes.insert(node.id.clone(), visual_node);
        }
        
        visual_nodes
    }
    
    fn render_node_graph(&mut self) {
        // This would render the visual nodes and connections
        // For now, we'll just print the current state
        println!("Rendering {} nodes", self.visual_nodes.len());
        
        for (id, node) in &self.visual_nodes {
            let selected_marker = if node.selected { "[SELECTED]" } else { "" };
            println!("  Node '{}' at ({}, {}) {}", 
                id, node.position.x, node.position.y, selected_marker);
        }
    }
    
    fn add_node(&mut self, node_type: NodeType, position: Vec2) {
        let node_id = format!("node_{}", self.visual_nodes.len());
        
        let visual_node = VisualNode {
            id: node_id.clone(),
            position,
            size: Vec2::new(150.0, 80.0),
            node_type: node_type.clone(),
            selected: false,
            connections: Vec::new(),
        };
        
        self.visual_nodes.insert(node_id.clone(), visual_node);
        
        // Add to script as well
        let script_node = ScriptNode {
            id: node_id,
            node_type,
            position: (position.x, position.y),
            properties: HashMap::new(),
        };
        
        self.current_script.nodes.push(script_node);
        
        println!("Added new node at ({}, {})", position.x, position.y);
    }
    
    fn select_node_at_position(&mut self, position: Vec2) {
        // Clear current selection
        for node in self.visual_nodes.values_mut() {
            node.selected = false;
        }
        
        // Find and select node at position
        for (id, node) in &mut self.visual_nodes {
            let node_rect = glam::Vec4::new(
                node.position.x,
                node.position.y,
                node.position.x + node.size.x,
                node.position.y + node.size.y,
            );
            
            if position.x >= node_rect.x && position.x <= node_rect.z &&
               position.y >= node_rect.y && position.y <= node_rect.w {
                node.selected = true;
                self.selected_node = Some(id.clone());
                println!("Selected node: {}", id);
                break;
            }
        }
    }
    
    fn handle_canvas_interaction(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::MouseInput { state, button: winit::event::MouseButton::Left, .. } => {
                match state {
                    ElementState::Pressed => {
                        let mouse_pos = self.ui_framework.input_handler.mouse_position();
                        
                        // Convert screen coordinates to canvas coordinates
                        let canvas_pos = (mouse_pos - self.canvas_offset) / self.canvas_zoom;
                        
                        self.select_node_at_position(canvas_pos);
                    }
                    ElementState::Released => {
                        // Handle node release
                    }
                }
            }
            WindowEvent::MouseInput { state, button: winit::event::MouseButton::Right, .. } => {
                match state {
                    ElementState::Pressed => {
                        let mouse_pos = self.ui_framework.input_handler.mouse_position();
                        let canvas_pos = (mouse_pos - self.canvas_offset) / self.canvas_zoom;
                        
                        // Right-click to add a new node
                        if let Some(node_type) = self.available_nodes.first() {
                            self.add_node(node_type.clone(), canvas_pos);
                        }
                    }
                    _ => {}
                }
            }
            WindowEvent::MouseInput { state, button: winit::event::MouseButton::Middle, .. } => {
                match state {
                    ElementState::Pressed => {
                        self.is_panning = true;
                        self.last_mouse_pos = self.ui_framework.input_handler.mouse_position();
                    }
                    ElementState::Released => {
                        self.is_panning = false;
                    }
                }
            }
            WindowEvent::CursorMoved { .. } => {
                if self.is_panning {
                    let current_mouse_pos = self.ui_framework.input_handler.mouse_position();
                    let delta = current_mouse_pos - self.last_mouse_pos;
                    self.canvas_offset += delta;
                    self.last_mouse_pos = current_mouse_pos;
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                // Handle zoom
                match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => {
                        let zoom_factor = 1.1;
                        if *y > 0.0 {
                            self.canvas_zoom *= zoom_factor;
                        } else {
                            self.canvas_zoom /= zoom_factor;
                        }
                        self.canvas_zoom = self.canvas_zoom.clamp(0.1, 5.0);
                        println!("Canvas zoom: {:.2}", self.canvas_zoom);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            
            self.ui_framework.renderer.resize([new_size.width as f32, new_size.height as f32].into());
        }
    }
    
    fn input(&mut self, event: &WindowEvent) -> bool {
        // Handle canvas-specific interactions first
        self.handle_canvas_interaction(event);
        
        // Then handle general UI input
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let raw_event = RawInputEvent::MouseMove {
                    position: [position.x as f32, position.y as f32].into(),
                };
                let input_events = self.ui_framework.input_handler.process_input(&raw_event);
                for input_event in input_events {
                    self.ui_framework.handle_input(input_event);
                }
                true
            }
            WindowEvent::KeyboardInput { 
                input: KeyboardInput { 
                    state: ElementState::Pressed,
                    virtual_keycode: Some(keycode), 
                    .. 
                }, 
                .. 
            } => {
                match keycode {
                    VirtualKeyCode::Delete => {
                        if let Some(selected_id) = &self.selected_node {
                            self.visual_nodes.remove(selected_id);
                            self.current_script.nodes.retain(|n| &n.id != selected_id);
                            self.selected_node = None;
                            println!("Deleted selected node");
                        }
                    }
                    VirtualKeyCode::S if self.ui_framework.input_handler.is_key_pressed(KeyCode::Control) => {
                        println!("Saving script: {}", self.current_script.name);
                        // TODO: Implement actual saving
                    }
                    VirtualKeyCode::O if self.ui_framework.input_handler.is_key_pressed(KeyCode::Control) => {
                        println!("Opening script dialog");
                        // TODO: Implement file dialog
                    }
                    VirtualKeyCode::N if self.ui_framework.input_handler.is_key_pressed(KeyCode::Control) => {
                        println!("Creating new script");
                        self.visual_nodes.clear();
                        self.current_script = VisualScript {
                            name: "New Script".to_string(),
                            nodes: Vec::new(),
                            connections: Vec::new(),
                            variables: HashMap::new(),
                        };
                    }
                    _ => return false,
                }
                true
            }
            _ => false,
        }
    }
    
    fn update(&mut self) {
        self.ui_framework.update_layout([self.size.width as f32, self.size.height as f32].into());
        self.ui_framework.input_handler.begin_frame();
        
        // Render the node graph
        self.render_node_graph();
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
            });
        }
        
        // Render UI
        self.ui_framework.render();
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    
    println!("🎮 Lumina Visual Script Editor");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Controls:");
    println!("  • Left Click: Select node");
    println!("  • Right Click: Add new node");
    println!("  • Middle Click + Drag: Pan canvas");
    println!("  • Mouse Wheel: Zoom in/out");
    println!("  • Delete: Remove selected node");
    println!("  • Ctrl+S: Save script");
    println!("  • Ctrl+O: Open script");
    println!("  • Ctrl+N: New script");
    println!();
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Lumina Visual Script Editor - Game Creation Made Easy!")
        .with_inner_size(winit::dpi::LogicalSize::new(1200, 800))
        .build(&event_loop)
        .unwrap();
    
    let mut editor = VisualScriptEditor::new(&window).await;
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !editor.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            editor.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            editor.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                editor.update();
                match editor.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => editor.resize(editor.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}