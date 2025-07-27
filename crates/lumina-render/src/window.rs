//! Window management utilities
//!
//! Provides window creation and management functionality.

use crate::{WindowConfig, RenderResult};
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use std::sync::Arc;

/// Window manager for the application
pub struct WindowManager {
    /// Window instance
    pub window: Arc<Window>,
    /// Event loop
    pub event_loop: Option<EventLoop<()>>,
}

impl WindowManager {
    /// Create a new window with the given configuration
    pub fn new(config: WindowConfig) -> RenderResult<Self> {
        let event_loop = EventLoop::new().map_err(|e| {
            crate::RenderError::InvalidOperation(format!("Failed to create event loop: {}", e))
        })?;

        let window = WindowBuilder::new()
            .with_title(&config.title)
            .with_inner_size(winit::dpi::LogicalSize::new(config.size.0, config.size.1))
            .with_resizable(config.resizable)
            .with_decorations(config.decorations)
            .build(&event_loop)
            .map_err(|e| {
                crate::RenderError::InvalidOperation(format!("Failed to create window: {}", e))
            })?;

        let window = Arc::new(window);

        Ok(Self {
            window,
            event_loop: Some(event_loop),
        })
    }

    /// Get the window instance
    pub fn window(&self) -> Arc<Window> {
        self.window.clone()
    }

    /// Take the event loop (can only be done once)
    pub fn take_event_loop(&mut self) -> Option<EventLoop<()>> {
        self.event_loop.take()
    }
}