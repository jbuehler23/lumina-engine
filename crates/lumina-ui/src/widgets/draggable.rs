//! Draggable widget for implementing drag-and-drop functionality

use crate::{
    Widget, WidgetId, LayoutConstraints, InputEvent, InputResponse, 
    UiRenderer, Rect, widgets::BaseWidget, input::DragData,
    layout::LayoutResult,
};
use glam::{Vec2, Vec4};

/// A draggable container widget that can be dragged around the screen
#[derive(Debug)]
pub struct Draggable {
    /// Base widget properties
    base: BaseWidget,
    /// Content widget ID (what's being dragged)
    content: Option<WidgetId>,
    /// Drag data to transfer when dragging
    drag_data: DragData,
    /// Whether this widget is currently being dragged
    is_dragging: bool,
    /// Offset from mouse to widget position when dragging starts
    drag_offset: Vec2,
    /// Original position before dragging started
    original_position: Vec2,
    /// Background color
    background_color: Vec4,
    /// Border color when dragging
    drag_border_color: Vec4,
    /// Whether to constrain dragging to parent bounds
    constrain_to_parent: bool,
}

impl Draggable {
    /// Create a new draggable widget
    pub fn new(drag_data: DragData) -> Self {
        Self {
            base: BaseWidget::default(),
            content: None,
            drag_data,
            is_dragging: false,
            drag_offset: Vec2::ZERO,
            original_position: Vec2::ZERO,
            background_color: Vec4::new(0.2, 0.2, 0.3, 0.9),
            drag_border_color: Vec4::new(0.4, 0.7, 1.0, 1.0),
            constrain_to_parent: true,
        }
    }
    
    /// Set the content widget
    pub fn content(mut self, content_id: WidgetId) -> Self {
        self.content = Some(content_id);
        self
    }
    
    /// Set the background color
    pub fn background_color(mut self, color: Vec4) -> Self {
        self.background_color = color;
        self
    }
    
    /// Set the drag border color
    pub fn drag_border_color(mut self, color: Vec4) -> Self {
        self.drag_border_color = color;
        self
    }
    
    /// Set whether to constrain dragging to parent bounds
    pub fn constrain_to_parent(mut self, constrain: bool) -> Self {
        self.constrain_to_parent = constrain;
        self
    }
    
    /// Get the drag data
    pub fn drag_data(&self) -> &DragData {
        &self.drag_data
    }
    
    /// Check if currently being dragged
    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }
    
    /// Set the position (for external positioning)
    pub fn set_position(&mut self, position: Vec2) {
        self.base.style.position = Some([position.x, position.y]);
    }
    
    /// Get current position
    pub fn position(&self) -> Vec2 {
        if let Some(pos) = self.base.style.position {
            Vec2::new(pos[0], pos[1])
        } else {
            Vec2::ZERO
        }
    }
}

impl Widget for Draggable {
    fn id(&self) -> WidgetId {
        self.base.id
    }
    
    fn layout_constraints(&self) -> LayoutConstraints {
        self.base.constraints.clone()
    }
    
    fn layout(&mut self, available_space: Vec2) -> LayoutResult {
        // Use fixed size for draggable elements
        let size = self.base.style.size.map(|s| Vec2::new(s[0], s[1]))
            .unwrap_or(Vec2::new(100.0, 60.0)); // Default size
        
        let position = self.base.style.position.map(|p| Vec2::new(p[0], p[1]))
            .unwrap_or(Vec2::ZERO);
        
        let bounds = Rect::new(position.x, position.y, size.x, size.y);
        
        LayoutResult {
            bounds,
            overflow: position.x + size.x > available_space.x || position.y + size.y > available_space.y,
            content_size: size,
        }
    }
    
    fn handle_input(&mut self, input: &InputEvent) -> InputResponse {
        if !self.base.enabled {
            return InputResponse::NotHandled;
        }
        
        match input {
            InputEvent::MouseDown { position, .. } => {
                // Start dragging
                self.is_dragging = true;
                self.original_position = self.position();
                self.drag_offset = *position - self.original_position;
                InputResponse::Handled
            }
            
            InputEvent::MouseMove { position, .. } => {
                if self.is_dragging {
                    // Update position while dragging
                    let new_position = *position - self.drag_offset;
                    
                    // Optionally constrain to parent bounds
                    let final_position = if self.constrain_to_parent {
                        // TODO: Get parent bounds and constrain
                        new_position
                    } else {
                        new_position
                    };
                    
                    self.set_position(final_position);
                    InputResponse::Handled
                } else {
                    InputResponse::NotHandled
                }
            }
            
            InputEvent::MouseUp { .. } => {
                if self.is_dragging {
                    self.is_dragging = false;
                    InputResponse::Handled
                } else {
                    InputResponse::NotHandled
                }
            }
            
            InputEvent::DragStart { .. } => {
                // Already handled by MouseDown
                InputResponse::Handled
            }
            
            InputEvent::DragUpdate { position, .. } => {
                if self.is_dragging {
                    let new_position = *position - self.drag_offset;
                    self.set_position(new_position);
                    InputResponse::Handled
                } else {
                    InputResponse::NotHandled
                }
            }
            
            InputEvent::DragEnd { .. } => {
                if self.is_dragging {
                    self.is_dragging = false;
                    InputResponse::Handled
                } else {
                    InputResponse::NotHandled
                }
            }
            
            InputEvent::Drop { data, position } => {
                // Handle drop events
                println!("Drop received at {:?}: {:?}", position, data);
                InputResponse::Handled
            }
            
            _ => InputResponse::NotHandled
        }
    }
    
    fn render(&self, renderer: &mut UiRenderer, bounds: Rect) {
        if !self.base.visible {
            return;
        }
        
        // Choose color based on drag state
        let bg_color = if self.is_dragging {
            // Slightly transparent when dragging
            Vec4::new(
                self.background_color.x,
                self.background_color.y,
                self.background_color.z,
                self.background_color.w * 0.8,
            )
        } else {
            self.background_color
        };
        
        // Draw background
        renderer.draw_rounded_rect(bounds, bg_color, 8.0);
        
        // Draw border when dragging
        if self.is_dragging {
            let border_bounds = Rect::new(
                bounds.position.x - 1.0,
                bounds.position.y - 1.0,
                bounds.size.x + 2.0,
                bounds.size.y + 2.0,
            );
            renderer.draw_rounded_rect(border_bounds, self.drag_border_color, 8.0);
        }
    }
    
    fn children(&self) -> Vec<WidgetId> {
        if let Some(content) = self.content {
            vec![content]
        } else {
            Vec::new()
        }
    }
    
    fn add_child(&mut self, child_id: WidgetId) {
        self.content = Some(child_id);
    }
    
    fn remove_child(&mut self, child_id: WidgetId) {
        if self.content == Some(child_id) {
            self.content = None;
        }
    }
}

impl Default for Draggable {
    fn default() -> Self {
        Self::new(DragData::Text("Default".to_string()))
    }
}

/// Builder for creating draggable widgets with a fluent API
pub struct DraggableBuilder {
    draggable: Draggable,
}

impl DraggableBuilder {
    /// Create a new draggable builder
    pub fn new(drag_data: DragData) -> Self {
        Self {
            draggable: Draggable::new(drag_data),
        }
    }
    
    /// Set the content widget
    pub fn content(mut self, content_id: WidgetId) -> Self {
        self.draggable = self.draggable.content(content_id);
        self
    }
    
    /// Set the background color
    pub fn background_color(mut self, color: Vec4) -> Self {
        self.draggable = self.draggable.background_color(color);
        self
    }
    
    /// Set the drag border color
    pub fn drag_border_color(mut self, color: Vec4) -> Self {
        self.draggable = self.draggable.drag_border_color(color);
        self
    }
    
    /// Set whether to constrain dragging to parent bounds
    pub fn constrain_to_parent(mut self, constrain: bool) -> Self {
        self.draggable = self.draggable.constrain_to_parent(constrain);
        self
    }
    
    /// Build the draggable widget
    pub fn build(self) -> Draggable {
        self.draggable
    }
}

/// Convenience function for creating a draggable widget
pub fn draggable(drag_data: DragData) -> DraggableBuilder {
    DraggableBuilder::new(drag_data)
}

/// Create a draggable node for visual scripting
pub fn draggable_node(node_type: String, label: String) -> DraggableBuilder {
    DraggableBuilder::new(DragData::NodeType(node_type))
        .background_color(Vec4::new(0.15, 0.15, 0.25, 0.95))
        .drag_border_color(Vec4::new(0.3, 0.6, 1.0, 1.0))
}