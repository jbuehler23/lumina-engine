//! WASM Game Runtime for Lumina Engine
//! 
//! This crate provides a WebAssembly-compiled game runtime that can run
//! Lumina Engine games directly in the browser with full ECS, physics,
//! and rendering support.

use wasm_bindgen::prelude::*;
use web_sys::{console, HtmlCanvasElement, KeyboardEvent};
use lumina_ai::SceneDescription;

pub mod game;
pub mod input;
pub mod renderer;

use game::GameInstance;
use input::WebInputSystem;
use renderer::WebRenderer;

/// Main WASM game application
#[wasm_bindgen]
pub struct LuminaGame {
    game_instance: Option<GameInstance>,
    canvas: HtmlCanvasElement,
    web_renderer: Option<WebRenderer>,
    input_system: WebInputSystem,
    animation_frame_id: Option<i32>,
    last_frame_time: f64,
}

#[wasm_bindgen]
impl LuminaGame {
    /// Create a new game instance
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<LuminaGame, JsValue> {
        // Set up panic handler for better error messages
        console_error_panic_hook::set_once();
        
        console::log_1(&"🎮 Lumina Game WASM initializing...".into());
        
        // Get the canvas element
        let window = web_sys::window().ok_or("No window object")?;
        let document = window.document().ok_or("No document object")?;
        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or_else(|| format!("Canvas element '{}' not found", canvas_id))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "Element is not a canvas")?;
        
        Ok(LuminaGame {
            game_instance: None,
            canvas,
            web_renderer: None,
            input_system: WebInputSystem::new(),
            animation_frame_id: None,
            last_frame_time: 0.0,
        })
    }
    
    /// Initialize the game with WGPU and load a scene
    #[wasm_bindgen]
    pub async fn initialize(&mut self, scene_json: &str) -> Result<(), JsValue> {
        console::log_1(&"🚀 Initializing WGPU and game systems...".into());
        
        // Initialize the web renderer
        let web_renderer = WebRenderer::new(&self.canvas).await
            .map_err(|e| JsValue::from_str(&format!("Failed to initialize renderer: {:?}", e)))?;
        
        // Parse the scene description
        let scene: SceneDescription = serde_json::from_str(scene_json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse scene: {:?}", e)))?;
        
        // Create game instance
        let mut game_instance = GameInstance::new();
        game_instance.load_scene(scene)
            .map_err(|e| JsValue::from_str(&format!("Failed to load scene: {:?}", e)))?;
        
        self.web_renderer = Some(web_renderer);
        self.game_instance = Some(game_instance);
        
        console::log_1(&"✅ Game initialized successfully!".into());
        Ok(())
    }
    
    /// Start the game loop
    #[wasm_bindgen]
    pub fn start(&mut self) -> Result<(), JsValue> {
        if self.game_instance.is_none() || self.web_renderer.is_none() {
            return Err(JsValue::from_str("Game not initialized. Call initialize() first."));
        }
        
        console::log_1(&"🔄 Starting game loop...".into());
        
        // Get current time
        let window = web_sys::window().unwrap();
        let performance = window.performance().unwrap();
        self.last_frame_time = performance.now();
        
        // Start the animation loop
        self.schedule_next_frame()?;
        
        Ok(())
    }
    
    /// Stop the game loop
    #[wasm_bindgen]
    pub fn stop(&mut self) {
        if let Some(id) = self.animation_frame_id.take() {
            let window = web_sys::window().unwrap();
            window.cancel_animation_frame(id).unwrap();
        }
        console::log_1(&"⏹️  Game loop stopped".into());
    }
    
    /// Handle keyboard input
    #[wasm_bindgen]
    pub fn handle_keydown(&mut self, event: KeyboardEvent) {
        if let Some(game_instance) = &mut self.game_instance {
            self.input_system.handle_keydown(&event);
            self.input_system.update_game_input(&mut game_instance.world);
        }
    }
    
    /// Handle keyboard release
    #[wasm_bindgen]
    pub fn handle_keyup(&mut self, event: KeyboardEvent) {
        if let Some(game_instance) = &mut self.game_instance {
            self.input_system.handle_keyup(&event);
            self.input_system.update_game_input(&mut game_instance.world);
        }
    }
    
    /// Update and render one frame
    #[wasm_bindgen]
    pub fn render_frame(&mut self, current_time: f64) -> Result<(), JsValue> {
        let dt = ((current_time - self.last_frame_time) / 1000.0) as f32;
        self.last_frame_time = current_time;
        
        // Clamp delta time to prevent large jumps
        let dt = dt.min(0.016); // Max 16ms (60 FPS minimum)
        
        if let (Some(game_instance), Some(web_renderer)) = (&mut self.game_instance, &mut self.web_renderer) {
            // Update input first
            self.input_system.update_game_input(&mut game_instance.world);
            
            // Update game systems
            game_instance.update(dt)?;
            
            // Render the frame
            web_renderer.render_frame(&game_instance.world)
                .map_err(|e| JsValue::from_str(&format!("Render error: {:?}", e)))?;
        }
        
        // Schedule next frame
        self.schedule_next_frame()?;
        
        Ok(())
    }
    
    /// Get game statistics for debugging
    #[wasm_bindgen]
    pub fn get_stats(&self) -> JsValue {
        if let Some(game_instance) = &self.game_instance {
            let stats = serde_json::json!({
                "entities": game_instance.world.entity_count(),
                "fps": (1000.0 / (web_sys::window().unwrap().performance().unwrap().now() - self.last_frame_time)).round() as u32,
                "physics_entities": game_instance.get_physics_entity_count()
            });
            serde_wasm_bindgen::to_value(&stats).unwrap()
        } else {
            JsValue::NULL
        }
    }
    
    /// Resize the game canvas
    #[wasm_bindgen]
    pub fn resize(&mut self, width: u32, height: u32) {
        if let Some(web_renderer) = &mut self.web_renderer {
            let _ = web_renderer.resize(width, height);
        }
    }
    
    /// Load a new scene
    #[wasm_bindgen]
    pub fn load_scene(&mut self, scene_json: &str) -> Result<(), JsValue> {
        if let Some(game_instance) = &mut self.game_instance {
            let scene: SceneDescription = serde_json::from_str(scene_json)
                .map_err(|e| JsValue::from_str(&format!("Failed to parse scene: {:?}", e)))?;
            
            game_instance.load_scene(scene)
                .map_err(|e| JsValue::from_str(&format!("Failed to load scene: {:?}", e)))?;
            
            console::log_1(&"🔄 Scene reloaded successfully".into());
        }
        Ok(())
    }
    
    /// Schedule the next animation frame
    fn schedule_next_frame(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        
        // Create closure that captures self
        let game_ptr = self as *mut LuminaGame;
        let closure = Closure::once_into_js(move |current_time: f64| {
            // Safety: We know this pointer is valid for the duration of the closure
            let game = unsafe { &mut *game_ptr };
            if let Err(e) = game.render_frame(current_time) {
                console::log_1(&format!("Frame render error: {:?}", e).into());
            }
        });
        
        let id = window.request_animation_frame(closure.as_ref().unchecked_ref())?;
        self.animation_frame_id = Some(id);
        
        Ok(())
    }
}

/// Initialize logging and panic hooks for WASM
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console::log_1(&"🎮 Lumina Game WASM module loaded".into());
}

/// Create a default platformer scene for testing
#[wasm_bindgen]
pub fn create_platformer_scene() -> String {
    let scene = create_demo_platformer_scene();
    serde_json::to_string(&scene).unwrap()
}

/// Helper function to create a demo scene
fn create_demo_platformer_scene() -> SceneDescription {
    use lumina_ai::scene_description::*;
    use std::collections::HashMap;

    let mut scene = SceneDescription::platformer_template("Web Platformer Demo");
    
    // Position the player and add sprite
    if let Some(player) = scene.scenes[0].entities.iter_mut().find(|e| e.name == "Player") {
        if let Some(transform) = player.components.get_mut("Transform2D") {
            transform.data = serde_json::json!({
                "pos": [120.0, 150.0],
                "rot": 0.0,
                "scale": [1.0, 1.0]
            });
        }
        
        // Add sprite component for visual rendering
        player.components.insert("Sprite".to_string(), ComponentData {
            component_type: "Sprite".to_string(),
            data: serde_json::json!({
                "atlas_id": "player",
                "frame": 0,
                "color": [0.2, 0.8, 0.2, 1.0], // Green player
                "flip_x": false,
                "flip_y": false,
                "layer": 1
            }),
        });
        
        // Add input components
        player.components.insert("InputMap".to_string(), ComponentData {
            component_type: "InputMap".to_string(),
            data: serde_json::json!({
                "move_left": ["KeyA", "ArrowLeft"],
                "move_right": ["KeyD", "ArrowRight"], 
                "jump": ["KeyW", "Space", "ArrowUp"],
                "enabled": true
            }),
        });
        
        player.components.insert("InputState".to_string(), ComponentData {
            component_type: "InputState".to_string(),
            data: serde_json::json!({
                "move_horizontal": 0.0,
                "jump_pressed": false,
                "jump_held": false,
                "was_jump_pressed": false
            }),
        });
    }
    
    // Add static platforms for collision testing
    let platform_positions = [
        (120.0, 220.0, 96.0, 16.0),   // High platform
        (320.0, 280.0, 96.0, 16.0),   // Mid platform
        (520.0, 320.0, 96.0, 16.0),   // Low platform
        (720.0, 200.0, 96.0, 16.0),   // Another platform
        (400.0, 440.0, 800.0, 32.0),  // Ground
    ];
    
    for (i, (x, y, width, height)) in platform_positions.iter().enumerate() {
        let mut components = HashMap::new();
        
        components.insert("Transform2D".to_string(), ComponentData {
            component_type: "Transform2D".to_string(),
            data: serde_json::json!({
                "pos": [*x, *y],
                "rot": 0.0,
                "scale": [1.0, 1.0]
            }),
        });
        
        components.insert("Collider2D".to_string(), ComponentData {
            component_type: "Collider2D".to_string(),
            data: serde_json::json!({
                "shape": { "AABB": { "width": *width, "height": *height } },
                "is_sensor": false,
                "friction": 0.8,
                "restitution": 0.0,
                "collision_layers": ["solid"]
            }),
        });
        
        // Add sprite component for platform rendering
        components.insert("Sprite".to_string(), ComponentData {
            component_type: "Sprite".to_string(),
            data: serde_json::json!({
                "atlas_id": "platform",
                "frame": 0,
                "color": [0.6, 0.4, 0.2, 1.0], // Brown platform
                "flip_x": false,
                "flip_y": false,
                "layer": 0
            }),
        });

        scene.scenes[0].entities.push(Entity {
            name: format!("Platform{}", i + 1),
            tags: vec!["platform".to_string(), "solid".to_string()],
            components,
        });
    }
    
    // Add collectible coins
    let coin_positions = [
        (200.0, 180.0),   // Above first platform
        (400.0, 240.0),   // Above second platform
        (600.0, 280.0),   // Above third platform
        (800.0, 160.0),   // Above fourth platform
    ];
    
    for (i, (x, y)) in coin_positions.iter().enumerate() {
        let mut components = HashMap::new();
        
        components.insert("Transform2D".to_string(), ComponentData {
            component_type: "Transform2D".to_string(),
            data: serde_json::json!({
                "pos": [*x, *y],
                "rot": 0.0,
                "scale": [1.0, 1.0]
            }),
        });
        
        components.insert("Collider2D".to_string(), ComponentData {
            component_type: "Collider2D".to_string(),
            data: serde_json::json!({
                "shape": { "Circle": { "radius": 12.0 } },
                "is_sensor": true, // Coins are collectible triggers
                "friction": 0.0,
                "restitution": 0.0,
                "collision_layers": ["collectible"]
            }),
        });
        
        // Add sprite component for coin rendering
        components.insert("Sprite".to_string(), ComponentData {
            component_type: "Sprite".to_string(),
            data: serde_json::json!({
                "atlas_id": "coin",
                "frame": 0,
                "color": [1.0, 0.8, 0.0, 1.0], // Gold coin
                "flip_x": false,
                "flip_y": false,
                "layer": 1
            }),
        });

        scene.scenes[0].entities.push(Entity {
            name: format!("Coin{}", i + 1),
            tags: vec!["collectible".to_string(), "coin".to_string()],
            components,
        });
    }
    
    scene
}