//! Test the model naming fix
//! 
//! This example tests that specialized models get the correct names

use std::path::PathBuf;
use anyhow::Result;

#[cfg(feature = "training")]
use lumina_ai::specialized_training::{
    SpecializedModelTrainer, SpecializedTrainingConfig, ModelSpecialty,
    ModelConfig, ModelTrainingParams, GlobalTrainingSettings
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    println!("🧪 Testing Model Naming Fix");
    println!("===========================");

    #[cfg(not(feature = "training"))]
    {
        println!("Training features not enabled. Run with:");
        println!("cargo run --example test_model_naming --features training");
        return Ok(());
    }

    #[cfg(feature = "training")]
    {
        // Create a minimal test configuration with just one model
        let config = SpecializedTrainingConfig {
            output_dir: PathBuf::from("./test_model_naming"),
            models: vec![
                ModelConfig {
                    specialty: ModelSpecialty::Design,
                    base_model: "llama3.1:8b".to_string(),
                    model_name: "test-lumina-design".to_string(),
                    training_params: ModelTrainingParams {
                        learning_rate: 2e-5,
                        epochs: 1,  // Just 1 epoch for testing
                        temperature: 0.8,
                        max_sequence_length: 512,  // Smaller for testing
                        target_dataset_size: 10,  // Very small dataset
                    },
                    system_prompt: "You are a game design assistant.".to_string(),
                },
            ],
            global_settings: GlobalTrainingSettings {
                enable_augmentation: false,
                augmentation_factor: 1,
                validation_split: 0.1,
                parallel_training: false,
            },
        };

        let mut trainer = SpecializedModelTrainer::new(config.clone());

        // Test dataset generation
        println!("📚 Generating test dataset...");
        trainer.generate_all_datasets().await?;
        println!("✅ Dataset generated");

        // Test model training (this will show if the naming works)
        println!("🤖 Training test model...");
        let trained_models = trainer.train_all_models().await?;
        
        println!("🎉 Training complete!");
        for (specialty, model_name) in &trained_models {
            println!("  • {:?}: {}", specialty, model_name);
            
            // Verify the name is correct
            if model_name.starts_with("test-lumina-") {
                println!("    ✅ Correct naming! Model name: {}", model_name);
            } else {
                println!("    ❌ Wrong naming! Expected 'test-lumina-*', got: {}", model_name);
            }
        }

        // Clean up test files
        println!("🧹 Cleaning up test files...");
        std::fs::remove_dir_all("./test_model_naming").ok();
        println!("✅ Cleanup complete");
    }

    Ok(())
}