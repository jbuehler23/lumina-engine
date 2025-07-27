//! Lumina Visual Editor
//! 
//! A comprehensive visual editor for game development built with the Lumina Engine's
//! own UI framework. Demonstrates the "dogfooding" approach where the engine's tools
//! are built using the engine itself.

use anyhow::Result;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

pub mod app;
pub mod panels;
pub mod project;
pub mod ui_integration;

pub use app::EditorApp;
pub use project::EditorProject;

/// Main editor runner that handles window management and event loop
pub struct EditorRunner {
    event_loop: EventLoop<()>,
    window: Window,
}

impl EditorRunner {
    /// Create a new editor runner with default configuration
    pub fn new() -> Result<Self> {
        let event_loop = EventLoop::new()?;
        let window = WindowBuilder::new()
            .with_title("Lumina Engine - Visual Editor")
            .with_inner_size(winit::dpi::LogicalSize::new(1400, 900))
            .with_min_inner_size(winit::dpi::LogicalSize::new(800, 600))
            .build(&event_loop)?;

        Ok(Self {
            event_loop,
            window,
        })
    }

    /// Run the editor
    pub async fn run(self) -> Result<()> {
        let window_id = self.window.id();
        let mut app = EditorApp::new(self.window).await?;

        Ok(self.event_loop.run(move |event, elwt| {
            elwt.set_control_flow(winit::event_loop::ControlFlow::Poll);

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id: event_window_id,
                } if event_window_id == window_id => {
                    if !app.handle_window_event(event) {
                        match event {
                            WindowEvent::CloseRequested => {
                                log::info!("Editor closing");
                                elwt.exit()
                            },
                            WindowEvent::Resized(physical_size) => {
                                app.resize(*physical_size);
                            },
                            WindowEvent::ScaleFactorChanged { .. } => {
                                // Handle scale factor changes
                                let size = app.size();
                                app.resize(size);
                            },
                            _ => {}
                        }
                    }
                },
                Event::AboutToWait => {
                    app.update();
                    match app.render() {
                        Ok(_) => {},
                        Err(wgpu::SurfaceError::Lost) => app.resize(app.size()),
                        Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                        Err(e) => log::error!("Render error: {:?}", e),
                    }
                    app.request_redraw();
                },
                _ => {}
            }
        })?)
    }
}

impl Default for EditorRunner {
    fn default() -> Self {
        Self::new().expect("Failed to create editor runner")
    }
}
