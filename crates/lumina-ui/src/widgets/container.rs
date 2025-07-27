//! Container widget for layout management

use crate::{
    Widget, WidgetId, LayoutConstraints, InputEvent, InputResponse, 
    UiRenderer, Rect, widgets::BaseWidget,
    layout::LayoutResult,
};
use glam::Vec2;

/// Container widget for managing child widget layouts
#[derive(Debug)]
pub struct Container {
    /// Base widget properties
    base: BaseWidget,
    /// Child widgets
    children: Vec<WidgetId>,
}

impl Container {
    /// Create new container
    pub fn new() -> Self {
        Self {
            base: BaseWidget::default(),
            children: Vec::new(),
        }
    }
}

impl Widget for Container {
    fn id(&self) -> WidgetId {
        self.base.id
    }
    
    fn layout_constraints(&self) -> LayoutConstraints {
        self.base.constraints.clone()
    }
    
    fn layout(&mut self, available_space: Vec2) -> LayoutResult {
        let bounds = Rect::new(0.0, 0.0, available_space.x, available_space.y);
        LayoutResult {
            bounds,
            overflow: false,
            content_size: available_space,
        }
    }
    
    fn handle_input(&mut self, _input: &InputEvent) -> InputResponse {
        InputResponse::NotHandled
    }
    
    fn render(&self, _renderer: &mut UiRenderer, _bounds: Rect) {
        // Render background if specified
    }
    
    fn children(&self) -> Vec<WidgetId> {
        self.children.clone()
    }
    
    fn add_child(&mut self, child_id: WidgetId) {
        self.children.push(child_id);
    }
    
    fn remove_child(&mut self, child_id: WidgetId) {
        self.children.retain(|&id| id != child_id);
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}