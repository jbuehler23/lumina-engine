//! UI integration utilities for the editor

use anyhow::Result;
use lumina_ui::{UiFramework, WidgetId};
use winit::event::WindowEvent;

/// Helper trait for converting winit events to UI events
pub trait EventConverter {
    /// Convert a winit window event to a UI input event
    fn to_ui_event(&self) -> Option<lumina_ui::InputEvent>;
}

impl EventConverter for WindowEvent {
    fn to_ui_event(&self) -> Option<lumina_ui::InputEvent> {
        match self {
            WindowEvent::CursorMoved { position, .. } => {
                Some(lumina_ui::InputEvent::MouseMove {
                    position: glam::Vec2::new(position.x as f32, position.y as f32),
                    delta: glam::Vec2::ZERO, // TODO: Calculate actual delta
                })
            },
            WindowEvent::MouseInput { state, button, .. } => {
                let mouse_button = match button {
                    winit::event::MouseButton::Left => lumina_ui::MouseButton::Left,
                    winit::event::MouseButton::Right => lumina_ui::MouseButton::Right,
                    winit::event::MouseButton::Middle => lumina_ui::MouseButton::Middle,
                    _ => return None,
                };
                
                match state {
                    winit::event::ElementState::Pressed => {
                        Some(lumina_ui::InputEvent::MouseDown {
                            button: mouse_button,
                            position: glam::Vec2::ZERO, // Position needs to be tracked separately
                            modifiers: lumina_ui::Modifiers::default(),
                        })
                    },
                    winit::event::ElementState::Released => {
                        Some(lumina_ui::InputEvent::MouseUp {
                            button: mouse_button,
                            position: glam::Vec2::ZERO, // Position needs to be tracked separately
                            modifiers: lumina_ui::Modifiers::default(),
                        })
                    },
                }
            },
            _ => None,
        }
    }
}

/// Layout helper for organizing editor panels
pub struct LayoutManager {
    /// Screen dimensions
    screen_size: glam::Vec2,
    /// Panel IDs and their layout constraints
    panels: Vec<(WidgetId, PanelLayout)>,
}

/// Panel layout configuration
#[derive(Debug, Clone)]
pub struct PanelLayout {
    /// Panel position
    pub position: glam::Vec2,
    /// Panel size
    pub size: glam::Vec2,
    /// Whether the panel is docked
    pub docked: bool,
    /// Dock side (if docked)
    pub dock_side: DockSide,
}

/// Dock side enumeration
#[derive(Debug, Clone, Copy)]
pub enum DockSide {
    Left,
    Right,
    Top,
    Bottom,
    Center,
}

impl LayoutManager {
    /// Create a new layout manager
    pub fn new(screen_size: glam::Vec2) -> Self {
        Self {
            screen_size,
            panels: Vec::new(),
        }
    }
    
    /// Add a panel to the layout
    pub fn add_panel(&mut self, panel_id: WidgetId, layout: PanelLayout) {
        self.panels.push((panel_id, layout));
    }
    
    /// Update screen size and recalculate layout
    pub fn update_screen_size(&mut self, new_size: glam::Vec2) {
        self.screen_size = new_size;
        self.recalculate_layout();
    }
    
    /// Recalculate panel positions based on screen size
    fn recalculate_layout(&mut self) {
        // Simple docking layout algorithm
        let menu_height = 30.0;
        let panel_width = 250.0;
        
        for (_, layout) in &mut self.panels {
            match layout.dock_side {
                DockSide::Top => {
                    layout.position = glam::Vec2::new(0.0, 0.0);
                    layout.size = glam::Vec2::new(self.screen_size.x, menu_height);
                },
                DockSide::Left => {
                    layout.position = glam::Vec2::new(0.0, menu_height);
                    layout.size = glam::Vec2::new(panel_width, self.screen_size.y - menu_height);
                },
                DockSide::Right => {
                    layout.position = glam::Vec2::new(self.screen_size.x - panel_width, menu_height);
                    layout.size = glam::Vec2::new(panel_width, self.screen_size.y - menu_height);
                },
                DockSide::Bottom => {
                    layout.position = glam::Vec2::new(panel_width, self.screen_size.y - 200.0);
                    layout.size = glam::Vec2::new(self.screen_size.x - panel_width * 2.0, 200.0);
                },
                DockSide::Center => {
                    layout.position = glam::Vec2::new(panel_width, menu_height);
                    layout.size = glam::Vec2::new(
                        self.screen_size.x - panel_width * 2.0,
                        self.screen_size.y - menu_height - 200.0
                    );
                },
            }
        }
    }
    
    /// Apply the layout to the UI framework
    pub fn apply_layout(&self, _ui: &mut UiFramework) -> Result<()> {
        // TODO: Apply calculated positions to widgets
        // This would require extending the UI framework to support absolute positioning
        Ok(())
    }
}