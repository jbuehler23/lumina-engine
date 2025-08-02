//! ECS rendering systems for the Lumina Engine
//! 
//! This module implements rendering systems that integrate with the core Engine
//! and follow the ECS-driven architecture outlined in ARCHITECTURE.md.

use crate::{System, SystemContext, Result};
use lumina_render::{RenderContext, Rect};
use lumina_ui::UiFramework;
use lumina_ecs::World;
use lumina_input::InputEvents;
use std::sync::Arc;
use parking_lot::Mutex;
use glam::{Vec2, Vec4};

/// UI button with click detection
#[derive(Debug, Clone)]
pub struct UiButton {
    pub position: Vec2,
    pub size: Vec2,
    pub label: String,
    pub icon: String,
    pub action: String,
}

impl UiButton {
    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.position.x 
            && point.x <= self.position.x + self.size.x
            && point.y >= self.position.y 
            && point.y <= self.position.y + self.size.y
    }
}

/// UI state resource for tracking interactive elements
pub struct UiState {
    pub toolbar_buttons: Vec<UiButton>,
    pub hovered_button: Option<usize>,
    pub last_mouse_position: Vec2,
}


/// Render system that manages the complete rendering pipeline
/// 
/// This system integrates with the existing Engine architecture and provides
/// ECS-driven rendering capabilities.
pub struct RenderSystem {
    /// ECS world containing all game state
    world: Arc<Mutex<World>>,
    /// Flag to track initialization
    initialized: bool,
}

impl RenderSystem {
    /// Create a new render system with the given ECS world
    pub fn new(world: Arc<Mutex<World>>) -> Self {
        Self {
            world,
            initialized: false,
        }
    }
}

impl System for RenderSystem {
    fn initialize(&mut self, _context: &mut SystemContext) -> Result<()> {
        log::info!("RenderSystem initialized");
        self.initialized = true;
        Ok(())
    }
    
    fn update(&mut self, _context: &mut SystemContext) -> Result<()> {
        if !self.initialized {
            return Ok(());
        }
        
        // Run the ECS rendering pipeline
        let mut world = self.world.lock();
        
        // Execute UI update system
        ui_update_system(&mut world)?;
        
        // Execute UI render system (includes game object rendering within proper frame setup)
        ui_render_system(&mut world)?;
        
        Ok(())
    }
    
    fn shutdown(&mut self, _context: &mut SystemContext) -> Result<()> {
        log::info!("RenderSystem shutdown");
        self.initialized = false;
        Ok(())
    }
}

/// UI update system that synchronizes ECS state with UI widgets
/// 
/// This system runs before the render system and updates UI widget properties
/// based on the current state of ECS components.
pub fn ui_update_system(world: &mut World) -> Result<()> {
    // Update UI layout based on current window size
    let window_size = world.with_resource::<RenderContext, _>(|render_context_opt| {
        render_context_opt
            .as_ref()
            .map(|ctx| ctx.window_size())
            .unwrap_or(glam::Vec2::new(1280.0, 720.0))
    });
    
    // Update the UI framework layout
    world.with_resource_mut::<UiFramework, _>(|mut ui_framework_opt| {
        if let Some(ui_framework) = ui_framework_opt.as_mut() {
            ui_framework.update_layout(window_size);
        }
    });
    
    // Here you would add logic to query for game entities and update UI elements
    // based on their state. For example:
    // - Query for Player components and update health bars
    // - Query for GameState components and update score displays
    // - Query for Inventory components and update item lists
    
    Ok(())
}

/// UI render system that encapsulates all UI rendering logic
/// 
/// This system queries for the RenderContext and UiFramework resources
/// and renders the UI using the WGPU render pass.
pub fn ui_render_system(world: &mut World) -> Result<()> {
    log::debug!("🎮 UI render system called");
    // Get the required resources
    let has_render_context = world.has_resource::<RenderContext>();
    let has_ui_framework = world.has_resource::<UiFramework>();
    
    if has_render_context && has_ui_framework {
        // Create storage for UI button data
        let mut toolbar_buttons = Vec::new();
        
        // Begin the frame and perform basic rendering
        world.with_resource_mut::<RenderContext, _>(|mut render_context_opt| {
            if let Some(render_context) = render_context_opt.as_mut() {
                match render_context.begin_frame() {
                    Ok(Some(output)) => {
                        // Create a command encoder to submit rendering work
                        let mut encoder = render_context.device().create_command_encoder(
                            &wgpu::CommandEncoderDescriptor {
                                label: Some("Render Encoder"),
                            }
                        );
                        
                        // Create a render pass for UI rendering
                        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
                        {
                            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                label: Some("UI Render Pass"),
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                    view: &view,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Clear(wgpu::Color {
                                            r: 0.08,
                                            g: 0.09,
                                            b: 0.12,
                                            a: 1.0,
                                        }),
                                        store: wgpu::StoreOp::Store,
                                    },
                                })],
                                depth_stencil_attachment: None,
                                occlusion_query_set: None,
                                timestamp_writes: None,
                            });
                            
                            // Draw basic UI elements using the renderer's UiRenderer
                            let window_size = render_context.window_size();
                            
                            // Get the references we need to avoid borrow checker issues
                            let device_ptr = render_context.device() as *const wgpu::Device;
                            let queue_ptr = render_context.queue() as *const wgpu::Queue;
                            
                            if let Some(ui_renderer) = render_context.renderer_mut().ui_renderer_mut() {
                                // Begin UI frame (using unsafe to bypass borrow checker)
                                unsafe {
                                    ui_renderer.begin_frame(&*queue_ptr);
                                }
                                
                                // Draw toolbar background
                                use lumina_render::Rect;
                                use glam::Vec4;
                                
                                let toolbar_height = 64.0;
                                let toolbar_rect = Rect {
                                    position: glam::Vec2::new(0.0, 0.0),
                                    size: glam::Vec2::new(window_size.x, toolbar_height),
                                };
                                ui_renderer.draw_rect(toolbar_rect, Vec4::new(0.2, 0.22, 0.25, 1.0));
                                
                                // Draw main panel area
                                let main_panel_rect = Rect {
                                    position: glam::Vec2::new(0.0, toolbar_height),
                                    size: glam::Vec2::new(window_size.x, window_size.y - toolbar_height),
                                };
                                ui_renderer.draw_rect(main_panel_rect, Vec4::new(0.15, 0.16, 0.18, 1.0));
                                
                                // Draw scene panel on the left (adjust for taller toolbar)
                                let panel_top = toolbar_height + 10.0;
                                let scene_panel_rect = Rect {
                                    position: glam::Vec2::new(10.0, panel_top),
                                    size: glam::Vec2::new(400.0, window_size.y - panel_top - 10.0),
                                };
                                ui_renderer.draw_rect(scene_panel_rect, Vec4::new(0.12, 0.14, 0.20, 1.0));
                                
                                // Draw property panel on the right
                                let prop_panel_rect = Rect {
                                    position: glam::Vec2::new(window_size.x - 300.0, panel_top),
                                    size: glam::Vec2::new(290.0, window_size.y - panel_top - 10.0),
                                };
                                ui_renderer.draw_rect(prop_panel_rect, Vec4::new(0.12, 0.14, 0.20, 1.0));
                                
                                // Add text labels to make panels visible
                                let font = ui_renderer.get_default_font();
                                
                                // Draw larger, clickable toolbar buttons
                                let toolbar_height = 64.0;
                                let button_y = 8.0;
                                let button_height = 48.0;
                                let button_width = 70.0;
                                let mut button_x = 15.0;
                                
                                // Use the toolbar_buttons from outer scope
                                
                                // Tool buttons
                                let tools = [
                                    ("📍", "Select", "select"), ("✋", "Move", "move"), ("🔄", "Rotate", "rotate"), 
                                    ("📏", "Scale", "scale"), ("🖌️", "Brush", "brush"), ("🧽", "Eraser", "eraser")
                                ];
                                
                                for (button_index, (icon, name, action)) in tools.iter().enumerate() {
                                    let button_pos = Vec2::new(button_x, button_y);
                                    let button_size = Vec2::new(button_width, button_height);
                                    
                                    // Store button for click detection
                                    toolbar_buttons.push(UiButton {
                                        position: button_pos,
                                        size: button_size,
                                        label: name.to_string(),
                                        icon: icon.to_string(),
                                        action: action.to_string(),
                                    });
                                    
                                    // Check if this button is being hovered (get from existing UI state if available)
                                    let is_hovered = world.with_resource::<UiState, _>(|ui_state_opt| {
                                        ui_state_opt.as_ref()
                                            .and_then(|ui_state| ui_state.hovered_button)
                                            .map(|hovered_idx| hovered_idx == button_index)
                                            .unwrap_or(false)
                                    });
                                    
                                    // Button colors based on hover state
                                    let (bg_color, border_color) = if is_hovered {
                                        (Vec4::new(0.4, 0.45, 0.5, 1.0), Vec4::new(0.5, 0.55, 0.6, 1.0)) // Brighter when hovered
                                    } else {
                                        (Vec4::new(0.3, 0.32, 0.35, 1.0), Vec4::new(0.4, 0.42, 0.45, 1.0)) // Normal colors
                                    };
                                    
                                    // Button background
                                    let button_rect = Rect {
                                        position: button_pos,
                                        size: button_size,
                                    };
                                    
                                    // Button border for definition
                                    let border_rect = Rect {
                                        position: button_pos - Vec2::new(1.0, 1.0),
                                        size: button_size + Vec2::new(2.0, 2.0),
                                    };
                                    ui_renderer.draw_rect(border_rect, border_color);
                                    ui_renderer.draw_rect(button_rect, bg_color);
                                    
                                    // Button icon (larger)
                                    unsafe {
                                        let _ = ui_renderer.draw_text(
                                            icon,
                                            Vec2::new(button_x + 22.0, button_y + 6.0),
                                            font,
                                            28.0, // Much larger icon
                                            Vec4::new(0.95, 0.95, 0.95, 1.0),
                                            &*queue_ptr
                                        );
                                    }
                                    
                                    // Button label (smaller text below icon)
                                    unsafe {
                                        let _ = ui_renderer.draw_text(
                                            name,
                                            Vec2::new(button_x + 10.0, button_y + 36.0),
                                            font,
                                            14.0, // Larger label
                                            Vec4::new(0.8, 0.8, 0.8, 1.0),
                                            &*queue_ptr
                                        );
                                    }
                                    
                                    button_x += button_width + 10.0;
                                }
                                
                                // Separator
                                button_x += 20.0;
                                
                                // File operations (larger buttons)
                                let file_ops = [("📄", "New", "new"), ("📂", "Open", "open"), ("💾", "Save", "save")];
                                for (file_button_index, (icon, name, action)) in file_ops.iter().enumerate() {
                                    let button_pos = Vec2::new(button_x, button_y);
                                    let button_size = Vec2::new(button_width, button_height);
                                    
                                    // Store button for click detection
                                    toolbar_buttons.push(UiButton {
                                        position: button_pos,
                                        size: button_size,
                                        label: name.to_string(),
                                        icon: icon.to_string(),
                                        action: action.to_string(),
                                    });
                                    
                                    // Check if this button is being hovered (offset by tool buttons count)
                                    let button_index = tools.len() + file_button_index;
                                    let is_hovered = world.with_resource::<UiState, _>(|ui_state_opt| {
                                        ui_state_opt.as_ref()
                                            .and_then(|ui_state| ui_state.hovered_button)
                                            .map(|hovered_idx| hovered_idx == button_index)
                                            .unwrap_or(false)
                                    });
                                    
                                    // Button colors based on hover state
                                    let (bg_color, border_color) = if is_hovered {
                                        (Vec4::new(0.4, 0.45, 0.5, 1.0), Vec4::new(0.5, 0.55, 0.6, 1.0)) // Brighter when hovered
                                    } else {
                                        (Vec4::new(0.3, 0.32, 0.35, 1.0), Vec4::new(0.4, 0.42, 0.45, 1.0)) // Normal colors
                                    };
                                    
                                    let button_rect = Rect {
                                        position: button_pos,
                                        size: button_size,
                                    };
                                    
                                    // Button border
                                    let border_rect = Rect {
                                        position: button_pos - Vec2::new(1.0, 1.0),
                                        size: button_size + Vec2::new(2.0, 2.0),
                                    };
                                    ui_renderer.draw_rect(border_rect, border_color);
                                    ui_renderer.draw_rect(button_rect, bg_color);
                                    
                                    // Button icon
                                    unsafe {
                                        let _ = ui_renderer.draw_text(
                                            icon,
                                            Vec2::new(button_x + 22.0, button_y + 6.0),
                                            font,
                                            28.0,
                                            Vec4::new(0.95, 0.95, 0.95, 1.0),
                                            &*queue_ptr
                                        );
                                    }
                                    
                                    // Button label
                                    unsafe {
                                        let _ = ui_renderer.draw_text(
                                            name,
                                            Vec2::new(button_x + 16.0, button_y + 36.0),
                                            font,
                                            14.0,
                                            Vec4::new(0.8, 0.8, 0.8, 1.0),
                                            &*queue_ptr
                                        );
                                    }
                                    
                                    button_x += button_width + 10.0;
                                }
                                
                                // We'll store the button data after the closure
                                
                                // Scene panel text (larger and positioned for new toolbar)
                                unsafe {
                                    let _ = ui_renderer.draw_text(
                                        "🎮 Scene Editor",
                                        glam::Vec2::new(20.0, panel_top + 10.0),
                                        font,
                                        22.0, // Larger text
                                        Vec4::new(0.9, 0.95, 1.0, 1.0),
                                        &*queue_ptr
                                    );
                                    let _ = ui_renderer.draw_text(
                                        "Drop objects here to create your game",
                                        glam::Vec2::new(20.0, panel_top + 35.0),
                                        font,
                                        16.0,
                                        Vec4::new(0.7, 0.8, 0.9, 1.0),
                                        &*queue_ptr
                                    );
                                }
                                
                                // Property panel text (larger and positioned for new toolbar)
                                unsafe {
                                    let _ = ui_renderer.draw_text(
                                        "🔍 Properties",
                                        glam::Vec2::new(window_size.x - 290.0, panel_top + 10.0),
                                        font,
                                        22.0, // Larger text
                                        Vec4::new(0.9, 0.95, 1.0, 1.0),
                                        &*queue_ptr
                                    );
                                    let _ = ui_renderer.draw_text(
                                        "Select objects to edit their properties",
                                        glam::Vec2::new(window_size.x - 290.0, panel_top + 35.0),
                                        font,
                                        16.0,
                                        Vec4::new(0.7, 0.8, 0.9, 1.0),
                                        &*queue_ptr
                                    );
                                }
                                
                                // *** RENDER GAME OBJECTS HERE (within active frame) ***
                                // Render all entities that have Renderable components
                                let mut render_count = 0;
                                for (_entity, renderable) in world.query::<Renderable>() {
                                    render_count += 1;
                                    let bounds = Rect {
                                        position: renderable.position,
                                        size: renderable.size,
                                    };
                                    ui_renderer.draw_rect(bounds, renderable.color);
                                    
                                    if render_count <= 3 { // Only log first few to avoid spam
                                        log::debug!("🎮 Rendering game object at {:?} with size {:?} and color {:?}", 
                                                   renderable.position, renderable.size, renderable.color);
                                    }
                                }
                                
                                if render_count > 0 {
                                    log::debug!("🎮 Total rendered game objects: {}", render_count);
                                } else {
                                    log::warn!("⚠️  No Renderable components found in world");
                                }
                                
                                // End UI frame and submit to render pass
                                let result = unsafe {
                                    ui_renderer.end_frame(&*device_ptr, &*queue_ptr)
                                };
                                
                                if let Err(e) = result {
                                    log::warn!("Failed to end UI frame: {}", e);
                                } else if let Err(e) = ui_renderer.submit_to_render_pass(&mut render_pass) {
                                    log::warn!("Failed to submit UI to render pass: {}", e);
                                } else {
                                    log::debug!("UI elements rendered successfully: toolbar + panels");
                                }
                            }
                        }
                        
                        // Submit the rendering work
                        render_context.queue().submit(std::iter::once(encoder.finish()));
                        
                        // Present the frame
                        render_context.end_frame(Some(output));
                    }
                    Ok(None) => {
                        log::warn!("No surface texture available");
                    }
                    Err(e) => {
                        log::warn!("Failed to begin frame: {}", e);
                    }
                }
            }
        });
        
        // Store the UI state in the world for click detection
        world.add_resource(UiState {
            toolbar_buttons,
            hovered_button: None,
            last_mouse_position: Vec2::ZERO,
        });
        
        log::debug!("UI render system executed with clear screen");
    }
    
    Ok(())
}

/// Input system that processes user input and stores events for later processing
/// 
/// This system handles input events from the window and stores them in InputEvents
/// resource for processing after the UI is rendered.
pub fn input_system(world: &mut World, event: &winit::event::WindowEvent) -> Result<()> {
    use lumina_ui::InputEvent;
    use glam::Vec2;
    
    // Store input events in the InputEvents resource
    match event {
        winit::event::WindowEvent::CursorMoved { position, .. } => {
            let mouse_pos = Vec2::new(position.x as f32, position.y as f32);
            
            // Store mouse position for UI processing
            world.with_resource_mut::<InputEvents, _>(|mut input_events_opt| {
                if let Some(input_events) = input_events_opt.as_mut() {
                    input_events.set_mouse_position(mouse_pos);
                }
            });
            
            // Also create UI event for immediate UI framework processing
            let ui_event = InputEvent::MouseMove {
                position: mouse_pos,
                delta: Vec2::ZERO, // Could be improved with delta tracking
            };
            
            world.with_resource_mut::<UiFramework, _>(|mut ui_framework_opt| {
                if let Some(ui_framework) = ui_framework_opt.as_mut() {
                    ui_framework.handle_input(ui_event);
                }
            });
        }
        winit::event::WindowEvent::MouseInput { state, button, .. } => {
            if let winit::event::ElementState::Pressed = state {
                // Store the click for later processing
                world.with_resource_mut::<InputEvents, _>(|mut input_events_opt| {
                    if let Some(input_events) = input_events_opt.as_mut() {
                        if let Some(mouse_pos) = input_events.mouse_position() {
                            input_events.add_click(mouse_pos, *button, true);
                            println!("🖱️ CLICK STORED at position: {:?}", mouse_pos);
                            log::info!("🖱️ Click stored at position: {:?}", mouse_pos);
                        }
                    }
                });
                
                // Also forward to UI framework for general UI handling 
                let mouse_button = match button {
                    winit::event::MouseButton::Left => lumina_ui::MouseButton::Left,
                    winit::event::MouseButton::Right => lumina_ui::MouseButton::Right,
                    winit::event::MouseButton::Middle => lumina_ui::MouseButton::Middle,
                    _ => return Ok(()),
                };
                
                world.with_resource::<InputEvents, _>(|input_events_opt| {
                    if let Some(input_events) = input_events_opt {
                        if let Some(mouse_pos) = input_events.mouse_position {
                            let ui_event = InputEvent::MouseClick {
                                button: mouse_button,
                                position: mouse_pos,
                                modifiers: lumina_ui::Modifiers::default(),
                            };
                            
                            world.with_resource_mut::<UiFramework, _>(|mut ui_framework_opt| {
                                if let Some(ui_framework) = ui_framework_opt.as_mut() {
                                    ui_framework.handle_input(ui_event);
                                }
                            });
                        }
                    }
                });
            }
        }
        _ => {}
    };
    
    Ok(())
}

/// Input processing system that handles queued input events after UI is rendered
/// 
/// This system processes input events that were stored during the event loop
/// and checks them against the current UI state.
pub fn process_input_events(world: &mut World) -> Result<Vec<String>> {
    let mut actions = Vec::new();
    
    // Get any pending mouse clicks
    let clicks = world.with_resource_mut::<InputEvents, _>(|mut input_events_opt| {
        if let Some(input_events) = input_events_opt.as_mut() {
            let clicks = input_events.mouse_clicks().to_vec();
            input_events.clear();
            clicks
        } else {
            Vec::new()
        }
    });
    
    // Process clicks against UI state
    if !clicks.is_empty() {
        world.with_resource::<UiState, _>(|ui_state_opt| {
            if let Some(ui_state) = ui_state_opt {
                for click in &clicks {
                    for (_index, ui_button) in ui_state.toolbar_buttons.iter().enumerate() {
                        if ui_button.contains_point(click.position) {
                            println!("🖱️ BUTTON CLICKED: {} ({})", ui_button.label, ui_button.action);
                            log::info!("🖱️ Button clicked: {} ({})", ui_button.label, ui_button.action);
                            actions.push(ui_button.action.clone());
                        }
                    }
                }
            }
        });
    }
    
    Ok(actions)
}

/// Handle UI button clicks (legacy function for backwards compatibility)
/// 
/// This system checks for button clicks based on mouse position and
/// triggers appropriate actions.
pub fn handle_ui_click(world: &mut World, mouse_pos: Vec2) -> Result<Option<String>> {
    world.with_resource::<UiState, _>(|ui_state_opt| {
        if let Some(ui_state) = ui_state_opt {
            for (_index, button) in ui_state.toolbar_buttons.iter().enumerate() {
                if button.contains_point(mouse_pos) {
                    log::info!("🖱️ Button clicked: {} ({})", button.label, button.action);
                    return Ok(Some(button.action.clone()));
                }
            }
        }
        Ok(None)
    })
}

/// Update UI hover state based on mouse position
/// 
/// This system updates which button is being hovered over
/// for visual feedback.
pub fn update_ui_hover(world: &mut World, mouse_pos: Vec2) -> Result<()> {
    world.with_resource_mut::<UiState, _>(|mut ui_state_opt| {
        if let Some(ui_state) = ui_state_opt.as_mut() {
            ui_state.last_mouse_position = mouse_pos;
            
            // Find which button is being hovered
            let mut hovered = None;
            for (index, button) in ui_state.toolbar_buttons.iter().enumerate() {
                if button.contains_point(mouse_pos) {
                    hovered = Some(index);
                    break;
                }
            }
            ui_state.hovered_button = hovered;
        }
    });
    Ok(())
}

/// UI event handler system that processes UI-generated events
/// 
/// This system processes events generated by UI interactions
/// and applies them to ECS components.
pub fn ui_event_handler_system(_world: &mut World) -> Result<()> {
    // Process UI events and modify ECS state
    // For example:
    // - Button clicks that start/pause the game
    // - Slider changes that modify game settings
    // - Menu selections that change game state
    
    // This is where the UI -> ECS data flow happens
    
    // For now, this is a placeholder - in a real implementation,
    // you would have an event queue that collects UI events
    // and processes them here to modify ECS components
    
    Ok(())
}

/// Simple renderable component for game objects
/// Games can add this component to entities to make them render as colored rectangles
#[derive(Debug, Clone)]
pub struct Renderable {
    pub position: Vec2,
    pub size: Vec2,
    pub color: Vec4,
}