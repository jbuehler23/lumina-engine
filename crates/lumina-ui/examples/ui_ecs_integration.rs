//! UI-ECS Integration Example
//! 
//! This example demonstrates how to integrate the Lumina UI system with ECS
//! for proper event handling and state management. It shows:
//! - UI events triggering ECS system updates
//! - ECS state driving UI updates
//! - Proper separation of concerns between UI and game logic

use lumina_ui::{
    UiApplication, UiBuilder, Color, ButtonStyle, InputEvent,
    UiAppConfig, run_ui_app
};
use lumina_ecs::{World, Entity, System};
use std::collections::HashMap;

// ECS Components
#[derive(Debug, Clone)]
struct Player {
    name: String,
    health: i32,
    max_health: i32,
    level: i32,
    experience: i32,
}

#[derive(Debug, Clone)]
struct GameState {
    score: i32,
    lives: i32,
    paused: bool,
    game_over: bool,
}

// UI Events that can be sent to ECS
#[derive(Debug, Clone)]
enum UiEvent {
    StartGame,
    PauseGame,
    ResumeGame,
    RestartGame,
    HealPlayer,
    AddExperience(i32),
    ResetScore,
}

/// Game application that integrates UI with ECS
struct GameApp {
    world: World,
    player_entity: Option<Entity>,
    game_state_entity: Option<Entity>,
    pending_events: Vec<UiEvent>,
}

impl GameApp {
    fn new() -> Self {
        let mut world = World::new();
        
        // Create player entity
        let player_entity = world.spawn();
        world.insert_component(player_entity, Player {
            name: "Hero".to_string(),
            health: 100,
            max_health: 100,
            level: 1,
            experience: 0,
        });
        
        // Create game state entity
        let game_state_entity = world.spawn();
        world.insert_component(game_state_entity, GameState {
            score: 0,
            lives: 3,
            paused: false,
            game_over: false,
        });
        
        Self {
            world,
            player_entity: Some(player_entity),
            game_state_entity: Some(game_state_entity),
            pending_events: Vec::new(),
        }
    }
    
    fn process_ui_events(&mut self) {
        let events = std::mem::take(&mut self.pending_events);
        
        for event in events {
            match event {
                UiEvent::StartGame => {
                    if let Some(entity) = self.game_state_entity {
                        if let Some(mut state) = self.world.get_component_mut::<GameState>(entity) {
                            state.paused = false;
                            state.game_over = false;
                            println!("🎮 Game Started!");
                        }
                    }
                }
                UiEvent::PauseGame => {
                    if let Some(entity) = self.game_state_entity {
                        if let Some(mut state) = self.world.get_component_mut::<GameState>(entity) {
                            state.paused = true;
                            println!("⏸️ Game Paused");
                        }
                    }
                }
                UiEvent::ResumeGame => {
                    if let Some(entity) = self.game_state_entity {
                        if let Some(mut state) = self.world.get_component_mut::<GameState>(entity) {
                            state.paused = false;
                            println!("▶️ Game Resumed");
                        }
                    }
                }
                UiEvent::RestartGame => {
                    // Reset player
                    if let Some(entity) = self.player_entity {
                        if let Some(mut player) = self.world.get_component_mut::<Player>(entity) {
                            player.health = player.max_health;
                            player.level = 1;
                            player.experience = 0;
                        }
                    }
                    
                    // Reset game state
                    if let Some(entity) = self.game_state_entity {
                        if let Some(mut state) = self.world.get_component_mut::<GameState>(entity) {
                            state.score = 0;
                            state.lives = 3;
                            state.paused = false;
                            state.game_over = false;
                        }
                    }
                    
                    println!("🔄 Game Restarted!");
                }
                UiEvent::HealPlayer => {
                    if let Some(entity) = self.player_entity {
                        if let Some(mut player) = self.world.get_component_mut::<Player>(entity) {
                            player.health = (player.health + 20).min(player.max_health);
                            println!("💚 Player healed! Health: {}/{}", player.health, player.max_health);
                        }
                    }
                }
                UiEvent::AddExperience(xp) => {
                    if let Some(entity) = self.player_entity {
                        if let Some(mut player) = self.world.get_component_mut::<Player>(entity) {
                            player.experience += xp;
                            
                            // Level up logic
                            let required_xp = player.level * 100;
                            if player.experience >= required_xp {
                                player.level += 1;
                                player.experience -= required_xp;
                                player.max_health += 10;
                                player.health = player.max_health; // Full heal on level up
                                println!("🎉 Level up! Now level {}", player.level);
                            }
                        }
                    }
                }
                UiEvent::ResetScore => {
                    if let Some(entity) = self.game_state_entity {
                        if let Some(mut state) = self.world.get_component_mut::<GameState>(entity) {
                            state.score = 0;
                            println!("📊 Score reset!");
                        }
                    }
                }
            }
        }
    }
    
    fn simulate_game_tick(&mut self) {
        // Simulate some game logic - add score over time if not paused
        if let Some(entity) = self.game_state_entity {
            if let Some(mut state) = self.world.get_component_mut::<GameState>(entity) {
                if !state.paused && !state.game_over {
                    state.score += 1;
                }
            }
        }
    }
}

impl UiApplication for GameApp {
    fn build_ui(&mut self, ui: &mut UiBuilder) {
        // Process any pending events first
        self.process_ui_events();
        
        // Get current game state for UI
        let (player, game_state) = if let (Some(p_entity), Some(g_entity)) = (self.player_entity, self.game_state_entity) {
            let player = self.world.get_component::<Player>(p_entity).cloned();
            let game_state = self.world.get_component::<GameState>(g_entity).cloned();
            (player, game_state)
        } else {
            (None, None)
        };
        
        // Build the UI based on current state
        let title = ui.text("🎮 UI-ECS Integration Demo")
            .size(28.0)
            .color(Color::hex("#00D9FF").unwrap())
            .build();
        
        if let Some(player) = &player {
            // Player stats
            let player_title = ui.text(&format!("👤 Player: {}", player.name))
                .size(20.0)
                .color(Color::WHITE)
                .build();
            
            let health_color = if player.health > player.max_health / 2 {
                Color::GREEN
            } else if player.health > player.max_health / 4 {
                Color::hex("#FFA500").unwrap() // Orange
            } else {
                Color::RED
            };
            
            let health_text = ui.text(&format!("❤️ Health: {}/{}", player.health, player.max_health))
                .size(16.0)
                .color(health_color)
                .build();
            
            let level_text = ui.text(&format!("⭐ Level: {} (XP: {})", player.level, player.experience))
                .size(16.0)
                .color(Color::hex("#FFD700").unwrap())
                .build();
        }
        
        if let Some(state) = &game_state {
            // Game state
            let score_text = ui.text(&format!("🏆 Score: {}", state.score))
                .size(18.0)
                .color(Color::hex("#FFD700").unwrap())
                .build();
            
            let lives_text = ui.text(&format!("💖 Lives: {}", state.lives))
                .size(16.0)
                .color(Color::RED)
                .build();
            
            let status_color = if state.game_over {
                Color::RED
            } else if state.paused {
                Color::hex("#FFA500").unwrap()
            } else {
                Color::GREEN
            };
            
            let status = if state.game_over {
                "💀 Game Over"
            } else if state.paused {
                "⏸️ Paused"
            } else {
                "▶️ Playing"
            };
            
            let status_text = ui.text(&format!("Status: {}", status))
                .size(16.0)
                .color(status_color)
                .build();
        }
        
        // Game control buttons
        let start_button = ui.button("🚀 Start Game")
            .style(ButtonStyle::Success)
            .on_click({
                || {
                    // This callback will be handled in handle_input
                    println!("Start button clicked");
                }
            })
            .build();
        
        let pause_button = ui.button("⏸️ Pause")
            .style(ButtonStyle::Warning)
            .on_click(|| println!("Pause button clicked"))
            .build();
        
        let resume_button = ui.button("▶️ Resume")
            .style(ButtonStyle::Primary)
            .on_click(|| println!("Resume button clicked"))
            .build();
        
        let restart_button = ui.button("🔄 Restart")
            .style(ButtonStyle::Secondary)
            .on_click(|| println!("Restart button clicked"))
            .build();
        
        // Player action buttons
        let heal_button = ui.button("💚 Heal (+20 HP)")
            .style(ButtonStyle::Success)
            .on_click(|| println!("Heal button clicked"))
            .build();
        
        let xp_button = ui.button("⭐ Gain XP (+50)")
            .style(ButtonStyle::Primary)
            .on_click(|| println!("XP button clicked"))
            .build();
        
        let reset_score_button = ui.button("📊 Reset Score")
            .style(ButtonStyle::Danger)
            .on_click(|| println!("Reset score button clicked"))
            .build();
    }
    
    fn update(&mut self, _ui: &mut UiBuilder) {
        // Process ECS events
        self.process_ui_events();
        
        // Simulate game logic
        self.simulate_game_tick();
        
        // This is where you'd run your ECS systems
        // For now, we'll just do a simple simulation
    }
    
    fn handle_input(&mut self, event: &InputEvent, _ui: &mut UiBuilder) -> bool {
        match event {
            InputEvent::MouseClick { .. } => {
                // For this demo, we'll use simple logic to determine which button was clicked
                // In a real implementation, you'd have proper widget identification
                // For now, we'll cycle through actions on each click for demonstration
                
                static mut CLICK_COUNTER: usize = 0;
                unsafe {
                    CLICK_COUNTER += 1;
                    match CLICK_COUNTER % 7 {
                        1 => self.pending_events.push(UiEvent::StartGame),
                        2 => self.pending_events.push(UiEvent::PauseGame),
                        3 => self.pending_events.push(UiEvent::ResumeGame),
                        4 => self.pending_events.push(UiEvent::HealPlayer),
                        5 => self.pending_events.push(UiEvent::AddExperience(50)),
                        6 => self.pending_events.push(UiEvent::ResetScore),
                        0 => self.pending_events.push(UiEvent::RestartGame),
                        _ => {}
                    }
                }
                
                true // Event handled
            }
            _ => false
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = GameApp::new();
    
    let config = UiAppConfig {
        title: "UI-ECS Integration Demo".to_string(),
        size: (1000, 700),
        resizable: true,
        decorations: true,
    };
    
    println!("🎮 UI-ECS Integration Demo");
    println!("💡 Click anywhere to cycle through different actions:");
    println!("   1. Start Game → 2. Pause → 3. Resume → 4. Heal Player");
    println!("   5. Gain XP → 6. Reset Score → 7. Restart Game");
    println!("📊 Watch the UI update in real-time as ECS state changes!");
    
    run_ui_app(app, config)
}