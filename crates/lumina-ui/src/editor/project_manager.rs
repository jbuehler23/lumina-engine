//! Project manager component

use crate::{Widget, WidgetId, LayoutConstraints, LayoutResult, InputEvent, InputResponse, UiRenderer, Rect};
use glam::Vec2;

/// Project manager widget
#[derive(Debug)]
pub struct ProjectManager {
    id: WidgetId,
}

impl ProjectManager {
    pub fn new() -> Self {
        Self { id: WidgetId::new() }
    }
}

impl Widget for ProjectManager {
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
    fn render(&self, _renderer: &mut UiRenderer, _bounds: Rect) {}
}