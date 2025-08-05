//! Scene Description Language (SDL) for AI-generated games
//! 
//! This module defines the intermediate representation used to communicate
//! between AI models and the Lumina Engine's ECS system.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use glam::{Vec2, Vec3};
use uuid::Uuid;

/// Complete scene description that can be loaded into the ECS world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneDescription {
    /// Scene metadata
    pub metadata: SceneMetadata,
    /// List of entities to create
    pub entities: Vec<EntityDescription>,
    /// Global scene resources
    pub resources: HashMap<String, ResourceDescription>,
    /// Game logic scripts
    pub scripts: Vec<ScriptDescription>,
}

/// Scene metadata and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneMetadata {
    /// Scene name
    pub name: String,
    /// Scene description
    pub description: String,
    /// Target screen resolution
    pub resolution: [u32; 2],
    /// Background color (RGBA)
    pub background_color: [f32; 4],
    /// Physics configuration
    pub physics: PhysicsConfig,
    /// Audio configuration
    pub audio: AudioConfig,
}

/// Physics system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsConfig {
    pub enabled: bool,
    pub gravity: [f32; 2],
    pub timestep: f32,
}

/// Audio system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub enabled: bool,
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

/// Description of a single entity and its components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDescription {
    /// Entity name for identification
    pub name: String,
    /// Entity tags for grouping and queries
    pub tags: Vec<String>,
    /// Component data
    pub components: HashMap<String, ComponentDescription>,
}

/// Enum representing all possible component types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ComponentDescription {
    Transform {
        position: [f32; 2],
        rotation: f32,
        scale: [f32; 2],
    },
    Sprite {
        texture: String,
        color: [f32; 4],
        flip_x: bool,
        flip_y: bool,
        layer: i32,
    },
    Collider {
        shape: ColliderShape,
        is_sensor: bool,
        friction: f32,
        restitution: f32,
    },
    RigidBody {
        body_type: RigidBodyType,
        mass: f32,
        linear_damping: f32,
        angular_damping: f32,
    },
    Velocity {
        linear: [f32; 2],
        angular: f32,
    },
    Player {
        speed: f32,
        jump_force: f32,
        health: i32,
        max_health: i32,
    },
    Enemy {
        damage: i32,
        attack_range: f32,
        ai_type: String,
    },
    Collectible {
        points: i32,
        effect: String,
    },
    AudioSource {
        clip: String,
        volume: f32,
        looping: bool,
        spatial: bool,
    },
    Text {
        content: String,
        font: String,
        size: f32,
        color: [f32; 4],
        alignment: TextAlignment,
    },
    Animation {
        sprites: Vec<String>,
        frame_duration: f32,
        looping: bool,
        current_frame: usize,
    },
    Camera {
        size: f32,
        target: Option<String>,
        smooth_follow: bool,
    },
    Lifetime {
        duration: f32,
    },
    Health {
        current: i32,
        maximum: i32,
    },
    Damage {
        amount: i32,
        damage_type: String,
    },
}

/// Collider shape types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "shape_type")]
pub enum ColliderShape {
    Rectangle { width: f32, height: f32 },
    Circle { radius: f32 },
    Capsule { height: f32, radius: f32 },
}

/// Rigid body types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RigidBodyType {
    Dynamic,
    Static,
    Kinematic,
}

/// Text alignment options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

/// Scene resource descriptions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ResourceDescription {
    GameState {
        score: i32,
        level: i32,
        lives: i32,
        time_remaining: f32,
    },
    InputMapping {
        bindings: HashMap<String, String>,
    },
    SpawnTimer {
        interval: f32,
        next_spawn: f32,
        entity_template: String,
    },
}

/// Script description for game logic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDescription {
    /// Script identifier
    pub id: String,
    /// Script name
    pub name: String,
    /// Script type (system, event_handler, etc.)
    pub script_type: ScriptType,
    /// Trigger conditions
    pub triggers: Vec<TriggerDescription>,
    /// Actions to execute
    pub actions: Vec<ActionDescription>,
}

/// Types of scripts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptType {
    System,
    EventHandler,
    Timer,
    Collision,
}

/// Trigger conditions for scripts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum TriggerDescription {
    KeyPressed { key: String },
    Collision { entity_a: String, entity_b: String },
    Timer { interval: f32 },
    HealthReduced { entity: String, threshold: i32 },
    ScoreChanged { threshold: i32 },
    EntityDestroyed { entity: String },
}

/// Actions that scripts can perform
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ActionDescription {
    MoveEntity { entity: String, direction: [f32; 2], speed: f32 },
    DestroyEntity { entity: String },
    SpawnEntity { template: String, position: [f32; 2] },
    PlaySound { clip: String, volume: f32 },
    UpdateScore { change: i32 },
    ChangeLevel { level: i32 },
    ShowText { message: String, duration: f32 },
    ModifyHealth { entity: String, change: i32 },
    ApplyForce { entity: String, force: [f32; 2] },
}

impl Default for SceneMetadata {
    fn default() -> Self {
        Self {
            name: "Untitled Scene".to_string(),
            description: "AI-generated game scene".to_string(),
            resolution: [1280, 720],
            background_color: [0.2, 0.3, 0.4, 1.0],
            physics: PhysicsConfig {
                enabled: true,
                gravity: [0.0, -9.81],
                timestep: 1.0 / 60.0,
            },
            audio: AudioConfig {
                enabled: true,
                master_volume: 1.0,
                music_volume: 0.7,
                sfx_volume: 0.8,
            },
        }
    }
}

impl SceneDescription {
    /// Create a new empty scene
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            metadata: SceneMetadata {
                name: name.into(),
                ..Default::default()
            },
            entities: Vec::new(),
            resources: HashMap::new(),
            scripts: Vec::new(),
        }
    }

    /// Add an entity to the scene
    pub fn add_entity(&mut self, entity: EntityDescription) {
        self.entities.push(entity);
    }

    /// Add a resource to the scene
    pub fn add_resource(&mut self, name: String, resource: ResourceDescription) {
        self.resources.insert(name, resource);
    }

    /// Add a script to the scene
    pub fn add_script(&mut self, script: ScriptDescription) {
        self.scripts.push(script);
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(json)?)
    }
}

impl EntityDescription {
    /// Create a new entity description
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tags: Vec::new(),
            components: HashMap::new(),
        }
    }

    /// Add a component to the entity
    pub fn with_component(mut self, name: impl Into<String>, component: ComponentDescription) -> Self {
        self.components.insert(name.into(), component);
        self
    }

    /// Add a tag to the entity
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
}