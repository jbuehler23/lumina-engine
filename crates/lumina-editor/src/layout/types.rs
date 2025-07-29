//! Core types for the docking system

use glam::Vec2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for panels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PanelId(pub Uuid);

impl PanelId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_name(name: &str) -> Self {
        // Create deterministic UUIDs for built-in panels by hashing the name
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        let hash = hasher.finish();
        
        // Use the hash to create a deterministic UUID
        let bytes = hash.to_le_bytes();
        let mut uuid_bytes = [0u8; 16];
        uuid_bytes[..8].copy_from_slice(&bytes);
        uuid_bytes[8..16].copy_from_slice(&bytes);
        
        Self(Uuid::from_bytes(uuid_bytes))
    }
}

/// Unique identifier for split nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SplitId(pub Uuid);

impl SplitId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Unique identifier for tab containers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TabId(pub Uuid);

impl TabId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Unique identifier for layout nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub Uuid);

impl NodeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Rectangle bounds for layout calculations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn from_min_max(min: Vec2, max: Vec2) -> Self {
        Self {
            x: min.x,
            y: min.y,
            width: max.x - min.x,
            height: max.y - min.y,
        }
    }
    
    pub fn min(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    
    pub fn max(&self) -> Vec2 {
        Vec2::new(self.x + self.width, self.y + self.height)
    }
    
    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x + self.width * 0.5, self.y + self.height * 0.5)
    }
    
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.x && point.x <= self.x + self.width &&
        point.y >= self.y && point.y <= self.y + self.height
    }
    
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }
    
    pub fn split_horizontal(&self, ratio: f32) -> (Rect, Rect) {
        let split_x = self.x + self.width * ratio;
        (
            Rect::new(self.x, self.y, split_x - self.x, self.height),
            Rect::new(split_x, self.y, self.x + self.width - split_x, self.height),
        )
    }
    
    pub fn split_vertical(&self, ratio: f32) -> (Rect, Rect) {
        let split_y = self.y + self.height * ratio;
        (
            Rect::new(self.x, self.y, self.width, split_y - self.y),
            Rect::new(self.x, split_y, self.width, self.y + self.height - split_y),
        )
    }
    
    pub fn shrink(&self, margin: f32) -> Rect {
        Rect::new(
            self.x + margin,
            self.y + margin,
            (self.width - margin * 2.0).max(0.0),
            (self.height - margin * 2.0).max(0.0),
        )
    }
}

/// Specifies where a panel should be docked
#[derive(Debug, Clone)]
pub enum DockTarget {
    /// Add as tab to existing tab container
    Tab {
        tab_id: TabId,
        index: Option<usize>,
    },
    /// Create new split
    Split {
        target_node: NodeId,
        direction: SplitDirection,
        ratio: f32,
    },
    /// Dock to a specific position relative to a node
    Position {
        target_node: NodeId,
        position: DockPosition,
    },
}

/// Direction for splitting layout nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitDirection {
    /// Left/Right split (vertical divider)
    Horizontal,
    /// Top/Bottom split (horizontal divider)
    Vertical,
}

/// Position relative to a target for docking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DockPosition {
    Left,
    Right,
    Top,
    Bottom,
    Center,
}

/// Current drag state for drag & drop operations
#[derive(Debug)]
pub struct DragState {
    /// Panel being dragged
    pub panel_id: PanelId,
    /// Original position when drag started
    pub original_position: Vec2,
    /// Current mouse position
    pub current_position: Vec2,
    /// Offset from mouse to panel origin
    pub drag_offset: Vec2,
    /// Potential drop target
    pub drop_target: Option<DockTarget>,
    /// Visual feedback for drop zones
    pub drop_zones: Vec<DropZone>,
}

/// Visual feedback for potential drop zones
#[derive(Debug)]
pub struct DropZone {
    /// Screen bounds of the drop zone
    pub bounds: Rect,
    /// What will happen if dropped here
    pub target: DockTarget,
    /// Whether this zone is currently highlighted
    pub highlight: bool,
    /// Visual style for this drop zone
    pub style: DropZoneStyle,
}

/// Visual style for drop zones
#[derive(Debug, Clone, Copy)]
pub enum DropZoneStyle {
    /// Dock to left side
    Left,
    /// Dock to right side
    Right,
    /// Dock to top
    Top,
    /// Dock to bottom
    Bottom,
    /// Add as tab (center)
    Center,
}

/// Theme configuration for the docking system
#[derive(Debug, Clone)]
pub struct DockingTheme {
    /// Main background color
    pub background: [f32; 4],
    /// Panel background color
    pub panel_background: [f32; 4],
    /// Active tab color
    pub tab_active: [f32; 4],
    /// Inactive tab color
    pub tab_inactive: [f32; 4],
    /// Tab hover color
    pub tab_hover: [f32; 4],
    /// Splitter color
    pub splitter: [f32; 4],
    /// Splitter hover color
    pub splitter_hover: [f32; 4],
    /// Drop zone highlight color
    pub drop_zone: [f32; 4],
    /// Border color
    pub border: [f32; 4],
    /// Text color
    pub text: [f32; 4],
    /// Tab height in pixels
    pub tab_height: f32,
    /// Splitter width in pixels
    pub splitter_width: f32,
    /// Border width in pixels
    pub border_width: f32,
}

impl Default for DockingTheme {
    fn default() -> Self {
        Self {
            background: [0.06, 0.06, 0.14, 1.0],
            panel_background: [0.12, 0.12, 0.20, 1.0],
            tab_active: [0.15, 0.15, 0.25, 1.0],
            tab_inactive: [0.10, 0.10, 0.18, 1.0],
            tab_hover: [0.13, 0.13, 0.22, 1.0],
            splitter: [0.2, 0.2, 0.3, 1.0],
            splitter_hover: [0.3, 0.3, 0.4, 1.0],
            drop_zone: [0.2, 0.6, 1.0, 0.3],
            border: [0.3, 0.3, 0.4, 1.0],
            text: [1.0, 1.0, 1.0, 1.0],
            tab_height: 32.0,
            splitter_width: 4.0,
            border_width: 1.0,
        }
    }
}

/// Built-in panel identifiers for the editor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinPanelId {
    MenuBar,
    ProjectPanel,
    SceneEditor,
    PropertyInspector,
    AssetBrowser,
    ConsolePanel,
    VisualScriptEditor,
}

impl BuiltinPanelId {
    pub fn panel_id(self) -> PanelId {
        match self {
            BuiltinPanelId::MenuBar => PanelId::from_name("MenuBar"),
            BuiltinPanelId::ProjectPanel => PanelId::from_name("ProjectPanel"),
            BuiltinPanelId::SceneEditor => PanelId::from_name("SceneEditor"),
            BuiltinPanelId::PropertyInspector => PanelId::from_name("PropertyInspector"),
            BuiltinPanelId::AssetBrowser => PanelId::from_name("AssetBrowser"),
            BuiltinPanelId::ConsolePanel => PanelId::from_name("ConsolePanel"),
            BuiltinPanelId::VisualScriptEditor => PanelId::from_name("VisualScriptEditor"),
        }
    }
    
    pub fn title(self) -> &'static str {
        match self {
            BuiltinPanelId::MenuBar => "Menu Bar",
            BuiltinPanelId::ProjectPanel => "Project",
            BuiltinPanelId::SceneEditor => "Scene Editor",
            BuiltinPanelId::PropertyInspector => "Properties",
            BuiltinPanelId::AssetBrowser => "Assets",
            BuiltinPanelId::ConsolePanel => "Console",
            BuiltinPanelId::VisualScriptEditor => "Visual Script",
        }
    }
}