//! Central coordinator for the docking system

use anyhow::Result;
use lumina_ui::{UiFramework, InputEvent};
use std::collections::HashMap;

use super::types::{PanelId, Rect, DockingTheme, DragState, DockTarget};
use super::panel_trait::DockablePanel;
use super::layout_node::{LayoutNode, LayoutBounds};
use super::tab_bar::{TabBar, TabAction};

/// Central coordinator for the entire docking system
pub struct DockingManager {
    /// Root layout node containing the entire layout tree
    root_node: LayoutNode,
    /// All available panels registered with the system
    panels: HashMap<PanelId, Box<dyn DockablePanel>>,
    /// Cached layout bounds for rendering
    layout_bounds: LayoutBounds,
    /// Current drag operation state
    drag_state: Option<DragState>,
    /// Theme configuration
    theme: DockingTheme,
    /// Tab bars cache (recreated when layout changes)
    tab_bars: HashMap<super::types::TabId, TabBar>,
}

impl DockingManager {
    /// Create a new docking manager with an empty layout
    pub fn new() -> Self {
        Self {
            root_node: LayoutNode::empty(),
            panels: HashMap::new(),
            layout_bounds: LayoutBounds::new(),
            drag_state: None,
            theme: DockingTheme::default(),
            tab_bars: HashMap::new(),
        }
    }

    /// Create a new docking manager with a default layout
    pub fn with_default_layout() -> Self {
        // Create a simple default layout with tabs for common panels
        let root_node = LayoutNode::tabs(vec![
            super::types::BuiltinPanelId::SceneEditor.panel_id(),
            super::types::BuiltinPanelId::PropertyInspector.panel_id(),
        ]);

        Self {
            root_node,
            panels: HashMap::new(),
            layout_bounds: LayoutBounds::new(),
            drag_state: None,
            theme: DockingTheme::default(),
            tab_bars: HashMap::new(),
        }
    }

    /// Register a panel with the docking system
    pub fn add_panel(&mut self, panel: Box<dyn DockablePanel>) {
        let panel_id = panel.id();
        self.panels.insert(panel_id, panel);
        
        // If this is the first panel and we have an empty layout, create a tab for it
        if self.root_node.is_empty() {
            self.root_node = LayoutNode::single_tab(panel_id);
            self.rebuild_tab_bars();
        }
    }

    /// Remove a panel from the system
    pub fn remove_panel(&mut self, panel_id: PanelId) -> Option<Box<dyn DockablePanel>> {
        // Remove from layout
        if let Err(e) = self.root_node.remove_panel(panel_id) {
            log::warn!("Failed to remove panel from layout: {}", e);
        }
        
        // Optimize layout after removal
        self.root_node.optimize();
        self.rebuild_tab_bars();
        
        // Remove from panels map
        self.panels.remove(&panel_id)
    }

    /// Dock a panel to a specific target location
    pub fn dock_panel(&mut self, panel_id: PanelId, target: DockTarget) -> Result<()> {
        match target {
            DockTarget::Tab { tab_id, index } => {
                self.root_node.add_panel_to_tabs(tab_id, panel_id, index)?;
                self.rebuild_tab_bars();
            }
            DockTarget::Split { .. } => {
                // TODO: Implement split docking
                log::warn!("Split docking not yet implemented");
            }
            DockTarget::Position { .. } => {
                // TODO: Implement position-based docking
                log::warn!("Position docking not yet implemented");
            }
        }
        Ok(())
    }

    /// Update the docking manager (called every frame)
    pub fn update(&mut self) {
        // Update all panels
        for panel in self.panels.values_mut() {
            panel.update();
        }
    }

    /// Render the entire docking layout
    pub fn render(&mut self, ui: &mut UiFramework, bounds: Rect) -> Result<()> {
        // Calculate layout bounds
        self.layout_bounds = self.root_node.calculate_bounds(bounds);
        
        // Render the layout tree
        self.render_node(&self.root_node.clone(), ui)?;
        
        Ok(())
    }

    fn render_node(&mut self, node: &LayoutNode, ui: &mut UiFramework) -> Result<()> {
        match node {
            LayoutNode::Split { left, right, .. } => {
                // Render child nodes
                self.render_node(left, ui)?;
                self.render_node(right, ui)?;
                
                // TODO: Render splitter between children
            }
            LayoutNode::Tabs { id, active_tab, panels } => {
                if let Some(bounds) = self.layout_bounds.get_tab_bounds(*id) {
                    self.render_tab_container(*id, *active_tab, panels, bounds, ui)?;
                }
            }
            LayoutNode::Empty => {
                // Nothing to render for empty nodes
            }
        }
        Ok(())
    }

    fn render_tab_container(
        &mut self,
        tab_id: super::types::TabId,
        active_tab: usize,
        panel_ids: &[PanelId],
        bounds: Rect,
        ui: &mut UiFramework,
    ) -> Result<()> {
        // Get or create tab bar
        if !self.tab_bars.contains_key(&tab_id) {
            self.rebuild_tab_bar(tab_id, panel_ids);
        }

        if let Some(tab_bar) = self.tab_bars.get_mut(&tab_id) {
            // Update tab bar bounds and render
            tab_bar.set_bounds(Rect::new(bounds.x, bounds.y, bounds.width, self.theme.tab_height));
            tab_bar.render(ui);

            // Render the active panel content
            if active_tab < panel_ids.len() {
                let active_panel_id = panel_ids[active_tab];
                if let Some(panel) = self.panels.get_mut(&active_panel_id) {
                    let content_bounds = tab_bar.get_content_bounds();
                    panel.render(ui, content_bounds)?;
                }
            }
        }

        Ok(())
    }

    /// Handle input events
    pub fn handle_input(&mut self, event: &InputEvent) -> bool {
        // Handle tab bar interactions
        for (tab_id, tab_bar) in &mut self.tab_bars {
            if let InputEvent::MouseDown { position, .. } = event {
                match tab_bar.handle_click(*position) {
                    TabAction::SelectTab(index) => {
                        if let Err(e) = self.root_node.set_active_tab(*tab_id, index) {
                            log::warn!("Failed to set active tab: {}", e);
                        }
                        return true;
                    }
                    TabAction::CloseTab(index) => {
                        if let Some(panel_id) = tab_bar.remove_tab(index) {
                            self.remove_panel(panel_id);
                        }
                        return true;
                    }
                    TabAction::StartDrag(_index) => {
                        // TODO: Implement drag starting
                        return true;
                    }
                    TabAction::None => {}
                }
            }
        }

        // Forward input to active panel
        if let Some(active_panel_id) = self.get_active_panel() {
            if let Some(panel) = self.panels.get_mut(&active_panel_id) {
                return panel.handle_input(event);
            }
        }

        false
    }

    /// Get the currently active panel (if any)
    pub fn get_active_panel(&self) -> Option<PanelId> {
        // For now, just return the first active panel we find
        // TODO: Implement proper focus tracking
        match &self.root_node {
            LayoutNode::Tabs { panels, active_tab, .. } => {
                if *active_tab < panels.len() {
                    Some(panels[*active_tab])
                } else {
                    None
                }
            }
            LayoutNode::Split { left, .. } => {
                // Try to find active panel in left subtree first
                self.find_active_panel_in_node(left)
            }
            LayoutNode::Empty => None,
        }
    }

    fn find_active_panel_in_node(&self, node: &LayoutNode) -> Option<PanelId> {
        match node {
            LayoutNode::Tabs { panels, active_tab, .. } => {
                if *active_tab < panels.len() {
                    Some(panels[*active_tab])
                } else {
                    None
                }
            }
            LayoutNode::Split { left, right, .. } => {
                self.find_active_panel_in_node(left)
                    .or_else(|| self.find_active_panel_in_node(right))
            }
            LayoutNode::Empty => None,
        }
    }

    /// Get all registered panels
    pub fn get_all_panels(&self) -> Vec<PanelId> {
        self.panels.keys().copied().collect()
    }

    /// Get a reference to a specific panel
    pub fn get_panel(&self, panel_id: PanelId) -> Option<&dyn DockablePanel> {
        self.panels.get(&panel_id).map(|p| p.as_ref())
    }

    /// Get a mutable reference to a specific panel
    pub fn get_panel_mut(&mut self, panel_id: PanelId) -> Option<&mut (dyn DockablePanel + '_)> {
        if let Some(panel) = self.panels.get_mut(&panel_id) {
            Some(panel.as_mut())
        } else {
            None
        }
    }

    /// Set the theme for the docking system
    pub fn set_theme(&mut self, theme: DockingTheme) {
        self.theme = theme;
        // Update all tab bars with new theme
        for tab_bar in self.tab_bars.values_mut() {
            tab_bar.theme = self.theme.clone();
        }
    }

    /// Get the current theme
    pub fn theme(&self) -> &DockingTheme {
        &self.theme
    }

    /// Rebuild all tab bars from the current layout
    fn rebuild_tab_bars(&mut self) {
        self.tab_bars.clear();
        self.collect_tab_bars(&self.root_node.clone());
    }

    fn collect_tab_bars(&mut self, node: &LayoutNode) {
        match node {
            LayoutNode::Split { left, right, .. } => {
                self.collect_tab_bars(left);
                self.collect_tab_bars(right);
            }
            LayoutNode::Tabs { id, panels, .. } => {
                self.rebuild_tab_bar(*id, panels);
            }
            LayoutNode::Empty => {}
        }
    }

    fn rebuild_tab_bar(&mut self, tab_id: super::types::TabId, panel_ids: &[PanelId]) {
        let titles: Vec<String> = panel_ids
            .iter()
            .filter_map(|&panel_id| {
                self.panels.get(&panel_id).map(|panel| panel.title().to_string())
            })
            .collect();

        let tab_bar = TabBar::new(tab_id, panel_ids.to_vec(), titles);
        self.tab_bars.insert(tab_id, tab_bar);
    }

    /// Save the current layout to a string
    pub fn save_layout(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.root_node)?)
    }

    /// Load a layout from a string
    pub fn load_layout(&mut self, data: &str) -> Result<()> {
        let root_node: LayoutNode = serde_json::from_str(data)?;
        self.root_node = root_node;
        self.rebuild_tab_bars();
        Ok(())
    }

    /// Get the root layout node (for debugging/inspection)
    pub fn root_node(&self) -> &LayoutNode {
        &self.root_node
    }
}

impl Default for DockingManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock panel for testing
    struct MockPanel {
        id: PanelId,
        title: String,
    }

    impl MockPanel {
        fn new(id: PanelId, title: &str) -> Self {
            Self {
                id,
                title: title.to_string(),
            }
        }
    }

    impl DockablePanel for MockPanel {
        fn id(&self) -> PanelId {
            self.id
        }

        fn title(&self) -> &str {
            &self.title
        }

        fn render(&mut self, _ui: &mut UiFramework, _bounds: Rect) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_docking_manager_creation() {
        let manager = DockingManager::new();
        assert!(manager.root_node.is_empty());
        assert_eq!(manager.panels.len(), 0);
    }

    #[test]
    fn test_add_panel() {
        let mut manager = DockingManager::new();
        let panel_id = PanelId::new();
        let panel = Box::new(MockPanel::new(panel_id, "Test Panel"));
        
        manager.add_panel(panel);
        
        assert!(!manager.root_node.is_empty());
        assert_eq!(manager.panels.len(), 1);
        assert!(manager.panels.contains_key(&panel_id));
    }

    #[test]
    fn test_layout_serialization() {
        let manager = DockingManager::with_default_layout();
        
        let serialized = manager.save_layout().expect("Failed to serialize layout");
        assert!(!serialized.is_empty());
        
        let mut new_manager = DockingManager::new();
        new_manager.load_layout(&serialized).expect("Failed to load layout");
        
        // Should have the same panel structure
        assert_eq!(
            manager.root_node.get_all_panels().len(),
            new_manager.root_node.get_all_panels().len()
        );
    }
}