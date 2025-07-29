//! Dockable Scene Panel implementation

use anyhow::Result;
use glam::Vec2;
use lumina_ui::{UiFramework, Button, Panel, Text, WidgetId, InputEvent};
use lumina_ui::widgets::button::ButtonVariant;
use glam::Vec4;
use log::debug;

use crate::layout::types::{PanelId, Rect, BuiltinPanelId};
use crate::layout::panel_trait::{DockablePanel, ContextMenuItem};
use crate::scene::{SceneObject, ObjectType};

/// Dockable Scene Panel for viewing and editing the game scene
pub struct DockableScenePanel {
    scene_objects: Vec<SceneObject>,
    selected_object: Option<String>,
    viewport_size: Vec2,
    widgets: Vec<WidgetId>, // Track widgets for cleanup
}

impl DockableScenePanel {
    pub fn new() -> Self {
        Self {
            scene_objects: Vec::new(),
            selected_object: None,
            viewport_size: Vec2::new(800.0, 600.0),
            widgets: Vec::new(),
        }
    }

    /// Add a new object to the scene
    pub fn add_object(&mut self, object_type: ObjectType, position: Vec2) -> String {
        let id = format!("obj_{}", self.scene_objects.len());
        let name = format!("{} {}", object_type.display_name(), self.scene_objects.len() + 1);
        
        let scene_object = SceneObject {
            id: id.clone(),
            name,
            position,
            rotation: 0.0,
            scale: Vec2::ONE,
            object_type,
            visible: true,
            properties: std::collections::HashMap::new(),
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

impl DockablePanel for DockableScenePanel {
    fn id(&self) -> PanelId {
        BuiltinPanelId::SceneEditor.panel_id()
    }

    fn title(&self) -> &str {
        "Scene Editor"
    }

    fn icon(&self) -> Option<&str> {
        Some("🎬") // Scene/movie icon
    }

    fn render(&mut self, ui: &mut UiFramework, _bounds: Rect) -> Result<()> {
        // Clear existing widgets
        self.widgets.clear();

        // Create main scene panel within bounds
        let scene_panel = Panel::new()
            .style(lumina_ui::widgets::WidgetStyle {
                background_color: Some([0.06, 0.06, 0.14, 1.0]), // theme.background.primary
                border_radius: Some(8.0),
                ..Default::default()
            });

        // Scene tools section
        let tools_title = Text::new("Scene Tools")
            .font_size(14.0)
            .color(Vec4::new(0.7, 0.9, 0.7, 1.0)); // Green

        let add_player_btn = Button::new("Add Player")
            .variant(ButtonVariant::Primary);

        let add_enemy_btn = Button::new("Add Enemy")
            .variant(ButtonVariant::Secondary);

        let add_platform_btn = Button::new("Add Platform")
            .variant(ButtonVariant::Secondary);

        let add_collectible_btn = Button::new("Add Collectible")
            .variant(ButtonVariant::Ghost);

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

        let objects_text = if self.scene_objects.is_empty() {
            Text::new("No objects in scene")
                .font_size(12.0)
                .color(Vec4::new(0.5, 0.5, 0.5, 1.0))
        } else {
            Text::new(&format!("{} objects in scene", self.scene_objects.len()))
                .font_size(12.0)
                .color(Vec4::new(0.7, 0.7, 0.7, 1.0))
        };

        // Scene actions
        let clear_scene_btn = Button::new("Clear Scene")
            .variant(ButtonVariant::Ghost);

        let save_scene_btn = Button::new("Save Scene")
            .variant(ButtonVariant::Primary);

        let load_scene_btn = Button::new("Load Scene")
            .variant(ButtonVariant::Secondary);

        // Add to UI framework
        let panel_id = ui.add_root_widget(Box::new(scene_panel));
        self.widgets.push(panel_id);
        
        let tools_title_id = ui.add_widget(Box::new(tools_title));
        let add_player_id = ui.add_widget(Box::new(add_player_btn));
        let add_enemy_id = ui.add_widget(Box::new(add_enemy_btn));
        let add_platform_id = ui.add_widget(Box::new(add_platform_btn));
        let add_collectible_id = ui.add_widget(Box::new(add_collectible_btn));
        
        let viewport_title_id = ui.add_widget(Box::new(viewport_title));
        let viewport_info_id = ui.add_widget(Box::new(viewport_info));
        
        let objects_title_id = ui.add_widget(Box::new(objects_title));
        let objects_text_id = ui.add_widget(Box::new(objects_text));
        
        let clear_id = ui.add_widget(Box::new(clear_scene_btn));
        let save_id = ui.add_widget(Box::new(save_scene_btn));
        let load_id = ui.add_widget(Box::new(load_scene_btn));
        
        // Store widget IDs for cleanup
        self.widgets.extend([
            tools_title_id, add_player_id, add_enemy_id, add_platform_id, add_collectible_id,
            viewport_title_id, viewport_info_id, objects_title_id, objects_text_id,
            clear_id, save_id, load_id
        ]);
        
        // Establish parent-child relationships
        ui.add_child_to_parent(panel_id, tools_title_id);
        ui.add_child_to_parent(panel_id, add_player_id);
        ui.add_child_to_parent(panel_id, add_enemy_id);
        ui.add_child_to_parent(panel_id, add_platform_id);
        ui.add_child_to_parent(panel_id, add_collectible_id);
        ui.add_child_to_parent(panel_id, viewport_title_id);
        ui.add_child_to_parent(panel_id, viewport_info_id);
        ui.add_child_to_parent(panel_id, objects_title_id);
        ui.add_child_to_parent(panel_id, objects_text_id);
        ui.add_child_to_parent(panel_id, clear_id);
        ui.add_child_to_parent(panel_id, save_id);
        ui.add_child_to_parent(panel_id, load_id);

        Ok(())
    }

    fn handle_input(&mut self, event: &InputEvent) -> bool {
        // Handle scene-specific input events like object selection, movement, etc.
        match event {
            InputEvent::MouseDown { position, .. } => {
                // Check if click was in viewport area - for now just log
                debug!("Scene panel received click at {:?}", position);
                
                // In a real implementation, this would:
                // 1. Check if click was in viewport bounds
                // 2. Convert screen coordinates to world coordinates
                // 3. Find objects under cursor
                // 4. Handle selection/deselection
                
                true // Consume the event
            }
            _ => false, // Don't handle other events for now
        }
    }

    fn min_size(&self) -> Vec2 {
        Vec2::new(300.0, 400.0) // Minimum size for scene editing
    }

    fn preferred_size(&self) -> Vec2 {
        Vec2::new(800.0, 600.0) // Good size for scene editing
    }

    fn max_size(&self) -> Option<Vec2> {
        None // No maximum size limit
    }

    fn can_close(&self) -> bool {
        false // Scene editor shouldn't be closable
    }

    fn visible_by_default(&self) -> bool {
        true // Always visible in the editor
    }

    fn on_active_changed(&mut self, active: bool) {
        if active {
            debug!("Scene Editor panel became active");
        } else {
            debug!("Scene Editor panel became inactive");
        }
    }

    fn update(&mut self) {
        // Update scene panel - handle object positioning, selection animations, etc.
        // This is called every frame when the panel is active
    }

    fn context_menu_items(&self) -> Vec<ContextMenuItem> {
        vec![
            ContextMenuItem::new("new_scene", "New Scene"),
            ContextMenuItem::new("load_scene", "Load Scene"),
            ContextMenuItem::new("save_scene", "Save Scene").with_separator(),
            ContextMenuItem::new("clear_scene", "Clear Scene"),
            ContextMenuItem::new("scene_settings", "Scene Settings"),
        ]
    }

    fn handle_context_menu(&mut self, item_id: &str) {
        match item_id {
            "new_scene" => {
                self.clear_scene();
                debug!("Created new scene");
            }
            "load_scene" => {
                debug!("Load scene requested");
                // TODO: Open file dialog and load scene
            }
            "save_scene" => {
                debug!("Save scene requested");
                // TODO: Open save dialog and save scene
            }
            "clear_scene" => {
                self.clear_scene();
            }
            "scene_settings" => {
                debug!("Scene settings requested");
                // TODO: Open scene settings dialog
            }
            _ => {
                debug!("Unknown context menu item: {}", item_id);
            }
        }
    }
}

impl Default for DockableScenePanel {
    fn default() -> Self {
        Self::new()
    }
}