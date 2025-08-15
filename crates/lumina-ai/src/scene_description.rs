//! Scene Description Language (SDL) - Simplified format for AI-generated games
//! 
//! This format follows GameSpec principles: human-diffable JSON, explicit references,
//! schema-enforced, and focused on 2D platformer games.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Complete scene description - the root of all game definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneDescription {
    /// Game metadata and configuration
    pub game: GameMeta,
    /// Asset definitions
    pub assets: Assets,
    /// Scene definitions
    pub scenes: Vec<Scene>,
    /// Game rules and event handling
    pub rules: Vec<Rule>,
}

/// Game metadata and global settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMeta {
    pub name: String,
    pub template: String, // "platformer", "shooter", "puzzle"
    pub resolution: [u32; 2],
    pub background_color: [f32; 4],
    pub physics: PhysicsSettings,
}

/// Physics world configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsSettings {
    pub enabled: bool,
    pub gravity: [f32; 2],
    pub timestep: f32,
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            gravity: [0.0, -9.81],
            timestep: 1.0 / 60.0,
        }
    }
}

/// Asset definitions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Assets {
    #[serde(default)]
    pub tilesets: Vec<TilesetAsset>,
    #[serde(default)]
    pub sprites: Vec<SpriteAsset>,
    #[serde(default)]
    pub sounds: Vec<SoundAsset>,
}

/// Tileset asset definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TilesetAsset {
    pub id: String,
    pub src: String,
    pub tile_w: u32,
    pub tile_h: u32,
}

/// Sprite asset definition  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteAsset {
    pub id: String,
    pub src: String,
    #[serde(default)]
    pub atlas: Option<AtlasConfig>,
}

/// Atlas configuration for sprite sheets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtlasConfig {
    pub frame_w: u32,
    pub frame_h: u32,
}

/// Sound asset definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundAsset {
    pub id: String,
    pub src: String,
}

/// Scene definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    #[serde(default)]
    pub tilemaps: Vec<Tilemap>,
    #[serde(default)]
    pub entities: Vec<Entity>,
}

/// Tilemap in a scene - first-class citizen for platformers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tilemap {
    pub tileset: String, // Asset ID reference
    pub grid: Vec<String>, // Simple string format: ["...###...", "........"]
}

/// Entity specification with components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub components: HashMap<String, ComponentData>,
}

/// Generic component data holder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentData {
    #[serde(rename = "type")]
    pub component_type: String,
    pub data: serde_json::Value,
}

/// Game rules and event handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub event: String,
    pub action: Action,
}

/// Action specification for rules
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    LoadScene { scene: String },
    SpawnEntity { entity: Entity },
    DestroyEntity { name: String },
    PlaySound { sound: String },
    UpdateScore { points: i32 },
    ShowMessage { text: String },
}

impl SceneDescription {
    /// Create a platformer template
    pub fn platformer_template(name: &str) -> Self {
        Self {
            game: GameMeta {
                name: name.to_string(),
                template: "platformer".to_string(),
                resolution: [1280, 720],
                background_color: [0.2, 0.3, 0.8, 1.0],
                physics: PhysicsSettings::default(),
            },
            assets: Assets {
                tilesets: vec![TilesetAsset {
                    id: "base_ts".to_string(),
                    src: "tiles/base.png".to_string(),
                    tile_w: 16,
                    tile_h: 16,
                }],
                sprites: vec![SpriteAsset {
                    id: "hero".to_string(),
                    src: "sprites/hero.png".to_string(),
                    atlas: Some(AtlasConfig {
                        frame_w: 16,
                        frame_h: 16,
                    }),
                }],
                sounds: vec![],
            },
            scenes: vec![Scene {
                id: "level_1".to_string(),
                tilemaps: vec![Tilemap {
                    tileset: "base_ts".to_string(),
                    grid: vec![
                        "................".to_string(),
                        "................".to_string(),
                        "....####........".to_string(),
                        ".............###".to_string(),
                        "###.........####".to_string(),
                        "################".to_string(),
                    ],
                }],
                entities: vec![
                    create_player_entity(),
                    create_goal_entity(),
                ],
            }],
            rules: vec![Rule {
                event: "win_level".to_string(),
                action: Action::ShowMessage {
                    text: "Level Complete!".to_string(),
                },
            }],
        }
    }

    /// Validate the scene description
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        // Check asset references
        for scene in &self.scenes {
            // Check tilemap references
            for tilemap in &scene.tilemaps {
                if !self.assets.tilesets.iter().any(|t| t.id == tilemap.tileset) {
                    errors.push(format!("Tilemap references unknown tileset: {}", tilemap.tileset));
                }
            }
            
            // Check sprite references in entities
            for entity in &scene.entities {
                if let Some(sprite_comp) = entity.components.get("Sprite") {
                    if let Ok(atlas_id) = serde_json::from_value::<String>(sprite_comp.data["atlas_id"].clone()) {
                        if !self.assets.sprites.iter().any(|s| s.id == atlas_id) {
                            errors.push(format!("Entity '{}' references unknown sprite: {}", entity.name, atlas_id));
                        }
                    }
                }
            }
        }

        // Check for required components
        for scene in &self.scenes {
            for entity in &scene.entities {
                if entity.tags.contains(&"player".to_string()) {
                    if !entity.components.contains_key("Transform2D") {
                        errors.push(format!("Player entity '{}' missing Transform2D component", entity.name));
                    }
                    if !entity.components.contains_key("CharacterController2D") {
                        errors.push(format!("Player entity '{}' missing CharacterController2D component", entity.name));
                    }
                }
            }
        }

        errors
    }
}

/// Create a basic player entity with 2D components
fn create_player_entity() -> Entity {
    let mut components = HashMap::new();
    
    components.insert("Transform2D".to_string(), ComponentData {
        component_type: "Transform2D".to_string(),
        data: serde_json::json!({
            "pos": [32.0, 200.0],
            "rot": 0.0,
            "scale": [1.0, 1.0]
        }),
    });
    
    components.insert("Sprite".to_string(), ComponentData {
        component_type: "Sprite".to_string(),
        data: serde_json::json!({
            "atlas_id": "hero",
            "frame": 0,
            "color": [1.0, 1.0, 1.0, 1.0],
            "flip_x": false,
            "flip_y": false,
            "layer": 10
        }),
    });
    
    components.insert("RigidBody2D".to_string(), ComponentData {
        component_type: "RigidBody2D".to_string(),
        data: serde_json::json!({
            "kind": "Dynamic",
            "vel": [0.0, 0.0],
            "mass": 1.0,
            "linear_damping": 0.1,
            "angular_damping": 0.5,
            "fixed_rotation": true
        }),
    });
    
    components.insert("Collider2D".to_string(), ComponentData {
        component_type: "Collider2D".to_string(),
        data: serde_json::json!({
            "shape": {
                "AABB": { "width": 14.0, "height": 16.0 }
            },
            "is_sensor": false,
            "friction": 0.8,
            "restitution": 0.0,
            "collision_layers": ["solid", "enemy", "collectible"]
        }),
    });
    
    components.insert("CharacterController2D".to_string(), ComponentData {
        component_type: "CharacterController2D".to_string(),
        data: serde_json::json!({
            "speed": 200.0,
            "jump_power": 500.0,
            "coyote_time_ms": 100,
            "is_grounded": false,
            "jump_buffer_time": 0.15
        }),
    });

    Entity {
        name: "Player".to_string(),
        tags: vec!["player".to_string(), "dynamic".to_string()],
        components,
    }
}

/// Create a basic goal entity
fn create_goal_entity() -> Entity {
    let mut components = HashMap::new();
    
    components.insert("Transform2D".to_string(), ComponentData {
        component_type: "Transform2D".to_string(),
        data: serde_json::json!({
            "pos": [220.0, 200.0],
            "rot": 0.0,
            "scale": [1.0, 1.0]
        }),
    });
    
    components.insert("Collider2D".to_string(), ComponentData {
        component_type: "Collider2D".to_string(),
        data: serde_json::json!({
            "shape": {
                "AABB": { "width": 16.0, "height": 16.0 }
            },
            "is_sensor": true,
            "friction": 0.0,
            "restitution": 0.0,
            "collision_layers": ["goal"]
        }),
    });
    
    components.insert("Trigger".to_string(), ComponentData {
        component_type: "Trigger".to_string(),
        data: serde_json::json!({
            "on_enter": "win_level",
            "on_exit": null
        }),
    });

    Entity {
        name: "Goal".to_string(),
        tags: vec!["goal".to_string()],
        components,
    }
}

// Legacy compatibility - keep these for existing code
pub type SceneMetadata = GameMeta;
pub type EntityDescription = Entity;
pub type ComponentDescription = ComponentData;

impl Default for SceneMetadata {
    fn default() -> Self {
        Self {
            name: "Untitled Game".to_string(),
            template: "platformer".to_string(),
            resolution: [1280, 720],
            background_color: [0.2, 0.3, 0.8, 1.0],
            physics: PhysicsSettings::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platformer_template() {
        let scene = SceneDescription::platformer_template("Test Game");
        assert_eq!(scene.game.name, "Test Game");
        assert_eq!(scene.game.template, "platformer");
        
        let errors = scene.validate();
        assert!(errors.is_empty(), "Template should be valid: {:?}", errors);
        
        // Should have a tilemap
        assert!(!scene.scenes[0].tilemaps.is_empty());
        
        // Should have player and goal entities
        let entity_names: Vec<&str> = scene.scenes[0].entities.iter().map(|e| e.name.as_str()).collect();
        assert!(entity_names.contains(&"Player"));
        assert!(entity_names.contains(&"Goal"));
    }

    #[test]
    fn test_validation_missing_tileset() {
        let mut scene = SceneDescription::platformer_template("Test");
        scene.scenes[0].tilemaps[0].tileset = "nonexistent".to_string();
        
        let errors = scene.validate();
        assert!(!errors.is_empty());
        assert!(errors[0].contains("unknown tileset"));
    }

    #[test]
    fn test_json_serialization() {
        let scene = SceneDescription::platformer_template("Test Game");
        let json = serde_json::to_string_pretty(&scene).expect("Should serialize to JSON");
        let parsed: SceneDescription = serde_json::from_str(&json).expect("Should parse from JSON");
        
        assert_eq!(scene.game.name, parsed.game.name);
        assert_eq!(scene.scenes.len(), parsed.scenes.len());
    }
}