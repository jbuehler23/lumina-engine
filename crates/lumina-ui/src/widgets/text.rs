//! Text widget for displaying text

use crate::{
    Widget, WidgetId, LayoutConstraints, InputEvent, InputResponse, 
    UiRenderer, Rect, widgets::BaseWidget,
    layout::LayoutResult,
};
use glam::{Vec2, Vec4};

/// Text widget for displaying text
#[derive(Debug)]
pub struct Text {
    /// Base widget properties
    base: BaseWidget,
    /// Text content
    content: String,
    /// Font size
    font_size: f32,
    /// Text color
    color: Vec4,
}

impl Text {
    /// Create new text widget
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            base: BaseWidget::default(),
            content: content.into(),
            font_size: 14.0,
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
        }
    }
    
    /// Set text content
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }
    
    /// Set font size
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }
    
    /// Set text color
    pub fn color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }
}

impl Widget for Text {
    fn id(&self) -> WidgetId {
        self.base.id
    }
    
    fn layout_constraints(&self) -> LayoutConstraints {
        self.base.constraints.clone()
    }
    
    fn layout(&mut self, available_space: Vec2) -> LayoutResult {
        // Estimate text size
        let estimated_width = self.content.len() as f32 * self.font_size * 0.6;
        let estimated_height = self.font_size * 1.2;
        
        let width = estimated_width.min(available_space.x);
        let height = estimated_height.min(available_space.y);
        
        let bounds = Rect::new(0.0, 0.0, width, height);
        
        LayoutResult {
            bounds,
            overflow: estimated_width > available_space.x || estimated_height > available_space.y,
            content_size: Vec2::new(estimated_width, estimated_height),
        }
    }
    
    fn handle_input(&mut self, _input: &InputEvent) -> InputResponse {
        InputResponse::NotHandled
    }
    
    fn render(&self, renderer: &mut UiRenderer, bounds: Rect, queue: &wgpu::Queue, theme: &crate::Theme) {
        if !self.base.visible || self.content.is_empty() {
            return;
        }
        
        // Use theme's primary text color by default
        let text_color = if self.color == Vec4::new(1.0, 1.0, 1.0, 1.0) {
            theme.colors.text.primary
        } else {
            self.color
        };
        
        // Use default font (should be loaded from Inter font file)
        let font_handle = renderer.get_default_font();
        let _ = renderer.draw_text(&self.content, bounds.position, font_handle, self.font_size, text_color, queue);
    }
}