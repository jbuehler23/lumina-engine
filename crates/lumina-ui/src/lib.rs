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
pub use widgets::{Button, Panel, Text, TextInput, Canvas, Container};
pub use layout::{LayoutConstraints, LayoutEngine, Alignment, HorizontalAlign, VerticalAlign};
pub use theming::Theme;
pub use input::{InputEvent, InputResponse, MouseButton, KeyCode, Modifiers, InputHandler};
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
        
        // Layout root widgets
        for &root_id in &self.state.root_widgets.clone() {
            self.layout_widget(root_id, available_space);
        }
        
        self.state.needs_render = true;
    }
    
    /// Render the entire UI
    pub fn render(&mut self, queue: &wgpu::Queue) {
        if !self.state.needs_render {
            return;
        }
        
        // Clone root widgets to avoid borrow checker issues
        let root_widgets = self.state.root_widgets.clone();
        
        if self.renderer.is_some() {
            // Begin rendering
            self.renderer.as_mut().unwrap().begin_frame(queue);
            
            // Render root widgets
            for root_id in root_widgets {
                if let Some(layout) = self.state.layout_cache.get(&root_id).cloned() {
                    if let Some(widget) = self.state.widgets.get(&root_id) {
                        widget.render(self.renderer.as_mut().unwrap(), layout.bounds);
                        
                        // Render children recursively
                        self.render_children(root_id);
                    }
                }
            }
            
            // End rendering
            self.renderer.as_mut().unwrap().end_frame(queue);
            
            self.state.needs_render = false;
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
    
    /// Layout a specific widget
    fn layout_widget(&mut self, widget_id: WidgetId, available_space: Vec2) {
        if let Some(widget) = self.state.widgets.get_mut(&widget_id) {
            let layout_result = widget.layout(available_space);
            self.state.layout_cache.insert(widget_id, layout_result.clone());
            
            // Layout children
            let layout_bounds_size = layout_result.bounds.size;
            if let Some(children) = self.state.hierarchy.get(&widget_id).cloned() {
                for child_id in children {
                    self.layout_widget(child_id, layout_bounds_size);
                }
            }
        }
    }
    
    /// Render children of a specific widget
    fn render_children(&mut self, widget_id: WidgetId) {
        let children = self.state.hierarchy.get(&widget_id).cloned();
        if let Some(children) = children {
            for child_id in children {
                if let Some(layout) = self.state.layout_cache.get(&child_id).cloned() {
                    if let Some(widget) = self.state.widgets.get(&child_id) {
                        widget.render(self.renderer.as_mut().unwrap(), layout.bounds);
                        
                        // Recursively render grandchildren
                        self.render_children(child_id);
                    }
                }
            }
        }
    }
}