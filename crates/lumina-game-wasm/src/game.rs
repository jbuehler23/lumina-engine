//! Game instance management for WASM runtime
//! 
//! Handles the main game loop, ECS world management, and system updates.

use lumina_ecs::*;
use lumina_ai::{SceneDescription, SceneLoader};
use anyhow::Result;
use wasm_bindgen::JsValue;

/// Main game instance that manages the ECS world and systems
pub struct GameInstance {
    pub world: World,
    scene_loader: SceneLoader,
    physics_system: PhysicsSystem,
    input_system: InputSystem,
}

impl GameInstance {
    /// Create a new game instance
    pub fn new() -> Self {
        web_sys::console::log_1(&"🏗️  Creating game instance...".into());
        
        let world = World::new();
        let scene_loader = SceneLoader::new();
        
        // Initialize systems with web-friendly configs
        let mut physics_config = PhysicsConfig::default();
        physics_config.gravity.y = -600.0; // Slightly lighter gravity for web
        physics_config.timestep = 1.0 / 60.0; // 60 FPS target
        let physics_system = PhysicsSystem::with_config(physics_config);
        
        let mut input_config = InputConfig::default();
        input_config.move_speed = 250.0;
        input_config.jump_speed = 400.0;
        input_config.air_control = 0.8;
        let input_system = InputSystem::with_config(input_config);
        
        Self {
            world,
            scene_loader,
            physics_system,
            input_system,
        }
    }
    
    /// Load a scene into the game world
    pub fn load_scene(&mut self, scene: SceneDescription) -> Result<()> {
        web_sys::console::log_1(&"📄 Loading scene into game world...".into());
        
        // Clear existing world
        self.world.clear();
        
        // Load the new scene
        let load_result = self.scene_loader.load_scene(&mut self.world, &scene)?;
        
        web_sys::console::log_1(&format!("✅ Scene loaded: {} entities, {} tilemaps", 
            load_result.entities_created, load_result.tilemaps_loaded).into());
        
        Ok(())
    }
    
    /// Update all game systems for one frame
    pub fn update(&mut self, dt: f32) -> Result<(), JsValue> {
        // Clamp delta time to prevent instability
        let dt = dt.min(0.033); // Max 33ms (30 FPS minimum)
        
        // Update systems in the correct order:
        // 1. Input system processes keyboard input into component data
        self.input_system.update(&mut self.world, dt);
        
        // 2. Physics system processes movement and collisions
        self.physics_system.update(&mut self.world);
        
        // Note: Rendering is handled separately in the web renderer
        
        Ok(())
    }
    
    /// Get the number of entities with physics components (for debugging)
    pub fn get_physics_entity_count(&self) -> usize {
        let mut count = 0;
        for entity in self.world.iter_entities() {
            if self.world.has_component::<RigidBody2D>(entity) {
                count += 1;
            }
        }
        count
    }
    
    /// Get all entities with their positions (for rendering)
    pub fn get_renderable_entities(&self) -> Vec<RenderableEntity> {
        let mut entities = Vec::new();
        
        for entity in self.world.iter_entities() {
            if let Some(transform) = self.world.get_component::<Transform2D>(entity) {
                let entity_type = self.determine_entity_type(entity);
                
                entities.push(RenderableEntity {
                    id: entity.id(),
                    position: transform.pos,
                    rotation: transform.rot,
                    scale: transform.scale,
                    entity_type,
                });
            }
        }
        
        entities
    }
    
    /// Determine what type of entity this is for rendering purposes
    fn determine_entity_type(&self, entity: Entity) -> EntityType {
        // Check for character controller (player)
        if self.world.has_component::<CharacterController2D>(entity) {
            return EntityType::Player;
        }
        
        // Check for collectible (coins)
        if let Some(collider) = self.world.get_component::<Collider2D>(entity) {
            if collider.is_sensor {
                return EntityType::Coin;
            }
        }
        
        // Check for rigidbody (moving objects)
        if let Some(rb) = self.world.get_component::<RigidBody2D>(entity) {
            match rb.kind {
                BodyType::Dynamic => EntityType::Dynamic,
                BodyType::Kinematic => EntityType::Platform,
                BodyType::Static => EntityType::Platform,
            }
        } else {
            // Static platform without rigidbody
            EntityType::Platform
        }
    }
    
    /// Get player position for camera following
    pub fn get_player_position(&self) -> Option<glam::Vec2> {
        for entity in self.world.iter_entities() {
            if self.world.has_component::<CharacterController2D>(entity) {
                if let Some(transform) = self.world.get_component::<Transform2D>(entity) {
                    return Some(transform.pos);
                }
            }
        }
        None
    }
}

/// Renderable entity data for the web renderer
pub struct RenderableEntity {
    pub id: u32,
    pub position: glam::Vec2,
    pub rotation: f32,
    pub scale: glam::Vec2,
    pub entity_type: EntityType,
}

/// Types of entities for rendering
#[derive(Debug, Clone, Copy)]
pub enum EntityType {
    Player,
    Platform,
    Coin,
    Dynamic,
}

