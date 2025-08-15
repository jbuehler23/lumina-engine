//! Web input system for handling browser keyboard events
//! 
//! Maps JavaScript KeyboardEvent to our ECS input components.

use web_sys::KeyboardEvent;
use lumina_ecs::{World, InputState, InputMap};
use std::collections::HashSet;

/// Web input system that handles browser keyboard events
pub struct WebInputSystem {
    pressed_keys: HashSet<String>,
}

impl WebInputSystem {
    /// Create a new web input system
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
        }
    }
    
    /// Handle keydown event from browser
    pub fn handle_keydown(&mut self, event: &KeyboardEvent) {
        let key = self.normalize_key_code(&event.code());
        if !self.pressed_keys.contains(&key) {
            self.pressed_keys.insert(key.clone());
            // console::log_1(&format!("Key down: {}", key).into());
        }
        
        // Prevent default for game keys to avoid browser shortcuts
        if self.is_game_key(&key) {
            event.prevent_default();
        }
    }
    
    /// Handle keyup event from browser
    pub fn handle_keyup(&mut self, event: &KeyboardEvent) {
        let key = self.normalize_key_code(&event.code());
        self.pressed_keys.remove(&key);
        // console::log_1(&format!("Key up: {}", key).into());
        
        if self.is_game_key(&key) {
            event.prevent_default();
        }
    }
    
    /// Update game world input components based on current key state
    pub fn update_game_input(&self, world: &mut World) {
        let entity_count = world.entity_count();
        
        // Find all entities with InputMap and InputState components
        for i in 0..entity_count {
            let entity = lumina_ecs::Entity::from_raw(i as u64);
            
            if world.has_component::<InputMap>(entity) && world.has_component::<InputState>(entity) {
                // Get the input mapping
                let input_map = if let Some(map) = world.get_component::<InputMap>(entity) {
                    map
                } else {
                    continue;
                };
                
                if !input_map.enabled {
                    continue;
                }
                
                // Update the input state based on pressed keys
                world.with_component_mut::<InputState, _>(entity, |state_opt| {
                    if let Some(state) = state_opt {
                        // Store previous jump state for edge detection
                        state.was_jump_pressed = state.jump_held;
                        
                        // Check movement keys
                        let move_left = input_map.move_left.iter()
                            .any(|key| self.pressed_keys.contains(key));
                        let move_right = input_map.move_right.iter()
                            .any(|key| self.pressed_keys.contains(key));
                        
                        // Update horizontal movement (-1.0 = left, 1.0 = right)
                        state.move_horizontal = match (move_left, move_right) {
                            (true, false) => -1.0,
                            (false, true) => 1.0,
                            _ => 0.0, // Both or neither pressed
                        };
                        
                        // Check jump keys
                        let jump_held = input_map.jump.iter()
                            .any(|key| self.pressed_keys.contains(key));
                        
                        state.jump_held = jump_held;
                        state.jump_pressed = jump_held && !state.was_jump_pressed; // Rising edge
                    }
                });
            }
        }
    }
    
    /// Normalize browser key codes to our standard format
    fn normalize_key_code(&self, code: &str) -> String {
        match code {
            "KeyA" => "KeyA".to_string(),
            "KeyD" => "KeyD".to_string(),
            "KeyW" => "KeyW".to_string(),
            "KeyS" => "KeyS".to_string(),
            "Space" => "Space".to_string(),
            "ArrowLeft" => "ArrowLeft".to_string(),
            "ArrowRight" => "ArrowRight".to_string(),
            "ArrowUp" => "ArrowUp".to_string(),
            "ArrowDown" => "ArrowDown".to_string(),
            "Enter" => "Enter".to_string(),
            "Escape" => "Escape".to_string(),
            other => other.to_string(),
        }
    }
    
    /// Check if this is a game control key (to prevent browser shortcuts)
    fn is_game_key(&self, key: &str) -> bool {
        matches!(key, 
            "KeyA" | "KeyD" | "KeyW" | "KeyS" | 
            "Space" | "ArrowLeft" | "ArrowRight" | "ArrowUp" | "ArrowDown"
        )
    }
    
    /// Get current input state for debugging
    pub fn get_debug_info(&self) -> String {
        if self.pressed_keys.is_empty() {
            "No keys pressed".to_string()
        } else {
            format!("Pressed: {}", self.pressed_keys.iter().cloned().collect::<Vec<_>>().join(", "))
        }
    }
}