//! Editor panels that make up the visual editor interface

use anyhow::Result;
use lumina_ui::{UiFramework, Button, Panel, Text, WidgetId};
use lumina_ui::widgets::button::ButtonVariant;
use lumina_scripting::visual_scripting::{VisualScript, ScriptNode, NodeType};
use glam::Vec4;
use log::{info, debug, warn, error};

/// Menu bar panel with file operations and tools
pub struct MenuBar {
    panel_id: Option<WidgetId>,
}

impl MenuBar {
    pub fn new(ui: &mut UiFramework) -> Result<Self> {
        debug!("Creating menu bar panel...");
        // Create menu bar with modern dark theme styling
        let menu_panel = Panel::new()
            .style(lumina_ui::widgets::WidgetStyle {
                background_color: Some([0.1, 0.1, 0.18, 1.0]), // theme.surface.default
                border_radius: Some(8.0),
                ..Default::default()
            });

        let file_button = Button::new("File")
            .variant(ButtonVariant::Ghost);
            
        let edit_button = Button::new("Edit")
            .variant(ButtonVariant::Ghost);
            
        let view_button = Button::new("View")
            .variant(ButtonVariant::Ghost);
            
        let help_button = Button::new("Help")
            .variant(ButtonVariant::Ghost);

        // Add to UI framework
        let panel_id = ui.add_root_widget(Box::new(menu_panel));
        let file_id = ui.add_widget(Box::new(file_button));
        let edit_id = ui.add_widget(Box::new(edit_button));
        let view_id = ui.add_widget(Box::new(view_button));
        let help_id = ui.add_widget(Box::new(help_button));
        
        // Establish parent-child relationships
        ui.add_child_to_parent(panel_id, file_id);
        ui.add_child_to_parent(panel_id, edit_id);
        ui.add_child_to_parent(panel_id, view_id);
        ui.add_child_to_parent(panel_id, help_id);

        Ok(Self {
            panel_id: Some(panel_id),
        })
    }

    pub fn update(&mut self, _ui: &mut UiFramework) {
        // Update menu bar state
    }
}

/// Project panel showing the current project structure
pub struct ProjectPanel {
    panel_id: Option<WidgetId>,
}

impl ProjectPanel {
    pub fn new(ui: &mut UiFramework) -> Result<Self> {
        let project_panel = Panel::new()
            .style(lumina_ui::widgets::WidgetStyle {
                background_color: Some([0.15, 0.15, 0.25, 1.0]), // theme.surface.elevated
                border_radius: Some(12.0),
                ..Default::default()
            });

        let title = Text::new("Project")
            .font_size(24.0)
            .color(Vec4::new(1.0, 1.0, 1.0, 1.0));

        let new_project_btn = Button::new("New Project")
            .variant(ButtonVariant::Primary)
            .on_click(|| {
                println!("📁 Creating new project...");
            });

        let load_project_btn = Button::new("Load Project")
            .variant(ButtonVariant::Secondary)
            .on_click(|| {
                println!("📂 Loading existing project...");
            });

        // Add to UI framework
        let panel_id = ui.add_root_widget(Box::new(project_panel));
        let title_id = ui.add_widget(Box::new(title));
        let new_id = ui.add_widget(Box::new(new_project_btn));
        let load_id = ui.add_widget(Box::new(load_project_btn));
        
        // Establish parent-child relationships
        ui.add_child_to_parent(panel_id, title_id);
        ui.add_child_to_parent(panel_id, new_id);
        ui.add_child_to_parent(panel_id, load_id);

        Ok(Self {
            panel_id: Some(panel_id),
        })
    }

    pub fn update(&mut self, _ui: &mut UiFramework) {
        // Update project panel
    }
}

/// Scene panel for viewing and editing the game scene
pub struct ScenePanel {
    panel_id: Option<WidgetId>,
}

impl ScenePanel {
    pub fn new(ui: &mut UiFramework) -> Result<Self> {
        let scene_panel = Panel::new()
            .style(lumina_ui::widgets::WidgetStyle {
                background_color: Some([0.06, 0.06, 0.14, 1.0]), // theme.background.primary
                border_radius: Some(12.0),
                ..Default::default()
            });

        let title = Text::new("Scene")
            .font_size(16.0)
            .color(Vec4::new(1.0, 1.0, 1.0, 1.0));

        let scene_info = Text::new("Scene viewport will go here")
            .font_size(14.0)
            .color(Vec4::new(0.8, 0.8, 0.8, 1.0));

        // Add to UI framework
        let panel_id = ui.add_root_widget(Box::new(scene_panel));
        let title_id = ui.add_widget(Box::new(title));
        let info_id = ui.add_widget(Box::new(scene_info));
        
        // Establish parent-child relationships
        ui.add_child_to_parent(panel_id, title_id);
        ui.add_child_to_parent(panel_id, info_id);

        Ok(Self {
            panel_id: Some(panel_id),
        })
    }

    pub fn update(&mut self, _ui: &mut UiFramework) {
        // Update scene panel
    }
}

/// Properties panel for editing object properties
pub struct PropertiesPanel {
    panel_id: Option<WidgetId>,
}

impl PropertiesPanel {
    pub fn new(ui: &mut UiFramework) -> Result<Self> {
        let properties_panel = Panel::new()
            .style(lumina_ui::widgets::WidgetStyle {
                background_color: Some([0.15, 0.15, 0.25, 1.0]), // theme.surface.elevated
                border_radius: Some(12.0),
                ..Default::default()
            });

        let title = Text::new("Properties")
            .font_size(16.0)
            .color(Vec4::new(1.0, 1.0, 1.0, 1.0));

        let no_selection = Text::new("No object selected")
            .font_size(14.0)
            .color(Vec4::new(0.6, 0.6, 0.6, 1.0));

        // Add to UI framework
        let panel_id = ui.add_root_widget(Box::new(properties_panel));
        let title_id = ui.add_widget(Box::new(title));
        let info_id = ui.add_widget(Box::new(no_selection));
        
        // Establish parent-child relationships
        ui.add_child_to_parent(panel_id, title_id);
        ui.add_child_to_parent(panel_id, info_id);

        Ok(Self {
            panel_id: Some(panel_id),
        })
    }

    pub fn update(&mut self, _ui: &mut UiFramework) {
        // Update properties panel
    }
}

/// Console panel for logs and debugging
pub struct ConsolePanel {
    panel_id: Option<WidgetId>,
}

impl ConsolePanel {
    pub fn new(ui: &mut UiFramework) -> Result<Self> {
        let console_panel = Panel::new()
            .style(lumina_ui::widgets::WidgetStyle {
                background_color: Some([0.1, 0.1, 0.18, 1.0]), // theme.surface.default
                border_radius: Some(12.0),
                ..Default::default()
            });

        let title = Text::new("Console")
            .font_size(16.0)
            .color(Vec4::new(1.0, 1.0, 1.0, 1.0));

        let log_text = Text::new("Editor initialized successfully")
            .font_size(12.0)
            .color(Vec4::new(0.0, 0.8, 0.4, 1.0));

        let clear_btn = Button::new("Clear")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🧹 Console cleared");
            });

        // Add to UI framework  
        let panel_id = ui.add_root_widget(Box::new(console_panel));
        let title_id = ui.add_widget(Box::new(title));
        let log_id = ui.add_widget(Box::new(log_text));
        let clear_id = ui.add_widget(Box::new(clear_btn));
        
        // Establish parent-child relationships
        ui.add_child_to_parent(panel_id, title_id);
        ui.add_child_to_parent(panel_id, log_id);
        ui.add_child_to_parent(panel_id, clear_id);

        Ok(Self {
            panel_id: Some(panel_id),
        })
    }

    pub fn update(&mut self, _ui: &mut UiFramework) {
        // Update console panel
    }
}

/// Visual scripting panel for node-based programming
pub struct VisualScriptingPanel {
    panel_id: Option<WidgetId>,
    current_script: Option<VisualScript>,
    selected_node_type: Option<NodeType>,
}

impl VisualScriptingPanel {
    pub fn new(ui: &mut UiFramework) -> Result<Self> {
        let scripting_panel = Panel::new()
            .style(lumina_ui::widgets::WidgetStyle {
                background_color: Some([0.15, 0.15, 0.25, 1.0]), // theme.surface.elevated  
                border_radius: Some(12.0),
                ..Default::default()
            });

        let title = Text::new("Visual Scripting")
            .font_size(16.0)
            .color(Vec4::new(1.0, 1.0, 1.0, 1.0));

        let script_info = Text::new("No script loaded")
            .font_size(12.0)
            .color(Vec4::new(0.7, 0.7, 0.7, 1.0));

        let new_script_btn = Button::new("New Script")
            .variant(ButtonVariant::Primary)
            .on_click(|| {
                println!("📜 Creating new visual script...");
                // TODO: Open script creation dialog
            });

        let load_script_btn = Button::new("Load Script")
            .variant(ButtonVariant::Secondary)
            .on_click(|| {
                println!("📂 Loading visual script...");
            });

        // Example script buttons
        let examples_title = Text::new("Example Scripts")
            .font_size(14.0)
            .color(Vec4::new(0.6, 0.9, 0.6, 1.0)); // Green

        let player_movement_btn = Button::new("Player Movement")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🎮 Creating Player Movement script...");
            });

        let coin_collection_btn = Button::new("Coin Collection")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🪙 Creating Coin Collection script...");
            });

        let enemy_ai_btn = Button::new("Enemy AI")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🤖 Creating Enemy AI script...");
            });

        // Node type buttons for adding nodes
        let event_nodes_title = Text::new("Event Nodes")
            .font_size(14.0)
            .color(Vec4::new(0.3, 0.6, 1.0, 1.0)); // Blue

        let on_start_btn = Button::new("On Start")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🟦 Adding On Start event node");
            });

        let on_update_btn = Button::new("On Update")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🟦 Adding On Update event node");
            });

        let action_nodes_title = Text::new("Action Nodes")
            .font_size(14.0)
            .color(Vec4::new(1.0, 0.3, 0.3, 1.0)); // Red

        let move_node_btn = Button::new("Move Towards")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🟥 Adding Move Towards action node");
            });

        let play_sound_btn = Button::new("Play Sound")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🟥 Adding Play Sound action node");
            });

        let logic_nodes_title = Text::new("Logic Nodes")
            .font_size(14.0)
            .color(Vec4::new(1.0, 0.9, 0.2, 1.0)); // Yellow

        let if_node_btn = Button::new("If Statement")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🟨 Adding If Statement logic node");
            });

        let compare_btn = Button::new("Compare")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🟨 Adding Compare logic node");
            });

        // Add to UI framework
        let panel_id = ui.add_root_widget(Box::new(scripting_panel));
        let title_id = ui.add_widget(Box::new(title));
        let script_info_id = ui.add_widget(Box::new(script_info));
        let new_id = ui.add_widget(Box::new(new_script_btn));
        let load_id = ui.add_widget(Box::new(load_script_btn));
        
        let examples_title_id = ui.add_widget(Box::new(examples_title));
        let player_movement_id = ui.add_widget(Box::new(player_movement_btn));
        let coin_collection_id = ui.add_widget(Box::new(coin_collection_btn));
        let enemy_ai_id = ui.add_widget(Box::new(enemy_ai_btn));
        
        let event_title_id = ui.add_widget(Box::new(event_nodes_title));
        let on_start_id = ui.add_widget(Box::new(on_start_btn));
        let on_update_id = ui.add_widget(Box::new(on_update_btn));
        
        let action_title_id = ui.add_widget(Box::new(action_nodes_title));
        let move_id = ui.add_widget(Box::new(move_node_btn));
        let sound_id = ui.add_widget(Box::new(play_sound_btn));
        
        let logic_title_id = ui.add_widget(Box::new(logic_nodes_title));
        let if_id = ui.add_widget(Box::new(if_node_btn));
        let compare_id = ui.add_widget(Box::new(compare_btn));
        
        // Establish parent-child relationships
        ui.add_child_to_parent(panel_id, title_id);
        ui.add_child_to_parent(panel_id, script_info_id);
        ui.add_child_to_parent(panel_id, new_id);
        ui.add_child_to_parent(panel_id, load_id);
        ui.add_child_to_parent(panel_id, examples_title_id);
        ui.add_child_to_parent(panel_id, player_movement_id);
        ui.add_child_to_parent(panel_id, coin_collection_id);
        ui.add_child_to_parent(panel_id, enemy_ai_id);
        ui.add_child_to_parent(panel_id, event_title_id);
        ui.add_child_to_parent(panel_id, on_start_id);
        ui.add_child_to_parent(panel_id, on_update_id);
        ui.add_child_to_parent(panel_id, action_title_id);
        ui.add_child_to_parent(panel_id, move_id);
        ui.add_child_to_parent(panel_id, sound_id);
        ui.add_child_to_parent(panel_id, logic_title_id);
        ui.add_child_to_parent(panel_id, if_id);
        ui.add_child_to_parent(panel_id, compare_id);

        Ok(Self {
            panel_id: Some(panel_id),
            current_script: None,
            selected_node_type: None,
        })
    }

    pub fn update(&mut self, _ui: &mut UiFramework) {
        // Update visual scripting panel
        // Here we would handle node creation, connection, and script execution
    }

    /// Create a new empty visual script
    pub fn new_script(&mut self, name: String) {
        self.current_script = Some(VisualScript {
            name,
            nodes: Vec::new(),
            connections: Vec::new(),
            variables: std::collections::HashMap::new(),
        });
        println!("Created new script: {}", self.current_script.as_ref().unwrap().name);
    }

    /// Create a predefined player movement script
    pub fn create_player_movement_script(&mut self) {
        use lumina_scripting::visual_scripting::create_player_movement_script;
        self.current_script = Some(create_player_movement_script());
        println!("Created Player Movement script with {} nodes", 
                 self.current_script.as_ref().unwrap().nodes.len());
    }

    /// Create a predefined coin collection script
    pub fn create_coin_collection_script(&mut self) {
        use lumina_scripting::visual_scripting::create_coin_collection_script;
        self.current_script = Some(create_coin_collection_script());
        println!("Created Coin Collection script with {} nodes", 
                 self.current_script.as_ref().unwrap().nodes.len());
    }

    /// Create a predefined enemy AI script
    pub fn create_enemy_ai_script(&mut self) {
        use lumina_scripting::visual_scripting::create_enemy_ai_script;
        self.current_script = Some(create_enemy_ai_script());
        println!("Created Enemy AI script with {} nodes", 
                 self.current_script.as_ref().unwrap().nodes.len());
    }

    /// Create a predefined top-down movement script
    pub fn create_topdown_movement_script(&mut self) {
        use lumina_scripting::visual_scripting::create_topdown_movement_script;
        self.current_script = Some(create_topdown_movement_script());
        println!("Created Top-Down Movement script with {} nodes", 
                 self.current_script.as_ref().unwrap().nodes.len());
    }

    /// Add a node to the current script
    pub fn add_node(&mut self, node_type: NodeType, position: (f32, f32)) {
        if let Some(script) = &mut self.current_script {
            let node = ScriptNode {
                id: format!("node_{}", script.nodes.len()),
                node_type,
                position,
                properties: std::collections::HashMap::new(),
            };
            script.nodes.push(node);
        }
    }

    /// Get the current script
    pub fn current_script(&self) -> Option<&VisualScript> {
        self.current_script.as_ref()
    }

    /// Save the current script to a file
    pub fn save_script(&self, path: &str) -> Result<()> {
        if let Some(script) = &self.current_script {
            let json = serde_json::to_string_pretty(script)?;
            std::fs::write(path, json)?;
        }
        Ok(())
    }

    /// Load a script from a file
    pub fn load_script(&mut self, path: &str) -> Result<()> {
        let content = std::fs::read_to_string(path)?;
        let script: VisualScript = serde_json::from_str(&content)?;
        self.current_script = Some(script);
        Ok(())
    }
}