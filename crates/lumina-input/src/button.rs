//! Button input state tracking
//! 
//! This module provides Bevy-style button state tracking with
//! `just_pressed`, `pressed`, and `just_released` functionality.

use std::collections::HashSet;
use std::hash::Hash;

/// Resource for tracking button state over time (like Bevy's ButtonInput<T>)
/// 
/// This tracks the current state of buttons and provides convenient
/// methods for checking if buttons were just pressed, are currently held,
/// or were just released.
#[derive(Debug, Clone)]
pub struct ButtonInput<T: Eq + Hash + Clone> {
    /// Buttons that are currently pressed
    pressed: HashSet<T>,
    /// Buttons that were just pressed this frame
    just_pressed: HashSet<T>,
    /// Buttons that were just released this frame
    just_released: HashSet<T>,
}

impl<T: Eq + Hash + Clone> Default for ButtonInput<T> {
    fn default() -> Self {
        Self {
            pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }
}

impl<T: Eq + Hash + Clone> ButtonInput<T> {
    /// Create a new ButtonInput tracker
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Press a button (call when button press event occurs)
    pub fn press(&mut self, button: T) {
        if !self.pressed.contains(&button) {
            self.just_pressed.insert(button.clone());
        }
        self.pressed.insert(button);
    }
    
    /// Release a button (call when button release event occurs)
    pub fn release(&mut self, button: T) {
        if self.pressed.contains(&button) {
            self.just_released.insert(button.clone());
        }
        self.pressed.remove(&button);
    }
    
    /// Check if a button is currently pressed
    pub fn pressed(&self, button: T) -> bool {
        self.pressed.contains(&button)
    }
    
    /// Check if a button was just pressed this frame
    pub fn just_pressed(&self, button: T) -> bool {
        self.just_pressed.contains(&button)
    }
    
    /// Check if a button was just released this frame
    pub fn just_released(&self, button: T) -> bool {
        self.just_released.contains(&button)
    }
    
    /// Get all buttons that are currently pressed
    pub fn get_pressed(&self) -> impl Iterator<Item = &T> {
        self.pressed.iter()
    }
    
    /// Get all buttons that were just pressed this frame
    pub fn get_just_pressed(&self) -> impl Iterator<Item = &T> {
        self.just_pressed.iter()
    }
    
    /// Get all buttons that were just released this frame
    pub fn get_just_released(&self) -> impl Iterator<Item = &T> {
        self.just_released.iter()
    }
    
    /// Clear the "just pressed" and "just released" states
    /// 
    /// This should be called at the end of each frame to reset
    /// the transient state while preserving the pressed state.
    pub fn clear_just_pressed(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }
    
    /// Reset all button states
    pub fn reset(&mut self) {
        self.pressed.clear();
        self.just_pressed.clear();
        self.just_released.clear();
    }
    
    /// Get the number of buttons currently pressed
    pub fn len(&self) -> usize {
        self.pressed.len()
    }
    
    /// Check if any buttons are currently pressed
    pub fn is_empty(&self) -> bool {
        self.pressed.is_empty()
    }
}

// Convenience type aliases for common button types
pub type MouseButtonInput = ButtonInput<winit::event::MouseButton>;
pub type KeyboardInput = ButtonInput<winit::keyboard::PhysicalKey>;

#[cfg(test)]
mod tests {
    use super::*;
    use winit::event::MouseButton;
    
    #[test]
    fn test_button_press_and_release() {
        let mut input = ButtonInput::new();
        
        // Initially nothing is pressed
        assert!(!input.pressed(MouseButton::Left));
        assert!(!input.just_pressed(MouseButton::Left));
        assert!(!input.just_released(MouseButton::Left));
        
        // Press button
        input.press(MouseButton::Left);
        assert!(input.pressed(MouseButton::Left));
        assert!(input.just_pressed(MouseButton::Left));
        assert!(!input.just_released(MouseButton::Left));
        
        // Clear just_pressed state (simulating end of frame)
        input.clear_just_pressed();
        assert!(input.pressed(MouseButton::Left));
        assert!(!input.just_pressed(MouseButton::Left));
        assert!(!input.just_released(MouseButton::Left));
        
        // Release button
        input.release(MouseButton::Left);
        assert!(!input.pressed(MouseButton::Left));
        assert!(!input.just_pressed(MouseButton::Left));
        assert!(input.just_released(MouseButton::Left));
    }
    
    #[test]
    fn test_multiple_buttons() {
        let mut input = ButtonInput::new();
        
        input.press(MouseButton::Left);
        input.press(MouseButton::Right);
        
        assert_eq!(input.len(), 2);
        assert!(input.pressed(MouseButton::Left));
        assert!(input.pressed(MouseButton::Right));
        
        input.release(MouseButton::Left);
        
        assert_eq!(input.len(), 1);
        assert!(!input.pressed(MouseButton::Left));
        assert!(input.pressed(MouseButton::Right));
    }
    
    #[test]
    fn test_repeated_press() {
        let mut input = ButtonInput::new();
        
        // First press
        input.press(MouseButton::Left);
        assert!(input.just_pressed(MouseButton::Left));
        assert!(input.pressed(MouseButton::Left));
        
        // Clear just_pressed state (simulating end of frame)
        input.clear_just_pressed();
        assert!(!input.just_pressed(MouseButton::Left));
        assert!(input.pressed(MouseButton::Left)); // Still pressed
        
        // Second press while still held - should not be "just pressed" again
        input.press(MouseButton::Left);
        assert!(!input.just_pressed(MouseButton::Left)); // Only just_pressed on first press
        assert!(input.pressed(MouseButton::Left)); // Still pressed
    }
}