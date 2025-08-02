//! Input event storage and management
//! 
//! This module provides the core event storage system that queues input
//! events for frame-consistent processing, following Bevy's approach.

use glam::Vec2;
use winit::event::MouseButton;

/// Resource for storing input events until they can be processed
/// 
/// This follows Bevy's pattern of collecting input during the event loop
/// and processing it in systems during the frame update.
#[derive(Default, Debug)]
pub struct InputEvents {
    /// Mouse clicks that occurred this frame
    pub mouse_clicks: Vec<MouseClick>,
    /// Current mouse position
    pub mouse_position: Option<Vec2>,
    /// Mouse movement delta since last frame
    pub mouse_delta: Vec2,
    /// Mouse wheel scroll delta
    pub scroll_delta: Vec2,
}

/// A mouse click event with position and button information
#[derive(Debug, Clone)]
pub struct MouseClick {
    /// The mouse button that was clicked
    pub button: MouseButton,
    /// The position where the click occurred
    pub position: Vec2,
    /// Whether this was a press or release
    pub pressed: bool,
}

impl InputEvents {
    /// Create a new InputEvents resource
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a mouse click event
    pub fn add_click(&mut self, position: Vec2, button: MouseButton, pressed: bool) {
        self.mouse_clicks.push(MouseClick {
            button,
            position,
            pressed,
        });
    }
    
    /// Set the current mouse position
    pub fn set_mouse_position(&mut self, position: Vec2) {
        if let Some(old_pos) = self.mouse_position {
            self.mouse_delta = position - old_pos;
        }
        self.mouse_position = Some(position);
    }
    
    /// Add scroll wheel delta
    pub fn add_scroll(&mut self, delta: Vec2) {
        self.scroll_delta += delta;
    }
    
    /// Clear all events (called after processing)
    pub fn clear(&mut self) {
        self.mouse_clicks.clear();
        self.mouse_delta = Vec2::ZERO;
        self.scroll_delta = Vec2::ZERO;
    }
    
    /// Get all mouse clicks that occurred this frame
    pub fn mouse_clicks(&self) -> &[MouseClick] {
        &self.mouse_clicks
    }
    
    /// Get pressed mouse clicks only
    pub fn mouse_presses(&self) -> impl Iterator<Item = &MouseClick> {
        self.mouse_clicks.iter().filter(|click| click.pressed)
    }
    
    /// Get released mouse clicks only
    pub fn mouse_releases(&self) -> impl Iterator<Item = &MouseClick> {
        self.mouse_clicks.iter().filter(|click| !click.pressed)
    }
    
    /// Get the current mouse position
    pub fn mouse_position(&self) -> Option<Vec2> {
        self.mouse_position
    }
    
    /// Get the mouse movement delta since last frame
    pub fn mouse_delta(&self) -> Vec2 {
        self.mouse_delta
    }
    
    /// Get the scroll wheel delta since last frame
    pub fn scroll_delta(&self) -> Vec2 {
        self.scroll_delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::event::MouseButton;
    
    #[test]
    fn test_mouse_click_storage() {
        let mut events = InputEvents::new();
        let pos = Vec2::new(100.0, 200.0);
        
        events.add_click(pos, MouseButton::Left, true);
        
        assert_eq!(events.mouse_clicks().len(), 1);
        assert_eq!(events.mouse_clicks()[0].position, pos);
        assert_eq!(events.mouse_clicks()[0].button, MouseButton::Left);
        assert!(events.mouse_clicks()[0].pressed);
    }
    
    #[test]
    fn test_mouse_position_tracking() {
        let mut events = InputEvents::new();
        
        events.set_mouse_position(Vec2::new(10.0, 20.0));
        assert_eq!(events.mouse_position(), Some(Vec2::new(10.0, 20.0)));
        assert_eq!(events.mouse_delta(), Vec2::ZERO);
        
        events.set_mouse_position(Vec2::new(15.0, 25.0));
        assert_eq!(events.mouse_delta(), Vec2::new(5.0, 5.0));
    }
    
    #[test]
    fn test_event_filtering() {
        let mut events = InputEvents::new();
        let pos = Vec2::new(50.0, 75.0);
        
        events.add_click(pos, MouseButton::Left, true);
        events.add_click(pos, MouseButton::Right, false);
        
        assert_eq!(events.mouse_presses().count(), 1);
        assert_eq!(events.mouse_releases().count(), 1);
    }
    
    #[test]
    fn test_clear_events() {
        let mut events = InputEvents::new();
        
        events.add_click(Vec2::ZERO, MouseButton::Left, true);
        events.add_scroll(Vec2::new(1.0, 2.0));
        events.set_mouse_position(Vec2::new(10.0, 20.0));
        
        events.clear();
        
        assert!(events.mouse_clicks().is_empty());
        assert_eq!(events.scroll_delta(), Vec2::ZERO);
        assert_eq!(events.mouse_delta(), Vec2::ZERO);
        // Note: mouse_position is preserved across clears
        assert_eq!(events.mouse_position(), Some(Vec2::new(10.0, 20.0)));
    }
}