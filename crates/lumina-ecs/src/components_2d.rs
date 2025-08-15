//! Core 2D components for game development
//! 
//! These components follow the target GameSpec specification for simple,
//! efficient 2D game development focused on platformers.

use crate::Component;
use glam::Vec2;
use serde::{Deserialize, Serialize};

/// 2D Transform component - position, rotation, scale
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transform2D {
    pub pos: Vec2,
    pub rot: f32,
    pub scale: Vec2,
}

impl Transform2D {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            rot: 0.0,
            scale: Vec2::ONE,
        }
    }

    pub fn at(x: f32, y: f32) -> Self {
        Self::new(Vec2::new(x, y))
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::new(Vec2::ZERO)
    }
}

impl Component for Transform2D {}

/// 2D Rigid Body types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BodyType {
    Dynamic,
    Kinematic, 
    Static,
}

/// 2D Rigid Body component for physics simulation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RigidBody2D {
    pub kind: BodyType,
    pub vel: Vec2,
    pub mass: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub fixed_rotation: bool,
}

impl RigidBody2D {
    pub fn dynamic() -> Self {
        Self {
            kind: BodyType::Dynamic,
            vel: Vec2::ZERO,
            mass: 1.0,
            linear_damping: 0.1,
            angular_damping: 0.5,
            fixed_rotation: true,
        }
    }

    pub fn kinematic() -> Self {
        Self {
            kind: BodyType::Kinematic,
            vel: Vec2::ZERO,
            mass: 0.0,
            linear_damping: 0.0,
            angular_damping: 0.0,
            fixed_rotation: true,
        }
    }

    pub fn static_body() -> Self {
        Self {
            kind: BodyType::Static,
            vel: Vec2::ZERO,
            mass: 0.0,
            linear_damping: 0.0,
            angular_damping: 0.0,
            fixed_rotation: true,
        }
    }
}

impl Component for RigidBody2D {}

/// 2D Collision shapes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CollisionShape {
    AABB { width: f32, height: f32 },
    Circle { radius: f32 },
    Capsule { width: f32, height: f32 },
}

impl CollisionShape {
    pub fn rectangle(width: f32, height: f32) -> Self {
        Self::AABB { width, height }
    }

    pub fn circle(radius: f32) -> Self {
        Self::Circle { radius }
    }
}

/// 2D Collider component for collision detection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collider2D {
    pub shape: CollisionShape,
    pub is_sensor: bool,
    pub friction: f32,
    pub restitution: f32,
    pub collision_layers: Vec<String>,
}

impl Collider2D {
    pub fn rectangle(width: f32, height: f32) -> Self {
        Self {
            shape: CollisionShape::rectangle(width, height),
            is_sensor: false,
            friction: 0.8,
            restitution: 0.0,
            collision_layers: vec!["solid".to_string()],
        }
    }

    pub fn sensor_rectangle(width: f32, height: f32) -> Self {
        Self {
            shape: CollisionShape::rectangle(width, height),
            is_sensor: true,
            friction: 0.0,
            restitution: 0.0,
            collision_layers: vec![],
        }
    }

    pub fn circle(radius: f32) -> Self {
        Self {
            shape: CollisionShape::circle(radius),
            is_sensor: false,
            friction: 0.8,
            restitution: 0.0,
            collision_layers: vec!["solid".to_string()],
        }
    }
}

impl Component for Collider2D {}

/// Character Controller for platformer movement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterController2D {
    pub speed: f32,
    pub jump_power: f32,
    pub coyote_time_ms: u32,
    pub is_grounded: bool,
    pub jump_buffer_time: f32,
    // New fields for input system
    pub coyote_timer: f32,
    pub jump_buffer_timer: f32,
    pub jump_consumed: bool,
}

impl CharacterController2D {
    pub fn new(speed: f32, jump_power: f32) -> Self {
        Self {
            speed,
            jump_power,
            coyote_time_ms: 100,
            is_grounded: false,
            jump_buffer_time: 0.15,
            coyote_timer: 0.0,
            jump_buffer_timer: 0.0,
            jump_consumed: false,
        }
    }
}

impl Default for CharacterController2D {
    fn default() -> Self {
        Self::new(200.0, 500.0)
    }
}

impl Component for CharacterController2D {}

/// Input mapping component for entities that respond to keyboard input
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputMap {
    pub move_left: Vec<String>,    // Key names for left movement
    pub move_right: Vec<String>,   // Key names for right movement  
    pub jump: Vec<String>,         // Key names for jump
    pub enabled: bool,             // Whether input is enabled
}

impl InputMap {
    pub fn wasd() -> Self {
        Self {
            move_left: vec!["KeyA".to_string(), "ArrowLeft".to_string()],
            move_right: vec!["KeyD".to_string(), "ArrowRight".to_string()],
            jump: vec!["KeyW".to_string(), "Space".to_string(), "ArrowUp".to_string()],
            enabled: true,
        }
    }
    
    pub fn arrows() -> Self {
        Self {
            move_left: vec!["ArrowLeft".to_string()],
            move_right: vec!["ArrowRight".to_string()],
            jump: vec!["ArrowUp".to_string(), "Space".to_string()],
            enabled: true,
        }
    }
}

impl Default for InputMap {
    fn default() -> Self {
        Self::wasd()
    }
}

impl Component for InputMap {}

/// Component that tracks current input state for an entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputState {
    pub move_horizontal: f32,  // -1.0 to 1.0
    pub jump_pressed: bool,    // True if jump was pressed this frame
    pub jump_held: bool,       // True if jump is being held
    pub was_jump_pressed: bool, // Previous frame jump state for edge detection
}

impl InputState {
    pub fn new() -> Self {
        Self {
            move_horizontal: 0.0,
            jump_pressed: false,
            jump_held: false,
            was_jump_pressed: false,
        }
    }
    
    pub fn clear(&mut self) {
        self.move_horizontal = 0.0;
        self.jump_pressed = false;
        self.jump_held = false;
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for InputState {}

/// Camera 2D component for viewport management
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Camera2D {
    pub follow: Option<String>, // Entity name to follow
    pub zoom: f32,
    pub smooth_follow: bool,
    pub look_ahead: f32,
}

impl Camera2D {
    pub fn new() -> Self {
        Self {
            follow: None,
            zoom: 1.0,
            smooth_follow: true,
            look_ahead: 100.0,
        }
    }

    pub fn following(target: &str) -> Self {
        Self {
            follow: Some(target.to_string()),
            zoom: 1.0,
            smooth_follow: true,
            look_ahead: 100.0,
        }
    }
}

impl Default for Camera2D {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Camera2D {}

/// Gravity component for physics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gravity2D {
    pub g: f32,
}

impl Default for Gravity2D {
    fn default() -> Self {
        Self { g: -9.81 }
    }
}

impl Component for Gravity2D {}

/// Sprite component for 2D rendering
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sprite {
    pub atlas_id: String, // Asset ID for sprite atlas
    pub frame: u16,
    pub color: [f32; 4], // RGBA
    pub flip_x: bool,
    pub flip_y: bool,
    pub layer: i32,
}

impl Sprite {
    pub fn new(atlas_id: &str) -> Self {
        Self {
            atlas_id: atlas_id.to_string(),
            frame: 0,
            color: [1.0, 1.0, 1.0, 1.0],
            flip_x: false,
            flip_y: false,
            layer: 0,
        }
    }
}

impl Component for Sprite {}

/// Tilemap component for efficient tile rendering
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tilemap {
    pub tileset: String, // Asset ID for tileset
    pub grid: Vec<Vec<u16>>, // 2D grid of tile IDs (0 = empty)
    pub tile_size: (u32, u32),
}

impl Tilemap {
    pub fn new(tileset: &str, width: usize, height: usize, tile_size: (u32, u32)) -> Self {
        Self {
            tileset: tileset.to_string(),
            grid: vec![vec![0; width]; height],
            tile_size,
        }
    }

    pub fn from_string(tileset: &str, data: &str, tile_size: (u32, u32)) -> Self {
        let lines: Vec<&str> = data.trim().lines().collect();
        let height = lines.len();
        let width = lines.get(0).map(|line| line.chars().count()).unwrap_or(0);

        let mut grid = vec![vec![0u16; width]; height];
        
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let tile_id = match ch {
                    '.' => 0,  // Empty
                    '#' => 1,  // Solid block
                    '=' => 2,  // Platform
                    _ => 0,
                };
                if x < width && y < height {
                    grid[y][x] = tile_id;
                }
            }
        }

        Self {
            tileset: tileset.to_string(),
            grid,
            tile_size,
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile_id: u16) {
        if y < self.grid.len() && x < self.grid[y].len() {
            self.grid[y][x] = tile_id;
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> u16 {
        self.grid.get(y).and_then(|row| row.get(x)).copied().unwrap_or(0)
    }
}

impl Component for Tilemap {}

/// Trigger component for events
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trigger {
    pub on_enter: Option<String>, // Event ID
    pub on_exit: Option<String>,  // Event ID
}

impl Trigger {
    pub fn new(on_enter: &str) -> Self {
        Self {
            on_enter: Some(on_enter.to_string()),
            on_exit: None,
        }
    }
}

impl Component for Trigger {}

/// ScriptTag for editor queries and organization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScriptTag {
    pub tags: Vec<String>,
}

impl ScriptTag {
    pub fn new(tags: Vec<&str>) -> Self {
        Self {
            tags: tags.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag.to_string())
    }
}

impl Component for ScriptTag {}