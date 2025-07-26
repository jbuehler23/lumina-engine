use lumina_core::{VisualScript, visual_scripting::*};
use lumina_ecs::World;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a game project in the web editor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub template: GameTemplate,
    pub scenes: HashMap<String, Scene>,
    pub assets: HashMap<String, Asset>,
    pub scripts: HashMap<String, VisualScript>,
    pub settings: ProjectSettings,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

/// Different game templates available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameTemplate {
    Blank,
    Platformer2D,
    TopDownAdventure,
    PuzzleGame,
    ArcadeShooter,
}

/// A scene contains game objects and their configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub name: String,
    pub game_objects: Vec<GameObject>,
    pub camera: Camera,
    pub background: Option<String>, // Asset ID
}

/// A game object in the scene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameObject {
    pub id: Uuid,
    pub name: String,
    pub transform: Transform,
    pub sprite: Option<SpriteComponent>,
    pub collider: Option<ColliderComponent>,
    pub scripts: Vec<String>, // Script IDs
    pub tags: Vec<String>,
}

/// Transform component for positioning objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub position: (f32, f32),
    pub rotation: f32,
    pub scale: (f32, f32),
}

/// Sprite rendering component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteComponent {
    pub asset_id: String,
    pub color: (f32, f32, f32, f32), // RGBA
    pub visible: bool,
    pub layer: i32,
}

/// Physics collider component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColliderComponent {
    pub shape: ColliderShape,
    pub is_sensor: bool,
    pub physics_body: PhysicsBodyType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColliderShape {
    Rectangle { width: f32, height: f32 },
    Circle { radius: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhysicsBodyType {
    Static,
    Dynamic,
    Kinematic,
}

/// Camera configuration for the scene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub position: (f32, f32),
    pub zoom: f32,
    pub follow_target: Option<Uuid>, // GameObject ID to follow
}

/// Asset (image, sound, etc.) in the project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub asset_type: AssetType,
    pub file_path: String,
    pub metadata: AssetMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Image,
    Audio,
    Font,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMetadata {
    pub size_bytes: u64,
    pub dimensions: Option<(u32, u32)>, // For images
    pub duration: Option<f32>, // For audio in seconds
}

/// Project-wide settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub window_width: u32,
    pub window_height: u32,
    pub background_color: (f32, f32, f32, f32),
    pub physics_enabled: bool,
    pub audio_enabled: bool,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            window_width: 1280,
            window_height: 720,
            background_color: (0.2, 0.3, 0.8, 1.0), // Nice blue
            physics_enabled: true,
            audio_enabled: true,
        }
    }
}

/// Manages all projects
pub struct ProjectManager {
    projects: HashMap<Uuid, Project>,
}

impl ProjectManager {
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
        }
    }

    /// Create a new project from a template
    pub fn create_project(&mut self, name: String, template: GameTemplate) -> Project {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();
        
        let mut project = Project {
            id,
            name,
            description: String::new(),
            template: template.clone(),
            scenes: HashMap::new(),
            assets: HashMap::new(),
            scripts: HashMap::new(),
            settings: ProjectSettings::default(),
            created_at: now,
            modified_at: now,
        };

        // Initialize based on template
        match template {
            GameTemplate::Platformer2D => {
                // Create a basic platformer scene with a player and some platforms
                let player_id = Uuid::new_v4();
                let platform_id = Uuid::new_v4();

                project.scenes.insert("main".to_string(), Scene {
                    name: "Level 1".to_string(),
                    game_objects: vec![
                        GameObject {
                            id: player_id,
                            name: "Player".to_string(),
                            transform: Transform {
                                position: (0.0, 100.0),
                                rotation: 0.0,
                                scale: (1.0, 1.0),
                            },
                            sprite: Some(SpriteComponent {
                                asset_id: "player_sprite".to_string(),
                                color: (1.0, 1.0, 1.0, 1.0),
                                visible: true,
                                layer: 1,
                            }),
                            collider: Some(ColliderComponent {
                                shape: ColliderShape::Rectangle { width: 32.0, height: 48.0 },
                                is_sensor: false,
                                physics_body: PhysicsBodyType::Dynamic,
                            }),
                            scripts: vec!["player_movement".to_string()],
                            tags: vec!["Player".to_string()],
                        },
                        GameObject {
                            id: platform_id,
                            name: "Platform".to_string(),
                            transform: Transform {
                                position: (0.0, 0.0),
                                rotation: 0.0,
                                scale: (1.0, 1.0),
                            },
                            sprite: Some(SpriteComponent {
                                asset_id: "platform_sprite".to_string(),
                                color: (0.8, 0.8, 0.8, 1.0),
                                visible: true,
                                layer: 0,
                            }),
                            collider: Some(ColliderComponent {
                                shape: ColliderShape::Rectangle { width: 200.0, height: 32.0 },
                                is_sensor: false,
                                physics_body: PhysicsBodyType::Static,
                            }),
                            scripts: Vec::new(),
                            tags: vec!["Platform".to_string()],
                        },
                    ],
                    camera: Camera {
                        position: (0.0, 0.0),
                        zoom: 1.0,
                        follow_target: Some(player_id),
                    },
                    background: None,
                });

                // Add default player movement script
                project.scripts.insert("player_movement".to_string(), 
                    lumina_core::create_player_movement_script());
            },
            GameTemplate::TopDownAdventure => {
                // Create a top-down adventure scene with a player and some environment
                let player_id = Uuid::new_v4();
                let wall_id = Uuid::new_v4();
                let tree_id = Uuid::new_v4();
                let npc_id = Uuid::new_v4();

                project.scenes.insert("main".to_string(), Scene {
                    name: "Forest Area".to_string(),
                    game_objects: vec![
                        GameObject {
                            id: player_id,
                            name: "Player".to_string(),
                            transform: Transform {
                                position: (0.0, 0.0),
                                rotation: 0.0,
                                scale: (1.0, 1.0),
                            },
                            sprite: Some(SpriteComponent {
                                asset_id: "player_topdown".to_string(),
                                color: (0.2, 0.8, 0.2, 1.0), // Green player
                                visible: true,
                                layer: 2,
                            }),
                            collider: Some(ColliderComponent {
                                shape: ColliderShape::Circle { radius: 16.0 },
                                is_sensor: false,
                                physics_body: PhysicsBodyType::Dynamic,
                            }),
                            scripts: vec!["topdown_movement".to_string()],
                            tags: vec!["Player".to_string()],
                        },
                        GameObject {
                            id: wall_id,
                            name: "Wall".to_string(),
                            transform: Transform {
                                position: (100.0, 0.0),
                                rotation: 0.0,
                                scale: (1.0, 1.0),
                            },
                            sprite: Some(SpriteComponent {
                                asset_id: "wall_sprite".to_string(),
                                color: (0.5, 0.3, 0.1, 1.0), // Brown wall
                                visible: true,
                                layer: 1,
                            }),
                            collider: Some(ColliderComponent {
                                shape: ColliderShape::Rectangle { width: 32.0, height: 64.0 },
                                is_sensor: false,
                                physics_body: PhysicsBodyType::Static,
                            }),
                            scripts: Vec::new(),
                            tags: vec!["Wall".to_string()],
                        },
                        GameObject {
                            id: tree_id,
                            name: "Tree".to_string(),
                            transform: Transform {
                                position: (-80.0, 50.0),
                                rotation: 0.0,
                                scale: (1.0, 1.0),
                            },
                            sprite: Some(SpriteComponent {
                                asset_id: "tree_sprite".to_string(),
                                color: (0.1, 0.7, 0.1, 1.0), // Dark green tree
                                visible: true,
                                layer: 1,
                            }),
                            collider: Some(ColliderComponent {
                                shape: ColliderShape::Circle { radius: 20.0 },
                                is_sensor: false,
                                physics_body: PhysicsBodyType::Static,
                            }),
                            scripts: Vec::new(),
                            tags: vec!["Environment".to_string()],
                        },
                        GameObject {
                            id: npc_id,
                            name: "Village NPC".to_string(),
                            transform: Transform {
                                position: (50.0, -50.0),
                                rotation: 0.0,
                                scale: (1.0, 1.0),
                            },
                            sprite: Some(SpriteComponent {
                                asset_id: "npc_sprite".to_string(),
                                color: (0.8, 0.6, 0.4, 1.0), // Skin tone
                                visible: true,
                                layer: 2,
                            }),
                            collider: Some(ColliderComponent {
                                shape: ColliderShape::Circle { radius: 14.0 },
                                is_sensor: true,
                                physics_body: PhysicsBodyType::Static,
                            }),
                            scripts: vec!["npc_dialogue".to_string()],
                            tags: vec!["NPC".to_string()],
                        },
                    ],
                    camera: Camera {
                        position: (0.0, 0.0),
                        zoom: 1.0,
                        follow_target: Some(player_id),
                    },
                    background: Some("forest_background".to_string()),
                });

                // Add scripts for top-down movement and NPC interaction
                project.scripts.insert("topdown_movement".to_string(), 
                    create_player_movement_script());
                project.scripts.insert("npc_dialogue".to_string(), 
                    create_player_movement_script());
            },
            GameTemplate::PuzzleGame => {
                // Create a puzzle game scene with a game board and pieces
                let board_id = Uuid::new_v4();
                let piece1_id = Uuid::new_v4();
                let piece2_id = Uuid::new_v4();
                let piece3_id = Uuid::new_v4();

                project.scenes.insert("main".to_string(), Scene {
                    name: "Puzzle Board".to_string(),
                    game_objects: vec![
                        GameObject {
                            id: board_id,
                            name: "Game Board".to_string(),
                            transform: Transform {
                                position: (0.0, 0.0),
                                rotation: 0.0,
                                scale: (1.0, 1.0),
                            },
                            sprite: Some(SpriteComponent {
                                asset_id: "puzzle_board".to_string(),
                                color: (0.9, 0.9, 0.9, 1.0), // Light gray board
                                visible: true,
                                layer: 0,
                            }),
                            collider: None,
                            scripts: vec!["puzzle_manager".to_string()],
                            tags: vec!["Board".to_string()],
                        },
                        GameObject {
                            id: piece1_id,
                            name: "Red Piece".to_string(),
                            transform: Transform {
                                position: (-60.0, 60.0),
                                rotation: 0.0,
                                scale: (1.0, 1.0),
                            },
                            sprite: Some(SpriteComponent {
                                asset_id: "puzzle_piece".to_string(),
                                color: (1.0, 0.2, 0.2, 1.0), // Red
                                visible: true,
                                layer: 1,
                            }),
                            collider: Some(ColliderComponent {
                                shape: ColliderShape::Rectangle { width: 32.0, height: 32.0 },
                                is_sensor: true,
                                physics_body: PhysicsBodyType::Kinematic,
                            }),
                            scripts: vec!["draggable_piece".to_string()],
                            tags: vec!["PuzzlePiece".to_string(), "Red".to_string()],
                        },
                        GameObject {
                            id: piece2_id,
                            name: "Blue Piece".to_string(),
                            transform: Transform {
                                position: (0.0, 60.0),
                                rotation: 0.0,
                                scale: (1.0, 1.0),
                            },
                            sprite: Some(SpriteComponent {
                                asset_id: "puzzle_piece".to_string(),
                                color: (0.2, 0.2, 1.0, 1.0), // Blue
                                visible: true,
                                layer: 1,
                            }),
                            collider: Some(ColliderComponent {
                                shape: ColliderShape::Rectangle { width: 32.0, height: 32.0 },
                                is_sensor: true,
                                physics_body: PhysicsBodyType::Kinematic,
                            }),
                            scripts: vec!["draggable_piece".to_string()],
                            tags: vec!["PuzzlePiece".to_string(), "Blue".to_string()],
                        },
                        GameObject {
                            id: piece3_id,
                            name: "Green Piece".to_string(),
                            transform: Transform {
                                position: (60.0, 60.0),
                                rotation: 0.0,
                                scale: (1.0, 1.0),
                            },
                            sprite: Some(SpriteComponent {
                                asset_id: "puzzle_piece".to_string(),
                                color: (0.2, 1.0, 0.2, 1.0), // Green
                                visible: true,
                                layer: 1,
                            }),
                            collider: Some(ColliderComponent {
                                shape: ColliderShape::Rectangle { width: 32.0, height: 32.0 },
                                is_sensor: true,
                                physics_body: PhysicsBodyType::Kinematic,
                            }),
                            scripts: vec!["draggable_piece".to_string()],
                            tags: vec!["PuzzlePiece".to_string(), "Green".to_string()],
                        },
                    ],
                    camera: Camera {
                        position: (0.0, 0.0),
                        zoom: 1.0,
                        follow_target: None,
                    },
                    background: None,
                });

                // Add puzzle-specific scripts
                project.scripts.insert("puzzle_manager".to_string(), 
                    create_player_movement_script());
                project.scripts.insert("draggable_piece".to_string(), 
                    create_player_movement_script());
            },
            GameTemplate::ArcadeShooter => {
                // Create an arcade shooter scene with player ship and enemies
                let player_id = Uuid::new_v4();
                let enemy1_id = Uuid::new_v4();
                let enemy2_id = Uuid::new_v4();

                project.scenes.insert("main".to_string(), Scene {
                    name: "Space Battle".to_string(),
                    game_objects: vec![
                        GameObject {
                            id: player_id,
                            name: "Player Ship".to_string(),
                            transform: Transform {
                                position: (0.0, -100.0),
                                rotation: 0.0,
                                scale: (1.0, 1.0),
                            },
                            sprite: Some(SpriteComponent {
                                asset_id: "player_ship".to_string(),
                                color: (0.3, 0.8, 1.0, 1.0), // Light blue ship
                                visible: true,
                                layer: 2,
                            }),
                            collider: Some(ColliderComponent {
                                shape: ColliderShape::Rectangle { width: 24.0, height: 32.0 },
                                is_sensor: false,
                                physics_body: PhysicsBodyType::Dynamic,
                            }),
                            scripts: vec!["player_ship_control".to_string(), "ship_shooter".to_string()],
                            tags: vec!["Player".to_string(), "Ship".to_string()],
                        },
                        GameObject {
                            id: enemy1_id,
                            name: "Enemy Ship 1".to_string(),
                            transform: Transform {
                                position: (-50.0, 100.0),
                                rotation: 180.0,
                                scale: (1.0, 1.0),
                            },
                            sprite: Some(SpriteComponent {
                                asset_id: "enemy_ship".to_string(),
                                color: (1.0, 0.3, 0.3, 1.0), // Red enemy
                                visible: true,
                                layer: 2,
                            }),
                            collider: Some(ColliderComponent {
                                shape: ColliderShape::Rectangle { width: 20.0, height: 24.0 },
                                is_sensor: false,
                                physics_body: PhysicsBodyType::Dynamic,
                            }),
                            scripts: vec!["enemy_ai".to_string(), "enemy_shooter".to_string()],
                            tags: vec!["Enemy".to_string(), "Ship".to_string()],
                        },
                        GameObject {
                            id: enemy2_id,
                            name: "Enemy Ship 2".to_string(),
                            transform: Transform {
                                position: (50.0, 120.0),
                                rotation: 180.0,
                                scale: (1.0, 1.0),
                            },
                            sprite: Some(SpriteComponent {
                                asset_id: "enemy_ship".to_string(),
                                color: (1.0, 0.3, 0.3, 1.0), // Red enemy
                                visible: true,
                                layer: 2,
                            }),
                            collider: Some(ColliderComponent {
                                shape: ColliderShape::Rectangle { width: 20.0, height: 24.0 },
                                is_sensor: false,
                                physics_body: PhysicsBodyType::Dynamic,
                            }),
                            scripts: vec!["enemy_ai".to_string(), "enemy_shooter".to_string()],
                            tags: vec!["Enemy".to_string(), "Ship".to_string()],
                        },
                    ],
                    camera: Camera {
                        position: (0.0, 0.0),
                        zoom: 1.0,
                        follow_target: Some(player_id),
                    },
                    background: Some("space_background".to_string()),
                });

                // Add arcade shooter scripts
                project.scripts.insert("player_ship_control".to_string(), 
                    create_player_movement_script());
                project.scripts.insert("ship_shooter".to_string(), 
                    create_player_movement_script());
                project.scripts.insert("enemy_ai".to_string(), 
                    create_player_movement_script());
                project.scripts.insert("enemy_shooter".to_string(), 
                    create_player_movement_script());
            },
            GameTemplate::Blank => {
                // Create an empty scene for users to build from scratch
                project.scenes.insert("main".to_string(), Scene {
                    name: "Main Scene".to_string(),
                    game_objects: Vec::new(),
                    camera: Camera {
                        position: (0.0, 0.0),
                        zoom: 1.0,
                        follow_target: None,
                    },
                    background: None,
                });
            }
        }

        self.projects.insert(id, project.clone());
        project
    }

    pub fn get_project(&self, id: &Uuid) -> Option<&Project> {
        self.projects.get(id)
    }

    pub fn get_project_mut(&mut self, id: &Uuid) -> Option<&mut Project> {
        self.projects.get_mut(id)
    }

    pub fn list_projects(&self) -> Vec<&Project> {
        self.projects.values().collect()
    }

    pub fn update_project(&mut self, id: &Uuid, project: Project) -> Result<(), String> {
        if let Some(existing) = self.projects.get_mut(id) {
            *existing = project;
            existing.modified_at = chrono::Utc::now();
            Ok(())
        } else {
            Err("Project not found".to_string())
        }
    }

    pub fn delete_project(&mut self, id: &Uuid) -> Result<(), String> {
        if self.projects.remove(id).is_some() {
            Ok(())
        } else {
            Err("Project not found".to_string())
        }
    }

    /// Convert project to Lumina ECS World for game execution
    pub fn project_to_world(&self, project_id: &Uuid, scene_name: &str) -> Option<World> {
        let project = self.get_project(project_id)?;
        let scene = project.scenes.get(scene_name)?;

        let world = World::new();
        
        // TODO: Convert scene game objects to ECS entities with components
        // This would create entities in the world based on the scene configuration
        
        Some(world)
    }
}

impl Default for ProjectManager {
    fn default() -> Self {
        Self::new()
    }
}