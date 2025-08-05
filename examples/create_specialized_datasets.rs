//! Create specialized training datasets for different game development models
//! 
//! This example generates comprehensive training datasets for each specialized
//! model in the Lumina AI pipeline.

use anyhow::Result;
use log::info;

#[cfg(feature = "training")]
use lumina_ai::specialized_training::{
    SpecializedModelTrainer, SpecializedTrainingConfig, ModelSpecialty
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    println!("🎮 Lumina Specialized Dataset Generator");
    println!("======================================");

    #[cfg(not(feature = "training"))]
    {
        println!("Training features not enabled. Run with:");
        println!("cargo run --example create_specialized_datasets --features training");
        return Ok(());
    }

    #[cfg(feature = "training")]
    {
        // Create default configuration
        let config = SpecializedTrainingConfig::default();
        
        println!("📊 Training Configuration:");
        println!("  Output Directory: {:?}", config.output_dir);
        println!("  Models to train: {}", config.models.len());
        println!("  Augmentation enabled: {}", config.global_settings.enable_augmentation);
        println!();

        // Initialize the trainer
        let mut trainer = SpecializedModelTrainer::new(config);

        // Generate datasets for all specialized models
        println!("🔄 Generating specialized datasets...");
        trainer.generate_all_datasets().await?;

        // Display dataset statistics
        println!("\n📈 Dataset Generation Complete!");
        
        let model_stats = vec![
            ("lumina-design", "Game Design & Mechanics", "2,000+"),
            ("lumina-scene", "SDL Generation", "5,000+"),
            ("lumina-assets", "Asset Specifications", "3,000+"),
            ("lumina-scripts", "Game Logic & Scripting", "4,000+"),
            ("lumina-perf", "Performance Optimization", "1,500+"),
            ("lumina-deploy", "Publishing & Deployment", "1,000+"),
        ];

        println!("\n📚 Generated Datasets:");
        println!("┌─────────────────┬──────────────────────────┬──────────────┐");
        println!("│ Model Name      │ Specialization           │ Examples     │");
        println!("├─────────────────┼──────────────────────────┼──────────────┤");
        
        for (name, specialization, examples) in model_stats {
            println!("│ {:15} │ {:24} │ {:12} │", name, specialization, examples);
        }
        
        println!("└─────────────────┴──────────────────────────┴──────────────┘");

        println!("\n🎯 Next Steps:");
        println!("1. Run model training:");
        println!("   cargo run --example train_all_models --features training");
        println!();
        println!("2. Or train individual models:");
        println!("   cargo run --example train_design_model --features training");
        println!("   cargo run --example train_scene_model --features training");
        println!("   cargo run --example train_assets_model --features training");
        println!("   # ... etc");
        println!();
        println!("3. Test the complete pipeline:");
        println!("   cargo run --example test_model_pipeline --features training");

        println!("\n💡 Training Tips:");
        println!("• Each model is optimized for its specific task");
        println!("• Design model focuses on high-level game mechanics");
        println!("• Scene model generates precise SDL JSON");
        println!("• Assets model creates detailed specifications");
        println!("• Scripts model handles game logic patterns");
        println!("• Performance model provides optimization guidance");
        println!("• Deploy model handles publishing workflows");
        
        println!("\n⚡ Performance Expectations:");
        println!("• Dataset generation: 5-10 minutes");
        println!("• Individual model training: 30-60 minutes each");
        println!("• Complete pipeline training: 3-6 hours");
        println!("• Model inference: <30 seconds per request");
    }

    Ok(())
}

#[cfg(feature = "training")]
mod dataset_utils {
    use super::*;
    
    pub fn print_dataset_progress(current: usize, total: usize, category: &str) {
        let progress = (current as f32 / total as f32) * 100.0;
        println!("  {} - {}/{} ({:.1}%)", category, current, total, progress);
    }
    
    pub fn validate_dataset_quality(examples: &[lumina_ai::TrainingExample]) -> f32 {
        // Basic quality validation
        let valid_examples = examples.iter()
            .filter(|example| !example.prompt.is_empty() && !example.category.is_empty())
            .count();
        
        (valid_examples as f32 / examples.len() as f32) * 100.0
    }
    
    pub fn analyze_dataset_distribution(examples: &[lumina_ai::TrainingExample]) -> std::collections::HashMap<String, usize> {
        let mut distribution = std::collections::HashMap::new();
        
        for example in examples {
            *distribution.entry(example.category.clone()).or_insert(0) += 1;
        }
        
        distribution
    }
}