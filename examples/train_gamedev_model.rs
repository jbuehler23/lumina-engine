//! Train a specialized game development model using Ollama
//! 
//! This example demonstrates how to create and fine-tune a model specifically
//! for game development tasks with the Lumina Engine.

use std::path::PathBuf;
use anyhow::Result;
use log::info;

#[cfg(feature = "training")]
use lumina_ai::{
    TrainingConfig, TrainingDatasetBuilder, ModelTrainer,
    TrainingHyperparameters, ModelSpecialization, DataAugmentation,
    DifficultyLevel
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    println!("🎮 Lumina GameDev Model Trainer");
    println!("================================");

    #[cfg(not(feature = "training"))]
    {
        println!("Training features not enabled. Run with:");
        println!("cargo run --example train_gamedev_model --features training");
        return Ok(());
    }

    #[cfg(feature = "training")]
    {
        // Configure training parameters
        let config = TrainingConfig {
            base_model: "llama3".to_string(),
            dataset_path: PathBuf::from("training_dataset.json"),
            output_dir: PathBuf::from("./trained_models"),
            hyperparameters: TrainingHyperparameters {
                learning_rate: 2e-5,
                batch_size: 4,
                epochs: 5,
                max_sequence_length: 4096,
                warmup_steps: 100,
                gradient_accumulation_steps: 8,
            },
            specialization: ModelSpecialization {
                target_genres: vec![
                    "platformer".to_string(),
                    "shooter".to_string(),
                    "puzzle".to_string(),
                    "rpg".to_string(),
                    "racing".to_string(),
                ],
                preferred_components: vec![
                    "Transform".to_string(),
                    "Sprite".to_string(),
                    "Player".to_string(),
                    "Collider".to_string(),
                    "RigidBody".to_string(),
                    "Enemy".to_string(),
                    "Collectible".to_string(),
                ],
                augmentation: DataAugmentation {
                    variation_multiplier: 3,
                    add_noise: false,
                    synthetic_examples: true,
                },
            },
        };

        // Step 1: Generate training dataset
        println!("\n📚 Step 1: Generating Training Dataset");
        println!("=====================================");

        let mut dataset_builder = TrainingDatasetBuilder::new(config.clone());
        
        // Generate comprehensive gamedev dataset
        dataset_builder.generate_gamedev_dataset()?;
        
        // Apply data augmentation
        println!("🔄 Applying data augmentation...");
        dataset_builder.apply_augmentation()?;
        
        // Save dataset
        dataset_builder.save_dataset(&config.dataset_path)?;

        println!("✅ Dataset generation complete!");

        // Step 2: Train the model
        println!("\n🤖 Step 2: Training Specialized Model");
        println!("====================================");

        let trainer = ModelTrainer::new(config.clone(), dataset_builder.examples);
        let trained_model = trainer.train().await?;

        println!("🎉 Model training complete!");
        println!("📊 Training Results:");
        println!("  Model: {}", trained_model.model_name);
        println!("  Base: {}", trained_model.base_model);
        println!("  Examples: {}", trained_model.training_examples);
        println!("  Accuracy: {:.1}%", trained_model.performance_metrics.scene_generation_accuracy * 100.0);
        println!("  Completeness: {:.1}%", trained_model.performance_metrics.component_completeness * 100.0);

        // Step 3: Setup instructions
        println!("\n🚀 Step 3: Setup Instructions");
        println!("=============================");

        println!("To use your trained model:");
        println!();
        println!("1. Install Ollama: https://ollama.com/");
        println!("2. Pull the base model:");
        println!("   ollama pull {}", config.base_model);
        println!();
        println!("3. Create your custom model:");
        println!("   cd {}", config.output_dir.display());
        println!("   ollama create lumina-gamedev -f Modelfile");
        println!();
        println!("4. Test the model:");
        println!("   ollama run lumina-gamedev \"Create a simple platformer game\"");
        println!();
        println!("5. Use with Lumina Engine:");
        println!("   Update your AiConfig to use:");
        println!("   - provider: AiProvider::Ollama");
        println!("   - text_model: \"lumina-gamedev:latest\"");
        println!("   - base_url: \"http://localhost:11434\"");

        // Step 4: Model evaluation recommendations
        println!("\n📈 Step 4: Model Evaluation");
        println!("==========================");

        println!("Test your model with these prompts:");
        
        let test_prompts = vec![
            "Create a simple platformer with coins and enemies",
            "Make a space shooter with power-ups and boss fights",
            "Design a puzzle game with blocks and switches",
            "Build an RPG with combat and NPCs",
            "Create a racing game with multiple tracks",
        ];

        for (i, prompt) in test_prompts.iter().enumerate() {
            println!("  {}. {}", i + 1, prompt);
        }

        println!("\nExpected outputs should include:");
        println!("  ✅ Complete Scene Description Language JSON");
        println!("  ✅ Appropriate entities for the genre");
        println!("  ✅ Proper component relationships");
        println!("  ✅ Game logic scripts");
        println!("  ✅ Win/lose conditions");

        println!("\n🎯 Performance Benchmarks:");
        println!("  • Scene generation should complete in <30 seconds");
        println!("  • Generated games should be immediately playable");
        println!("  • Component accuracy should be >90%");
        println!("  • Genre appropriateness should be >85%");
    }

    Ok(())
}

#[cfg(feature = "training")]
mod training_utils {
    use super::*;
    
    pub fn print_training_progress(epoch: usize, total_epochs: usize, loss: f32) {
        let progress = (epoch as f32 / total_epochs as f32) * 100.0;
        println!("Epoch {}/{} ({:.1}%) - Loss: {:.4}", 
                epoch, total_epochs, progress, loss);
    }
    
    pub fn validate_model_output(output: &str) -> bool {
        // Basic validation of SDL output
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(output) {
            parsed.get("metadata").is_some() && 
            parsed.get("entities").is_some()
        } else {
            false
        }
    }
}