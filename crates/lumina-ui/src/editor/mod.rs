//! Editor-specific UI components for the Lumina Engine

pub mod scene_view;
pub mod property_inspector;
pub mod node_editor;
pub mod asset_browser;
pub mod project_manager;

pub use scene_view::*;
pub use property_inspector::*;
pub use node_editor::*;
pub use asset_browser::*;
pub use project_manager::*;

use crate::{Widget, WidgetId, LayoutConstraints, layout::LayoutResult, InputEvent, InputResponse, UiRenderer, Rect};
use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Main editor application widget that combines all editor components
#[derive(Debug)]
pub struct EditorApp {
    /// Widget ID
    id: WidgetId,
    /// Scene view component
    scene_view: SceneView,
    /// Property inspector component
    property_inspector: PropertyInspector,
    /// Asset browser component
    asset_browser: AssetBrowser,
    /// Project manager component
    project_manager: ProjectManager,
    /// Current layout mode
    layout_mode: EditorLayout,
    /// Whether the editor is in fullscreen mode
    fullscreen: bool,
}

/// Editor layout configurations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EditorLayout {
    /// Default layout with all panels visible
    Default,
    /// Compact layout for smaller screens
    Compact,
    /// Scene-focused layout
    SceneFocused,
    /// Code-focused layout (for visual scripting)
    CodeFocused,
}

impl EditorApp {
    /// Create a new editor application
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
            scene_view: SceneView::new(),
            property_inspector: PropertyInspector::new(),
            asset_browser: AssetBrowser::new(),
            project_manager: ProjectManager::new(),
            layout_mode: EditorLayout::Default,
            fullscreen: false,
        }
    }
    
    /// Set the layout mode
    pub fn set_layout_mode(&mut self, mode: EditorLayout) {
        self.layout_mode = mode;
    }
    
    /// Toggle fullscreen mode
    pub fn toggle_fullscreen(&mut self) {
        self.fullscreen = !self.fullscreen;
    }
    
    /// Get the scene view
    pub fn scene_view(&self) -> &SceneView {
        &self.scene_view
    }
    
    /// Get the scene view mutably
    pub fn scene_view_mut(&mut self) -> &mut SceneView {
        &mut self.scene_view
    }
    
    /// Get the property inspector
    pub fn property_inspector(&self) -> &PropertyInspector {
        &self.property_inspector
    }
    
    /// Get the property inspector mutably
    pub fn property_inspector_mut(&mut self) -> &mut PropertyInspector {
        &mut self.property_inspector
    }
    
    /// Get the asset browser
    pub fn asset_browser(&self) -> &AssetBrowser {
        &self.asset_browser
    }
    
    /// Get the asset browser mutably
    pub fn asset_browser_mut(&mut self) -> &mut AssetBrowser {
        &mut self.asset_browser
    }
    
    /// Get the project manager
    pub fn project_manager(&self) -> &ProjectManager {
        &self.project_manager
    }
    
    /// Get the project manager mutably
    pub fn project_manager_mut(&mut self) -> &mut ProjectManager {
        &mut self.project_manager
    }
}

impl Widget for EditorApp {
    fn id(&self) -> WidgetId {
        self.id
    }
    
    fn layout_constraints(&self) -> LayoutConstraints {
        LayoutConstraints::default()
    }
    
    fn layout(&mut self, available_space: Vec2) -> LayoutResult {
        // Calculate layout based on current mode
        match self.layout_mode {
            EditorLayout::Default => self.layout_default(available_space),
            EditorLayout::Compact => self.layout_compact(available_space),
            EditorLayout::SceneFocused => self.layout_scene_focused(available_space),
            EditorLayout::CodeFocused => self.layout_code_focused(available_space),
        }
    }
    
    fn handle_input(&mut self, _input: &InputEvent) -> InputResponse {
        // Route input to appropriate child component based on focus/hover
        InputResponse::NotHandled
    }
    
    fn render(&self, _renderer: &mut UiRenderer, _bounds: Rect) {
        // Render all child components
        // Implementation would render each component in its calculated position
    }
}

impl EditorApp {
    /// Layout for default editor mode
    fn layout_default(&mut self, available_space: Vec2) -> LayoutResult {
        // Default layout:
        // +----------------+----------------+
        // |                |                |
        // |   Scene View   | Property       |
        // |                | Inspector      |
        // |                |                |
        // +----------------+----------------+
        // |   Asset Browser                 |
        // +----------------------------------+
        
        let scene_width = available_space.x * 0.7;
        let property_width = available_space.x * 0.3;
        let browser_height = available_space.y * 0.3;
        let main_height = available_space.y - browser_height;
        
        // Layout scene view
        self.scene_view.layout(Vec2::new(scene_width, main_height));
        
        // Layout property inspector
        self.property_inspector.layout(Vec2::new(property_width, main_height));
        
        // Layout asset browser
        self.asset_browser.layout(Vec2::new(available_space.x, browser_height));
        
        LayoutResult {
            bounds: Rect::new(0.0, 0.0, available_space.x, available_space.y),
            overflow: false,
            content_size: available_space,
        }
    }
    
    /// Layout for compact editor mode
    fn layout_compact(&mut self, available_space: Vec2) -> LayoutResult {
        // Compact layout - hide some panels or make them collapsible
        self.layout_default(available_space)
    }
    
    /// Layout for scene-focused mode
    fn layout_scene_focused(&mut self, available_space: Vec2) -> LayoutResult {
        // Scene-focused layout - maximize scene view
        let scene_width = available_space.x * 0.85;
        let sidebar_width = available_space.x * 0.15;
        
        self.scene_view.layout(Vec2::new(scene_width, available_space.y));
        self.property_inspector.layout(Vec2::new(sidebar_width, available_space.y * 0.5));
        self.asset_browser.layout(Vec2::new(sidebar_width, available_space.y * 0.5));
        
        LayoutResult {
            bounds: Rect::new(0.0, 0.0, available_space.x, available_space.y),
            overflow: false,
            content_size: available_space,
        }
    }
    
    /// Layout for code-focused mode
    fn layout_code_focused(&mut self, available_space: Vec2) -> LayoutResult {
        // Code-focused layout - show visual script editor prominently
        self.layout_default(available_space)
    }
}

impl Default for EditorApp {
    fn default() -> Self {
        Self::new()
    }
}