//! Panel widget - a basic container for other widgets

use crate::{
    Widget, WidgetId, LayoutConstraints, InputEvent, InputResponse, 
    UiRenderer, Rect, widgets::{BaseWidget, WidgetStyle},
    layout::LayoutResult,
};
use glam::Vec2;

/// Panel widget - a basic container
#[derive(Debug)]
pub struct Panel {
    /// Base widget properties
    base: BaseWidget,
    /// Child widgets
    children: Vec<WidgetId>,
}

impl Panel {
    /// Create a new panel
    pub fn new() -> Self {
        Self {
            base: BaseWidget::default(),
            children: Vec::new(),
        }
    }
    
    /// Set the panel style
    pub fn style(mut self, style: WidgetStyle) -> Self {
        self.base.style = style;
        self
    }
}

impl Widget for Panel {
    fn id(&self) -> WidgetId {
        self.base.id
    }
    
    fn layout_constraints(&self) -> LayoutConstraints {
        self.base.constraints.clone()
    }
    
    fn layout(&mut self, available_space: Vec2) -> LayoutResult {
        let bounds = Rect::new(0.0, 0.0, available_space.x, available_space.y);
        
        let result = LayoutResult {
            bounds,
            overflow: false,
            content_size: available_space,
        };
        
        self.base.layout_cache = Some(result.clone());
        result
    }
    
    fn handle_input(&mut self, _input: &InputEvent) -> InputResponse {
        InputResponse::NotHandled
    }
    
    fn render(&self, renderer: &mut UiRenderer, bounds: Rect) {
        if !self.base.visible {
            return;
        }
        
        // Draw panel background if specified
        if let Some(bg_color) = self.base.style.background_color {
            let border_radius = self.base.style.border_radius.unwrap_or(0.0);
            renderer.draw_rounded_rect(bounds, bg_color.into(), border_radius);
        }
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

impl Default for Panel {
    fn default() -> Self {
        Self::new()
    }
}