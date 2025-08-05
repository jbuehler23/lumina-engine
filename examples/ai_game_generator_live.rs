//! AI Game Generator with Live Preview
//! 
//! This example demonstrates the complete AI-native workflow:
//! Text prompt → AI generation → Immediate playable game

use std::io::{self, Write};
use anyhow::Result;
use log::{info, error};

use lumina_ai::{
    AiConfig, AiProvider, GamePrompt, GameGenerationRequest, ComplexityLevel,
    initialize_ai_system, LivePreviewSystem, PreviewConfig
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    println!("🎮 Lumina AI Game Generator with Live Preview");
    println!("=============================================");
    println!();
    
    // Check if specialized models are available
    let available_models = check_available_models().await;
    display_available_models(&available_models);

    // Configure AI system (prioritize specialized models if available)
    let config = create_ai_config(&available_models);
    
    println!("🤖 Initializing AI system...");
    let mut game_generator = match initialize_ai_system(config.clone()).await {
        Ok(generator) => {
            println!("✅ AI system initialized successfully!");
            generator
        }
        Err(e) => {
            error!("Failed to initialize AI system: {}", e);
            println!("❌ AI system initialization failed. Please check your configuration.");
            return Ok(());
        }
    };

    // Initialize live preview system
    let preview_config = PreviewConfig {
        auto_preview: true,
        debug_overlay: true,
        target_fps: 60,
        max_entities: 100,
        window_size: (1280, 720),
    };
    
    println!("🎬 Initializing live preview system...");
    let mut preview_system = LivePreviewSystem::new(preview_config);
    println!("✅ Live preview system ready!");
    println!();

    // Interactive mode
    loop {
        println!("🎯 AI-Native Game Creation");
        println!("==========================");
        println!();
        println!("Choose an option:");
        println!("1. 🚀 Quick game generation (with examples)");
        println!("2. ✏️  Custom game prompt");
        println!("3. 🔄 Refine existing game");
        println!("4. 🧪 Test specialized models");
        println!("5. ❌ Exit");
        println!();
        
        print!("Enter your choice (1-5): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();

        match choice {
            "1" => quick_game_generation(&mut game_generator, &mut preview_system).await?,
            "2" => custom_game_prompt(&mut game_generator, &mut preview_system).await?,
            "3" => refine_existing_game(&mut game_generator, &mut preview_system).await?,
            "4" => test_specialized_models().await?,
            "5" => break,
            _ => println!("Invalid choice. Please enter 1-5."),
        }
        
        println!();
    }

    println!("👋 Thanks for using Lumina AI Game Generator!");
    Ok(())
}

/// Quick game generation with predefined examples
async fn quick_game_generation(
    generator: &mut lumina_ai::GameGenerator,
    preview: &mut LivePreviewSystem
) -> Result<()> {
    println!("\n🚀 Quick Game Generation");
    println!("========================");
    
    let examples = vec![
        ("Simple Platformer", "Create a basic platformer where the player jumps on platforms to collect coins", ComplexityLevel::Simple),
        ("Space Shooter", "Make a top-down space shooter with enemies, bullets, and power-ups", ComplexityLevel::Medium),
        ("Puzzle Game", "Design a block-pushing puzzle game with multiple levels", ComplexityLevel::Medium),
        ("Racing Game", "Create a top-down racing game with checkpoints and obstacles", ComplexityLevel::Simple),
        ("Tower Defense", "Build a tower defense game with different tower types and enemy waves", ComplexityLevel::Complex),
    ];

    for (i, (name, description, complexity)) in examples.iter().enumerate() {
        println!("{}. {} - {}", i + 1, name, description);
    }
    
    print!("\nChoose an example (1-{}): ", examples.len());
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    if let Ok(choice) = input.trim().parse::<usize>() {
        if choice > 0 && choice <= examples.len() {
            let (name, description, complexity) = &examples[choice - 1];
            
            println!("\n🎮 Generating: {}", name);
            println!("📝 Description: {}", description);
            
            let prompt = GamePrompt::new(description)
                .with_complexity(complexity.clone())
                .with_visual_style("pixel art");

            let request = GameGenerationRequest::new(prompt);
            
            generate_and_preview(generator, preview, request, name).await?;
        } else {
            println!("Invalid choice. Please select a number between 1 and {}.", examples.len());
        }
    } else {
        println!("Invalid input. Please enter a number.");
    }

    Ok(())
}

/// Custom game prompt input
async fn custom_game_prompt(
    generator: &mut lumina_ai::GameGenerator,
    preview: &mut LivePreviewSystem
) -> Result<()> {
    println!("\n✏️  Custom Game Creation");
    println!("========================");
    
    print!("Enter your game description: ");
    io::stdout().flush()?;
    
    let mut description = String::new();
    io::stdin().read_line(&mut description)?;
    let description = description.trim();
    
    if description.is_empty() {
        println!("No description provided.");
        return Ok(());
    }

    // Optional: Get additional parameters
    print!("Genre (optional, press Enter to skip): ");
    io::stdout().flush()?;
    
    let mut genre = String::new();
    io::stdin().read_line(&mut genre)?;
    let genre = genre.trim();
    
    print!("Visual style (optional, default: pixel art): ");
    io::stdout().flush()?;
    
    let mut style = String::new();
    io::stdin().read_line(&mut style)?;
    let style = if style.trim().is_empty() { "pixel art" } else { style.trim() };

    println!("\nComplexity level:");
    println!("1. Simple");
    println!("2. Medium");
    println!("3. Complex");
    print!("Choose complexity (1-3, default: 2): ");
    io::stdout().flush()?;
    
    let mut complexity_input = String::new();
    io::stdin().read_line(&mut complexity_input)?;
    let complexity = match complexity_input.trim() {
        "1" => ComplexityLevel::Simple,
        "3" => ComplexityLevel::Complex,
        _ => ComplexityLevel::Medium,
    };

    println!("\n🎮 Generating your custom game...");
    println!("📝 Description: {}", description);
    if !genre.is_empty() {
        println!("🎯 Genre: {}", genre);
    }
    println!("🎨 Style: {}", style);
    println!("⚙️  Complexity: {:?}", complexity);

    let mut prompt = GamePrompt::new(description)
        .with_complexity(complexity)
        .with_visual_style(style);

    if !genre.is_empty() {
        prompt = prompt.with_genre(genre);
    }

    let request = GameGenerationRequest::new(prompt);
    
    generate_and_preview(generator, preview, request, "Custom Game").await?;

    Ok(())
}

/// Refine an existing game
async fn refine_existing_game(
    generator: &mut lumina_ai::GameGenerator,
    preview: &mut LivePreviewSystem
) -> Result<()> {
    println!("\n🔄 Game Refinement");
    println!("==================");
    
    // Check if there's a current scene
    let current_state = preview.get_state().await;
    
    match current_state.current_scene {
        Some(scene) => {
            println!("📝 Current game: {}", scene.metadata.name);
            println!("📖 Description: {}", scene.metadata.description);
            println!();
            
            print!("Enter your refinement request (e.g., 'make the player faster', 'add more enemies'): ");
            io::stdout().flush()?;
            
            let mut refinement = String::new();
            io::stdin().read_line(&mut refinement)?;
            let refinement = refinement.trim();
            
            if refinement.is_empty() {
                println!("No refinement provided.");
                return Ok(());
            }

            println!("\n🔧 Refining game: {}", refinement);
            
            match generator.refine_game(scene, refinement).await {
                Ok(refined_scene) => {
                    println!("✅ Game refinement complete!");
                    
                    println!("\n🎬 Previewing refined game...");
                    match preview.update_preview(refined_scene).await {
                        Ok(result) => {
                            print_preview_result(&result);
                            if result.success {
                                println!("🎮 Refined game is now running! Check the preview window.");
                            }
                        }
                        Err(e) => {
                            error!("Preview failed: {}", e);
                            println!("❌ Failed to preview refined game: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Refinement failed: {}", e);
                    println!("❌ Failed to refine game: {}", e);
                }
            }
        }
        None => {
            println!("❌ No game currently loaded. Please generate a game first.");
        }
    }

    Ok(())
}

/// Test specialized models individually
async fn test_specialized_models() -> Result<()> {
    println!("\n🧪 Testing Specialized Models");
    println!("=============================");
    
    let models = vec![
        ("lumina-design", "Design a unique jumping mechanic for a puzzle platformer"),
        ("lumina-scene", "Create SDL for a simple space shooter"),
        ("lumina-assets", "Generate sprite specifications for a medieval knight"),
        ("lumina-scripts", "Create enemy AI behavior for a platformer"),
        ("lumina-perf", "Analyze performance of a particle system"),
        ("lumina-deploy", "Configure Steam store settings for an indie game"),
    ];

    for (i, (model, prompt)) in models.iter().enumerate() {
        println!("{}. {} - {}", i + 1, model, prompt);
    }
    
    print!("\nChoose a model to test (1-{}, 0 to skip): ", models.len());
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    if let Ok(choice) = input.trim().parse::<usize>() {
        if choice > 0 && choice <= models.len() {
            let (model, prompt) = &models[choice - 1];
            
            println!("\n🧪 Testing {} with prompt:", model);
            println!("\"{}\"", prompt);
            println!("\nNote: This requires the specialized models to be trained and deployed.");
            println!("Run: cargo run --example train_all_models --features training");
            
            // Here you would actually test the individual model
            // For now, we'll just show what the command would be
            println!("\nTo test manually, run:");
            println!("ollama run {} \"{}\"", model, prompt);
        } else if choice != 0 {
            println!("Invalid choice.");
        }
    }

    Ok(())
}

/// Generate game and preview immediately
async fn generate_and_preview(
    generator: &mut lumina_ai::GameGenerator,
    preview: &mut LivePreviewSystem,
    request: GameGenerationRequest,
    game_name: &str
) -> Result<()> {
    let start_time = std::time::Instant::now();
    
    println!("\n⏳ Generating {}...", game_name);
    
    match generator.generate_game(request).await {
        Ok(result) => {
            let generation_time = start_time.elapsed();
            
            println!("✅ Game generation complete in {:.2}s!", generation_time.as_secs_f32());
            print_generation_result(&result);
            
            println!("\n🎬 Starting live preview...");
            
            match preview.preview_generation(result).await {
                Ok(preview_result) => {
                    print_preview_result(&preview_result);
                    
                    if preview_result.success {
                        println!("\n🎮 Game is now running in the preview window!");
                        println!("   • Use WASD or Arrow keys to move");
                        println!("   • Press Space to jump/shoot");
                        println!("   • Press Escape to close preview");
                        println!("\n💡 Try refining the game with option 3 in the main menu!");
                    }
                }
                Err(e) => {
                    error!("Preview failed: {}", e);
                    println!("❌ Failed to start preview: {}", e);
                }
            }
        }
        Err(e) => {
            error!("Generation failed: {}", e);
            println!("❌ Game generation failed: {}", e);
        }
    }

    Ok(())
}

/// Print generation result statistics
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
    
    if !result.warnings.is_empty() {
        println!("\n⚠️  Warnings:");
        for warning in &result.warnings {
            println!("    • {}", warning);
        }
    }
}

/// Print preview result information
fn print_preview_result(result: &lumina_ai::PreviewResult) {
    if result.success {
        println!("📊 Preview Metrics:");
        println!("  ⏱️  Load time: {:.2}ms", result.metrics.load_time.as_millis());
        println!("  🎪 Entities loaded: {}", result.metrics.entities_loaded);
        println!("  🖼️  Target FPS: {}", result.metrics.fps);
        
        if let Some(ref load_result) = result.load_result {
            println!("  📜 Scripts loaded: {}", load_result.scripts_loaded);
        }
    }
    
    if !result.messages.is_empty() {
        println!("📝 Messages:");
        for message in &result.messages {
            println!("    • {}", message);
        }
    }
}

/// Check which AI models are available
async fn check_available_models() -> Vec<String> {
    // This would actually check which Ollama models are available
    // For now, return a default list
    vec![
        "lumina-scene".to_string(),
        "lumina-design".to_string(),
        "llama3".to_string(),
    ]
}

/// Display available models to the user
fn display_available_models(models: &[String]) {
    println!("🤖 Available AI Models:");
    for model in models {
        if model.starts_with("lumina-") {
            println!("  ✅ {} (specialized)", model);
        } else {
            println!("  🔧 {} (general)", model);
        }
    }
    println!();
}

/// Create AI configuration based on available models
fn create_ai_config(available_models: &[String]) -> AiConfig {
    // Prioritize specialized models if available
    let text_model = if available_models.contains(&"lumina-scene".to_string()) {
        "lumina-scene:latest".to_string()
    } else if available_models.contains(&"llama3".to_string()) {
        "llama3:latest".to_string()
    } else {
        "llama3".to_string() // Fallback
    };

    AiConfig {
        provider: AiProvider::Ollama,
        api_key: None,
        base_url: Some("http://localhost:11434".to_string()),
        text_model,
        image_model: None,
        max_tokens: 4096,
        temperature: 0.7,
    }
}