//! Scene loader that converts SDL (Scene Description Language) into ECS entities
//! 
//! This module bridges the gap between AI-generated scene descriptions and
//! the Lumina Engine's ECS system.

use std::collections::HashMap;
use anyhow::{Result, Context, anyhow};
use log::{info, warn, error};
use glam::Vec2;

use lumina_ecs::{World, Entity, Commands};
use lumina_core::components::*;

use crate::scene_description::{
    SceneDescription, EntityDescription, ComponentDescription, 
    ColliderShape, RigidBodyType, TextAlignment, ScriptDescription,
    TriggerDescription, ActionDescription
};

/// Scene loader that converts SDL to ECS entities
pub struct SceneLoader {
    /// Entity name to Entity ID mapping for cross-references
    entity_map: HashMap<String, Entity>,
    /// Pending entity references that need to be resolved
    pending_references: Vec<PendingReference>,
}

/// Reference that needs to be resolved after all entities are created
#[derive(Debug)]
struct PendingReference {
    entity: Entity,
    component_type: String,
    reference_name: String,
    target_entity_name: String,
}

/// Result of loading a scene
#[derive(Debug)]
pub struct SceneLoadResult {
    /// Number of entities created
    pub entities_created: usize,
    /// Number of scripts loaded
    pub scripts_loaded: usize,
    /// Warnings encountered during loading
    pub warnings: Vec<String>,
    /// Root entities (entities without parents)
    pub root_entities: Vec<Entity>,
}

impl SceneLoader {
    /// Create a new scene loader
    pub fn new() -> Self {
        Self {
            entity_map: HashMap::new(),
            pending_references: Vec::new(),
        }
    }

    /// Load a scene description into the ECS world
    pub fn load_scene(&mut self, world: &mut World, scene: SceneDescription) -> Result<SceneLoadResult> {
        info!("Loading scene: {}", scene.metadata.name);
        
        let mut result = SceneLoadResult {
            entities_created: 0,
            scripts_loaded: 0,
            warnings: Vec::new(),
            root_entities: Vec::new(),
        };

        // Clear previous state
        self.entity_map.clear();
        self.pending_references.clear();

        // Set up global scene resources
        self.setup_scene_resources(world, &scene)?;

        // Create all entities first (so we can reference them)
        for entity_desc in &scene.entities {
            let entity = self.create_entity(world, entity_desc)?;
            self.entity_map.insert(entity_desc.name.clone(), entity);
            result.entities_created += 1;
        }

        // Add components to entities
        for entity_desc in &scene.entities {
            if let Some(&entity) = self.entity_map.get(&entity_desc.name) {
                self.add_entity_components(world, entity, entity_desc, &mut result.warnings)?;
            }
        }

        // Resolve pending entity references
        self.resolve_pending_references(world, &mut result.warnings)?;

        // Load scripts and game logic
        for script in &scene.scripts {
            self.load_script(world, script, &mut result.warnings)?;
            result.scripts_loaded += 1;
        }

        // Identify root entities (entities that aren't children of others)
        result.root_entities = self.entity_map.values().copied().collect();

        info!("Scene loaded successfully: {} entities, {} scripts", 
              result.entities_created, result.scripts_loaded);

        Ok(result)
    }

    /// Set up global scene resources
    fn setup_scene_resources(&self, world: &mut World, scene: &SceneDescription) -> Result<()> {
        // Set up physics configuration
        if scene.metadata.physics.enabled {
            // Physics setup would go here when physics system is implemented
            info!("Physics enabled with gravity: {:?}", scene.metadata.physics.gravity);
        }

        // Set up audio configuration  
        if scene.metadata.audio.enabled {
            // Audio setup would go here when audio system is implemented
            info!("Audio enabled with master volume: {}", scene.metadata.audio.master_volume);
        }

        // Set up scene metadata as a resource
        world.insert_resource(SceneMetadata {
            name: scene.metadata.name.clone(),
            description: scene.metadata.description.clone(),
            background_color: scene.metadata.background_color,
        });

        Ok(())
    }

    /// Create an entity in the ECS world
    fn create_entity(&self, world: &mut World, entity_desc: &EntityDescription) -> Result<Entity> {
        let entity = world.spawn_empty().id();
        
        // Add tags as components if they represent special entity types
        for tag in &entity_desc.tags {
            match tag.as_str() {
                "player" => {
                    // Player tag handling would go here
                }
                "enemy" => {
                    // Enemy tag handling would go here  
                }
                "collectible" => {
                    // Collectible tag handling would go here
                }
                _ => {
                    // Generic tag handling
                }
            }
        }

        Ok(entity)
    }

    /// Add components to an entity based on its description
    fn add_entity_components(
        &mut self, 
        world: &mut World, 
        entity: Entity, 
        entity_desc: &EntityDescription,
        warnings: &mut Vec<String>
    ) -> Result<()> {
        
        for (component_name, component_desc) in &entity_desc.components {
            match self.create_component(component_desc, &entity_desc.name, warnings) {
                Ok(Some(component)) => {
                    self.add_component_to_entity(world, entity, component_name, component)?;
                }
                Ok(None) => {
                    // Component was handled elsewhere or is a reference
                }
                Err(e) => {
                    let warning = format!("Failed to create component '{}' for entity '{}': {}", 
                                        component_name, entity_desc.name, e);
                    warnings.push(warning);
                }
            }
        }

        Ok(())
    }

    /// Create a component from its description
    fn create_component(
        &mut self, 
        component_desc: &ComponentDescription, 
        entity_name: &str,
        warnings: &mut Vec<String>
    ) -> Result<Option<Box<dyn std::any::Any>>> {
        
        match component_desc {
            ComponentDescription::Transform { position, rotation, scale } => {
                Ok(Some(Box::new(Transform {
                    translation: Vec2::new(position[0], position[1]),
                    rotation: *rotation,
                    scale: Vec2::new(scale[0], scale[1]),
                })))
            }
            
            ComponentDescription::Sprite { texture, color, flip_x, flip_y, layer } => {
                Ok(Some(Box::new(Sprite {
                    texture_path: texture.clone(),
                    color: [color[0], color[1], color[2], color[3]],
                    flip_x: *flip_x,
                    flip_y: *flip_y,
                    layer: *layer,
                })))
            }

            ComponentDescription::Player { speed, jump_force, health, max_health } => {
                Ok(Some(Box::new(Player {
                    speed: *speed,
                    jump_force: *jump_force,
                    health: *health,
                    max_health: *max_health,
                })))
            }

            ComponentDescription::Velocity { linear, angular } => {
                Ok(Some(Box::new(Velocity {
                    linear: Vec2::new(linear[0], linear[1]),
                    angular: *angular,
                })))
            }

            ComponentDescription::Health { current, maximum } => {
                Ok(Some(Box::new(Health {
                    current: *current,
                    maximum: *maximum,
                })))
            }

            ComponentDescription::Collider { shape, is_sensor, friction, restitution } => {
                let collider_shape = match shape {
                    ColliderShape::Rectangle { width, height } => {
                        ColliderShapeType::Rectangle { width: *width, height: *height }
                    }
                    ColliderShape::Circle { radius } => {
                        ColliderShapeType::Circle { radius: *radius }
                    }
                    ColliderShape::Capsule { height, radius } => {
                        ColliderShapeType::Capsule { height: *height, radius: *radius }
                    }
                };

                Ok(Some(Box::new(Collider {
                    shape: collider_shape,
                    is_sensor: *is_sensor,
                    friction: *friction,
                    restitution: *restitution,
                })))
            }

            ComponentDescription::Text { content, font, size, color, alignment } => {
                let text_align = match alignment {
                    TextAlignment::Left => TextAlign::Left,
                    TextAlignment::Center => TextAlign::Center,
                    TextAlignment::Right => TextAlign::Right,
                };

                Ok(Some(Box::new(Text {
                    content: content.clone(),
                    font: font.clone(),
                    size: *size,
                    color: [color[0], color[1], color[2], color[3]],
                    alignment: text_align,
                })))
            }

            // Components that require entity references are handled as pending
            ComponentDescription::Camera { target, .. } => {
                if let Some(target_name) = target {
                    self.pending_references.push(PendingReference {
                        entity: Entity::from_raw(0), // Will be filled in later
                        component_type: "Camera".to_string(),
                        reference_name: "target".to_string(),
                        target_entity_name: target_name.clone(),
                    });
                }
                
                Ok(Some(Box::new(Camera {
                    target: None, // Will be resolved later
                    size: component_desc.get_camera_size().unwrap_or(10.0),
                    smooth_follow: component_desc.get_camera_smooth_follow().unwrap_or(false),
                })))
            }

            // Add more component types as needed
            _ => {
                warnings.push(format!("Unsupported component type for entity '{}'", entity_name));
                Ok(None)
            }
        }
    }

    /// Add a component to an entity (type-erased)
    fn add_component_to_entity(
        &self,
        world: &mut World,
        entity: Entity,
        component_name: &str,
        component: Box<dyn std::any::Any>
    ) -> Result<()> {
        // This is a simplified version - in practice, you'd need a proper
        // component registration system or use macros to handle this
        
        match component_name {
            "Transform" => {
                if let Ok(transform) = component.downcast::<Transform>() {
                    world.entity_mut(entity).insert(*transform);
                }
            }
            "Sprite" => {
                if let Ok(sprite) = component.downcast::<Sprite>() {
                    world.entity_mut(entity).insert(*sprite);
                }
            }
            "Player" => {
                if let Ok(player) = component.downcast::<Player>() {
                    world.entity_mut(entity).insert(*player);
                }
            }
            "Velocity" => {
                if let Ok(velocity) = component.downcast::<Velocity>() {
                    world.entity_mut(entity).insert(*velocity);
                }
            }
            "Health" => {
                if let Ok(health) = component.downcast::<Health>() {
                    world.entity_mut(entity).insert(*health);
                }
            }
            "Collider" => {
                if let Ok(collider) = component.downcast::<Collider>() {
                    world.entity_mut(entity).insert(*collider);
                }
            }
            "Text" => {
                if let Ok(text) = component.downcast::<Text>() {
                    world.entity_mut(entity).insert(*text);
                }
            }
            "Camera" => {
                if let Ok(camera) = component.downcast::<Camera>() {
                    world.entity_mut(entity).insert(*camera);
                }
            }
            _ => {
                return Err(anyhow!("Unknown component type: {}", component_name));
            }
        }

        Ok(())
    }

    /// Resolve pending entity references
    fn resolve_pending_references(&mut self, world: &mut World, warnings: &mut Vec<String>) -> Result<()> {
        for reference in &self.pending_references {
            if let Some(&target_entity) = self.entity_map.get(&reference.target_entity_name) {
                // Update the component with the resolved entity reference
                match reference.component_type.as_str() {
                    "Camera" => {
                        // Update camera target - this would need proper implementation
                        // based on your ECS system's capabilities
                    }
                    _ => {
                        warnings.push(format!("Cannot resolve reference for component type: {}", 
                                            reference.component_type));
                    }
                }
            } else {
                warnings.push(format!("Could not resolve entity reference: {}", 
                                    reference.target_entity_name));
            }
        }

        Ok(())
    }

    /// Load a script into the game logic system
    fn load_script(&self, world: &mut World, script: &ScriptDescription, warnings: &mut Vec<String>) -> Result<()> {
        // This would integrate with the scripting system when implemented
        info!("Loading script: {} ({})", script.name, script.id);
        
        // For now, just log the script information
        for trigger in &script.triggers {
            info!("  Trigger: {:?}", trigger);
        }
        
        for action in &script.actions {
            info!("  Action: {:?}", action);
        }

        Ok(())
    }
}

// Helper trait for extracting component-specific data
trait ComponentDescriptionHelper {
    fn get_camera_size(&self) -> Option<f32>;
    fn get_camera_smooth_follow(&self) -> Option<bool>;
}

impl ComponentDescriptionHelper for ComponentDescription {
    fn get_camera_size(&self) -> Option<f32> {
        match self {
            ComponentDescription::Camera { size, .. } => Some(*size),
            _ => None,
        }
    }

    fn get_camera_smooth_follow(&self) -> Option<bool> {
        match self {
            ComponentDescription::Camera { smooth_follow, .. } => Some(*smooth_follow),
            _ => None,
        }
    }
}

/// Scene metadata resource
#[derive(Debug, Clone)]
pub struct SceneMetadata {
    pub name: String,
    pub description: String,
    pub background_color: [f32; 4],
}

// Placeholder component types - these would be defined in lumina-core
#[derive(Debug, Clone)]
pub struct Transform {
    pub translation: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

#[derive(Debug, Clone)]
pub struct Sprite {
    pub texture_path: String,
    pub color: [f32; 4],
    pub flip_x: bool,
    pub flip_y: bool,
    pub layer: i32,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub speed: f32,
    pub jump_force: f32,
    pub health: i32,
    pub max_health: i32,
}

#[derive(Debug, Clone)]
pub struct Velocity {
    pub linear: Vec2,
    pub angular: f32,
}

#[derive(Debug, Clone)]
pub struct Health {
    pub current: i32,
    pub maximum: i32,
}

#[derive(Debug, Clone)]
pub struct Collider {
    pub shape: ColliderShapeType,
    pub is_sensor: bool,
    pub friction: f32,
    pub restitution: f32,
}

#[derive(Debug, Clone)]
pub enum ColliderShapeType {
    Rectangle { width: f32, height: f32 },
    Circle { radius: f32 },
    Capsule { height: f32, radius: f32 },
}

#[derive(Debug, Clone)]
pub struct Text {
    pub content: String,
    pub font: String,
    pub size: f32,
    pub color: [f32; 4],
    pub alignment: TextAlign,
}

#[derive(Debug, Clone)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub target: Option<Entity>,
    pub size: f32,
    pub smooth_follow: bool,
}

impl Default for SceneLoader {
    fn default() -> Self {
        Self::new()
    }
}