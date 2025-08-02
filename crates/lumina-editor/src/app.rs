//! Main editor application that integrates UI framework with ECS rendering

use anyhow::Result;
use glam::Vec2;
use lumina_core::{EcsApp, WindowConfig};
use lumina_ecs::World;
use lumina_render::RenderConfig;
use lumina_ui::{
    UiFramework, Theme, InputEvent, MouseButton, KeyCode, Modifiers,
};
use winit::{
    event::{WindowEvent, ElementState},
    keyboard::{Key, NamedKey},
};
use log::{info, debug};

use crate::panels::{MenuBar, ProjectPanel, PropertiesPanel, ConsolePanel, VisualScriptingPanel, AssetBrowserPanel};
use crate::project::EditorProject;
use crate::scene::SceneManager;
use crate::assets::AssetBrowser;
use crate::layout::DockingManager;
use crate::dockable_scene_panel::DockableScenePanel;
use crate::toolbar::{EditorToolbar, ToolbarAction};

/// Main editor application that manages ECS world and UI
pub struct EditorApp {
    // ECS World
    world: World,
    
    // UI Framework (managed as ECS resource)
    ui_framework: UiFramework,
    
    // Editor state
    current_project: Option<EditorProject>,
    scene_manager: SceneManager,
    asset_browser: AssetBrowser,
    docking_manager: DockingManager,
    toolbar: EditorToolbar,
    panels: EditorPanels,
    
    // Input tracking
    mouse_position: Vec2,
}

/// Container for non-dockable editor panels
pub struct EditorPanels {
    pub menu_bar: MenuBar,
    pub project_panel: ProjectPanel,
    pub properties_panel: PropertiesPanel,
    pub console_panel: ConsolePanel,
    pub visual_scripting_panel: VisualScriptingPanel,
    pub asset_browser_panel: AssetBrowserPanel,
}

impl EditorApp {
    /// Create a new editor application with ECS architecture
    pub fn new() -> Result<Self> {
        println!("🔧 [DEBUG] EditorApp::new() starting...");
        debug!("Creating editor application...");
        
        // Initialize ECS World
        println!("🔧 [DEBUG] Creating ECS World...");
        let world = World::new();
        
        // Create UI framework with dark theme
        println!("🔧 [DEBUG] Setting up UI framework with dark theme...");
        debug!("Setting up UI framework with dark theme...");
        let theme = Theme::dark();
        let mut ui_framework = UiFramework::new(theme);
        
        // Initialize docking manager
        println!("🔧 [DEBUG] Initializing docking manager...");
        debug!("Initializing docking manager...");
        let mut docking_manager = DockingManager::with_default_layout();
        
        // Register dockable panels
        println!("🔧 [DEBUG] Creating DockableScenePanel...");
        let scene_panel = Box::new(DockableScenePanel::new());
        println!("🔧 [DEBUG] Adding scene panel to docking manager...");
        docking_manager.add_panel(scene_panel);
        
        // Initialize editor panels (non-dockable ones)
        println!("🔧 [DEBUG] Initializing editor panels...");
        debug!("Initializing editor panels...");
        let panels = EditorPanels::new(&mut ui_framework)?;
        
        println!("🔧 [DEBUG] Creating EditorToolbar...");
        let toolbar = EditorToolbar::new();
        
        println!("🔧 [DEBUG] EditorApp::new() completed successfully");
        info!("Editor app initialized");
        
        Ok(Self {
            world,
            ui_framework,
            current_project: None,
            scene_manager: SceneManager::new(),
            asset_browser: AssetBrowser::new(),
            docking_manager,
            toolbar,
            panels,
            mouse_position: Vec2::ZERO,
        })
    }
    
}

impl EcsApp for EditorApp {
    fn setup(&mut self, _world: &mut World) -> Result<()> {
        println!("🔧 [DEBUG] EditorApp::setup() starting...");
        info!("Setting up editor ECS systems...");
        debug!("Editor ECS setup complete");
        println!("🔧 [DEBUG] EditorApp::setup() completed successfully");
        Ok(())
    }
    
    fn window_config(&self) -> WindowConfig {
        WindowConfig {
            title: "Lumina Engine - Visual Editor".to_string(),
            size: winit::dpi::LogicalSize::new(1400, 900),
            resizable: true,
        }
    }
    
    fn render_config(&self) -> RenderConfig {
        RenderConfig::default()
    }
    
    fn theme(&self) -> Theme {
        Theme::dark()
    }
    
    fn update(&mut self, _world: &mut World) -> Result<()> {
        let screen_size = Vec2::new(1400.0, 900.0); // TODO: Get from window size
        
        // Update toolbar
        self.toolbar.update();
        
        // Render toolbar
        let toolbar_bounds = crate::layout::types::Rect::new(0.0, 0.0, screen_size.x, 40.0);
        self.toolbar.set_bounds(toolbar_bounds);
        if let Err(e) = self.toolbar.render(&mut self.ui_framework) {
            log::warn!("Failed to render toolbar: {}", e);
        }
        
        // Update docking manager
        self.docking_manager.update();
        
        // Render docked panels (leave space for toolbar and status bar)
        let docking_bounds = crate::layout::types::Rect::new(0.0, 40.0, screen_size.x, screen_size.y - 80.0);
        if let Err(e) = self.docking_manager.render(&mut self.ui_framework, docking_bounds) {
            log::warn!("Failed to render docking manager: {}", e);
        }
        
        // Update editor panels (non-dockable ones)
        self.panels.update(&mut self.ui_framework);
        
        // Update UI layout
        self.ui_framework.update_layout(screen_size);
        
        // Add a simple text widget to show the editor is working
        self.add_debug_overlay();
        
        Ok(())
    }
    
    fn handle_ui_action(&mut self, _world: &mut World, action: String) -> Result<()> {
        if action.starts_with("select_tool") {
            let tool_str = action.strip_prefix("select_tool_").unwrap();
            let tool = match tool_str {
                "Select" => ToolType::Select,
                "Move" => ToolType::Move,
                "Rotate" => ToolType::Rotate,
                "Scale" => ToolType::Scale,
                "Brush" => ToolType::Brush,
                "Eraser" => ToolType::Eraser,
                _ => return Ok(()),
            };
            self.toolbar.set_selected_tool(tool);
            self.handle_toolbar_action(ToolbarAction::ToolSelected(tool));
        } else {
            match action.as_str() {
                "new_project" => self.handle_toolbar_action(ToolbarAction::NewProject),
                "open_project" => self.handle_toolbar_action(ToolbarAction::OpenProject),
                "save_project" => self.handle_toolbar_action(ToolbarAction::SaveProject),
                "undo" => self.handle_toolbar_action(ToolbarAction::Undo),
                "redo" => self.handle_toolbar_action(ToolbarAction::Redo),
                "play" => self.handle_toolbar_action(ToolbarAction::Play),
                "pause" => self.handle_toolbar_action(ToolbarAction::Pause),
                "stop" => self.handle_toolbar_action(ToolbarAction::Stop),
                _ => {
                    println!("🖱️ UNKNOWN UI ACTION: {}", action);
                    info!("🖱️ Unknown UI action: {}", action);
                }
            }
        }
        Ok(())
    }
    
    fn handle_event(&mut self, _world: &mut World, event: &WindowEvent) -> Result<bool> {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = Vec2::new(position.x as f32, position.y as f32);
                let input_event = InputEvent::MouseMove {
                    position: self.mouse_position,
                    delta: Vec2::ZERO, // TODO: Calculate actual delta
                };
                
                // Update UI hover state
                let _ = lumina_core::render_systems::update_ui_hover(&mut self.world, self.mouse_position);
                
                // Handle input through docking manager first
                if !self.docking_manager.handle_input(&input_event) {
                    self.ui_framework.handle_input(input_event);
                }
                // Return false so the ECS input system can also process mouse movement
                Ok(false)
            },
            WindowEvent::MouseInput { state, button, .. } => {
                let mouse_button = match button {
                    winit::event::MouseButton::Left => MouseButton::Left,
                    winit::event::MouseButton::Right => MouseButton::Right,
                    winit::event::MouseButton::Middle => MouseButton::Middle,
                    _ => return Ok(false),
                };
                
                let input_event = match state {
                    ElementState::Pressed => InputEvent::MouseDown {
                        button: mouse_button,
                        position: self.mouse_position,
                        modifiers: Modifiers::default(),
                    },
                    ElementState::Released => InputEvent::MouseUp {
                        button: mouse_button,
                        position: self.mouse_position,
                        modifiers: Modifiers::default(),
                    },
                };
                
                // Handle input through docking manager and UI framework
                // Note: Button clicks are now handled by the ECS input processing system
                if !self.docking_manager.handle_input(&input_event) {
                    self.ui_framework.handle_input(input_event);
                }
                // Return false so the ECS input system can also process this event
                Ok(false)
            },
            WindowEvent::KeyboardInput { 
                event, .. 
            } => {
                let key_code = match event.logical_key {
                    Key::Named(NamedKey::Space) => KeyCode::Space,
                    Key::Named(NamedKey::Enter) => KeyCode::Enter,
                    Key::Named(NamedKey::Escape) => KeyCode::Escape,
                    Key::Named(NamedKey::Tab) => KeyCode::Tab,
                    Key::Character(ref s) if s == "a" => KeyCode::A,
                    Key::Character(ref s) if s == "s" => KeyCode::S,
                    Key::Character(ref s) if s == "d" => KeyCode::D,
                    Key::Character(ref s) if s == "w" => KeyCode::W,
                    _ => return Ok(false),
                };
                
                let input_event = match event.state {
                    ElementState::Pressed => InputEvent::KeyDown {
                        key: key_code,
                        modifiers: Modifiers::default(),
                    },
                    ElementState::Released => InputEvent::KeyUp {
                        key: key_code,
                        modifiers: Modifiers::default(),
                    },
                };
                
                // Handle keyboard shortcuts first (only on key down)
                if matches!(event.state, ElementState::Pressed) {
                    let key_string = match &event.logical_key {
                        Key::Character(s) => s.to_string(),
                        Key::Named(NamedKey::Space) => "space".to_string(),
                        _ => "".to_string(),
                    };
                    
                    let toolbar_action = self.toolbar.handle_keyboard_shortcut(&key_string);
                    if !matches!(toolbar_action, ToolbarAction::None) {
                        self.handle_toolbar_action(toolbar_action);
                        return Ok(true);
                    }
                }
                
                // Handle input through docking manager first
                if !self.docking_manager.handle_input(&input_event) {
                    self.ui_framework.handle_input(input_event);
                }
                // Return false so the ECS input system can also process keyboard input
                Ok(false)
            },
            _ => Ok(false),
        }
    }
}

// Additional editor-specific methods
impl EditorApp {
    /// Create a new project
    pub fn new_project(&mut self, name: String, path: String) -> Result<()> {
        info!("Creating new project: {} at {}", name, path);
        let project = EditorProject::new(name, path)?;
        self.current_project = Some(project);
        info!("Successfully created new project");
        Ok(())
    }
    
    /// Load an existing project
    pub fn load_project(&mut self, path: String) -> Result<()> {
        info!("Loading project from: {}", path);
        let project = EditorProject::load(path)?;
        self.current_project = Some(project);
        info!("Successfully loaded project");
        Ok(())
    }
    
    /// Get the current project
    pub fn current_project(&self) -> Option<&EditorProject> {
        self.current_project.as_ref()
    }

    /// Get the scene manager
    pub fn scene_manager(&self) -> &SceneManager {
        &self.scene_manager
    }

    /// Get a mutable reference to the scene manager
    pub fn scene_manager_mut(&mut self) -> &mut SceneManager {
        &mut self.scene_manager
    }

    /// Get the asset browser
    pub fn asset_browser(&self) -> &AssetBrowser {
        &self.asset_browser
    }

    /// Get a mutable reference to the asset browser
    pub fn asset_browser_mut(&mut self) -> &mut AssetBrowser {
        &mut self.asset_browser
    }

    /// Get the docking manager
    pub fn docking_manager(&self) -> &DockingManager {
        &self.docking_manager
    }

    /// Get a mutable reference to the docking manager
    pub fn docking_manager_mut(&mut self) -> &mut DockingManager {
        &mut self.docking_manager
    }

    /// Get the toolbar
    pub fn toolbar(&self) -> &EditorToolbar {
        &self.toolbar
    }

    /// Get a mutable reference to the toolbar
    pub fn toolbar_mut(&mut self) -> &mut EditorToolbar {
        &mut self.toolbar
    }

    /// Add a simple debug overlay to show the editor is working
    fn add_debug_overlay(&mut self) {
        use lumina_ui::{Text, WidgetId};
        
        // Create a simple text widget showing editor status
        let status_text = Text::new("🎮 Lumina Editor Running - Visual Rendering Active")
            .font_size(16.0)
            .color(glam::Vec4::new(0.8, 0.9, 1.0, 1.0));
        
        // Add it to the UI framework
        let text_id = self.ui_framework.add_root_widget(Box::new(status_text));
        
        // Also show current tool
        let tool_text = Text::new(&format!("🔧 Current Tool: {:?}", self.toolbar.selected_tool()))
            .font_size(14.0)
            .color(glam::Vec4::new(0.7, 0.8, 0.9, 1.0));
        
        let tool_id = self.ui_framework.add_root_widget(Box::new(tool_text));
        
        log::debug!("Added debug overlay widgets: status={:?}, tool={:?}", text_id, tool_id);
    }

    /// Handle UI actions from the new ECS-based click system
    fn handle_ui_action(&mut self, action: String) {
        match action.as_str() {
            "select" => {
                println!("🖱️ SELECT TOOL ACTIVATED");
                info!("🖱️ Select tool activated");
                // TODO: Activate select tool in scene editor
            }
            "move" => {
                println!("🖱️ MOVE TOOL ACTIVATED");
                info!("🖱️ Move tool activated");
                // TODO: Activate move tool in scene editor
            }
            "rotate" => {
                println!("🖱️ ROTATE TOOL ACTIVATED");
                info!("🖱️ Rotate tool activated");
                // TODO: Activate rotate tool in scene editor
            }
            "scale" => {
                println!("🖱️ SCALE TOOL ACTIVATED");
                info!("🖱️ Scale tool activated");
                // TODO: Activate scale tool in scene editor
            }
            "brush" => {
                println!("🖱️ BRUSH TOOL ACTIVATED");
                info!("🖱️ Brush tool activated");
                // TODO: Activate brush tool in scene editor
            }
            "eraser" => {
                println!("🖱️ ERASER TOOL ACTIVATED");
                info!("🖱️ Eraser tool activated");
                // TODO: Activate eraser tool in scene editor
            }
            "new" => {
                println!("🖱️ NEW PROJECT REQUESTED");
                info!("🖱️ New project requested");
                // TODO: Show new project dialog
            }
            "open" => {
                println!("🖱️ OPEN PROJECT REQUESTED");
                info!("🖱️ Open project requested");
                // TODO: Show open project dialog
            }
            "save" => {
                println!("🖱️ SAVE PROJECT REQUESTED");
                info!("🖱️ Save project requested");
                if let Some(project) = &self.current_project {
                    info!("Saving project: {}", project.name);
                } else {
                    println!("No project to save");
                    info!("No project to save");
                }
            }
            _ => {
                println!("🖱️ UNKNOWN UI ACTION: {}", action);
                info!("🖱️ Unknown UI action: {}", action);
            }
        }
    }

    /// Handle toolbar actions
    fn handle_toolbar_action(&mut self, action: ToolbarAction) {
        match action {
            ToolbarAction::ToolSelected(tool) => {
                debug!("Tool selected: {:?}", tool);
                // TODO: Update scene editor with selected tool
            }
            ToolbarAction::NewProject => {
                debug!("New project requested");
                // TODO: Show new project dialog
            }
            ToolbarAction::OpenProject => {
                debug!("Open project requested");
                // TODO: Show open project dialog
            }
            ToolbarAction::SaveProject => {
                debug!("Save project requested");
                if let Some(project) = &self.current_project {
                    debug!("Saving project: {}", project.name);
                    // TODO: Implement project saving
                } else {
                    debug!("No project to save");
                }
            }
            ToolbarAction::Undo => {
                debug!("Undo requested");
                // TODO: Implement undo system
            }
            ToolbarAction::Redo => {
                debug!("Redo requested");
                // TODO: Implement redo system
            }
            ToolbarAction::Play => {
                debug!("Play requested");
                // TODO: Start game preview
            }
            ToolbarAction::Pause => {
                debug!("Pause requested");
                // TODO: Pause game preview
            }
            ToolbarAction::Stop => {
                debug!("Stop requested");
                // TODO: Stop game preview
            }
            ToolbarAction::None => {}
        }
    }
}

impl EditorPanels {
    /// Create new editor panels
    pub fn new(ui: &mut UiFramework) -> Result<Self> {
        let menu_bar = MenuBar::new(ui)?;
        let project_panel = ProjectPanel::new(ui)?;
        let properties_panel = PropertiesPanel::new(ui)?;
        let console_panel = ConsolePanel::new(ui)?;
        let visual_scripting_panel = VisualScriptingPanel::new(ui)?;
        let asset_browser_panel = AssetBrowserPanel::new(ui)?;
        
        Ok(Self {
            menu_bar,
            project_panel,
            properties_panel,
            console_panel,
            visual_scripting_panel,
            asset_browser_panel,
        })
    }
    
    /// Update all panels
    pub fn update(&mut self, ui: &mut UiFramework) {
        self.menu_bar.update(ui);
        self.project_panel.update(ui);
        self.properties_panel.update(ui);
        self.console_panel.update(ui);
        self.visual_scripting_panel.update(ui);
        self.asset_browser_panel.update(ui);
    }
}