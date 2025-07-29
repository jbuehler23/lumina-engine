//! Dockable panel layout system for the Lumina Editor
//! 
//! This module provides a comprehensive docking system similar to modern IDEs,
//! allowing users to customize their workspace by dragging and docking panels.

pub mod docking;
pub mod layout_node;
pub mod panel_trait;
pub mod splitter;
pub mod tab_bar;
pub mod types;

pub use docking::DockingManager;
pub use layout_node::LayoutNode;
pub use panel_trait::DockablePanel;
pub use splitter::Splitter;
pub use tab_bar::{TabBar, TabInfo, TabAction};
pub use types::*;