//! Prompt engineering system for generating game content from natural language
//! 
//! This module handles the translation of user prompts into structured AI requests
//! that generate Scene Description Language (SDL) content.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use handlebars::Handlebars;
use anyhow::{Result, Context};

use crate::scene_description::SceneDescription;

/// Main prompt engine for processing user input
pub struct PromptEngine {
    handlebars: Handlebars<'static>,
    templates: HashMap<String, String>,
}

/// User's game creation prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamePrompt {
    /// The user's description of the game they want to create
    pub description: String,
    /// Game genre hint (optional)
    pub genre: Option<String>,
    /// Target complexity level
    pub complexity: ComplexityLevel,
    /// Visual style preference
    pub visual_style: Option<String>,
    /// Additional constraints or requirements
    pub constraints: Vec<String>,
    /// Existing scene to modify (for iterative generation)
    pub existing_scene: Option<SceneDescription>,
}

/// Complexity levels for game generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Simple,    // Basic mechanics, few entities
    Medium,    // Multiple systems, moderate complexity
    Complex,   // Advanced mechanics, many interactions
}

/// Template for generating AI prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub name: String,
    pub template: String,
    pub variables: HashMap<String, String>,
}

impl PromptEngine {
    /// Create a new prompt engine
    pub fn new() -> Self {
        let mut engine = Self {
            handlebars: Handlebars::new(),
            templates: HashMap::new(),
        };
        
        engine.register_default_templates();
        engine
    }

    /// Register default prompt templates
    fn register_default_templates(&mut self) {
        // Scene generation template
        let scene_template = include_str!("../templates/scene_generation.hbs");
        self.register_template("scene_generation", scene_template)
            .expect("Failed to register scene generation template");

        // Logic generation template
        let logic_template = include_str!("../templates/logic_generation.hbs");
        self.register_template("logic_generation", logic_template)
            .expect("Failed to register logic generation template");

        // Asset description template
        let asset_template = include_str!("../templates/asset_description.hbs");
        self.register_template("asset_description", asset_template)
            .expect("Failed to register asset description template");

        // Refinement template
        let refinement_template = include_str!("../templates/scene_refinement.hbs");
        self.register_template("scene_refinement", refinement_template)
            .expect("Failed to register scene refinement template");
    }

    /// Register a new template
    pub fn register_template(&mut self, name: &str, template: &str) -> Result<()> {
        self.handlebars.register_template_string(name, template)
            .context("Failed to register template")?;
        self.templates.insert(name.to_string(), template.to_string());
        Ok(())
    }

    /// Generate a scene creation prompt from user input
    pub fn generate_scene_prompt(&self, game_prompt: &GamePrompt) -> Result<String> {
        let mut context = serde_json::Map::new();
        
        context.insert("description".to_string(), 
                      serde_json::Value::String(game_prompt.description.clone()));
        
        if let Some(genre) = &game_prompt.genre {
            context.insert("genre".to_string(), 
                          serde_json::Value::String(genre.clone()));
        }
        
        context.insert("complexity".to_string(), 
                      serde_json::Value::String(format!("{:?}", game_prompt.complexity)));
        
        if let Some(style) = &game_prompt.visual_style {
            context.insert("visual_style".to_string(), 
                          serde_json::Value::String(style.clone()));
        }
        
        context.insert("constraints".to_string(), 
                      serde_json::Value::Array(
                          game_prompt.constraints.iter()
                              .map(|c| serde_json::Value::String(c.clone()))
                              .collect()
                      ));
        
        // Add example SDL structure
        context.insert("sdl_schema".to_string(), 
                      serde_json::Value::String(self.get_sdl_schema()));
        
        // Add example games for reference
        context.insert("examples".to_string(), 
                      serde_json::Value::Array(self.get_example_games()));

        self.handlebars.render("scene_generation", &context)
            .context("Failed to render scene generation prompt")
    }

    /// Generate a scene refinement prompt for iterative updates
    pub fn generate_refinement_prompt(&self, 
                                    user_request: &str, 
                                    current_scene: &SceneDescription) -> Result<String> {
        let mut context = serde_json::Map::new();
        
        context.insert("user_request".to_string(), 
                      serde_json::Value::String(user_request.to_string()));
        
        context.insert("current_scene".to_string(), 
                      serde_json::Value::String(current_scene.to_json()?));
        
        context.insert("sdl_schema".to_string(), 
                      serde_json::Value::String(self.get_sdl_schema()));

        self.handlebars.render("scene_refinement", &context)
            .context("Failed to render scene refinement prompt")
    }

    /// Generate asset description prompts
    pub fn generate_asset_prompt(&self, 
                               asset_type: &str, 
                               description: &str,
                               style: Option<&str>) -> Result<String> {
        let mut context = serde_json::Map::new();
        
        context.insert("asset_type".to_string(), 
                      serde_json::Value::String(asset_type.to_string()));
        
        context.insert("description".to_string(), 
                      serde_json::Value::String(description.to_string()));
        
        if let Some(style) = style {
            context.insert("style".to_string(), 
                          serde_json::Value::String(style.to_string()));
        }

        self.handlebars.render("asset_description", &context)
            .context("Failed to render asset description prompt")
    }

    /// Get SDL schema documentation for AI reference
    fn get_sdl_schema(&self) -> String {
        r#"
Scene Description Language (SDL) Schema:

{
  "metadata": {
    "name": "string",
    "description": "string", 
    "resolution": [width, height],
    "background_color": [r, g, b, a],
    "physics": {
      "enabled": boolean,
      "gravity": [x, y],
      "timestep": float
    },
    "audio": {
      "enabled": boolean,
      "master_volume": float,
      "music_volume": float,
      "sfx_volume": float
    }
  },
  "entities": [
    {
      "name": "string",
      "tags": ["string"],
      "components": {
        "Transform": {
          "type": "Transform",
          "data": {
            "position": [x, y],
            "rotation": float,
            "scale": [x, y]
          }
        },
        "Sprite": {
          "type": "Sprite", 
          "data": {
            "texture": "string",
            "color": [r, g, b, a],
            "flip_x": boolean,
            "flip_y": boolean,
            "layer": integer
          }
        },
        "Collider": {
          "type": "Collider",
          "data": {
            "shape": {
              "shape_type": "Rectangle|Circle|Capsule",
              "width": float, "height": float, // for Rectangle
              "radius": float, // for Circle/Capsule  
              "height": float  // for Capsule
            },
            "is_sensor": boolean,
            "friction": float,
            "restitution": float
          }
        },
        "Player": {
          "type": "Player",
          "data": {
            "speed": float,
            "jump_force": float,
            "health": integer,
            "max_health": integer
          }
        }
      }
    }
  ],
  "scripts": [
    {
      "id": "string",
      "name": "string", 
      "script_type": "System|EventHandler|Timer|Collision",
      "triggers": [
        {
          "type": "KeyPressed|Collision|Timer|HealthReduced",
          "data": { /* trigger-specific data */ }
        }
      ],
      "actions": [
        {
          "type": "MoveEntity|DestroyEntity|SpawnEntity|PlaySound|UpdateScore",
          "data": { /* action-specific data */ }
        }
      ]
    }
  ]
}
"#.to_string()
    }

    /// Get example games for AI reference
    fn get_example_games(&self) -> Vec<serde_json::Value> {
        vec![
            serde_json::json!({
                "description": "A simple pong game with two paddles and a ball",
                "genre": "arcade",
                "key_entities": ["left_paddle", "right_paddle", "ball", "score_ui"],
                "key_mechanics": ["paddle_movement", "ball_physics", "scoring", "collision_detection"]
            }),
            serde_json::json!({
                "description": "A platformer where the player collects coins and avoids enemies",
                "genre": "platformer", 
                "key_entities": ["player", "platforms", "coins", "enemies", "goal"],
                "key_mechanics": ["jumping", "collision", "collection", "enemy_ai", "level_progression"]
            }),
            serde_json::json!({
                "description": "A top-down shooter where the player fights waves of enemies",
                "genre": "shooter",
                "key_entities": ["player", "enemies", "bullets", "powerups", "spawners"],
                "key_mechanics": ["shooting", "movement", "enemy_spawning", "health", "powerup_collection"]
            })
        ]
    }
}

impl Default for ComplexityLevel {
    fn default() -> Self {
        ComplexityLevel::Medium
    }
}

impl GamePrompt {
    /// Create a new game prompt
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            genre: None,
            complexity: ComplexityLevel::default(),
            visual_style: None,
            constraints: Vec::new(),
            existing_scene: None,
        }
    }

    /// Set the genre
    pub fn with_genre(mut self, genre: impl Into<String>) -> Self {
        self.genre = Some(genre.into());
        self
    }

    /// Set the complexity level
    pub fn with_complexity(mut self, complexity: ComplexityLevel) -> Self {
        self.complexity = complexity;
        self
    }

    /// Set the visual style
    pub fn with_visual_style(mut self, style: impl Into<String>) -> Self {
        self.visual_style = Some(style.into());
        self
    }

    /// Add a constraint
    pub fn with_constraint(mut self, constraint: impl Into<String>) -> Self {
        self.constraints.push(constraint.into());
        self
    }

    /// Set existing scene for refinement
    pub fn with_existing_scene(mut self, scene: SceneDescription) -> Self {
        self.existing_scene = Some(scene);
        self
    }
}