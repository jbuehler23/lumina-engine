//! Hierarchical layout structure for the docking system

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types::{NodeId, PanelId, SplitId, TabId, Rect, SplitDirection};

/// Hierarchical layout node that can contain splits, tabs, or be empty
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutNode {
    /// Split container with two child nodes
    Split {
        id: SplitId,
        direction: SplitDirection,
        ratio: f32, // 0.0 to 1.0, position of split
        left: Box<LayoutNode>,
        right: Box<LayoutNode>,
    },
    /// Tab container holding multiple panels
    Tabs {
        id: TabId,
        active_tab: usize,
        panels: Vec<PanelId>,
    },
    /// Empty space that can accept drops
    Empty,
}

impl LayoutNode {
    /// Create a new empty node
    pub fn empty() -> Self {
        LayoutNode::Empty
    }
    
    /// Create a new tab container with a single panel
    pub fn single_tab(panel_id: PanelId) -> Self {
        LayoutNode::Tabs {
            id: TabId::new(),
            active_tab: 0,
            panels: vec![panel_id],
        }
    }
    
    /// Create a new tab container with multiple panels
    pub fn tabs(panels: Vec<PanelId>) -> Self {
        LayoutNode::Tabs {
            id: TabId::new(),
            active_tab: 0,
            panels,
        }
    }
    
    /// Create a new split with two child nodes
    pub fn split(direction: SplitDirection, ratio: f32, left: LayoutNode, right: LayoutNode) -> Self {
        LayoutNode::Split {
            id: SplitId::new(),
            direction,
            ratio: ratio.clamp(0.1, 0.9), // Ensure reasonable split ratios
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    
    /// Get all panel IDs in this layout tree
    pub fn get_all_panels(&self) -> Vec<PanelId> {
        let mut panels = Vec::new();
        self.collect_panels(&mut panels);
        panels
    }
    
    fn collect_panels(&self, panels: &mut Vec<PanelId>) {
        match self {
            LayoutNode::Split { left, right, .. } => {
                left.collect_panels(panels);
                right.collect_panels(panels);
            }
            LayoutNode::Tabs { panels: tab_panels, .. } => {
                panels.extend(tab_panels);
            }
            LayoutNode::Empty => {}
        }
    }
    
    /// Find a specific tab container by ID
    pub fn find_tab_container(&self, tab_id: TabId) -> Option<&LayoutNode> {
        match self {
            LayoutNode::Split { left, right, .. } => {
                left.find_tab_container(tab_id)
                    .or_else(|| right.find_tab_container(tab_id))
            }
            LayoutNode::Tabs { id, .. } if *id == tab_id => Some(self),
            _ => None,
        }
    }
    
    /// Find a specific tab container by ID (mutable)
    pub fn find_tab_container_mut(&mut self, tab_id: TabId) -> Option<&mut LayoutNode> {
        match self {
            LayoutNode::Split { left, right, .. } => {
                left.find_tab_container_mut(tab_id)
                    .or_else(|| right.find_tab_container_mut(tab_id))
            }
            LayoutNode::Tabs { id, .. } if *id == tab_id => Some(self),
            _ => None,
        }
    }
    
    /// Find which tab container contains a specific panel
    pub fn find_panel_container(&self, panel_id: PanelId) -> Option<TabId> {
        match self {
            LayoutNode::Split { left, right, .. } => {
                left.find_panel_container(panel_id)
                    .or_else(|| right.find_panel_container(panel_id))
            }
            LayoutNode::Tabs { id, panels, .. } => {
                if panels.contains(&panel_id) {
                    Some(*id)
                } else {
                    None
                }
            }
            LayoutNode::Empty => None,
        }
    }
    
    /// Add a panel to an existing tab container
    pub fn add_panel_to_tabs(&mut self, tab_id: TabId, panel_id: PanelId, index: Option<usize>) -> Result<()> {
        if let Some(container) = self.find_tab_container_mut(tab_id) {
            if let LayoutNode::Tabs { panels, active_tab, .. } = container {
                let insert_index = index.unwrap_or(panels.len()).min(panels.len());
                panels.insert(insert_index, panel_id);
                
                // If we inserted before the active tab, adjust the active index
                if insert_index <= *active_tab {
                    *active_tab += 1;
                }
                
                Ok(())
            } else {
                Err(anyhow::anyhow!("Found tab container is not a Tabs node"))
            }
        } else {
            Err(anyhow::anyhow!("Tab container not found: {:?}", tab_id))
        }
    }
    
    /// Remove a panel from its tab container
    pub fn remove_panel(&mut self, panel_id: PanelId) -> Result<()> {
        self.remove_panel_recursive(panel_id)
    }
    
    fn remove_panel_recursive(&mut self, panel_id: PanelId) -> Result<()> {
        match self {
            LayoutNode::Split { left, right, .. } => {
                // Try to remove from left child
                if left.remove_panel_recursive(panel_id).is_ok() {
                    return Ok(());
                }
                // Try to remove from right child
                right.remove_panel_recursive(panel_id)
            }
            LayoutNode::Tabs { panels, active_tab, .. } => {
                if let Some(index) = panels.iter().position(|&p| p == panel_id) {
                    panels.remove(index);
                    
                    // Adjust active tab index
                    if index < *active_tab {
                        *active_tab -= 1;
                    } else if index == *active_tab && *active_tab >= panels.len() && !panels.is_empty() {
                        *active_tab = panels.len() - 1;
                    }
                    
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("Panel not found in tab container"))
                }
            }
            LayoutNode::Empty => Err(anyhow::anyhow!("Panel not found in empty node")),
        }
    }
    
    /// Calculate bounds for all nodes in the layout
    pub fn calculate_bounds(&self, available_bounds: Rect) -> LayoutBounds {
        let mut bounds = LayoutBounds::new();
        self.calculate_bounds_recursive(available_bounds, &mut bounds);
        bounds
    }
    
    fn calculate_bounds_recursive(&self, bounds: Rect, layout_bounds: &mut LayoutBounds) {
        match self {
            LayoutNode::Split { id, direction, ratio, left, right } => {
                layout_bounds.splits.insert(*id, bounds);
                
                let (left_bounds, right_bounds) = match direction {
                    SplitDirection::Horizontal => bounds.split_horizontal(*ratio),
                    SplitDirection::Vertical => bounds.split_vertical(*ratio),
                };
                
                left.calculate_bounds_recursive(left_bounds, layout_bounds);
                right.calculate_bounds_recursive(right_bounds, layout_bounds);
            }
            LayoutNode::Tabs { id, .. } => {
                layout_bounds.tabs.insert(*id, bounds);
            }
            LayoutNode::Empty => {
                // Empty nodes don't have specific bounds tracking
            }
        }
    }
    
    /// Check if this layout tree is empty (no panels)
    pub fn is_empty(&self) -> bool {
        self.get_all_panels().is_empty()
    }
    
    /// Optimize the layout by removing empty nodes and unnecessary splits
    pub fn optimize(&mut self) {
        self.optimize_recursive()
    }
    
    fn optimize_recursive(&mut self) {
        match self {
            LayoutNode::Split { left, right, .. } => {
                // First optimize children
                left.optimize_recursive();
                right.optimize_recursive();
                
                // Check if we can simplify this split
                match (left.as_ref(), right.as_ref()) {
                    (LayoutNode::Empty, _) => {
                        // Left is empty, replace with right
                        *self = (**right).clone();
                    }
                    (_, LayoutNode::Empty) => {
                        // Right is empty, replace with left
                        *self = (**left).clone();
                    }
                    _ => {
                        // Both children have content, keep the split
                    }
                }
            }
            LayoutNode::Tabs { panels, .. } => {
                // Remove empty tab containers
                if panels.is_empty() {
                    *self = LayoutNode::Empty;
                }
            }
            LayoutNode::Empty => {
                // Already empty
            }
        }
    }
    
    /// Update the split ratio for a specific split
    pub fn update_split_ratio(&mut self, split_id: SplitId, new_ratio: f32) -> Result<()> {
        match self {
            LayoutNode::Split { id, ratio, left, right, .. } => {
                if *id == split_id {
                    *ratio = new_ratio.clamp(0.1, 0.9);
                    Ok(())
                } else {
                    // Try children
                    left.update_split_ratio(split_id, new_ratio)
                        .or_else(|_| right.update_split_ratio(split_id, new_ratio))
                }
            }
            LayoutNode::Tabs { .. } | LayoutNode::Empty => {
                Err(anyhow::anyhow!("Split not found: {:?}", split_id))
            }
        }
    }
    
    /// Set the active tab in a tab container
    pub fn set_active_tab(&mut self, tab_id: TabId, tab_index: usize) -> Result<()> {
        if let Some(container) = self.find_tab_container_mut(tab_id) {
            if let LayoutNode::Tabs { panels, active_tab, .. } = container {
                if tab_index < panels.len() {
                    *active_tab = tab_index;
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("Tab index out of bounds: {} >= {}", tab_index, panels.len()))
                }
            } else {
                Err(anyhow::anyhow!("Found tab container is not a Tabs node"))
            }
        } else {
            Err(anyhow::anyhow!("Tab container not found: {:?}", tab_id))
        }
    }
}

/// Calculated bounds for all layout elements
#[derive(Debug, Default)]
pub struct LayoutBounds {
    /// Bounds for split containers
    pub splits: HashMap<SplitId, Rect>,
    /// Bounds for tab containers
    pub tabs: HashMap<TabId, Rect>,
}

impl LayoutBounds {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Get bounds for a specific tab container
    pub fn get_tab_bounds(&self, tab_id: TabId) -> Option<Rect> {
        self.tabs.get(&tab_id).copied()
    }
    
    /// Get bounds for a specific split
    pub fn get_split_bounds(&self, split_id: SplitId) -> Option<Rect> {
        self.splits.get(&split_id).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty_layout() {
        let layout = LayoutNode::empty();
        assert!(layout.is_empty());
        assert_eq!(layout.get_all_panels().len(), 0);
    }
    
    #[test]
    fn test_single_tab() {
        let panel_id = PanelId::new();
        let layout = LayoutNode::single_tab(panel_id);
        
        assert!(!layout.is_empty());
        let panels = layout.get_all_panels();
        assert_eq!(panels.len(), 1);
        assert_eq!(panels[0], panel_id);
    }
    
    #[test]
    fn test_split_layout() {
        let panel1 = PanelId::new();
        let panel2 = PanelId::new();
        
        let left = LayoutNode::single_tab(panel1);
        let right = LayoutNode::single_tab(panel2);
        let layout = LayoutNode::split(SplitDirection::Horizontal, 0.5, left, right);
        
        let panels = layout.get_all_panels();
        assert_eq!(panels.len(), 2);
        assert!(panels.contains(&panel1));
        assert!(panels.contains(&panel2));
    }
    
    #[test]
    fn test_bounds_calculation() {
        let panel_id = PanelId::new();
        let layout = LayoutNode::single_tab(panel_id);
        
        let bounds = Rect::new(0.0, 0.0, 800.0, 600.0);
        let layout_bounds = layout.calculate_bounds(bounds);
        
        // Should have one tab container
        assert_eq!(layout_bounds.tabs.len(), 1);
        assert_eq!(layout_bounds.splits.len(), 0);
    }
    
    #[test]
    fn test_optimize_empty() {
        let mut layout = LayoutNode::empty();
        layout.optimize();
        
        match layout {
            LayoutNode::Empty => {}, // Expected
            _ => panic!("Empty layout should remain empty after optimization"),
        }
    }
}