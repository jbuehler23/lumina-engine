//! AI Game Generator Example
//! 
//! This example demonstrates the AI-native capabilities of Lumina Engine.
//! It shows how to generate a complete game from a text prompt.

use std::env;
use anyhow::Result;
use log::{info, error};

use lumina_ai::{
    AiConfig, AiProvider, GamePrompt, GameGenerationRequest, 
    initialize_ai_system, ComplexityLevel, TargetPlatform
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Get AI configuration from environment variables
    let config = AiConfig {
        provider: AiProvider::OpenAI,
        api_key: env::var("OPENAI_API_KEY").ok(),
        base_url: None,
        text_model: "gpt-4".to_string(),
        image_model: Some("dall-e-3".to_string()),
        max_tokens: 4096,
        temperature: 0.7,
    };

    // Check if API key is available
    if config.api_key.is_none() {
        println!("⚠️  No OpenAI API key found. Set OPENAI_API_KEY environment variable.");
        println!("Running in demo mode with placeholder generation...");
    }

    println!("🎮 Lumina AI Game Generator");
    println!("==========================");

    // Example game prompts to demonstrate different capabilities
    let example_prompts = vec![
        (
            "Simple Platformer",
            "Create a simple 2D platformer where the player jumps on platforms to collect coins and reach the goal. Include enemies that patrol back and forth.",
            Some("platformer".to_string()),
            ComplexityLevel::Simple
        ),
        (
            "Space Shooter", 
            "Make a top-down space shooter where the player controls a spaceship and shoots at incoming asteroids and enemy ships. Include power-ups and a scoring system.",
            Some("shooter".to_string()),
            ComplexityLevel::Medium
        ),
        (
            "Puzzle Game",
            "Design a puzzle game where the player pushes blocks to solve room layouts. Include multiple levels with increasing difficulty.",
            Some("puzzle".to_string()),
            ComplexityLevel::Medium
        ),
        (
            "Racing Game",
            "Create a simple top-down racing game with checkpoints, obstacles, and a timer. The player should be able to steer around a track.",
            Some("racing".to_string()),
            ComplexityLevel::Simple
        ),
    ];

    // Initialize the AI system
    let mut game_generator = match initialize_ai_system(config).await {
        Ok(generator) => generator,
        Err(e) => {
            error!("Failed to initialize AI system: {}", e);
            println!("❌ Failed to initialize AI system. Please check your configuration.");
            return Ok(());
        }
    };

    // Generate each example game
    for (i, (name, description, genre, complexity)) in example_prompts.iter().enumerate() {
        println!("\n🚀 Generating Game {}: {}", i + 1, name);
        println!("📝 Description: {}", description);
        
        // Create game prompt
        let prompt = GamePrompt::new(description)
            .with_genre(genre.clone().unwrap_or_default())
            .with_complexity(complexity.clone())
            .with_visual_style("pixel art")
            .with_constraint("Keep controls simple");

        // Create generation request
        let request = GameGenerationRequest::new(prompt)
            .for_platform(TargetPlatform::Desktop)
            .with_max_entities(15);

        // Generate the game
        match game_generator.generate_game(request).await {
            Ok(result) => {
                println!("✅ Game generated successfully!");
                print_generation_result(&result);
                
                // Save the generated scene to a file
                let filename = format!("generated_game_{}.json", i + 1);
                match save_scene_to_file(&result.scene, &filename) {
                    Ok(_) => println!("💾 Scene saved to: {}", filename),
                    Err(e) => println!("⚠️  Failed to save scene: {}", e),
                }
            }
            Err(e) => {
                error!("Failed to generate game '{}': {}", name, e);
                println!("❌ Generation failed: {}", e);
            }
        }

        // Add a small delay between generations to be respectful to API limits
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }

    println!("\n🎯 Interactive Mode");
    println!("==================");
    
    // Interactive mode for custom prompts
    loop {
        println!("\nEnter a game description (or 'quit' to exit):");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();
        
        if input.eq_ignore_ascii_case("quit") || input.is_empty() {
            break;
        }

        println!("🎮 Generating your custom game...");
        
        let prompt = GamePrompt::new(input)
            .with_complexity(ComplexityLevel::Medium)
            .with_visual_style("pixel art");

        let request = GameGenerationRequest::new(prompt)
            .for_platform(TargetPlatform::Desktop);

        match game_generator.generate_game(request).await {
            Ok(result) => {
                println!("✅ Custom game generated!");
                print_generation_result(&result);
                
                // Save custom game
                let filename = "custom_game.json";
                match save_scene_to_file(&result.scene, filename) {
                    Ok(_) => println!("💾 Scene saved to: {}", filename),
                    Err(e) => println!("⚠️  Failed to save scene: {}", e),
                }
            }
            Err(e) => {
                println!("❌ Generation failed: {}", e);
            }
        }
    }

    println!("\n👋 Thanks for trying Lumina AI Game Generator!");
    Ok(())
}

/// Print the results of game generation
fn print_generation_result(result: &lumina_ai::GameGenerationResult) {
    println!("📊 Generation Statistics:");
    println!("  ⏱️  Total time: {:.2}s", result.stats.total_time);
    println!("  🌐 API calls: {}", result.stats.api_calls);
    println!("  🎯 Tokens used: {}", result.stats.tokens_used);
    println!("  🎪 Entities created: {}", result.stats.entities_created);
    println!("  🎨 Assets generated: {}", result.stats.assets_generated);

    println!("\n🎮 Game Details:");
    println!("  📛 Name: {}", result.scene.metadata.name);
    println!("  📝 Description: {}", result.scene.metadata.description);
    println!("  📐 Resolution: {}x{}", 
             result.scene.metadata.resolution[0], 
             result.scene.metadata.resolution[1]);
    
    if !result.scene.entities.is_empty() {
        println!("  🎭 Entities:");
        for entity in &result.scene.entities {
            let component_count = entity.components.len();
            let tags = if entity.tags.is_empty() { 
                "none".to_string() 
            } else { 
                entity.tags.join(", ") 
            };
            println!("    • {} ({} components, tags: {})", 
                     entity.name, component_count, tags);
        }
    }

    if !result.scene.scripts.is_empty() {
        println!("  🎬 Scripts:");
        for script in &result.scene.scripts {
            println!("    • {} ({} triggers, {} actions)", 
                     script.name, script.triggers.len(), script.actions.len());
        }
    }

    if !result.warnings.is_empty() {
        println!("\n⚠️  Warnings:");
        for warning in &result.warnings {
            println!("    • {}", warning);
        }
    }
}

/// Save a scene description to a JSON file
fn save_scene_to_file(scene: &lumina_ai::SceneDescription, filename: &str) -> Result<()> {
    let json_content = scene.to_json()?;
    std::fs::write(filename, json_content)?;
    Ok(())
}