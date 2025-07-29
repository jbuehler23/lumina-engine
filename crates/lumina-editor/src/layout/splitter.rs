//! Splitter component for resizing docked panels

use glam::Vec2;
use lumina_ui::{UiFramework, InputEvent};

use super::types::{SplitId, Rect, SplitDirection, DockingTheme};

/// Draggable splitter for resizing panels
pub struct Splitter {
    pub id: SplitId,
    pub bounds: Rect,
    pub direction: SplitDirection,
    pub dragging: bool,
    pub hover: bool,
    pub theme: DockingTheme,
}

impl Splitter {
    /// Create a new splitter
    pub fn new(id: SplitId, direction: SplitDirection) -> Self {
        Self {
            id,
            bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
            direction,
            dragging: false,
            hover: false,
            theme: DockingTheme::default(),
        }
    }

    /// Update splitter bounds
    pub fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    /// Render the splitter
    pub fn render(&self, _ui: &mut UiFramework) {
        // TODO: Implement splitter rendering
        // This would draw a thin line/bar that users can drag to resize
    }

    /// Handle input events, returns new ratio if dragging occurred
    pub fn handle_input(&mut self, _event: &InputEvent) -> Option<f32> {
        // TODO: Implement splitter input handling
        // This would handle mouse hover, click, and drag events
        None
    }

    /// Check if a point is within the splitter's interaction area
    pub fn contains_point(&self, point: Vec2) -> bool {
        // Expand the interaction area slightly for easier clicking
        let interaction_bounds = match self.direction {
            SplitDirection::Horizontal => Rect::new(
                self.bounds.x - 2.0,
                self.bounds.y,
                self.bounds.width + 4.0,
                self.bounds.height,
            ),
            SplitDirection::Vertical => Rect::new(
                self.bounds.x,
                self.bounds.y - 2.0,
                self.bounds.width,
                self.bounds.height + 4.0,
            ),
        };
        
        interaction_bounds.contains(point)
    }
}