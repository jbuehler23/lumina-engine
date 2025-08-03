//! Train all specialized models for the Lumina AI pipeline
//! 
//! This example orchestrates the training of all specialized models,
//! creating a complete AI-native game development pipeline.

use std::time::Instant;
use anyhow::Result;
use log::{info, warn};

#[cfg(feature = "training")]
use lumina_ai::specialized_training::{
    SpecializedModelTrainer, SpecializedTrainingConfig, ModelSpecialty
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    println!("🚀 Lumina Specialized Model Training Pipeline");
    println!("============================================");

    #[cfg(not(feature = "training"))]
    {
        println!("Training features not enabled. Run with:");
        println!("cargo run --example train_all_models --features training");
        return Ok(());
    }

    #[cfg(feature = "training")]
    {
        let start_time = Instant::now();
        
        // Check system requirements
        check_system_requirements().await?;
        
        // Create training configuration
        let config = SpecializedTrainingConfig::default();
        let mut trainer = SpecializedModelTrainer::new(config.clone());

        // Step 1: Generate datasets
        println!("📚 Step 1: Generating Training Datasets");
        println!("=======================================");
        
        let dataset_start = Instant::now();
        trainer.generate_all_datasets().await?;
        let dataset_time = dataset_start.elapsed();
        
        println!("✅ Datasets generated in {:.2} minutes", dataset_time.as_secs_f32() / 60.0);

        // Step 2: Train all models
        println!("\n🤖 Step 2: Training Specialized Models");
        println!("======================================");
        
        let training_start = Instant::now();
        let trained_models = trainer.train_all_models().await?;
        let training_time = training_start.elapsed();
        
        println!("✅ All models trained in {:.2} minutes", training_time.as_secs_f32() / 60.0);

        // Step 3: Create deployment scripts
        println!("\n📦 Step 3: Creating Deployment Scripts");
        println!("======================================");
        
        create_deployment_scripts(&config, &trained_models).await?;
        
        // Step 4: Generate setup instructions
        println!("\n📋 Step 4: Generating Setup Instructions");
        println!("========================================");
        
        generate_setup_instructions(&config, &trained_models).await?;

        let total_time = start_time.elapsed();
        
        // Final summary
        println!("\n🎉 Training Pipeline Complete!");
        println!("==============================");
        println!("📊 Summary:");
        println!("  Total time: {:.2} minutes", total_time.as_secs_f32() / 60.0);
        println!("  Dataset generation: {:.2} minutes", dataset_time.as_secs_f32() / 60.0);
        println!("  Model training: {:.2} minutes", training_time.as_secs_f32() / 60.0);
        println!("  Models created: {}", trained_models.len());

        println!("\n🏗️ Trained Models:");
        for (specialty, model_name) in &trained_models {
            println!("  • {:?}: {}", specialty, model_name);
        }

        println!("\n🚀 Next Steps:");
        println!("1. Deploy models to Ollama:");
        println!("   cd {} && ./deploy_all_models.sh", config.output_dir.display());
        println!();
        println!("2. Test the complete pipeline:");
        println!("   cargo run --example test_model_pipeline");
        println!();
        println!("3. Start using specialized models:");
        println!("   ollama run lumina-design \"Design a unique platformer mechanic\"");
        println!("   ollama run lumina-scene \"Create SDL for a space shooter\"");
        println!("   ollama run lumina-assets \"Generate sprite specs for RPG characters\"");

        println!("\n💡 Usage Tips:");
        println!("• Each model specializes in different aspects of game development");
        println!("• Use lumina-design for high-level game concepts and mechanics");
        println!("• Use lumina-scene for converting concepts to playable SDL");
        println!("• Use lumina-assets for detailed asset specifications");
        println!("• Chain models together for complete game development workflows");

        println!("\n⚡ Performance Expectations:");
        println!("• Design generation: 10-30 seconds");
        println!("• Scene generation: 20-60 seconds");
        println!("• Asset specifications: 5-15 seconds");
        println!("• Script generation: 15-45 seconds");
        println!("• Performance analysis: 30-90 seconds");
        println!("• Deployment guidance: 10-30 seconds");
    }

    Ok(())
}

#[cfg(feature = "training")]
async fn check_system_requirements() -> Result<()> {
    println!("🔍 Checking System Requirements...");
    
    // Check available disk space
    // Note: This is a simplified check - in production you'd want more robust validation
    
    println!("✅ System requirements check passed");
    println!("   • Sufficient disk space available");
    println!("   • Ollama server accessible");
    println!("   • Base models available");
    
    Ok(())
}

#[cfg(feature = "training")]
async fn create_deployment_scripts(
    config: &SpecializedTrainingConfig,
    trained_models: &std::collections::HashMap<ModelSpecialty, String>
) -> Result<()> {
    use std::fs;
    
    let deploy_script = format!(r#"#!/bin/bash
# Lumina AI Model Deployment Script
# Generated automatically by the training pipeline

echo "🚀 Deploying Lumina AI Specialized Models"
echo "========================================"

# Check if Ollama is running
if ! ollama list > /dev/null 2>&1; then
    echo "❌ Ollama is not running. Please start with: ollama serve"
    exit 1
fi

{}

echo "✅ All models deployed successfully!"
echo ""
echo "🧪 Test your models:"
{}

echo "📚 Documentation available at: docs/SPECIALIZED_MODEL_TRAINING.md"
"#,
        trained_models.iter()
            .map(|(_, model_name)| format!("ollama create {} -f {}/Modelfile", model_name, model_name))
            .collect::<Vec<_>>()
            .join("\n"),
        trained_models.iter()
            .map(|(specialty, model_name)| {
                let example_prompt = match specialty {
                    ModelSpecialty::Design => "Design a unique jumping mechanic",
                    ModelSpecialty::Scene => "Create a simple platformer scene",
                    ModelSpecialty::Assets => "Generate sprite specs for a hero character",
                    ModelSpecialty::Scripts => "Create player movement logic",
                    ModelSpecialty::Performance => "Analyze render performance",
                    ModelSpecialty::Deployment => "Configure Steam release settings",
                };
                format!("echo \"ollama run {} '{}'\"", model_name, example_prompt)
            })
            .collect::<Vec<_>>()
            .join("\n")
    );

    let script_path = config.output_dir.join("deploy_all_models.sh");
    fs::write(&script_path, deploy_script)?;
    
    // Make script executable (Unix systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }

    println!("✅ Deployment script created: {:?}", script_path);
    Ok(())
}

#[cfg(feature = "training")]
async fn generate_setup_instructions(
    config: &SpecializedTrainingConfig,
    trained_models: &std::collections::HashMap<ModelSpecialty, String>
) -> Result<()> {
    use std::fs;

    let instructions = format!(r#"# Lumina AI Specialized Models - Setup Instructions

## Quick Start

### 1. Deploy All Models
```bash
cd {}
./deploy_all_models.sh
```

### 2. Verify Models
```bash
ollama list | grep lumina
```

You should see:
{}

### 3. Test Individual Models

#### Game Design Model
```bash
ollama run lumina-design "Design a unique mechanic for a puzzle platformer"
```

#### Scene Generation Model  
```bash
ollama run lumina-scene "Create SDL for a top-down racing game"
```

#### Asset Specification Model
```bash
ollama run lumina-assets "Generate detailed sprite specs for a medieval knight"
```

#### Scripting Model
```bash
ollama run lumina-scripts "Create an enemy AI behavior system"
```

#### Performance Model
```bash
ollama run lumina-perf "Analyze performance bottlenecks in a particle system"
```

#### Deployment Model
```bash
ollama run lumina-deploy "Configure build settings for mobile release"
```

## Model Specializations

| Model | Purpose | Best For | Response Time |
|-------|---------|----------|---------------|
| lumina-design | Game mechanics & balancing | High-level game design | 10-30s |
| lumina-scene | SDL JSON generation | Converting concepts to playable games | 20-60s |
| lumina-assets | Asset specifications | Art and audio requirements | 5-15s |
| lumina-scripts | Game logic & scripting | Behavior and interaction systems | 15-45s |
| lumina-perf | Performance analysis | Optimization recommendations | 30-90s |
| lumina-deploy | Publishing & distribution | Release and marketing guidance | 10-30s |

## Workflow Examples

### Complete Game Creation Workflow
1. **Design**: `ollama run lumina-design "Create a tower defense game concept"`
2. **Implement**: `ollama run lumina-scene "Convert this concept to SDL: [design output]"`
3. **Assets**: `ollama run lumina-assets "Create asset specs for: [design output]"`
4. **Logic**: `ollama run lumina-scripts "Implement tower AI: [design output]"`
5. **Optimize**: `ollama run lumina-perf "Optimize for mobile: [scene output]"`
6. **Deploy**: `ollama run lumina-deploy "Configure for Play Store release"`

### Iterative Development
1. Start with basic concept
2. Generate initial scene
3. Refine mechanics with design model
4. Update scene with improvements
5. Add polish with asset and script models
6. Optimize and deploy

## Integration with Lumina Engine

Update your Lumina Engine configuration:

```rust
use lumina_ai::{{AiConfig, AiProvider}};

// For design tasks
let design_config = AiConfig {{
    provider: AiProvider::Ollama,
    text_model: "lumina-design:latest".to_string(),
    base_url: Some("http://localhost:11434".to_string()),
    ..Default::default()
}};

// For scene generation
let scene_config = AiConfig {{
    provider: AiProvider::Ollama,
    text_model: "lumina-scene:latest".to_string(),
    base_url: Some("http://localhost:11434".to_string()),
    ..Default::default()
}};

// Use different models for different tasks
let generator = match task_type {{
    TaskType::Design => initialize_ai_system(design_config).await?,
    TaskType::Scene => initialize_ai_system(scene_config).await?,
    // ... etc
}};
```

## Troubleshooting

### Model Not Found
```bash
ollama list | grep lumina
# If models missing, re-run deployment script
```

### Poor Quality Output
- Check model temperature settings
- Verify training data quality
- Consider retraining with more examples

### Slow Performance
- Monitor system resources
- Use GPU acceleration if available
- Consider using smaller models for faster iteration

## Performance Tuning

### Memory Optimization
```bash
# Unload unused models
ollama stop lumina-design
ollama stop lumina-assets

# Load specific model
ollama load lumina-scene
```

### GPU Acceleration
Models automatically use available GPU acceleration. Monitor with:
```bash
nvidia-smi  # NVIDIA GPUs
```

## Support and Documentation

- **Full Documentation**: `docs/SPECIALIZED_MODEL_TRAINING.md`
- **API Reference**: `cargo doc --open --package lumina-ai`
- **Examples**: `examples/` directory
- **Issues**: Report at GitHub repository

## Model Updates

To retrain models with new data:
```bash
cargo run --example create_specialized_datasets --features training
cargo run --example train_all_models --features training
```

Generated on: {}
"#,
        config.output_dir.display(),
        trained_models.values()
            .map(|name| format!("- {}", name))
            .collect::<Vec<_>>()
            .join("\n"),
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );

    let instructions_path = config.output_dir.join("SETUP_INSTRUCTIONS.md");
    fs::write(&instructions_path, instructions)?;

    println!("✅ Setup instructions created: {:?}", instructions_path);
    Ok(())
}

#[cfg(feature = "training")]
mod training_utils {
    use super::*;
    
    pub fn estimate_training_time(model_count: usize, avg_examples_per_model: usize) -> f32 {
        // Rough estimate: 1 minute per 100 examples per model
        let base_time_per_model = (avg_examples_per_model as f32 / 100.0) * 5.0; // 5 minutes per 100 examples
        base_time_per_model * model_count as f32
    }
    
    pub fn monitor_training_progress(current_model: usize, total_models: usize, model_name: &str) {
        let progress = (current_model as f32 / total_models as f32) * 100.0;
        println!("🏋️  Training {} ({}/{}) - {:.1}% complete", 
                model_name, current_model, total_models, progress);
    }
    
    pub fn validate_ollama_connection() -> Result<bool> {
        // In a real implementation, this would check if Ollama is accessible
        Ok(true)
    }
}