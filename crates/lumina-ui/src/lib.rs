//! Lumina UI Framework
//! 
//! A pure Rust UI framework built on WGPU for game-first user interfaces.
//! Designed to work seamlessly with the Lumina Engine while providing
//! a modern, type-safe, and performant UI development experience.

#![warn(missing_docs)]

pub mod widgets;
pub mod layout;
pub mod theming;
pub mod input;
pub mod editor;
pub mod error;

#[cfg(target_arch = "wasm32")]
pub mod web;

// Re-export commonly used types  
pub use widgets::{Button, Panel, Text, TextInput, Canvas, Container, Draggable};
pub use layout::{LayoutConstraints, LayoutEngine, Alignment, HorizontalAlign, VerticalAlign};
pub use theming::Theme;
pub use input::{InputEvent, InputResponse, MouseButton, KeyCode, Modifiers, InputHandler, DragData};
pub use error::{UiError, UiResult};

// Re-export rendering types from lumina-render
pub use lumina_render::{UiRenderer, Rect};

use glam::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Core UI framework context that manages the entire UI system
pub struct UiFramework {
    /// UI renderer for drawing widgets (optional since it requires WGPU setup)
    pub renderer: Option<UiRenderer>,
    /// Layout engine for positioning widgets
    pub layout_engine: LayoutEngine,
    /// Input handler for processing user interactions
    pub input_handler: InputHandler,
    /// Theme system for consistent styling
    pub theme: Theme,
    /// Current UI state
    pub state: UiState,
}

/// Global UI state containing all widgets and their data
#[derive(Debug, Default)]
pub struct UiState {
    /// All widgets in the UI hierarchy
    pub widgets: HashMap<WidgetId, Box<dyn Widget>>,
    /// Root widget IDs (top-level containers)
    pub root_widgets: Vec<WidgetId>,
    /// Currently focused widget
    pub focused_widget: Option<WidgetId>,
    /// Currently hovered widget
    pub hovered_widget: Option<WidgetId>,
    /// Widget hierarchy (parent -> children mapping)
    pub hierarchy: HashMap<WidgetId, Vec<WidgetId>>,
    /// Layout cache to avoid unnecessary recalculations
    pub layout_cache: HashMap<WidgetId, layout::LayoutResult>,
    /// Whether UI needs to be re-rendered
    pub needs_render: bool,
}

/// Unique identifier for widgets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WidgetId(pub Uuid);

impl WidgetId {
    /// Generate a new unique widget ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for WidgetId {
    fn default() -> Self {
        Self::new()
    }
}

/// Core trait that all UI widgets must implement
pub trait Widget: std::fmt::Debug {
    /// Get the unique ID of this widget
    fn id(&self) -> WidgetId;
    
    /// Get the widget's layout constraints
    fn layout_constraints(&self) -> LayoutConstraints;
    
    /// Update the widget's layout based on available space
    fn layout(&mut self, available_space: Vec2) -> layout::LayoutResult;
    
    /// Handle input events
    fn handle_input(&mut self, input: &InputEvent) -> InputResponse;
    
    /// Render the widget
    fn render(&self, renderer: &mut UiRenderer, bounds: Rect);
    
    /// Get child widgets
    fn children(&self) -> Vec<WidgetId> {
        Vec::new()
    }
    
    /// Add a child widget
    fn add_child(&mut self, _child_id: WidgetId) {
        // Default implementation does nothing
        // Override for container widgets
    }
    
    /// Remove a child widget
    fn remove_child(&mut self, _child_id: WidgetId) {
        // Default implementation does nothing
        // Override for container widgets
    }
    
    /// Check if this widget can receive focus
    fn can_focus(&self) -> bool {
        false
    }
    
    /// Called when widget gains focus
    fn on_focus_gained(&mut self) {}
    
    /// Called when widget loses focus
    fn on_focus_lost(&mut self) {}
}

// Rect is now re-exported from lumina-render

impl UiFramework {
    /// Create a new UI framework instance
    pub fn new(theme: Theme) -> Self {
        Self {
            renderer: None,
            layout_engine: LayoutEngine::new(),
            input_handler: InputHandler::new(),
            theme,
            state: UiState::default(),
        }
    }
    
    /// Set the renderer (called after WGPU setup)
    pub fn set_renderer(&mut self, renderer: UiRenderer) {
        self.renderer = Some(renderer);
    }
    
    /// Add a widget to the UI
    pub fn add_widget(&mut self, widget: Box<dyn Widget>) -> WidgetId {
        let id = widget.id();
        self.state.widgets.insert(id, widget);
        self.state.needs_render = true;
        id
    }
    
    /// Add a root widget (top-level container)
    pub fn add_root_widget(&mut self, widget: Box<dyn Widget>) -> WidgetId {
        let id = self.add_widget(widget);
        self.state.root_widgets.push(id);
        id
    }
    
    /// Remove a widget from the UI
    pub fn remove_widget(&mut self, id: WidgetId) {
        if let Some(_) = self.state.widgets.remove(&id) {
            self.state.root_widgets.retain(|&widget_id| widget_id != id);
            self.state.hierarchy.remove(&id);
            
            // Remove from parent's children
            for children in self.state.hierarchy.values_mut() {
                children.retain(|&child_id| child_id != id);
            }
            
            self.state.needs_render = true;
        }
    }
    
    /// Get a widget by ID
    pub fn get_widget(&self, id: WidgetId) -> Option<&dyn Widget> {
        self.state.widgets.get(&id).map(|w| w.as_ref())
    }
    
    /// Get a mutable widget by ID  
    pub fn get_widget_mut(&mut self, id: WidgetId) -> Option<&mut (dyn Widget + '_)> {
        if let Some(widget) = self.state.widgets.get_mut(&id) {
            Some(widget.as_mut())
        } else {
            None
        }
    }
    
    /// Add a child widget to a parent widget
    pub fn add_child_to_parent(&mut self, parent_id: WidgetId, child_id: WidgetId) {
        // Add to parent widget
        if let Some(parent_widget) = self.state.widgets.get_mut(&parent_id) {
            parent_widget.add_child(child_id);
        }
        
        // Add to hierarchy tracking
        self.state.hierarchy.entry(parent_id).or_insert_with(Vec::new).push(child_id);
        self.state.needs_render = true;
    }
    
    /// Process input events
    pub fn handle_input(&mut self, input: InputEvent) {
        match &input {
            InputEvent::MouseMove { position, delta: _ } => {
                // Update hovered widget
                let mut new_hovered = None;
                for &root_id in &self.state.root_widgets {
                    if let Some(hovered_id) = self.find_widget_at_position(*position, root_id) {
                        new_hovered = Some(hovered_id);
                        break;
                    }
                }
                
                if self.state.hovered_widget != new_hovered {
                    // Handle hover exit
                    if let Some(old_hovered) = self.state.hovered_widget {
                        if let Some(widget) = self.get_widget_mut(old_hovered) {
                            widget.handle_input(&InputEvent::MouseExit);
                        }
                    }
                    
                    // Handle hover enter
                    if let Some(new_hovered_id) = new_hovered {
                        if let Some(widget) = self.get_widget_mut(new_hovered_id) {
                            widget.handle_input(&InputEvent::MouseEnter);
                        }
                    }
                    
                    self.state.hovered_widget = new_hovered;
                    self.state.needs_render = true;
                }
            }
            
            InputEvent::MouseClick { position, .. } => {
                // Find clicked widget and set focus
                let mut clicked_widget = None;
                for &root_id in &self.state.root_widgets {
                    if let Some(widget_id) = self.find_widget_at_position(*position, root_id) {
                        clicked_widget = Some(widget_id);
                        break;
                    }
                }
                
                if let Some(widget_id) = clicked_widget {
                    // Handle focus change
                    if let Some(widget) = self.get_widget(widget_id) {
                        if widget.can_focus() {
                            self.set_focus(Some(widget_id));
                        }
                    }
                    
                    // Send click event to widget
                    if let Some(widget) = self.get_widget_mut(widget_id) {
                        widget.handle_input(&input);
                    }
                }
            }
            
            _ => {
                // Send other input events to focused widget
                if let Some(focused_id) = self.state.focused_widget {
                    if let Some(widget) = self.get_widget_mut(focused_id) {
                        widget.handle_input(&input);
                    }
                }
            }
        }
    }
    
    /// Set the focused widget
    pub fn set_focus(&mut self, widget_id: Option<WidgetId>) {
        if self.state.focused_widget == widget_id {
            return;
        }
        
        // Handle focus lost
        if let Some(old_focused) = self.state.focused_widget {
            if let Some(widget) = self.get_widget_mut(old_focused) {
                widget.on_focus_lost();
            }
        }
        
        // Handle focus gained
        if let Some(new_focused) = widget_id {
            if let Some(widget) = self.get_widget_mut(new_focused) {
                widget.on_focus_gained();
            }
        }
        
        self.state.focused_widget = widget_id;
        self.state.needs_render = true;
    }
    
    /// Update layout for all widgets
    pub fn update_layout(&mut self, available_space: Vec2) {
        // Clear layout cache
        self.state.layout_cache.clear();
        
        // Layout root widgets with improved spacing
        self.layout_root_widgets(available_space);
        
        self.state.needs_render = true;
    }
    
    /// Layout root widgets with better positioning logic
    fn layout_root_widgets(&mut self, available_space: Vec2) {
        let root_widgets = self.state.root_widgets.clone();
        
        if root_widgets.is_empty() {
            return;
        }
        
        // For now, use a simple grid-like layout for root widgets
        // In a full editor, we'd have a proper docking system
        let padding = 12.0;
        let menu_height = 60.0;
        let bottom_panel_height = 200.0;
        
        for (index, &root_id) in root_widgets.iter().enumerate() {
            if let Some(widget) = self.state.widgets.get_mut(&root_id) {
                let layout_result = match index {
                    // Menu bar - spans the top
                    0 => {
                        let bounds = Rect::new(
                            padding,
                            padding,
                            available_space.x - padding * 2.0,
                            menu_height
                        );
                        layout::LayoutResult {
                            bounds,
                            overflow: false,
                            content_size: bounds.size,
                        }
                    },
                    // Left panel
                    1 => {
                        let panel_width = 350.0;
                        let bounds = Rect::new(
                            padding,
                            menu_height + padding * 2.0,
                            panel_width,
                            available_space.y - menu_height - bottom_panel_height - padding * 4.0
                        );
                        layout::LayoutResult {
                            bounds,
                            overflow: false,
                            content_size: bounds.size,
                        }
                    },
                    // Center panel (scene)
                    2 => {
                        let left_width = 350.0;
                        let right_width = 350.0;
                        let bounds = Rect::new(
                            left_width + padding * 2.0,
                            menu_height + padding * 2.0,
                            available_space.x - left_width - right_width - padding * 4.0,
                            available_space.y - menu_height - bottom_panel_height - padding * 4.0
                        );
                        layout::LayoutResult {
                            bounds,
                            overflow: false,
                            content_size: bounds.size,
                        }
                    },
                    // Right panel
                    3 => {
                        let panel_width = 350.0;
                        let bounds = Rect::new(
                            available_space.x - panel_width - padding,
                            menu_height + padding * 2.0,
                            panel_width,
                            available_space.y - menu_height - bottom_panel_height - padding * 4.0
                        );
                        layout::LayoutResult {
                            bounds,
                            overflow: false,
                            content_size: bounds.size,
                        }
                    },
                    // Bottom left panel (console)
                    4 => {
                        let left_width = available_space.x * 0.6;
                        let bounds = Rect::new(
                            padding,
                            available_space.y - bottom_panel_height - padding,
                            left_width - padding,
                            bottom_panel_height
                        );
                        layout::LayoutResult {
                            bounds,
                            overflow: false,
                            content_size: bounds.size,
                        }
                    },
                    // Bottom right panel (visual scripting)
                    _ => {
                        let left_width = available_space.x * 0.6;
                        let bounds = Rect::new(
                            left_width + padding,
                            available_space.y - bottom_panel_height - padding,
                            available_space.x - left_width - padding * 2.0,
                            bottom_panel_height
                        );
                        layout::LayoutResult {
                            bounds,
                            overflow: false,
                            content_size: bounds.size,
                        }
                    }
                };
                
                self.state.layout_cache.insert(root_id, layout_result.clone());
                
                // Layout children within the parent bounds
                if let Some(children) = self.state.hierarchy.get(&root_id).cloned() {
                    let parent_bounds = layout_result.bounds;
                    let mut y_offset = 10.0; // Start with padding from top
                    
                    for child_id in children {
                        if let Some(child_widget) = self.state.widgets.get_mut(&child_id) {
                            // Layout child within parent's content area
                            let available_space = Vec2::new(parent_bounds.size.x - 20.0, parent_bounds.size.y - y_offset);
                            let mut child_layout = child_widget.layout(available_space);
                            
                            // Position child relative to parent with vertical stacking
                            child_layout.bounds.position.x = parent_bounds.position.x + 10.0; // Left padding
                            child_layout.bounds.position.y = parent_bounds.position.y + y_offset;
                            
                            self.state.layout_cache.insert(child_id, child_layout.clone());
                            
                            // Move y_offset down for next child
                            y_offset += child_layout.bounds.size.y + 5.0; // Child height + spacing
                        }
                    }
                }
            }
        }
    }
    
    /// Render the entire UI
    pub fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>, queue: &wgpu::Queue) {
        if !self.state.needs_render {
            return;
        }
        
        if self.renderer.is_none() {
            return;
        }
        
        // Clone what we need to avoid borrow checker issues
        let root_widgets = self.state.root_widgets.clone();
        let layout_cache = self.state.layout_cache.clone();
        let hierarchy = self.state.hierarchy.clone();
        
        // Begin rendering
        self.renderer.as_mut().unwrap().begin_frame(queue);
        
        // Render all widgets
        for root_id in root_widgets {
            self.render_widget_hierarchy(root_id, &layout_cache, &hierarchy);
        }
        
        // End rendering and submit draw commands to render pass
        let renderer = self.renderer.as_mut().unwrap();
        renderer.end_frame(queue);
        renderer.submit_to_render_pass(render_pass);
        
        self.state.needs_render = false;
    }
    
    /// Render a widget and its children recursively using cached data
    fn render_widget_hierarchy(&mut self, widget_id: WidgetId, layout_cache: &std::collections::HashMap<WidgetId, layout::LayoutResult>, hierarchy: &std::collections::HashMap<WidgetId, Vec<WidgetId>>) {
        if let Some(layout) = layout_cache.get(&widget_id) {
            if let Some(widget) = self.state.widgets.get(&widget_id) {
                if let Some(renderer) = &mut self.renderer {
                    widget.render(renderer, layout.bounds);
                }
                
                // Render children
                if let Some(children) = hierarchy.get(&widget_id) {
                    for &child_id in children {
                        self.render_widget_hierarchy(child_id, layout_cache, hierarchy);
                    }
                }
            }
        }
    }
    
    /// Find widget at a specific position
    fn find_widget_at_position(&self, position: Vec2, widget_id: WidgetId) -> Option<WidgetId> {
        if let Some(layout) = self.state.layout_cache.get(&widget_id) {
            if layout.bounds.contains(position) {
                // Check children first (they're on top)
                if let Some(children) = self.state.hierarchy.get(&widget_id) {
                    for &child_id in children.iter().rev() {
                        if let Some(found) = self.find_widget_at_position(position, child_id) {
                            return Some(found);
                        }
                    }
                }
                
                // Return this widget if no children match
                return Some(widget_id);
            }
        }
        
        None
    }
    
    
}