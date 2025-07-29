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

use crate::panels::{MenuBar, ProjectPanel, ScenePanel, PropertiesPanel, ConsolePanel, VisualScriptingPanel, AssetBrowserPanel};
use crate::project::EditorProject;
use crate::scene::SceneManager;
use crate::assets::AssetBrowser;

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
    panels: EditorPanels,
    
    // Input tracking
    mouse_position: Vec2,
}

/// Container for all editor panels
pub struct EditorPanels {
    pub menu_bar: MenuBar,
    pub project_panel: ProjectPanel,
    pub scene_panel: ScenePanel,
    pub properties_panel: PropertiesPanel,
    pub console_panel: ConsolePanel,
    pub visual_scripting_panel: VisualScriptingPanel,
    pub asset_browser_panel: AssetBrowserPanel,
}

impl EditorApp {
    /// Create a new editor application with ECS architecture
    pub fn new() -> Result<Self> {
        debug!("Creating editor application...");
        
        // Initialize ECS World
        let world = World::new();
        
        // Create UI framework with dark theme
        debug!("Setting up UI framework with dark theme...");
        let theme = Theme::dark();
        let mut ui_framework = UiFramework::new(theme);
        
        // Initialize editor panels
        debug!("Initializing editor panels...");
        let panels = EditorPanels::new(&mut ui_framework)?;
        
        info!("Editor app initialized");
        
        Ok(Self {
            world,
            ui_framework,
            current_project: None,
            scene_manager: SceneManager::new(),
            asset_browser: AssetBrowser::new(),
            panels,
            mouse_position: Vec2::ZERO,
        })
    }
    
}

impl EcsApp for EditorApp {
    fn setup(&mut self, _world: &mut World) -> Result<()> {
        info!("Setting up editor ECS systems...");
        debug!("Editor ECS setup complete");
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
        // Update editor panels
        self.panels.update(&mut self.ui_framework);
        
        // Update UI layout
        let screen_size = Vec2::new(1400.0, 900.0); // TODO: Get from window size
        self.ui_framework.update_layout(screen_size);
        
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
                self.ui_framework.handle_input(input_event);
                Ok(true)
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
                
                self.ui_framework.handle_input(input_event);
                Ok(true)
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
                
                self.ui_framework.handle_input(input_event);
                Ok(true)
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
}

impl EditorPanels {
    /// Create new editor panels
    pub fn new(ui: &mut UiFramework) -> Result<Self> {
        let menu_bar = MenuBar::new(ui)?;
        let project_panel = ProjectPanel::new(ui)?;
        let scene_panel = ScenePanel::new(ui)?;
        let properties_panel = PropertiesPanel::new(ui)?;
        let console_panel = ConsolePanel::new(ui)?;
        let visual_scripting_panel = VisualScriptingPanel::new(ui)?;
        let asset_browser_panel = AssetBrowserPanel::new(ui)?;
        
        Ok(Self {
            menu_bar,
            project_panel,
            scene_panel,
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
        self.scene_panel.update(ui);
        self.properties_panel.update(ui);
        self.console_panel.update(ui);
        self.visual_scripting_panel.update(ui);
        self.asset_browser_panel.update(ui);
    }
}