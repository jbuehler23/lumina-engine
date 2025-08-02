//! # Lumina Input System
//! 
//! A Bevy-inspired input handling system for the Lumina Engine.
//! 
//! This crate provides a clean, resource-based input system that follows
//! the same patterns as Bevy's input handling, with proper system ordering
//! and ECS integration.
//! 
//! ## Core Concepts
//! 
//! - **Input Resources**: Store input state between frames (like Bevy's `ButtonInput<T>`)
//! - **Event Storage**: Queue input events for frame-consistent processing
//! - **System Ordering**: Input collection → Processing → Action handling
//! - **Type Safety**: Strong typing for all input types and actions
//! 
//! ## Example Usage
//! 
//! ```rust
//! use lumina_input::{InputEvents, ButtonInput, MouseButton};
//! 
//! // Create input resources
//! let mut input_events = InputEvents::default();
//! let mut mouse_input = ButtonInput::<MouseButton>::default();
//! 
//! // Simulate a mouse press
//! mouse_input.press(MouseButton::Left);
//! 
//! // Check if button was just pressed
//! if mouse_input.just_pressed(MouseButton::Left) {
//!     println!("Left mouse button was just pressed!");
//! }
//! ```

pub mod events;
pub mod button;
pub mod systems;

// Re-exports for convenience
pub use events::*;
pub use button::*;
pub use systems::*;

// Common input types
pub use winit::event::MouseButton;
pub use winit::keyboard::{KeyCode, PhysicalKey};
pub use glam::Vec2;

/// Common result type for input operations
pub type Result<T> = std::result::Result<T, InputError>;

/// Input system errors
#[derive(Debug, thiserror::Error)]
pub enum InputError {
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
    #[error("Invalid input state: {0}")]
    InvalidState(String),
}