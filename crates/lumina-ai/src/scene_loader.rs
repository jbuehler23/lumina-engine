//! Scene loader that converts simplified SDL into ECS entities
//! 
//! This module bridges the gap between AI-generated scene descriptions and
//! the Lumina Engine's ECS system, now using 2D-focused components.

use std::collections::HashMap;
use anyhow::{Result, Context};
use log::{info, warn};
use serde_json::Value;

use lumina_ecs::{World, Entity, Transform2D, Sprite, RigidBody2D, Collider2D, 
    CharacterController2D, Camera2D, Trigger, Tilemap, ScriptTag, BodyType, CollisionShape,
    InputMap, InputState};

use crate::scene_description::{SceneDescription, Entity as SDL_Entity, ComponentData};

/// Scene loader that converts SDL to ECS entities
pub struct SceneLoader {
    /// Entity name to Entity ID mapping for cross-references
    entity_map: HashMap<String, Entity>,
    /// Warnings encountered during loading
    warnings: Vec<String>,
}

/// Result of loading a scene
#[derive(Debug, Clone)]
pub struct SceneLoadResult {
    /// Number of entities created
    pub entities_created: usize,
    /// Number of tilemaps loaded
    pub tilemaps_loaded: usize,
    /// Warnings encountered during loading
    pub warnings: Vec<String>,
    /// Root entities (entities without parents)
    pub root_entities: Vec<Entity>,
}

impl SceneLoader {
    pub fn new() -> Self {
        Self {
            entity_map: HashMap::new(),
            warnings: Vec::new(),
        }
    }

    /// Load a complete scene into the ECS world
    pub fn load_scene(&mut self, world: &mut World, scene: &SceneDescription) -> Result<SceneLoadResult> {
        info!("Loading scene: {}", scene.game.name);

        let mut entities_created = 0;
        let mut tilemaps_loaded = 0;
        let mut root_entities = Vec::new();

        // Clear previous state
        self.entity_map.clear();
        self.warnings.clear();

        // Load the first scene (for now, we only support single scenes)
        if let Some(first_scene) = scene.scenes.first() {
            // Load tilemaps first
            for tilemap_spec in &first_scene.tilemaps {
                let entity = world.create_entity();
                
                // Create tilemap component
                let tilemap = Tilemap {
                    tileset: tilemap_spec.tileset.clone(),
                    grid: tilemap_spec.grid.iter()
                        .map(|row| {
                            row.chars().enumerate().map(|(_x, ch)| {
                                match ch {
                                    '.' => 0u16,  // Empty
                                    '#' => 1u16,  // Solid block
                                    '=' => 2u16,  // Platform
                                    _ => 0u16,
                                }
                            }).collect()
                        }).collect(),
                    tile_size: (16, 16), // Default tile size
                };
                
                world.add_component(entity, tilemap);
                world.add_component(entity, Transform2D::default());
                world.add_component(entity, ScriptTag::new(vec!["tilemap"]));

                tilemaps_loaded += 1;
                root_entities.push(entity);
                info!("Created tilemap entity using tileset: {}", tilemap_spec.tileset);
            }

            // Load entities
            for entity_desc in &first_scene.entities {
                match self.create_entity(world, entity_desc) {
                    Ok(entity) => {
                        entities_created += 1;
                        root_entities.push(entity);
                        info!("Created entity: {}", entity_desc.name);
                    }
                    Err(e) => {
                        warn!("Failed to create entity '{}': {}", entity_desc.name, e);
                        self.warnings.push(format!("Failed to create entity '{}': {}", entity_desc.name, e));
                    }
                }
            }
        } else {
            warn!("No scenes found in SceneDescription");
        }

        info!("Scene loading complete. Entities: {}, Tilemaps: {}", entities_created, tilemaps_loaded);

        Ok(SceneLoadResult {
            entities_created,
            tilemaps_loaded,
            warnings: self.warnings.clone(),
            root_entities,
        })
    }

    /// Create a single entity from SDL description
    fn create_entity(&mut self, world: &mut World, entity_desc: &SDL_Entity) -> Result<Entity> {
        let entity = world.create_entity();
        
        // Store entity mapping for references
        self.entity_map.insert(entity_desc.name.clone(), entity);

        // Add script tags for organization
        let tag_strings: Vec<&str> = entity_desc.tags.iter().map(|s| s.as_str()).collect();
        world.add_component(entity, ScriptTag::new(tag_strings));

        // Process components
        for (comp_name, comp_data) in &entity_desc.components {
            self.add_component(world, entity, comp_name, comp_data)
                .with_context(|| format!("Adding component '{}' to entity '{}'", comp_name, entity_desc.name))?;
        }

        Ok(entity)
    }

    /// Add a component to an entity based on component type
    fn add_component(&mut self, world: &mut World, entity: Entity, comp_name: &str, comp_data: &ComponentData) -> Result<()> {
        match comp_name {
            "Transform2D" => {
                let transform = self.parse_transform2d(&comp_data.data)?;
                world.add_component(entity, transform);
            }
            "Sprite" => {
                let sprite = self.parse_sprite(&comp_data.data)?;
                world.add_component(entity, sprite);
            }
            "RigidBody2D" => {
                let rigidbody = self.parse_rigidbody2d(&comp_data.data)?;
                world.add_component(entity, rigidbody);
            }
            "Collider2D" => {
                let collider = self.parse_collider2d(&comp_data.data)?;
                world.add_component(entity, collider);
            }
            "CharacterController2D" => {
                let controller = self.parse_character_controller2d(&comp_data.data)?;
                world.add_component(entity, controller);
            }
            "Camera2D" => {
                let camera = self.parse_camera2d(&comp_data.data)?;
                world.add_component(entity, camera);
            }
            "Trigger" => {
                let trigger = self.parse_trigger(&comp_data.data)?;
                world.add_component(entity, trigger);
            }
            "InputMap" => {
                let input_map = self.parse_input_map(&comp_data.data)?;
                world.add_component(entity, input_map);
            }
            "InputState" => {
                let input_state = self.parse_input_state(&comp_data.data)?;
                world.add_component(entity, input_state);
            }
            _ => {
                warn!("Unknown component type: {}", comp_name);
                self.warnings.push(format!("Unknown component type: {}", comp_name));
            }
        }

        Ok(())
    }

    // Component parsers

    fn parse_transform2d(&self, data: &Value) -> Result<Transform2D> {
        let pos = data["pos"].as_array()
            .and_then(|arr| Some([arr[0].as_f64()? as f32, arr[1].as_f64()? as f32]))
            .unwrap_or([0.0, 0.0]);
        
        let rot = data["rot"].as_f64().unwrap_or(0.0) as f32;
        
        let scale = data["scale"].as_array()
            .and_then(|arr| Some([arr[0].as_f64()? as f32, arr[1].as_f64()? as f32]))
            .unwrap_or([1.0, 1.0]);

        Ok(Transform2D {
            pos: glam::Vec2::from_array(pos),
            rot,
            scale: glam::Vec2::from_array(scale),
        })
    }

    fn parse_sprite(&self, data: &Value) -> Result<Sprite> {
        let atlas_id = data["atlas_id"].as_str()
            .unwrap_or("missing_sprite").to_string();
        
        let frame = data["frame"].as_u64().unwrap_or(0) as u16;
        
        let color = data["color"].as_array()
            .and_then(|arr| Some([
                arr[0].as_f64()? as f32,
                arr[1].as_f64()? as f32,
                arr[2].as_f64()? as f32,
                arr[3].as_f64()? as f32,
            ]))
            .unwrap_or([1.0, 1.0, 1.0, 1.0]);

        let flip_x = data["flip_x"].as_bool().unwrap_or(false);
        let flip_y = data["flip_y"].as_bool().unwrap_or(false);
        let layer = data["layer"].as_i64().unwrap_or(0) as i32;

        Ok(Sprite {
            atlas_id,
            frame,
            color,
            flip_x,
            flip_y,
            layer,
        })
    }

    fn parse_rigidbody2d(&self, data: &Value) -> Result<RigidBody2D> {
        let kind = match data["kind"].as_str().unwrap_or("Dynamic") {
            "Static" => BodyType::Static,
            "Kinematic" => BodyType::Kinematic,
            _ => BodyType::Dynamic,
        };

        let vel = data["vel"].as_array()
            .and_then(|arr| Some([arr[0].as_f64()? as f32, arr[1].as_f64()? as f32]))
            .unwrap_or([0.0, 0.0]);

        let mass = data["mass"].as_f64().unwrap_or(1.0) as f32;
        let linear_damping = data["linear_damping"].as_f64().unwrap_or(0.1) as f32;
        let angular_damping = data["angular_damping"].as_f64().unwrap_or(0.5) as f32;
        let fixed_rotation = data["fixed_rotation"].as_bool().unwrap_or(true);

        Ok(RigidBody2D {
            kind,
            vel: glam::Vec2::from_array(vel),
            mass,
            linear_damping,
            angular_damping,
            fixed_rotation,
        })
    }

    fn parse_collider2d(&self, data: &Value) -> Result<Collider2D> {
        let shape = if let Some(shape_data) = data.get("shape") {
            if let Some(aabb_data) = shape_data.get("AABB") {
                CollisionShape::AABB {
                    width: aabb_data["width"].as_f64().unwrap_or(16.0) as f32,
                    height: aabb_data["height"].as_f64().unwrap_or(16.0) as f32,
                }
            } else if let Some(circle_data) = shape_data.get("Circle") {
                CollisionShape::Circle {
                    radius: circle_data["radius"].as_f64().unwrap_or(8.0) as f32,
                }
            } else {
                CollisionShape::AABB { width: 16.0, height: 16.0 }
            }
        } else {
            CollisionShape::AABB { width: 16.0, height: 16.0 }
        };

        let is_sensor = data["is_sensor"].as_bool().unwrap_or(false);
        let friction = data["friction"].as_f64().unwrap_or(0.8) as f32;
        let restitution = data["restitution"].as_f64().unwrap_or(0.0) as f32;
        
        let collision_layers = data["collision_layers"].as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_else(|| vec!["solid".to_string()]);

        Ok(Collider2D {
            shape,
            is_sensor,
            friction,
            restitution,
            collision_layers,
        })
    }

    fn parse_character_controller2d(&self, data: &Value) -> Result<CharacterController2D> {
        let speed = data["speed"].as_f64().unwrap_or(200.0) as f32;
        let jump_power = data["jump_power"].as_f64().unwrap_or(500.0) as f32;
        let coyote_time_ms = data["coyote_time_ms"].as_u64().unwrap_or(100) as u32;
        let is_grounded = data["is_grounded"].as_bool().unwrap_or(false);
        let jump_buffer_time = data["jump_buffer_time"].as_f64().unwrap_or(0.15) as f32;

        Ok(CharacterController2D {
            speed,
            jump_power,
            coyote_time_ms,
            is_grounded,
            jump_buffer_time,
            coyote_timer: 0.0,
            jump_buffer_timer: 0.0,
            jump_consumed: false,
        })
    }

    fn parse_camera2d(&self, data: &Value) -> Result<Camera2D> {
        let follow = data["follow"].as_str().map(|s| s.to_string());
        let zoom = data["zoom"].as_f64().unwrap_or(1.0) as f32;
        let smooth_follow = data["smooth_follow"].as_bool().unwrap_or(true);
        let look_ahead = data["look_ahead"].as_f64().unwrap_or(100.0) as f32;

        Ok(Camera2D {
            follow,
            zoom,
            smooth_follow,
            look_ahead,
        })
    }

    fn parse_trigger(&self, data: &Value) -> Result<Trigger> {
        let on_enter = data["on_enter"].as_str().map(|s| s.to_string());
        let on_exit = data["on_exit"].as_str().map(|s| s.to_string());

        Ok(Trigger {
            on_enter,
            on_exit,
        })
    }

    fn parse_input_map(&self, data: &Value) -> Result<InputMap> {
        let move_left = data["move_left"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        let move_right = data["move_right"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        let jump = data["jump"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        let enabled = data["enabled"].as_bool().unwrap_or(true);

        Ok(InputMap {
            move_left,
            move_right,
            jump,
            enabled,
        })
    }

    fn parse_input_state(&self, data: &Value) -> Result<InputState> {
        let move_horizontal = data["move_horizontal"].as_f64().unwrap_or(0.0) as f32;
        let jump_pressed = data["jump_pressed"].as_bool().unwrap_or(false);
        let jump_held = data["jump_held"].as_bool().unwrap_or(false);
        let was_jump_pressed = data["was_jump_pressed"].as_bool().unwrap_or(false);

        Ok(InputState {
            move_horizontal,
            jump_pressed,
            jump_held,
            was_jump_pressed,
        })
    }
}

impl Default for SceneLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_loading() {
        let mut world = World::new();
        let mut loader = SceneLoader::new();
        
        let scene = SceneDescription::platformer_template("Test Scene");
        let result = loader.load_scene(&mut world, &scene).expect("Should load scene successfully");
        
        // Should have created entities and tilemaps
        assert!(result.entities_created > 0);
        assert!(result.tilemaps_loaded > 0);
        
        // Should have no warnings for a valid template
        assert!(result.warnings.is_empty(), "Template should load without warnings: {:?}", result.warnings);
    }

    #[test]
    fn test_component_parsing() {
        let loader = SceneLoader::new();
        
        // Test Transform2D parsing
        let transform_data = serde_json::json!({
            "pos": [10.0, 20.0],
            "rot": 45.0,
            "scale": [2.0, 2.0]
        });
        
        let transform = loader.parse_transform2d(&transform_data).expect("Should parse Transform2D");
        assert_eq!(transform.pos.x, 10.0);
        assert_eq!(transform.pos.y, 20.0);
        assert_eq!(transform.rot, 45.0);
        assert_eq!(transform.scale.x, 2.0);
        assert_eq!(transform.scale.y, 2.0);
    }

    #[test]
    fn test_collider_parsing() {
        let loader = SceneLoader::new();
        
        // Test AABB collider parsing
        let collider_data = serde_json::json!({
            "shape": {
                "AABB": { "width": 32.0, "height": 48.0 }
            },
            "is_sensor": false,
            "friction": 0.8,
            "restitution": 0.1,
            "collision_layers": ["solid", "player"]
        });
        
        let collider = loader.parse_collider2d(&collider_data).expect("Should parse Collider2D");
        if let CollisionShape::AABB { width, height } = collider.shape {
            assert_eq!(width, 32.0);
            assert_eq!(height, 48.0);
        } else {
            panic!("Expected AABB shape");
        }
        
        assert!(!collider.is_sensor);
        assert_eq!(collider.friction, 0.8);
        assert_eq!(collider.restitution, 0.1);
        assert_eq!(collider.collision_layers, vec!["solid", "player"]);
    }
}