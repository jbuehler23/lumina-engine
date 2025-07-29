# Dockable Panel Layout System - Implementation Plan

**Goal**: Create a professional dockable panel system for the Lumina Editor that allows users to customize their workspace like modern IDEs.

## 🎯 User Experience Goals

### Core Features
- **Drag & Drop Panels** - Click and drag panel tabs to move them around
- **Docking Zones** - Visual indicators showing where panels can be docked
- **Panel Tabs** - Multiple panels can share the same area with tabs
- **Resizable Splitters** - Drag dividers between panels to resize
- **Panel Visibility** - Hide/show panels with toggle buttons or menu
- **Layout Persistence** - Save and restore user's preferred layout

### Visual Layout Structure
```
┌─────────────────────────────────────────────────────────┐
│                        Menu Bar                         │
├─────────────┬─────────────────────────┬─────────────────┤
│             │                         │                 │
│   Project   │                         │   Properties    │
│   Panel     │      Scene Editor       │     Panel       │
│             │      (Center)           │                 │
├─────────────┤                         ├─────────────────┤
│             │                         │                 │
│    Asset    │                         │  Visual Script  │
│   Browser   │                         │     Editor      │
│             │                         │                 │
├─────────────┴─────────────────────────┴─────────────────┤
│                    Console Panel                        │
└─────────────────────────────────────────────────────────┘
```

## 🏗️ Technical Architecture

### 1. Core Components

#### DockingManager
Central coordinator for the entire docking system.

```rust
pub struct DockingManager {
    /// Root layout node
    root_node: LayoutNode,
    /// All available panels
    panels: HashMap<PanelId, Box<dyn DockablePanel>>,
    /// Current drag state
    drag_state: Option<DragState>,
    /// Layout serialization
    layout_config: LayoutConfig,
}

impl DockingManager {
    pub fn new() -> Self;
    pub fn add_panel(&mut self, panel: Box<dyn DockablePanel>);
    pub fn dock_panel(&mut self, panel_id: PanelId, target: DockTarget);
    pub fn undock_panel(&mut self, panel_id: PanelId);
    pub fn resize_split(&mut self, split_id: SplitId, delta: f32);
    pub fn render(&mut self, ui: &mut UiFramework, bounds: Rect);
    pub fn handle_input(&mut self, event: &InputEvent) -> bool;
    pub fn save_layout(&self) -> Result<String>;
    pub fn load_layout(&mut self, data: &str) -> Result<()>;
}
```

#### LayoutNode
Hierarchical layout structure using a tree of nodes.

```rust
#[derive(Debug, Clone)]
pub enum LayoutNode {
    /// Split container with two child nodes
    Split {
        id: SplitId,
        direction: SplitDirection,
        ratio: f32,  // 0.0 to 1.0
        left: Box<LayoutNode>,
        right: Box<LayoutNode>,
    },
    /// Tab container holding multiple panels
    Tabs {
        id: TabId,
        active_tab: usize,
        panels: Vec<PanelId>,
    },
    /// Empty space (can accept drops)
    Empty,
}

#[derive(Debug, Clone, Copy)]
pub enum SplitDirection {
    Horizontal, // Left/Right split
    Vertical,   // Top/Bottom split
}
```

#### DockablePanel Trait
Common interface for all panels in the editor.

```rust
pub trait DockablePanel {
    /// Unique identifier for this panel
    fn id(&self) -> PanelId;
    
    /// Display name for tabs
    fn title(&self) -> &str;
    
    /// Render the panel content
    fn render(&mut self, ui: &mut UiFramework, bounds: Rect);
    
    /// Handle input events
    fn handle_input(&mut self, event: &InputEvent) -> bool;
    
    /// Minimum size constraints
    fn min_size(&self) -> Vec2;
    
    /// Preferred size
    fn preferred_size(&self) -> Vec2;
    
    /// Whether this panel can be closed
    fn can_close(&self) -> bool;
    
    /// Called when panel is docked/undocked
    fn on_dock_changed(&mut self, docked: bool);
}
```

### 2. Docking System

#### DockTarget
Specifies where a panel should be docked.

```rust
#[derive(Debug, Clone)]
pub enum DockTarget {
    /// Dock to a specific zone
    Zone {
        zone_id: ZoneId,
        position: DockPosition,
    },
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
}

#[derive(Debug, Clone, Copy)]
pub enum DockPosition {
    Left,
    Right,
    Top,
    Bottom,
    Center,
}
```

#### DragState
Tracks current drag operation.

```rust
#[derive(Debug)]
pub struct DragState {
    /// Panel being dragged
    panel_id: PanelId,
    /// Original position
    original_position: Vec2,
    /// Current mouse position
    current_position: Vec2,
    /// Potential drop target
    drop_target: Option<DockTarget>,
    /// Visual feedback for drop zones
    drop_zones: Vec<DropZone>,
}

#[derive(Debug)]
pub struct DropZone {
    pub bounds: Rect,
    pub target: DockTarget,
    pub highlight: bool,
}
```

### 3. Visual Components

#### Panel Tab System
```rust
pub struct TabBar {
    pub tabs: Vec<TabInfo>,
    pub active_tab: usize,
    pub bounds: Rect,
}

pub struct TabInfo {
    pub panel_id: PanelId,
    pub title: String,
    pub bounds: Rect,
    pub close_button: Option<Rect>,
    pub can_close: bool,
}

impl TabBar {
    pub fn render(&self, ui: &mut UiFramework);
    pub fn handle_click(&mut self, position: Vec2) -> Option<TabAction>;
}

pub enum TabAction {
    SelectTab(usize),
    CloseTab(usize),
    StartDrag(usize),
}
```

#### Splitter System
```rust
pub struct Splitter {
    pub id: SplitId,
    pub bounds: Rect,
    pub direction: SplitDirection,
    pub dragging: bool,
    pub hover: bool,
}

impl Splitter {
    pub fn render(&self, ui: &mut UiFramework);
    pub fn handle_input(&mut self, event: &InputEvent) -> Option<f32>;
}
```

## 📋 Implementation Steps

### Phase 1: Core Layout System ✅ COMPLETED

#### Step 1.1: Create Layout Module ✅
- ✅ Created `layout/mod.rs` with all module exports
- ✅ Created `layout/types.rs` with core types (PanelId, Rect, DockTarget, etc.)
- ✅ Created `layout/panel_trait.rs` with DockablePanel trait
- ✅ Created `layout/layout_node.rs` with hierarchical layout system

#### Step 1.2: Implement LayoutNode ✅
- ✅ Tree structure for layout hierarchy (Split/Tabs/Empty)
- ✅ Serialization/deserialization support with serde
- ✅ Methods for finding nodes, adding/removing panels
- ✅ Bounds calculation with proper splitting
- ✅ Layout optimization to remove empty nodes
- ✅ Comprehensive unit tests

#### Step 1.3: Implement DockablePanel Trait ✅
- ✅ Complete trait definition with all necessary methods
- ✅ Panel metadata (id, title, icon, size constraints)
- ✅ Render methods that work with bounds
- ✅ Input handling and lifecycle callbacks
- ✅ Context menu support

#### Step 1.4: Basic DockingManager 🔄 IN PROGRESS
- 🔄 Create manager with simple layout
- 🔄 Implement panel registration
- 🔄 Basic rendering without docking yet

### ✅ CHECKPOINT COMPLETED: Runnable Layout System

**Goal**: Get a basic dockable layout system running with our existing panels converted to the new system.

**Completed Tasks**:
1. ✅ Create basic DockingManager - Full implementation with tab management, panel registration, rendering pipeline
2. ✅ Convert one existing panel (ScenePanel) to DockablePanel - Created DockableScenePanel with full trait implementation  
3. ✅ Create simple tab bar rendering - Complete TabBar component with click handling, styling, and layout
4. ✅ Update EditorApp to use DockingManager - Integrated docking system into main app loop with input handling
5. ✅ Test and validate the system works - Project compiles successfully, basic functionality confirmed

**System Status**: ✅ **RUNNABLE** - The dockable panel system is now functional and integrated into the editor!

### Phase 2: Panel Conversion (Week 1-2)

#### Step 2.1: Convert Existing Panels
```rust
// Example: Convert ScenePanel
impl DockablePanel for ScenePanel {
    fn id(&self) -> PanelId { 
        PanelId::SceneEditor 
    }
    
    fn title(&self) -> &str { 
        "Scene Editor" 
    }
    
    fn render(&mut self, ui: &mut UiFramework, bounds: Rect) {
        // Render within specified bounds
        // Convert existing rendering code
    }
    
    fn min_size(&self) -> Vec2 { 
        Vec2::new(400.0, 300.0) 
    }
    
    fn preferred_size(&self) -> Vec2 { 
        Vec2::new(800.0, 600.0) 
    }
}
```

#### Step 2.2: Panel Registration
- Update EditorApp to register all panels with DockingManager
- Remove direct panel rendering from EditorPanels
- Route all panel operations through DockingManager

### Phase 3: Tab System (Week 2)

#### Step 3.1: Implement TabBar
- Create tab rendering with proper styling
- Handle tab selection and close buttons
- Support scrolling for many tabs

#### Step 3.2: Tab Container Logic
- Update LayoutNode::Tabs rendering
- Handle panel switching
- Manage tab lifecycle (add/remove/reorder)

### Phase 4: Splitter System (Week 2-3)

#### Step 4.1: Implement Splitters
- Create draggable splitter widgets
- Handle mouse interaction for resizing
- Update layout ratios dynamically

#### Step 4.2: Layout Calculation
- Implement proper bounds calculation
- Handle minimum size constraints
- Smooth resizing with proper clamping

### Phase 5: Drag & Drop (Week 3-4)

#### Step 5.1: Drag Detection
- Detect when user starts dragging a tab
- Create drag ghost/preview
- Track mouse movement during drag

#### Step 5.2: Drop Zone Visualization
- Show drop zones when dragging
- Highlight valid drop targets
- Provide visual feedback for drop actions

#### Step 5.3: Drop Handling
- Implement panel undocking
- Handle dropping to different zones
- Update layout tree structure

### Phase 6: Layout Persistence (Week 4)

#### Step 6.1: Serialization
- Implement layout save/load
- Store user preferences
- Handle layout migration/versioning

#### Step 6.2: Default Layouts
- Create sensible default layouts
- Allow resetting to defaults
- Support layout presets

## 🎨 Visual Design Specifications

### Color Scheme (Dark Theme)
```rust
pub struct DockingTheme {
    pub background: [f32; 4] = [0.06, 0.06, 0.14, 1.0],
    pub panel_background: [f32; 4] = [0.12, 0.12, 0.20, 1.0],
    pub tab_active: [f32; 4] = [0.15, 0.15, 0.25, 1.0],
    pub tab_inactive: [f32; 4] = [0.10, 0.10, 0.18, 1.0],
    pub tab_hover: [f32; 4] = [0.13, 0.13, 0.22, 1.0],
    pub splitter: [f32; 4] = [0.2, 0.2, 0.3, 1.0],
    pub splitter_hover: [f32; 4] = [0.3, 0.3, 0.4, 1.0],
    pub drop_zone: [f32; 4] = [0.2, 0.6, 1.0, 0.3],
    pub border: [f32; 4] = [0.3, 0.3, 0.4, 1.0],
}
```

### Dimensions
- **Tab Height**: 32px
- **Splitter Width**: 4px (8px interaction area)
- **Minimum Panel Size**: 200x150px
- **Drop Zone Thickness**: 40px
- **Border Width**: 1px

### Typography
- **Tab Title**: 14px, medium weight
- **Panel Title**: 16px, medium weight
- **Icon Size**: 16px for tab icons

## 🧪 Testing Strategy

### Unit Tests
- LayoutNode tree operations
- Bounds calculation algorithms
- Panel conversion correctness
- Serialization round-trips

### Integration Tests
- Full drag & drop workflows
- Layout persistence
- Panel lifecycle management
- Input event handling

### User Testing
- Drag & drop feels natural
- Layouts are intuitive
- Performance with many panels
- Layout persistence works

## 📈 Success Metrics

### Functionality
- [ ] All existing panels work in docked layout
- [ ] Drag & drop works smoothly
- [ ] Resizing works with proper constraints
- [ ] Layout persistence works
- [ ] Performance: 60fps during dragging

### User Experience
- [ ] Intuitive without tutorial
- [ ] Feels responsive and polished
- [ ] Visual feedback is clear
- [ ] Comparable to VS Code/IDE experience

## 🚀 Future Enhancements

### Advanced Features
- **Floating Windows** - Panels can float outside main window
- **Multi-Monitor Support** - Drag panels between monitors
- **Layout Templates** - Predefined layouts for different workflows
- **Panel Groups** - Logical grouping of related panels
- **Quick Panel Switcher** - Ctrl+P style panel finder

### Performance Optimizations
- **Lazy Rendering** - Only render visible panels
- **Virtual Scrolling** - For panels with large content
- **GPU Acceleration** - Hardware-accelerated panel transitions
- **Memory Management** - Unload inactive panel content

---

**Next Action**: Start with Phase 1, Step 1.1 - Create the layout module structure and begin implementing the core LayoutNode system.