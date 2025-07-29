//! Tab bar component for the docking system

use glam::Vec2;
use lumina_ui::{UiFramework, Button, Panel, WidgetId};
use lumina_ui::widgets::button::ButtonVariant;

use super::types::{PanelId, TabId, Rect, DockingTheme};

/// Information about a single tab
#[derive(Debug, Clone)]
pub struct TabInfo {
    pub panel_id: PanelId,
    pub title: String,
    pub bounds: Rect,
    pub close_button: Option<Rect>,
    pub can_close: bool,
    pub active: bool,
}

/// Actions that can result from tab interactions
#[derive(Debug, Clone)]
pub enum TabAction {
    /// User clicked to select a tab
    SelectTab(usize),
    /// User clicked the close button on a tab
    CloseTab(usize),
    /// User started dragging a tab
    StartDrag(usize),
    /// No action
    None,
}

/// Tab bar component for rendering and managing tabs
pub struct TabBar {
    pub tab_id: TabId,
    pub tabs: Vec<TabInfo>,
    pub active_tab: usize,
    pub bounds: Rect,
    pub theme: DockingTheme,
    
    // UI widgets (will be recreated as needed)
    widgets: Vec<WidgetId>,
}

impl TabBar {
    /// Create a new tab bar
    pub fn new(tab_id: TabId, panel_ids: Vec<PanelId>, panel_titles: Vec<String>) -> Self {
        let tabs = panel_ids
            .into_iter()
            .zip(panel_titles.into_iter())
            .enumerate()
            .map(|(i, (panel_id, title))| TabInfo {
                panel_id,
                title,
                bounds: Rect::new(0.0, 0.0, 0.0, 0.0), // Will be calculated
                close_button: None,
                can_close: true,
                active: i == 0,
            })
            .collect();

        Self {
            tab_id,
            tabs,
            active_tab: 0,
            bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
            theme: DockingTheme::default(),
            widgets: Vec::new(),
        }
    }
    
    /// Update the tab bar bounds and recalculate tab positions
    pub fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
        self.calculate_tab_bounds();
    }
    
    /// Calculate bounds for individual tabs
    fn calculate_tab_bounds(&mut self) {
        if self.tabs.is_empty() {
            return;
        }
        
        let tab_height = self.theme.tab_height;
        let min_tab_width = 120.0;
        let max_tab_width = 200.0;
        let close_button_width = 20.0;
        
        // Calculate available width for tabs
        let available_width = self.bounds.width;
        let ideal_tab_width = available_width / self.tabs.len() as f32;
        let tab_width = ideal_tab_width.clamp(min_tab_width, max_tab_width);
        
        // Calculate tab bounds
        for (i, tab) in self.tabs.iter_mut().enumerate() {
            let x = self.bounds.x + i as f32 * tab_width;
            let y = self.bounds.y;
            
            tab.bounds = Rect::new(x, y, tab_width, tab_height);
            tab.active = i == self.active_tab;
            
            // Close button bounds
            if tab.can_close {
                tab.close_button = Some(Rect::new(
                    x + tab_width - close_button_width - 5.0,
                    y + (tab_height - close_button_width) * 0.5,
                    close_button_width,
                    close_button_width,
                ));
            }
        }
    }
    
    /// Render the tab bar
    pub fn render(&mut self, ui: &mut UiFramework) {
        // Clear existing widgets
        self.widgets.clear();
        
        if self.tabs.is_empty() {
            return;
        }
        
        // Create tab bar background
        let tab_bar_panel = Panel::new()
            .style(lumina_ui::widgets::WidgetStyle {
                background_color: Some(self.theme.tab_inactive),
                border_radius: Some(0.0),
                ..Default::default()
            });
        
        let panel_id = ui.add_root_widget(Box::new(tab_bar_panel));
        self.widgets.push(panel_id);
        
        // Collect tab data first to avoid borrowing issues
        let tabs: Vec<(usize, TabInfo)> = self.tabs.iter().enumerate().map(|(i, tab)| (i, tab.clone())).collect();
        
        // Render each tab
        for (i, tab) in tabs.iter() {
            self.render_tab(ui, panel_id, *i, tab);
        }
    }
    
    fn render_tab(&mut self, ui: &mut UiFramework, parent_id: WidgetId, _tab_index: usize, tab: &TabInfo) {
        // Tab button with appropriate styling
        let button_variant = if tab.active {
            ButtonVariant::Primary
        } else {
            ButtonVariant::Ghost
        };
        
        let tab_button = Button::new(&tab.title)
            .variant(button_variant);
        
        let button_id = ui.add_widget(Box::new(tab_button));
        ui.add_child_to_parent(parent_id, button_id);
        self.widgets.push(button_id);
        
        // Close button if closable
        if tab.can_close {
            let close_button = Button::new("×")
                .variant(ButtonVariant::Ghost);
            
            let close_id = ui.add_widget(Box::new(close_button));
            ui.add_child_to_parent(parent_id, close_id);
            self.widgets.push(close_id);
        }
    }
    
    /// Handle a click at the given position
    pub fn handle_click(&mut self, position: Vec2) -> TabAction {
        for (i, tab) in self.tabs.iter().enumerate() {
            if tab.bounds.contains(position) {
                // Check if click was on close button
                if let Some(close_bounds) = &tab.close_button {
                    if close_bounds.contains(position) && tab.can_close {
                        return TabAction::CloseTab(i);
                    }
                }
                
                // Regular tab click
                if i != self.active_tab {
                    return TabAction::SelectTab(i);
                }
            }
        }
        
        TabAction::None
    }
    
    /// Set the active tab
    pub fn set_active_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab = index;
            
            // Update tab states
            for (i, tab) in self.tabs.iter_mut().enumerate() {
                tab.active = i == index;
            }
        }
    }
    
    /// Add a new tab
    pub fn add_tab(&mut self, panel_id: PanelId, title: String, index: Option<usize>) {
        let tab_info = TabInfo {
            panel_id,
            title,
            bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
            close_button: None,
            can_close: true,
            active: false,
        };
        
        let insert_index = index.unwrap_or(self.tabs.len()).min(self.tabs.len());
        self.tabs.insert(insert_index, tab_info);
        
        // Recalculate bounds
        self.calculate_tab_bounds();
    }
    
    /// Remove a tab
    pub fn remove_tab(&mut self, index: usize) -> Option<PanelId> {
        if index < self.tabs.len() {
            let removed_tab = self.tabs.remove(index);
            
            // Adjust active tab if necessary
            if index < self.active_tab {
                self.active_tab -= 1;
            } else if index == self.active_tab && self.active_tab >= self.tabs.len() && !self.tabs.is_empty() {
                self.active_tab = self.tabs.len() - 1;
            }
            
            // Recalculate bounds
            self.calculate_tab_bounds();
            
            Some(removed_tab.panel_id)
        } else {
            None
        }
    }
    
    /// Get the currently active panel ID
    pub fn get_active_panel(&self) -> Option<PanelId> {
        if self.active_tab < self.tabs.len() {
            Some(self.tabs[self.active_tab].panel_id)
        } else {
            None
        }
    }
    
    /// Get all panel IDs in this tab bar
    pub fn get_all_panels(&self) -> Vec<PanelId> {
        self.tabs.iter().map(|tab| tab.panel_id).collect()
    }
    
    /// Check if the tab bar is empty
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }
    
    /// Get the content bounds (area below the tab bar)
    pub fn get_content_bounds(&self) -> Rect {
        Rect::new(
            self.bounds.x,
            self.bounds.y + self.theme.tab_height,
            self.bounds.width,
            self.bounds.height - self.theme.tab_height,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::types::PanelId;
    
    #[test]
    fn test_tab_bar_creation() {
        let tab_id = TabId::new();
        let panel_ids = vec![PanelId::new(), PanelId::new()];
        let titles = vec!["Tab 1".to_string(), "Tab 2".to_string()];
        
        let tab_bar = TabBar::new(tab_id, panel_ids.clone(), titles);
        
        assert_eq!(tab_bar.tabs.len(), 2);
        assert_eq!(tab_bar.active_tab, 0);
        assert_eq!(tab_bar.get_active_panel(), Some(panel_ids[0]));
    }
    
    #[test]
    fn test_tab_selection() {
        let tab_id = TabId::new();
        let panel_ids = vec![PanelId::new(), PanelId::new()];
        let titles = vec!["Tab 1".to_string(), "Tab 2".to_string()];
        
        let mut tab_bar = TabBar::new(tab_id, panel_ids.clone(), titles);
        
        // Switch to second tab
        tab_bar.set_active_tab(1);
        assert_eq!(tab_bar.active_tab, 1);
        assert_eq!(tab_bar.get_active_panel(), Some(panel_ids[1]));
    }
    
    #[test]
    fn test_add_remove_tab() {
        let tab_id = TabId::new();
        let panel_ids = vec![PanelId::new()];
        let titles = vec!["Tab 1".to_string()];
        
        let mut tab_bar = TabBar::new(tab_id, panel_ids, titles);
        
        // Add a tab
        let new_panel = PanelId::new();
        tab_bar.add_tab(new_panel, "New Tab".to_string(), None);
        assert_eq!(tab_bar.tabs.len(), 2);
        
        // Remove a tab
        let removed = tab_bar.remove_tab(1);
        assert_eq!(removed, Some(new_panel));
        assert_eq!(tab_bar.tabs.len(), 1);
    }
}