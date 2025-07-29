//! Trait definition for dockable panels

use anyhow::Result;
use glam::Vec2;
use lumina_ui::{UiFramework, InputEvent};

use super::types::{PanelId, Rect};

/// Trait for panels that can be docked in the layout system
pub trait DockablePanel {
    /// Unique identifier for this panel
    fn id(&self) -> PanelId;
    
    /// Display name for tabs and menus
    fn title(&self) -> &str;
    
    /// Optional icon name/path for the panel
    fn icon(&self) -> Option<&str> {
        None
    }
    
    /// Render the panel content within the given bounds
    fn render(&mut self, ui: &mut UiFramework, bounds: Rect) -> Result<()>;
    
    /// Handle input events, return true if handled
    fn handle_input(&mut self, _event: &InputEvent) -> bool {
        false
    }
    
    /// Minimum size constraints for this panel
    fn min_size(&self) -> Vec2 {
        Vec2::new(200.0, 150.0)
    }
    
    /// Preferred/initial size for this panel
    fn preferred_size(&self) -> Vec2 {
        Vec2::new(400.0, 300.0)
    }
    
    /// Maximum size constraints (None = no limit)
    fn max_size(&self) -> Option<Vec2> {
        None
    }
    
    /// Whether this panel can be closed by the user
    fn can_close(&self) -> bool {
        true
    }
    
    /// Whether this panel should be visible by default
    fn visible_by_default(&self) -> bool {
        true
    }
    
    /// Called when the panel is docked/undocked
    fn on_dock_changed(&mut self, _docked: bool) {
        // Default implementation does nothing
    }
    
    /// Called when the panel becomes active/inactive (for tabs)
    fn on_active_changed(&mut self, _active: bool) {
        // Default implementation does nothing
    }
    
    /// Called when the panel is about to be closed
    /// Return false to prevent closing
    fn on_close_requested(&mut self) -> bool {
        true
    }
    
    /// Update the panel (called every frame)
    fn update(&mut self) {
        // Default implementation does nothing
    }
    
    /// Get panel-specific context menu items
    fn context_menu_items(&self) -> Vec<ContextMenuItem> {
        Vec::new()
    }
    
    /// Handle context menu item selection
    fn handle_context_menu(&mut self, _item_id: &str) {
        // Default implementation does nothing
    }
}

/// Context menu item for panels
#[derive(Debug, Clone)]
pub struct ContextMenuItem {
    pub id: String,
    pub label: String,
    pub enabled: bool,
    pub separator_after: bool,
}

impl ContextMenuItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            enabled: true,
            separator_after: false,
        }
    }
    
    pub fn with_separator(mut self) -> Self {
        self.separator_after = true;
        self
    }
    
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}