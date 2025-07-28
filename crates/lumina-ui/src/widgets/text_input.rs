//! Text input widget

use crate::{
    Widget, WidgetId, LayoutConstraints, InputEvent, InputResponse, 
    UiRenderer, Rect, widgets::BaseWidget,
    layout::LayoutResult,
};
use glam::Vec2;

/// Text input widget
#[derive(Debug)]
pub struct TextInput {
    /// Base widget properties
    base: BaseWidget,
    /// Current text value
    value: String,
    /// Placeholder text
    placeholder: String,
}

impl TextInput {
    /// Create new text input
    pub fn new() -> Self {
        Self {
            base: BaseWidget::default(),
            value: String::new(),
            placeholder: String::new(),
        }
    }
    
    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
}

impl Widget for TextInput {
    fn id(&self) -> WidgetId {
        self.base.id
    }
    
    fn layout_constraints(&self) -> LayoutConstraints {
        self.base.constraints.clone()
    }
    
    fn layout(&mut self, available_space: Vec2) -> LayoutResult {
        let bounds = Rect::new(0.0, 0.0, available_space.x, 32.0); // Fixed height
        LayoutResult {
            bounds,
            overflow: false,
            content_size: bounds.size,
        }
    }
    
    fn handle_input(&mut self, _input: &InputEvent) -> InputResponse {
        InputResponse::NotHandled
    }
    
    fn render(&self, _renderer: &mut UiRenderer, _bounds: Rect, _queue: &wgpu::Queue) {
        // TODO: Implement text input rendering
    }
    
    fn can_focus(&self) -> bool {
        true
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}