//! Editor panels that make up the visual editor interface

use anyhow::Result;
use lumina_ui::{UiFramework, Button, Panel, Text, WidgetId};
use lumina_ui::widgets::button::ButtonVariant;
use lumina_scripting::visual_scripting::{VisualScript, ScriptNode, NodeType};
use glam::{Vec4, Vec2};
use log::debug;

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

/// Represents a game object in the scene
#[derive(Debug, Clone)]
pub struct SceneObject {
    pub id: String,
    pub name: String,
    pub position: Vec2,
    pub object_type: ObjectType,
}

/// Types of objects that can be placed in the scene
#[derive(Debug, Clone)]
pub enum ObjectType {
    Player,
    Enemy,
    Platform,
    Collectible,
    Background,
    Custom(String),
}

impl ObjectType {
    pub fn display_name(&self) -> &str {
        match self {
            ObjectType::Player => "Player",
            ObjectType::Enemy => "Enemy", 
            ObjectType::Platform => "Platform",
            ObjectType::Collectible => "Collectible",
            ObjectType::Background => "Background",
            ObjectType::Custom(name) => name,
        }
    }
}

/// Scene panel for viewing and editing the game scene
pub struct ScenePanel {
    panel_id: Option<WidgetId>,
    scene_objects: Vec<SceneObject>,
    selected_object: Option<String>,
    viewport_size: Vec2,
}

impl ScenePanel {
    pub fn new(ui: &mut UiFramework) -> Result<Self> {
        let scene_panel = Panel::new()
            .style(lumina_ui::widgets::WidgetStyle {
                background_color: Some([0.06, 0.06, 0.14, 1.0]), // theme.background.primary
                border_radius: Some(12.0),
                ..Default::default()
            });

        let title = Text::new("Scene Editor")
            .font_size(18.0)
            .color(Vec4::new(1.0, 1.0, 1.0, 1.0));

        // Scene tools section
        let tools_title = Text::new("Scene Tools")
            .font_size(14.0)
            .color(Vec4::new(0.7, 0.9, 0.7, 1.0)); // Green

        let add_player_btn = Button::new("Add Player")
            .variant(ButtonVariant::Primary)
            .on_click(|| {
                println!("🎮 Adding Player object to scene");
            });

        let add_enemy_btn = Button::new("Add Enemy")
            .variant(ButtonVariant::Secondary)
            .on_click(|| {
                println!("👹 Adding Enemy object to scene");
            });

        let add_platform_btn = Button::new("Add Platform")
            .variant(ButtonVariant::Secondary)
            .on_click(|| {
                println!("🧱 Adding Platform object to scene");
            });

        let add_collectible_btn = Button::new("Add Collectible")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("💎 Adding Collectible object to scene");
            });

        // Scene viewport section
        let viewport_title = Text::new("Scene Viewport")
            .font_size(16.0)
            .color(Vec4::new(0.9, 0.9, 0.9, 1.0));

        let viewport_info = Text::new("Drag and drop objects here")
            .font_size(12.0)
            .color(Vec4::new(0.6, 0.6, 0.6, 1.0));

        // Scene objects list
        let objects_title = Text::new("Scene Objects")
            .font_size(14.0)
            .color(Vec4::new(0.7, 0.7, 0.9, 1.0)); // Blue

        let empty_scene_text = Text::new("No objects in scene")
            .font_size(12.0)
            .color(Vec4::new(0.5, 0.5, 0.5, 1.0));

        // Scene actions
        let clear_scene_btn = Button::new("Clear Scene")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🧹 Clearing all objects from scene");
            });

        let save_scene_btn = Button::new("Save Scene")
            .variant(ButtonVariant::Primary)
            .on_click(|| {
                println!("💾 Saving scene to file");
            });

        let load_scene_btn = Button::new("Load Scene")
            .variant(ButtonVariant::Secondary)
            .on_click(|| {
                println!("📂 Loading scene from file");
            });

        // Add to UI framework
        let panel_id = ui.add_root_widget(Box::new(scene_panel));
        let title_id = ui.add_widget(Box::new(title));
        
        let tools_title_id = ui.add_widget(Box::new(tools_title));
        let add_player_id = ui.add_widget(Box::new(add_player_btn));
        let add_enemy_id = ui.add_widget(Box::new(add_enemy_btn));
        let add_platform_id = ui.add_widget(Box::new(add_platform_btn));
        let add_collectible_id = ui.add_widget(Box::new(add_collectible_btn));
        
        let viewport_title_id = ui.add_widget(Box::new(viewport_title));
        let viewport_info_id = ui.add_widget(Box::new(viewport_info));
        
        let objects_title_id = ui.add_widget(Box::new(objects_title));
        let empty_scene_id = ui.add_widget(Box::new(empty_scene_text));
        
        let clear_id = ui.add_widget(Box::new(clear_scene_btn));
        let save_id = ui.add_widget(Box::new(save_scene_btn));
        let load_id = ui.add_widget(Box::new(load_scene_btn));
        
        // Establish parent-child relationships
        ui.add_child_to_parent(panel_id, title_id);
        ui.add_child_to_parent(panel_id, tools_title_id);
        ui.add_child_to_parent(panel_id, add_player_id);
        ui.add_child_to_parent(panel_id, add_enemy_id);
        ui.add_child_to_parent(panel_id, add_platform_id);
        ui.add_child_to_parent(panel_id, add_collectible_id);
        ui.add_child_to_parent(panel_id, viewport_title_id);
        ui.add_child_to_parent(panel_id, viewport_info_id);
        ui.add_child_to_parent(panel_id, objects_title_id);
        ui.add_child_to_parent(panel_id, empty_scene_id);
        ui.add_child_to_parent(panel_id, clear_id);
        ui.add_child_to_parent(panel_id, save_id);
        ui.add_child_to_parent(panel_id, load_id);

        Ok(Self {
            panel_id: Some(panel_id),
            scene_objects: Vec::new(),
            selected_object: None,
            viewport_size: Vec2::new(800.0, 600.0),
        })
    }

    pub fn update(&mut self, _ui: &mut UiFramework) {
        // Update scene panel - handle object positioning, selection, etc.
    }

    /// Add a new object to the scene
    pub fn add_object(&mut self, object_type: ObjectType, position: Vec2) -> String {
        let id = format!("obj_{}", self.scene_objects.len());
        let name = format!("{} {}", object_type.display_name(), self.scene_objects.len() + 1);
        
        let scene_object = SceneObject {
            id: id.clone(),
            name,
            position,
            object_type,
        };
        
        self.scene_objects.push(scene_object);
        debug!("Added {} to scene at {:?}", id, position);
        id
    }

    /// Remove an object from the scene
    pub fn remove_object(&mut self, object_id: &str) {
        self.scene_objects.retain(|obj| obj.id != object_id);
        if self.selected_object.as_ref() == Some(&object_id.to_string()) {
            self.selected_object = None;
        }
        debug!("Removed object {} from scene", object_id);
    }

    /// Select an object in the scene
    pub fn select_object(&mut self, object_id: &str) {
        self.selected_object = Some(object_id.to_string());
        debug!("Selected object {}", object_id);
    }

    /// Get the currently selected object
    pub fn get_selected_object(&self) -> Option<&SceneObject> {
        if let Some(selected_id) = &self.selected_object {
            self.scene_objects.iter().find(|obj| &obj.id == selected_id)
        } else {
            None
        }
    }

    /// Get all objects in the scene
    pub fn get_scene_objects(&self) -> &Vec<SceneObject> {
        &self.scene_objects
    }

    /// Clear all objects from the scene
    pub fn clear_scene(&mut self) {
        self.scene_objects.clear();
        self.selected_object = None;
        debug!("Cleared all objects from scene");
    }

    /// Move an object to a new position
    pub fn move_object(&mut self, object_id: &str, new_position: Vec2) {
        if let Some(obj) = self.scene_objects.iter_mut().find(|obj| obj.id == object_id) {
            obj.position = new_position;
            debug!("Moved object {} to {:?}", object_id, new_position);
        }
    }
}

/// Properties panel for editing object properties
pub struct PropertiesPanel {
    panel_id: Option<WidgetId>,
    selected_object_id: Option<String>,
}

impl PropertiesPanel {
    pub fn new(ui: &mut UiFramework) -> Result<Self> {
        let properties_panel = Panel::new()
            .style(lumina_ui::widgets::WidgetStyle {
                background_color: Some([0.15, 0.15, 0.25, 1.0]), // theme.surface.elevated
                border_radius: Some(12.0),
                ..Default::default()
            });

        let title = Text::new("Property Inspector")
            .font_size(18.0)
            .color(Vec4::new(1.0, 1.0, 1.0, 1.0));

        // No selection state
        let no_selection = Text::new("No object selected")
            .font_size(14.0)
            .color(Vec4::new(0.6, 0.6, 0.6, 1.0));

        let selection_hint = Text::new("Select an object in the scene to edit its properties")
            .font_size(12.0)
            .color(Vec4::new(0.5, 0.5, 0.5, 1.0));

        // Property categories
        let transform_title = Text::new("Transform")
            .font_size(14.0)
            .color(Vec4::new(0.7, 0.9, 0.7, 1.0)); // Green

        let position_label = Text::new("Position:")
            .font_size(12.0)
            .color(Vec4::new(0.8, 0.8, 0.8, 1.0));

        let position_x_btn = Button::new("X: 0.0")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("📍 Editing X position");
            });

        let position_y_btn = Button::new("Y: 0.0")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("📍 Editing Y position");
            });

        let rotation_label = Text::new("Rotation:")
            .font_size(12.0)
            .color(Vec4::new(0.8, 0.8, 0.8, 1.0));

        let rotation_btn = Button::new("0.0°")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🔄 Editing rotation");
            });

        let scale_label = Text::new("Scale:")
            .font_size(12.0)
            .color(Vec4::new(0.8, 0.8, 0.8, 1.0));

        let scale_x_btn = Button::new("X: 1.0")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("📏 Editing X scale");
            });

        let scale_y_btn = Button::new("Y: 1.0")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("📏 Editing Y scale");
            });

        // Object properties
        let properties_title = Text::new("Object Properties")
            .font_size(14.0)
            .color(Vec4::new(0.7, 0.7, 0.9, 1.0)); // Blue

        let name_label = Text::new("Name:")
            .font_size(12.0)
            .color(Vec4::new(0.8, 0.8, 0.8, 1.0));

        let name_btn = Button::new("Object Name")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("✏️ Editing object name");
            });

        let type_label = Text::new("Type:")
            .font_size(12.0)
            .color(Vec4::new(0.8, 0.8, 0.8, 1.0));

        let type_display = Text::new("Player")
            .font_size(12.0)
            .color(Vec4::new(0.9, 0.9, 0.9, 1.0));

        let visible_btn = Button::new("✓ Visible")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("👁️ Toggling object visibility");
            });

        // Custom properties
        let custom_title = Text::new("Custom Properties")
            .font_size(14.0)
            .color(Vec4::new(0.9, 0.7, 0.7, 1.0)); // Red

        let add_property_btn = Button::new("+ Add Property")
            .variant(ButtonVariant::Primary)
            .on_click(|| {
                println!("➕ Adding custom property");
            });

        let example_prop_btn = Button::new("health: 100")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🔧 Editing custom property");
            });

        // Property actions
        let reset_btn = Button::new("Reset Properties")
            .variant(ButtonVariant::Secondary)
            .on_click(|| {
                println!("🔄 Resetting object properties to defaults");
            });

        let copy_btn = Button::new("Copy Properties")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("📋 Copying object properties");
            });

        let paste_btn = Button::new("Paste Properties")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("📝 Pasting object properties");
            });

        // Add to UI framework
        let panel_id = ui.add_root_widget(Box::new(properties_panel));
        let title_id = ui.add_widget(Box::new(title));
        let no_selection_id = ui.add_widget(Box::new(no_selection));
        let selection_hint_id = ui.add_widget(Box::new(selection_hint));
        
        let transform_title_id = ui.add_widget(Box::new(transform_title));
        let position_label_id = ui.add_widget(Box::new(position_label));
        let position_x_id = ui.add_widget(Box::new(position_x_btn));
        let position_y_id = ui.add_widget(Box::new(position_y_btn));
        let rotation_label_id = ui.add_widget(Box::new(rotation_label));
        let rotation_id = ui.add_widget(Box::new(rotation_btn));
        let scale_label_id = ui.add_widget(Box::new(scale_label));
        let scale_x_id = ui.add_widget(Box::new(scale_x_btn));
        let scale_y_id = ui.add_widget(Box::new(scale_y_btn));
        
        let properties_title_id = ui.add_widget(Box::new(properties_title));
        let name_label_id = ui.add_widget(Box::new(name_label));
        let name_id = ui.add_widget(Box::new(name_btn));
        let type_label_id = ui.add_widget(Box::new(type_label));
        let type_id = ui.add_widget(Box::new(type_display));
        let visible_id = ui.add_widget(Box::new(visible_btn));
        
        let custom_title_id = ui.add_widget(Box::new(custom_title));
        let add_property_id = ui.add_widget(Box::new(add_property_btn));
        let example_prop_id = ui.add_widget(Box::new(example_prop_btn));
        
        let reset_id = ui.add_widget(Box::new(reset_btn));
        let copy_id = ui.add_widget(Box::new(copy_btn));
        let paste_id = ui.add_widget(Box::new(paste_btn));
        
        // Establish parent-child relationships
        ui.add_child_to_parent(panel_id, title_id);
        ui.add_child_to_parent(panel_id, no_selection_id);
        ui.add_child_to_parent(panel_id, selection_hint_id);
        ui.add_child_to_parent(panel_id, transform_title_id);
        ui.add_child_to_parent(panel_id, position_label_id);
        ui.add_child_to_parent(panel_id, position_x_id);
        ui.add_child_to_parent(panel_id, position_y_id);
        ui.add_child_to_parent(panel_id, rotation_label_id);
        ui.add_child_to_parent(panel_id, rotation_id);
        ui.add_child_to_parent(panel_id, scale_label_id);
        ui.add_child_to_parent(panel_id, scale_x_id);
        ui.add_child_to_parent(panel_id, scale_y_id);
        ui.add_child_to_parent(panel_id, properties_title_id);
        ui.add_child_to_parent(panel_id, name_label_id);
        ui.add_child_to_parent(panel_id, name_id);
        ui.add_child_to_parent(panel_id, type_label_id);
        ui.add_child_to_parent(panel_id, type_id);
        ui.add_child_to_parent(panel_id, visible_id);
        ui.add_child_to_parent(panel_id, custom_title_id);
        ui.add_child_to_parent(panel_id, add_property_id);
        ui.add_child_to_parent(panel_id, example_prop_id);
        ui.add_child_to_parent(panel_id, reset_id);
        ui.add_child_to_parent(panel_id, copy_id);
        ui.add_child_to_parent(panel_id, paste_id);

        Ok(Self {
            panel_id: Some(panel_id),
            selected_object_id: None,
        })
    }

    pub fn update(&mut self, _ui: &mut UiFramework) {
        // Update properties panel - this would normally update the displayed values
        // based on the currently selected object
    }

    /// Set the currently selected object to display its properties
    pub fn set_selected_object(&mut self, object_id: Option<String>) {
        self.selected_object_id = object_id;
        if let Some(id) = &self.selected_object_id {
            debug!("Property inspector now showing object: {}", id);
        } else {
            debug!("Property inspector cleared selection");
        }
    }

    /// Get the currently selected object ID
    pub fn get_selected_object(&self) -> Option<&String> {
        self.selected_object_id.as_ref()
    }

    /// Update displayed values for the selected object
    pub fn update_values(&mut self, _ui: &mut UiFramework, _object: &crate::scene::SceneObject) {
        // This would update the actual displayed values in the UI
        // For now, we'll just log that an update is happening
        debug!("Updating property inspector values");
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

/// Asset browser panel for managing game assets
pub struct AssetBrowserPanel {
    panel_id: Option<WidgetId>,
    current_filter: Option<crate::assets::AssetType>,
    search_query: String,
}

impl AssetBrowserPanel {
    pub fn new(ui: &mut UiFramework) -> Result<Self> {
        let asset_panel = Panel::new()
            .style(lumina_ui::widgets::WidgetStyle {
                background_color: Some([0.12, 0.12, 0.20, 1.0]), // Slightly different shade
                border_radius: Some(12.0),
                ..Default::default()
            });

        let title = Text::new("Asset Browser")
            .font_size(18.0)
            .color(Vec4::new(1.0, 1.0, 1.0, 1.0));

        // Search and filter section
        let search_title = Text::new("Search & Filter")
            .font_size(14.0)
            .color(Vec4::new(0.8, 0.8, 1.0, 1.0)); // Light blue

        let search_btn = Button::new("🔍 Search assets...")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🔍 Opening asset search");
            });

        let filter_all_btn = Button::new("All Assets")
            .variant(ButtonVariant::Primary)
            .on_click(|| {
                println!("📂 Showing all assets");
            });

        // Asset type filters
        let filter_title = Text::new("Filter by Type")
            .font_size(14.0)
            .color(Vec4::new(0.9, 0.7, 0.9, 1.0)); // Light purple

        let filter_images_btn = Button::new("🖼️ Images")
            .variant(ButtonVariant::Secondary)
            .on_click(|| {
                println!("🖼️ Filtering to show only images");
            });

        let filter_audio_btn = Button::new("🔊 Audio")
            .variant(ButtonVariant::Secondary)
            .on_click(|| {
                println!("🔊 Filtering to show only audio files");
            });

        let filter_scripts_btn = Button::new("📜 Scripts")
            .variant(ButtonVariant::Secondary)
            .on_click(|| {
                println!("📜 Filtering to show only scripts");
            });

        let filter_scenes_btn = Button::new("🎬 Scenes")
            .variant(ButtonVariant::Secondary)
            .on_click(|| {
                println!("🎬 Filtering to show only scenes");
            });

        // Asset actions
        let actions_title = Text::new("Asset Actions")
            .font_size(14.0)
            .color(Vec4::new(0.7, 0.9, 0.7, 1.0)); // Green

        let import_btn = Button::new("📥 Import Assets")
            .variant(ButtonVariant::Primary)
            .on_click(|| {
                println!("📥 Opening file dialog to import assets");
            });

        let import_folder_btn = Button::new("📁 Import Folder")
            .variant(ButtonVariant::Secondary)
            .on_click(|| {
                println!("📁 Opening folder dialog to import assets");
            });

        let refresh_btn = Button::new("🔄 Refresh")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🔄 Refreshing asset database");
            });

        // Asset list section
        let assets_title = Text::new("Assets")
            .font_size(16.0)
            .color(Vec4::new(0.9, 0.9, 0.9, 1.0));

        let assets_count = Text::new("42 assets found")
            .font_size(12.0)
            .color(Vec4::new(0.7, 0.7, 0.7, 1.0));

        // Example assets (these would be dynamically generated)
        let example_image = Button::new("🖼️ player_sprite.png")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🖼️ Selected player_sprite.png");
            });

        let example_audio = Button::new("🔊 jump_sound.wav")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🔊 Selected jump_sound.wav");
            });

        let example_script = Button::new("📜 player_movement.lua")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("📜 Selected player_movement.lua");
            });

        let example_scene = Button::new("🎬 level_01.scene")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🎬 Selected level_01.scene");
            });

        // Asset details section
        let details_title = Text::new("Asset Details")
            .font_size(14.0)
            .color(Vec4::new(0.9, 0.9, 0.7, 1.0)); // Light yellow

        let no_selection = Text::new("No asset selected")
            .font_size(12.0)
            .color(Vec4::new(0.6, 0.6, 0.6, 1.0));

        let example_details = Text::new("Type: Image, Size: 64x64, 2.3 KB")
            .font_size(12.0)
            .color(Vec4::new(0.8, 0.8, 0.8, 1.0));

        // Asset management
        let management_title = Text::new("Asset Management")
            .font_size(14.0)
            .color(Vec4::new(0.9, 0.7, 0.7, 1.0)); // Light red

        let delete_btn = Button::new("🗑️ Delete Asset")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("🗑️ Deleting selected asset");
            });

        let rename_btn = Button::new("✏️ Rename Asset")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("✏️ Renaming selected asset");
            });

        let properties_btn = Button::new("⚙️ Asset Properties")
            .variant(ButtonVariant::Ghost)
            .on_click(|| {
                println!("⚙️ Opening asset properties dialog");
            });

        // Add to UI framework
        let panel_id = ui.add_root_widget(Box::new(asset_panel));
        let title_id = ui.add_widget(Box::new(title));
        
        let search_title_id = ui.add_widget(Box::new(search_title));
        let search_id = ui.add_widget(Box::new(search_btn));
        let filter_all_id = ui.add_widget(Box::new(filter_all_btn));
        
        let filter_title_id = ui.add_widget(Box::new(filter_title));
        let filter_images_id = ui.add_widget(Box::new(filter_images_btn));
        let filter_audio_id = ui.add_widget(Box::new(filter_audio_btn));
        let filter_scripts_id = ui.add_widget(Box::new(filter_scripts_btn));
        let filter_scenes_id = ui.add_widget(Box::new(filter_scenes_btn));
        
        let actions_title_id = ui.add_widget(Box::new(actions_title));
        let import_id = ui.add_widget(Box::new(import_btn));
        let import_folder_id = ui.add_widget(Box::new(import_folder_btn));
        let refresh_id = ui.add_widget(Box::new(refresh_btn));
        
        let assets_title_id = ui.add_widget(Box::new(assets_title));
        let assets_count_id = ui.add_widget(Box::new(assets_count));
        let example_image_id = ui.add_widget(Box::new(example_image));
        let example_audio_id = ui.add_widget(Box::new(example_audio));
        let example_script_id = ui.add_widget(Box::new(example_script));
        let example_scene_id = ui.add_widget(Box::new(example_scene));
        
        let details_title_id = ui.add_widget(Box::new(details_title));
        let no_selection_id = ui.add_widget(Box::new(no_selection));
        let example_details_id = ui.add_widget(Box::new(example_details));
        
        let management_title_id = ui.add_widget(Box::new(management_title));
        let delete_id = ui.add_widget(Box::new(delete_btn));
        let rename_id = ui.add_widget(Box::new(rename_btn));
        let properties_id = ui.add_widget(Box::new(properties_btn));
        
        // Establish parent-child relationships
        ui.add_child_to_parent(panel_id, title_id);
        ui.add_child_to_parent(panel_id, search_title_id);
        ui.add_child_to_parent(panel_id, search_id);
        ui.add_child_to_parent(panel_id, filter_all_id);
        ui.add_child_to_parent(panel_id, filter_title_id);
        ui.add_child_to_parent(panel_id, filter_images_id);
        ui.add_child_to_parent(panel_id, filter_audio_id);
        ui.add_child_to_parent(panel_id, filter_scripts_id);
        ui.add_child_to_parent(panel_id, filter_scenes_id);
        ui.add_child_to_parent(panel_id, actions_title_id);
        ui.add_child_to_parent(panel_id, import_id);
        ui.add_child_to_parent(panel_id, import_folder_id);
        ui.add_child_to_parent(panel_id, refresh_id);
        ui.add_child_to_parent(panel_id, assets_title_id);
        ui.add_child_to_parent(panel_id, assets_count_id);
        ui.add_child_to_parent(panel_id, example_image_id);
        ui.add_child_to_parent(panel_id, example_audio_id);
        ui.add_child_to_parent(panel_id, example_script_id);
        ui.add_child_to_parent(panel_id, example_scene_id);
        ui.add_child_to_parent(panel_id, details_title_id);
        ui.add_child_to_parent(panel_id, no_selection_id);
        ui.add_child_to_parent(panel_id, example_details_id);
        ui.add_child_to_parent(panel_id, management_title_id);
        ui.add_child_to_parent(panel_id, delete_id);
        ui.add_child_to_parent(panel_id, rename_id);
        ui.add_child_to_parent(panel_id, properties_id);

        Ok(Self {
            panel_id: Some(panel_id),
            current_filter: None,
            search_query: String::new(),
        })
    }

    pub fn update(&mut self, _ui: &mut UiFramework) {
        // Update asset browser - refresh asset list, handle filtering, etc.
    }

    /// Set the current asset type filter
    pub fn set_filter(&mut self, filter: Option<crate::assets::AssetType>) {
        debug!("Asset browser filter changed to: {:?}", filter);
        self.current_filter = filter;
    }

    /// Get the current filter
    pub fn get_filter(&self) -> Option<&crate::assets::AssetType> {
        self.current_filter.as_ref()
    }

    /// Set the current search query
    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
        debug!("Asset browser search query changed to: {}", self.search_query);
    }

    /// Get the current search query
    pub fn get_search_query(&self) -> &str {
        &self.search_query
    }

    /// Import assets from a directory
    pub fn import_from_directory(&mut self, _path: &str) {
        // This would use the AssetDatabase to import assets
        debug!("Importing assets from directory");
    }

    /// Refresh the asset database
    pub fn refresh_assets(&mut self) {
        debug!("Refreshing asset database");
    }
}