//! Node editor component for visual scripting

use crate::{Widget, WidgetId, LayoutConstraints, layout::LayoutResult, InputEvent, InputResponse, UiRenderer, Rect};
use glam::Vec2;

/// Node editor widget for visual scripting
#[derive(Debug)]
pub struct NodeEditor {
    id: WidgetId,
}

impl NodeEditor {
    /// Create a new node editor
    pub fn new() -> Self {
        Self { id: WidgetId::new() }
    }
}

impl Widget for NodeEditor {
    fn id(&self) -> WidgetId { self.id }
    fn layout_constraints(&self) -> LayoutConstraints { LayoutConstraints::default() }
    fn layout(&mut self, available_space: Vec2) -> LayoutResult {
        LayoutResult {
            bounds: Rect::new(0.0, 0.0, available_space.x, available_space.y),
            overflow: false,
            content_size: available_space,
        }
    }
    fn handle_input(&mut self, _input: &InputEvent) -> InputResponse { InputResponse::NotHandled }
    fn render(&self, _renderer: &mut UiRenderer, _bounds: Rect, _queue: &wgpu::Queue, _theme: &crate::Theme) {}
}