//! Canvas widget for custom drawing

use crate::{
    Widget, WidgetId, LayoutConstraints, InputEvent, InputResponse, 
    UiRenderer, Rect, widgets::BaseWidget,
    layout::LayoutResult,
};
use glam::Vec2;

/// Canvas widget for custom drawing operations
#[derive(Debug)]
pub struct Canvas {
    /// Base widget properties
    base: BaseWidget,
}

impl Canvas {
    /// Create new canvas
    pub fn new() -> Self {
        Self {
            base: BaseWidget::default(),
        }
    }
}

impl Widget for Canvas {
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
        // TODO: Implement canvas rendering
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}