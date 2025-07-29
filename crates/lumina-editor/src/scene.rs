//! Scene management system for the Lumina Editor
//! 
//! This module provides functionality for managing game scenes,
//! including object placement, selection, and scene serialization.

use anyhow::Result;
use glam::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a complete game scene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    /// Unique identifier for the scene
    pub id: Uuid,
    /// Scene name
    pub name: String,
    /// Scene dimensions
    pub size: Vec2,
    /// All game objects in the scene
    pub objects: HashMap<String, SceneObject>,
    /// Scene background color
    pub background_color: [f32; 4],
    /// Scene metadata
    pub metadata: SceneMetadata,
}

/// Scene metadata for editor functionality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneMetadata {
    /// When the scene was created
    pub created_at: u64,
    /// When the scene was last modified
    pub modified_at: u64,
    /// Scene description
    pub description: String,
    /// Scene tags for organization
    pub tags: Vec<String>,
}

/// Represents a game object in the scene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneObject {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Object position in world coordinates
    pub position: Vec2,
    /// Object rotation in radians
    pub rotation: f32,
    /// Object scale
    pub scale: Vec2,
    /// Object type
    pub object_type: ObjectType,
    /// Whether the object is visible
    pub visible: bool,
    /// Object-specific properties
    pub properties: HashMap<String, ObjectProperty>,
}

/// Types of objects that can be placed in the scene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectType {
    /// Player character
    Player,
    /// Enemy character
    Enemy,
    /// Platform or ground
    Platform,
    /// Collectible item
    Collectible,
    /// Background element
    Background,
    /// Trigger area
    Trigger,
    /// Custom object type
    Custom(String),
}

impl ObjectType {
    /// Get the display name for the object type
    pub fn display_name(&self) -> &str {
        match self {
            ObjectType::Player => "Player",
            ObjectType::Enemy => "Enemy",
            ObjectType::Platform => "Platform",
            ObjectType::Collectible => "Collectible",
            ObjectType::Background => "Background",
            ObjectType::Trigger => "Trigger",
            ObjectType::Custom(name) => name,
        }
    }

    /// Get the default color for this object type (for editor visualization)
    pub fn default_color(&self) -> [f32; 4] {
        match self {
            ObjectType::Player => [0.2, 0.8, 0.2, 1.0],      // Green
            ObjectType::Enemy => [0.8, 0.2, 0.2, 1.0],       // Red
            ObjectType::Platform => [0.6, 0.4, 0.2, 1.0],    // Brown
            ObjectType::Collectible => [1.0, 0.8, 0.2, 1.0], // Gold
            ObjectType::Background => [0.4, 0.4, 0.6, 1.0],  // Blue-gray
            ObjectType::Trigger => [0.8, 0.2, 0.8, 0.5],     // Purple (transparent)
            ObjectType::Custom(_) => [0.7, 0.7, 0.7, 1.0],   // Gray
        }
    }
}

/// Property value for scene objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectProperty {
    String(String),
    Float(f32),
    Int(i32),
    Bool(bool),
    Vec2(Vec2),
    Color([f32; 4]),
}

impl Scene {
    /// Create a new empty scene
    pub fn new(name: String) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id: Uuid::new_v4(),
            name,
            size: Vec2::new(1920.0, 1080.0), // Default 1080p size
            objects: HashMap::new(),
            background_color: [0.1, 0.1, 0.2, 1.0], // Dark blue
            metadata: SceneMetadata {
                created_at: now,
                modified_at: now,
                description: String::new(),
                tags: Vec::new(),
            },
        }
    }

    /// Add an object to the scene
    pub fn add_object(&mut self, mut object: SceneObject) -> String {
        // Ensure unique ID
        while self.objects.contains_key(&object.id) {
            object.id = format!("{}_{}", object.id, rand::random::<u32>());
        }

        let id = object.id.clone();
        self.objects.insert(id.clone(), object);
        self.touch();
        
        log::info!("Added object '{}' to scene '{}'", id, self.name);
        id
    }

    /// Remove an object from the scene
    pub fn remove_object(&mut self, object_id: &str) -> Option<SceneObject> {
        let removed = self.objects.remove(object_id);
        if removed.is_some() {
            self.touch();
            log::info!("Removed object '{}' from scene '{}'", object_id, self.name);
        }
        removed
    }

    /// Get an object by ID
    pub fn get_object(&self, object_id: &str) -> Option<&SceneObject> {
        self.objects.get(object_id)
    }

    /// Get a mutable reference to an object by ID
    pub fn get_object_mut(&mut self, object_id: &str) -> Option<&mut SceneObject> {
        self.objects.get_mut(object_id)
    }

    /// Get all objects in the scene
    pub fn get_all_objects(&self) -> &HashMap<String, SceneObject> {
        &self.objects
    }

    /// Move an object to a new position
    pub fn move_object(&mut self, object_id: &str, new_position: Vec2) -> bool {
        if let Some(object) = self.objects.get_mut(object_id) {
            object.position = new_position;
            self.touch();
            log::debug!("Moved object '{}' to {:?}", object_id, new_position);
            true
        } else {
            false
        }
    }

    /// Rotate an object
    pub fn rotate_object(&mut self, object_id: &str, rotation: f32) -> bool {
        if let Some(object) = self.objects.get_mut(object_id) {
            object.rotation = rotation;
            self.touch();
            log::debug!("Rotated object '{}' to {} radians", object_id, rotation);
            true
        } else {
            false
        }
    }

    /// Scale an object
    pub fn scale_object(&mut self, object_id: &str, scale: Vec2) -> bool {
        if let Some(object) = self.objects.get_mut(object_id) {
            object.scale = scale;
            self.touch();
            log::debug!("Scaled object '{}' to {:?}", object_id, scale);
            true
        } else {
            false
        }
    }

    /// Clear all objects from the scene
    pub fn clear_objects(&mut self) {
        let count = self.objects.len();
        self.objects.clear();
        self.touch();
        log::info!("Cleared {} objects from scene '{}'", count, self.name);
    }

    /// Find objects by type
    pub fn find_objects_by_type(&self, object_type: &ObjectType) -> Vec<&SceneObject> {
        self.objects
            .values()
            .filter(|obj| std::mem::discriminant(&obj.object_type) == std::mem::discriminant(object_type))
            .collect()
    }

    /// Find objects within a rectangular area
    pub fn find_objects_in_area(&self, min: Vec2, max: Vec2) -> Vec<&SceneObject> {
        self.objects
            .values()
            .filter(|obj| {
                obj.position.x >= min.x && obj.position.x <= max.x
                    && obj.position.y >= min.y && obj.position.y <= max.y
            })
            .collect()
    }

    /// Update the last modified timestamp
    pub fn touch(&mut self) {
        self.metadata.modified_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Save the scene to a JSON file
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        log::info!("Saved scene '{}' to {}", self.name, path);
        Ok(())
    }

    /// Load a scene from a JSON file
    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let scene: Scene = serde_json::from_str(&content)?;
        log::info!("Loaded scene '{}' from {}", scene.name, path);
        Ok(scene)
    }
}

impl SceneObject {
    /// Create a new scene object
    pub fn new(name: String, object_type: ObjectType, position: Vec2) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            position,
            rotation: 0.0,
            scale: Vec2::ONE,
            object_type,
            visible: true,
            properties: HashMap::new(),
        }
    }

    /// Set a property on the object
    pub fn set_property(&mut self, key: String, value: ObjectProperty) {
        self.properties.insert(key, value);
    }

    /// Get a property from the object
    pub fn get_property(&self, key: &str) -> Option<&ObjectProperty> {
        self.properties.get(key)
    }

    /// Remove a property from the object
    pub fn remove_property(&mut self, key: &str) -> Option<ObjectProperty> {
        self.properties.remove(key)
    }

    /// Get the bounding box of the object (for selection/collision)
    pub fn get_bounds(&self) -> (Vec2, Vec2) {
        let half_scale = self.scale * 0.5;
        let min = self.position - half_scale;
        let max = self.position + half_scale;
        (min, max)
    }

    /// Check if a point is inside this object's bounds
    pub fn contains_point(&self, point: Vec2) -> bool {
        let (min, max) = self.get_bounds();
        point.x >= min.x && point.x <= max.x && point.y >= min.y && point.y <= max.y
    }
}

/// Scene manager for handling multiple scenes
#[derive(Debug)]
pub struct SceneManager {
    /// Currently active scene
    pub active_scene: Option<Scene>,
    /// Recently opened scenes
    pub recent_scenes: Vec<String>,
}

impl SceneManager {
    /// Create a new scene manager
    pub fn new() -> Self {
        Self {
            active_scene: None,
            recent_scenes: Vec::new(),
        }
    }

    /// Create a new scene and set it as active
    pub fn new_scene(&mut self, name: String) -> &mut Scene {
        let scene = Scene::new(name);
        log::info!("Created new scene '{}'", scene.name);
        self.active_scene = Some(scene);
        self.active_scene.as_mut().unwrap()
    }

    /// Load a scene from file and set it as active
    pub fn load_scene(&mut self, path: &str) -> Result<&mut Scene> {
        let scene = Scene::load_from_file(path)?;
        
        // Add to recent scenes
        let path_string = path.to_string();
        self.recent_scenes.retain(|p| p != &path_string);
        self.recent_scenes.insert(0, path_string);
        
        // Keep only the 10 most recent
        if self.recent_scenes.len() > 10 {
            self.recent_scenes.truncate(10);
        }

        self.active_scene = Some(scene);
        Ok(self.active_scene.as_mut().unwrap())
    }

    /// Save the current scene to file
    pub fn save_scene(&mut self, path: &str) -> Result<()> {
        if let Some(scene) = &self.active_scene {
            scene.save_to_file(path)?;
            
            // Add to recent scenes
            let path_string = path.to_string();
            self.recent_scenes.retain(|p| p != &path_string);
            self.recent_scenes.insert(0, path_string);
            
            if self.recent_scenes.len() > 10 {
                self.recent_scenes.truncate(10);
            }
        }
        Ok(())
    }

    /// Get the currently active scene
    pub fn get_active_scene(&self) -> Option<&Scene> {
        self.active_scene.as_ref()
    }

    /// Get a mutable reference to the currently active scene
    pub fn get_active_scene_mut(&mut self) -> Option<&mut Scene> {
        self.active_scene.as_mut()
    }

    /// Check if there is an active scene
    pub fn has_active_scene(&self) -> bool {
        self.active_scene.is_some()
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}