//! Scene view component for the editor

use crate::{Widget, WidgetId, LayoutConstraints, layout::LayoutResult, InputEvent, InputResponse, UiRenderer, Rect};
use glam::Vec2;

/// Scene view widget for visualizing and editing game scenes
#[derive(Debug)]
pub struct SceneView {
    id: WidgetId,
}

impl SceneView {
    /// Create new scene view
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
        }
    }
}

impl Widget for SceneView {
    fn id(&self) -> WidgetId {
        self.id
    }
    
    fn layout_constraints(&self) -> LayoutConstraints {
        LayoutConstraints::default()
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
    
    fn render(&self, _renderer: &mut UiRenderer, _bounds: Rect, _queue: &wgpu::Queue) {
        // TODO: Render scene view
    }
}