//! Input processing systems
//! 
//! This module provides the core systems that process input events
//! and update input state resources.

use crate::{InputEvents, ButtonInput};
use glam::Vec2;
use winit::event::{WindowEvent, MouseButton, ElementState};

/// System that processes winit window events and stores them in InputEvents
/// 
/// This system should run early in the frame to collect all input events
/// from the window system before other systems process them.
pub fn collect_input_events(input_events: &mut InputEvents, event: &WindowEvent) {
    match event {
        WindowEvent::CursorMoved { position, .. } => {
            let mouse_pos = Vec2::new(position.x as f32, position.y as f32);
            input_events.set_mouse_position(mouse_pos);
        }
        WindowEvent::MouseInput { state, button, .. } => {
            if let Some(mouse_pos) = input_events.mouse_position() {
                let pressed = matches!(state, ElementState::Pressed);
                input_events.add_click(mouse_pos, *button, pressed);
            }
        }
        WindowEvent::MouseWheel { delta, .. } => {
            let scroll_delta = match delta {
                winit::event::MouseScrollDelta::LineDelta(x, y) => Vec2::new(*x, *y) * 20.0, // Arbitrary scale
                winit::event::MouseScrollDelta::PixelDelta(pos) => Vec2::new(pos.x as f32, pos.y as f32),
            };
            input_events.add_scroll(scroll_delta);
        }
        _ => {}
    }
}

/// System that processes InputEvents and updates ButtonInput resources
/// 
/// This system should run after collect_input_events to update the
/// button state based on the collected events.
pub fn update_button_input(
    input_events: &InputEvents,
    mouse_input: &mut ButtonInput<MouseButton>,
) {
    // Process mouse clicks
    for click in input_events.mouse_clicks() {
        if click.pressed {
            mouse_input.press(click.button);
        } else {
            mouse_input.release(click.button);
        }
    }
}

/// System that clears transient input state at the end of the frame
/// 
/// This system should run at the very end of the frame to clear
/// "just pressed" states and prepare for the next frame.
pub fn clear_input_state(
    input_events: &mut InputEvents,
    mouse_input: &mut ButtonInput<MouseButton>,
) {
    input_events.clear();
    mouse_input.clear_just_pressed();
}

/// Higher-level system that can process UI interactions
/// 
/// This demonstrates how you might build higher-level input processing
/// on top of the basic input resources.
pub fn process_ui_interactions<F>(
    input_events: &InputEvents,
    mouse_input: &ButtonInput<MouseButton>,
    mut on_click: F,
) where
    F: FnMut(Vec2, MouseButton),
{
    // Process clicks that just happened
    for click in input_events.mouse_presses() {
        if mouse_input.just_pressed(click.button) {
            on_click(click.position, click.button);
        }
    }
}

/// Utility function for checking if a point is within a rectangular area
pub fn point_in_rect(point: Vec2, rect_pos: Vec2, rect_size: Vec2) -> bool {
    point.x >= rect_pos.x
        && point.x <= rect_pos.x + rect_size.x
        && point.y >= rect_pos.y
        && point.y <= rect_pos.y + rect_size.y
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_collect_mouse_movement() {
        let mut input_events = InputEvents::new();
        
        // Create a dummy DeviceId - in real usage this comes from winit
        // For testing we can't easily create one, so we'll test the logic directly
        input_events.set_mouse_position(Vec2::new(100.0, 200.0));
        
        assert_eq!(input_events.mouse_position(), Some(Vec2::new(100.0, 200.0)));
    }
    
    #[test]
    fn test_collect_mouse_click() {
        let mut input_events = InputEvents::new();
        
        // First set mouse position
        input_events.set_mouse_position(Vec2::new(50.0, 75.0));
        
        // Simulate what collect_input_events would do for a mouse click
        input_events.add_click(Vec2::new(50.0, 75.0), MouseButton::Left, true);
        
        assert_eq!(input_events.mouse_clicks().len(), 1);
        assert_eq!(input_events.mouse_clicks()[0].position, Vec2::new(50.0, 75.0));
        assert!(input_events.mouse_clicks()[0].pressed);
    }
    
    #[test]
    fn test_update_button_input() {
        let mut input_events = InputEvents::new();
        let mut mouse_input = ButtonInput::new();
        
        // Add a click event
        input_events.add_click(Vec2::new(10.0, 20.0), MouseButton::Left, true);
        
        update_button_input(&input_events, &mut mouse_input);
        
        assert!(mouse_input.just_pressed(MouseButton::Left));
        assert!(mouse_input.pressed(MouseButton::Left));
    }
    
    #[test]
    fn test_point_in_rect() {
        let rect_pos = Vec2::new(10.0, 20.0);
        let rect_size = Vec2::new(100.0, 50.0);
        
        assert!(point_in_rect(Vec2::new(50.0, 30.0), rect_pos, rect_size));
        assert!(point_in_rect(Vec2::new(10.0, 20.0), rect_pos, rect_size)); // Top-left corner
        assert!(point_in_rect(Vec2::new(110.0, 70.0), rect_pos, rect_size)); // Bottom-right corner
        
        assert!(!point_in_rect(Vec2::new(5.0, 30.0), rect_pos, rect_size)); // Left of rect
        assert!(!point_in_rect(Vec2::new(50.0, 15.0), rect_pos, rect_size)); // Above rect
        assert!(!point_in_rect(Vec2::new(120.0, 30.0), rect_pos, rect_size)); // Right of rect
        assert!(!point_in_rect(Vec2::new(50.0, 80.0), rect_pos, rect_size)); // Below rect
    }
}