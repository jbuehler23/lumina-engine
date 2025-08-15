//! Input handling systems for 2D platformer games
//! 
//! This module implements input processing for character controllers,
//! allowing players to move, jump, and interact with the game world.

use crate::*;

/// Configuration for platformer input handling
#[derive(Debug, Clone)]
pub struct InputConfig {
    pub move_speed: f32,
    pub jump_speed: f32,
    pub air_control: f32,      // How much control player has in air (0.0 - 1.0)
    pub coyote_time: f32,      // Grace period for jumping after leaving ground
    pub jump_buffer_time: f32, // Grace period for jump input before landing
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            move_speed: 300.0,     // pixels/second
            jump_speed: 500.0,     // pixels/second  
            air_control: 0.8,      // 80% control in air
            coyote_time: 0.1,      // 100ms grace period
            jump_buffer_time: 0.2, // 200ms jump buffer
        }
    }
}

/// System for handling platformer character input
pub struct InputSystem {
    pub config: InputConfig,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            config: InputConfig::default(),
        }
    }
    
    pub fn with_config(config: InputConfig) -> Self {
        Self { config }
    }
    
    /// Process input for all entities with InputMap and InputState components
    /// This should be called before physics update
    pub fn update(&mut self, world: &mut World, dt: f32) {
        // Step 1: Update InputState for all entities based on their InputMap
        self.update_input_states(world);
        
        // Step 2: Process character controller input
        let entity_count = world.entity_count();
        let mut controllable_entities = Vec::new();
        
        for i in 0..entity_count {
            let entity = Entity::from_raw(i as u64);
            
            if world.has_component::<CharacterController2D>(entity) && 
               world.has_component::<InputState>(entity) {
                controllable_entities.push(entity);
            }
        }
        
        // Process input for each controllable entity
        for entity in controllable_entities {
            self.process_character_input(world, entity, dt);
        }
    }
    
    /// Update InputState components based on InputMap and global keyboard state
    fn update_input_states(&self, world: &mut World) {
        let entity_count = world.entity_count();
        
        for i in 0..entity_count {
            let entity = Entity::from_raw(i as u64);
            
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
                
                // Update the input state based on keyboard input
                world.with_component_mut::<InputState, _>(entity, |state_opt| {
                    if let Some(state) = state_opt {
                        // Store previous jump state for edge detection
                        state.was_jump_pressed = state.jump_held;
                        
                        // Read keyboard input from world resource
                        let (move_left, move_right, jump_held) = 
                            self.read_keyboard_input(world, &input_map);
                        
                        // Update horizontal movement (-1.0 = left, 1.0 = right)
                        state.move_horizontal = match (move_left, move_right) {
                            (true, false) => -1.0,
                            (false, true) => 1.0,
                            _ => 0.0, // Both or neither pressed
                        };
                        
                        // Update jump state
                        state.jump_held = jump_held;
                        state.jump_pressed = jump_held && !state.was_jump_pressed; // Rising edge
                    }
                });
            }
        }
    }
    
    /// Read keyboard input from the world's ButtonInput resource
    fn read_keyboard_input(&self, world: &World, input_map: &InputMap) -> (bool, bool, bool) {
        // Try to get keyboard input resource
        let mut move_left = false;
        let mut move_right = false; 
        let mut jump = false;
        
        // Check if we have keyboard input resource
        world.with_resource::<lumina_input::ButtonInput<winit::keyboard::PhysicalKey>, _>(|keyboard_opt| {
            if let Some(keyboard) = keyboard_opt {
                // Check left movement keys
                for key_name in &input_map.move_left {
                    if let Some(key) = self.parse_key(key_name) {
                        if keyboard.pressed(key) {
                            move_left = true;
                            break;
                        }
                    }
                }
                
                // Check right movement keys
                for key_name in &input_map.move_right {
                    if let Some(key) = self.parse_key(key_name) {
                        if keyboard.pressed(key) {
                            move_right = true;
                            break;
                        }
                    }
                }
                
                // Check jump keys
                for key_name in &input_map.jump {
                    if let Some(key) = self.parse_key(key_name) {
                        if keyboard.pressed(key) {
                            jump = true;
                            break;
                        }
                    }
                }
            }
        });
        
        (move_left, move_right, jump)
    }
    
    /// Parse key name string to PhysicalKey
    fn parse_key(&self, key_name: &str) -> Option<winit::keyboard::PhysicalKey> {
        use winit::keyboard::{PhysicalKey, KeyCode};
        
        let key_code = match key_name {
            "KeyA" => KeyCode::KeyA,
            "KeyD" => KeyCode::KeyD,
            "KeyW" => KeyCode::KeyW,
            "KeyS" => KeyCode::KeyS,
            "Space" => KeyCode::Space,
            "ArrowLeft" => KeyCode::ArrowLeft,
            "ArrowRight" => KeyCode::ArrowRight,
            "ArrowUp" => KeyCode::ArrowUp,
            "ArrowDown" => KeyCode::ArrowDown,
            "Enter" => KeyCode::Enter,
            "Escape" => KeyCode::Escape,
            _ => return None,
        };
        
        Some(PhysicalKey::Code(key_code))
    }
    
    /// Process input for a single character entity
    fn process_character_input(&self, world: &mut World, entity: Entity, dt: f32) {
        // Get input state from the entity's InputState component
        let (move_input, jump_pressed) = if let Some(input_state) = world.get_component::<InputState>(entity) {
            (input_state.move_horizontal, input_state.jump_pressed)
        } else {
            return; // No input state, nothing to process
        };
        
        // Update character controller state
        world.with_component_mut::<CharacterController2D, _>(entity, |controller_opt| {
            if let Some(controller) = controller_opt {
                // Update timers
                if controller.is_grounded {
                    controller.coyote_timer = self.config.coyote_time;
                } else {
                    controller.coyote_timer = (controller.coyote_timer - dt).max(0.0);
                }
                
                if jump_pressed {
                    controller.jump_buffer_timer = self.config.jump_buffer_time;
                } else {
                    controller.jump_buffer_timer = (controller.jump_buffer_timer - dt).max(0.0);
                }
                
                // Apply movement to rigidbody
                if world.has_component::<RigidBody2D>(entity) {
                    world.with_component_mut::<RigidBody2D, _>(entity, |rb_opt| {
                        if let Some(rb) = rb_opt {
                            // Horizontal movement
                            let control_factor = if controller.is_grounded { 1.0 } else { self.config.air_control };
                            let target_vel_x = move_input * self.config.move_speed * control_factor;
                            
                            // Apply movement (lerp for smoother control)
                            let lerp_factor = if controller.is_grounded { 10.0 * dt } else { 5.0 * dt };
                            rb.vel.x = rb.vel.x + (target_vel_x - rb.vel.x) * lerp_factor.min(1.0);
                            
                            // Jumping
                            let can_jump = (controller.is_grounded || controller.coyote_timer > 0.0) && 
                                         controller.jump_buffer_timer > 0.0;
                            
                            if can_jump && !controller.jump_consumed {
                                rb.vel.y = self.config.jump_speed;
                                controller.jump_consumed = true;
                                controller.jump_buffer_timer = 0.0;
                                println!("🦘 Jump! vel_y = {} (Entity {})", rb.vel.y, entity.id());
                            }
                            
                            // Reset jump when grounded
                            if controller.is_grounded && rb.vel.y <= 0.0 {
                                controller.jump_consumed = false;
                            }
                        }
                    });
                }
            }
        });
    }
}

impl Default for InputSystem {
    fn default() -> Self {
        Self::new()
    }
}