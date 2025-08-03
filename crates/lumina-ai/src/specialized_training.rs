//! Specialized model training for different game development tasks
//! 
//! This module handles training multiple specialized models that each excel
//! at different aspects of game development.

use std::path::PathBuf;
use std::collections::HashMap;
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use log::{info, warn, error};

use crate::training::{TrainingConfig, TrainingExample, DifficultyLevel, ModelTrainer};

/// Specialized model types for different game development tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelSpecialty {
    /// Game design and mechanics
    Design,
    /// Scene Description Language generation
    Scene,
    /// Asset specification and generation
    Assets,
    /// Game logic and scripting
    Scripts,
    /// Performance optimization
    Performance,
    /// Publishing and deployment
    Deployment,
}

/// Configuration for training specialized models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecializedTrainingConfig {
    /// Base output directory for all models
    pub output_dir: PathBuf,
    /// Models to train
    pub models: Vec<ModelConfig>,
    /// Global training settings
    pub global_settings: GlobalTrainingSettings,
}

/// Configuration for a single specialized model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Model specialty
    pub specialty: ModelSpecialty,
    /// Base model to fine-tune
    pub base_model: String,
    /// Model name suffix
    pub model_name: String,
    /// Specific training parameters
    pub training_params: ModelTrainingParams,
    /// System prompt for this specialty
    pub system_prompt: String,
}

/// Training parameters specific to a model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelTrainingParams {
    /// Learning rate
    pub learning_rate: f32,
    /// Number of epochs
    pub epochs: usize,
    /// Temperature for generation
    pub temperature: f32,
    /// Maximum sequence length
    pub max_sequence_length: usize,
    /// Training dataset size target
    pub target_dataset_size: usize,
}

/// Global settings that apply to all models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalTrainingSettings {
    /// Whether to generate augmented data
    pub enable_augmentation: bool,
    /// Augmentation multiplier
    pub augmentation_factor: usize,
    /// Validation split ratio
    pub validation_split: f32,
    /// Enable parallel training
    pub parallel_training: bool,
}

/// Orchestrates training of all specialized models
pub struct SpecializedModelTrainer {
    config: SpecializedTrainingConfig,
    datasets: HashMap<ModelSpecialty, Vec<TrainingExample>>,
}

impl SpecializedModelTrainer {
    /// Create a new specialized model trainer
    pub fn new(config: SpecializedTrainingConfig) -> Self {
        Self {
            config,
            datasets: HashMap::new(),
        }
    }

    /// Generate training datasets for all specialized models
    pub async fn generate_all_datasets(&mut self) -> Result<()> {
        info!("Generating specialized training datasets...");

        for model_config in &self.config.models {
            info!("Generating dataset for {:?} model", model_config.specialty);
            
            let dataset = match model_config.specialty {
                ModelSpecialty::Design => self.generate_design_dataset().await?,
                ModelSpecialty::Scene => self.generate_scene_dataset().await?,
                ModelSpecialty::Assets => self.generate_assets_dataset().await?,
                ModelSpecialty::Scripts => self.generate_scripts_dataset().await?,
                ModelSpecialty::Performance => self.generate_performance_dataset().await?,
                ModelSpecialty::Deployment => self.generate_deployment_dataset().await?,
            };

            self.datasets.insert(model_config.specialty.clone(), dataset);
        }

        info!("All datasets generated successfully");
        Ok(())
    }

    /// Train all specialized models
    pub async fn train_all_models(&self) -> Result<HashMap<ModelSpecialty, String>> {
        info!("Starting training for all specialized models...");
        
        let mut trained_models = HashMap::new();

        for model_config in &self.config.models {
            info!("Training {:?} model...", model_config.specialty);
            
            let model_path = self.train_single_model(model_config).await?;
            trained_models.insert(model_config.specialty.clone(), model_path);
            
            info!("Completed training for {:?} model", model_config.specialty);
        }

        info!("All models trained successfully");
        Ok(trained_models)
    }

    /// Train a single specialized model
    async fn train_single_model(&self, model_config: &ModelConfig) -> Result<String> {
        let dataset = self.datasets.get(&model_config.specialty)
            .ok_or_else(|| anyhow::anyhow!("Dataset not found for {:?}", model_config.specialty))?;

        let training_config = TrainingConfig {
            base_model: model_config.base_model.clone(),
            dataset_path: self.config.output_dir.join(format!("{}_dataset.json", model_config.model_name)),
            output_dir: self.config.output_dir.join(&model_config.model_name),
            hyperparameters: crate::training::TrainingHyperparameters {
                learning_rate: model_config.training_params.learning_rate,
                epochs: model_config.training_params.epochs,
                max_sequence_length: model_config.training_params.max_sequence_length,
                batch_size: 4,
                warmup_steps: 100,
                gradient_accumulation_steps: 8,
            },
            specialization: crate::training::ModelSpecialization::default(),
        };

        let trainer = ModelTrainer::new(training_config, dataset.clone());
        let trained_model = trainer.train().await?;

        // Generate Ollama Modelfile
        self.create_ollama_modelfile(model_config, &trained_model.model_name).await?;

        Ok(trained_model.model_name)
    }

    /// Generate design-focused training dataset
    async fn generate_design_dataset(&self) -> Result<Vec<TrainingExample>> {
        let mut examples = Vec::new();

        // Game mechanics examples
        examples.extend(self.create_mechanics_examples());
        
        // Balancing examples
        examples.extend(self.create_balancing_examples());
        
        // Genre analysis examples
        examples.extend(self.create_genre_examples());
        
        // Player psychology examples
        examples.extend(self.create_psychology_examples());

        Ok(examples)
    }

    /// Generate scene-focused training dataset
    async fn generate_scene_dataset(&self) -> Result<Vec<TrainingExample>> {
        let mut examples = Vec::new();

        // Load existing SDL examples
        examples.extend(self.load_sdl_examples()?);
        
        // Generate component relationship examples
        examples.extend(self.create_component_examples());
        
        // Entity composition patterns
        examples.extend(self.create_entity_patterns());

        Ok(examples)
    }

    /// Generate asset-focused training dataset
    async fn generate_assets_dataset(&self) -> Result<Vec<TrainingExample>> {
        let mut examples = Vec::new();

        // Sprite specification examples
        examples.extend(self.create_sprite_examples());
        
        // Audio specification examples
        examples.extend(self.create_audio_examples());
        
        // UI design examples
        examples.extend(self.create_ui_examples());

        Ok(examples)
    }

    /// Generate script-focused training dataset
    async fn generate_scripts_dataset(&self) -> Result<Vec<TrainingExample>> {
        let mut examples = Vec::new();

        // Event system examples
        examples.extend(self.create_event_examples());
        
        // State machine examples
        examples.extend(self.create_state_machine_examples());
        
        // Logic pattern examples
        examples.extend(self.create_logic_examples());

        Ok(examples)
    }

    /// Generate performance-focused training dataset
    async fn generate_performance_dataset(&self) -> Result<Vec<TrainingExample>> {
        let mut examples = Vec::new();

        // Performance analysis examples
        examples.extend(self.create_performance_examples());
        
        // Optimization strategy examples
        examples.extend(self.create_optimization_examples());

        Ok(examples)
    }

    /// Generate deployment-focused training dataset
    async fn generate_deployment_dataset(&self) -> Result<Vec<TrainingExample>> {
        let mut examples = Vec::new();

        // Platform configuration examples
        examples.extend(self.create_platform_examples());
        
        // Marketing copy examples
        examples.extend(self.create_marketing_examples());

        Ok(examples)
    }

    /// Create Ollama Modelfile for a specialized model
    async fn create_ollama_modelfile(&self, model_config: &ModelConfig, model_name: &str) -> Result<()> {
        let modelfile_content = format!(r#"FROM {}

SYSTEM """{}"""

PARAMETER temperature {}
PARAMETER top_p 0.9
PARAMETER top_k 40
PARAMETER num_predict {}

TEMPLATE """<|system|>
{{{{ .System }}}}<|end|>
<|user|>
{{{{ .Prompt }}}}<|end|>
<|assistant|>
"""
"#, 
            model_config.base_model,
            model_config.system_prompt,
            model_config.training_params.temperature,
            model_config.training_params.max_sequence_length
        );

        let modelfile_path = self.config.output_dir
            .join(&model_config.model_name)
            .join("Modelfile");

        std::fs::create_dir_all(modelfile_path.parent().unwrap())?;
        std::fs::write(&modelfile_path, modelfile_content)?;

        info!("Created Modelfile for {} at {:?}", model_name, modelfile_path);
        Ok(())
    }

    // Training data generation methods
    fn create_mechanics_examples(&self) -> Vec<TrainingExample> {
        vec![
            TrainingExample {
                prompt: "Design a unique jumping mechanic for a platformer".to_string(),
                expected_output: self.create_mock_scene_for_prompt("jumping mechanic"),
                category: "mechanics".to_string(),
                difficulty: DifficultyLevel::Intermediate,
                metadata: HashMap::new(),
            },
            TrainingExample {
                prompt: "Create a combo system for a fighting game".to_string(),
                expected_output: self.create_mock_scene_for_prompt("combo system"),
                category: "mechanics".to_string(),
                difficulty: DifficultyLevel::Advanced,
                metadata: HashMap::new(),
            },
            // Add more mechanics examples...
        ]
    }

    fn create_balancing_examples(&self) -> Vec<TrainingExample> {
        vec![
            TrainingExample {
                prompt: "Balance the damage values for a space shooter".to_string(),
                expected_output: self.create_mock_scene_for_prompt("damage balancing"),
                category: "balancing".to_string(),
                difficulty: DifficultyLevel::Advanced,
                metadata: HashMap::new(),
            },
            // Add more balancing examples...
        ]
    }

    fn create_genre_examples(&self) -> Vec<TrainingExample> {
        vec![
            TrainingExample {
                prompt: "Analyze the core mechanics of roguelike games".to_string(),
                expected_output: self.create_mock_scene_for_prompt("roguelike analysis"),
                category: "genre".to_string(),
                difficulty: DifficultyLevel::Intermediate,
                metadata: HashMap::new(),
            },
            // Add more genre examples...
        ]
    }

    fn create_psychology_examples(&self) -> Vec<TrainingExample> {
        vec![
            TrainingExample {
                prompt: "Design a progression system that keeps players engaged".to_string(),
                expected_output: self.create_mock_scene_for_prompt("progression system"),
                category: "psychology".to_string(),
                difficulty: DifficultyLevel::Expert,
                metadata: HashMap::new(),
            },
            // Add more psychology examples...
        ]
    }

    fn load_sdl_examples(&self) -> Result<Vec<TrainingExample>> {
        // Load existing SDL training data
        Ok(vec![]) // Placeholder
    }

    fn create_component_examples(&self) -> Vec<TrainingExample> {
        vec![
            TrainingExample {
                prompt: "Create a component composition for a moving platform".to_string(),
                expected_output: self.create_mock_scene_for_prompt("moving platform"),
                category: "components".to_string(),
                difficulty: DifficultyLevel::Intermediate,
                metadata: HashMap::new(),
            },
            // Add more component examples...
        ]
    }

    fn create_entity_patterns(&self) -> Vec<TrainingExample> {
        vec![
            TrainingExample {
                prompt: "Design entity patterns for an RTS game".to_string(),
                expected_output: self.create_mock_scene_for_prompt("RTS entities"),
                category: "entities".to_string(),
                difficulty: DifficultyLevel::Advanced,
                metadata: HashMap::new(),
            },
            // Add more entity examples...
        ]
    }

    fn create_sprite_examples(&self) -> Vec<TrainingExample> {
        vec![
            TrainingExample {
                prompt: "Create sprite specifications for a fantasy RPG character".to_string(),
                expected_output: self.create_mock_scene_for_prompt("RPG sprite specs"),
                category: "sprites".to_string(),
                difficulty: DifficultyLevel::Intermediate,
                metadata: HashMap::new(),
            },
            // Add more sprite examples...
        ]
    }

    fn create_audio_examples(&self) -> Vec<TrainingExample> {
        vec![
            TrainingExample {
                prompt: "Design audio specifications for a horror game".to_string(),
                expected_output: self.create_mock_scene_for_prompt("horror audio"),
                category: "audio".to_string(),
                difficulty: DifficultyLevel::Intermediate,
                metadata: HashMap::new(),
            },
            // Add more audio examples...
        ]
    }

    fn create_ui_examples(&self) -> Vec<TrainingExample> {
        vec![
            TrainingExample {
                prompt: "Design UI specifications for a mobile puzzle game".to_string(),
                expected_output: self.create_mock_scene_for_prompt("mobile UI"),
                category: "ui".to_string(),
                difficulty: DifficultyLevel::Intermediate,
                metadata: HashMap::new(),
            },
            // Add more UI examples...
        ]
    }

    fn create_event_examples(&self) -> Vec<TrainingExample> {
        vec![
            TrainingExample {
                prompt: "Create an event system for player interactions".to_string(),
                expected_output: self.create_mock_scene_for_prompt("interaction events"),
                category: "events".to_string(),
                difficulty: DifficultyLevel::Advanced,
                metadata: HashMap::new(),
            },
            // Add more event examples...
        ]
    }

    fn create_state_machine_examples(&self) -> Vec<TrainingExample> {
        vec![
            TrainingExample {
                prompt: "Design a state machine for enemy AI behavior".to_string(),
                expected_output: self.create_mock_scene_for_prompt("AI state machine"),
                category: "state_machines".to_string(),
                difficulty: DifficultyLevel::Advanced,
                metadata: HashMap::new(),
            },
            // Add more state machine examples...
        ]
    }

    fn create_logic_examples(&self) -> Vec<TrainingExample> {
        vec![
            TrainingExample {
                prompt: "Implement logic for a tower defense spawning system".to_string(),
                expected_output: self.create_mock_scene_for_prompt("TD spawning"),
                category: "logic".to_string(),
                difficulty: DifficultyLevel::Advanced,
                metadata: HashMap::new(),
            },
            // Add more logic examples...
        ]
    }

    fn create_performance_examples(&self) -> Vec<TrainingExample> {
        vec![
            TrainingExample {
                prompt: "Analyze performance bottlenecks in a particle system".to_string(),
                expected_output: self.create_mock_scene_for_prompt("particle performance"),
                category: "performance".to_string(),
                difficulty: DifficultyLevel::Expert,
                metadata: HashMap::new(),
            },
            // Add more performance examples...
        ]
    }

    fn create_optimization_examples(&self) -> Vec<TrainingExample> {
        vec![
            TrainingExample {
                prompt: "Optimize memory usage for a large open world game".to_string(),
                expected_output: self.create_mock_scene_for_prompt("memory optimization"),
                category: "optimization".to_string(),
                difficulty: DifficultyLevel::Expert,
                metadata: HashMap::new(),
            },
            // Add more optimization examples...
        ]
    }

    fn create_platform_examples(&self) -> Vec<TrainingExample> {
        vec![
            TrainingExample {
                prompt: "Configure build settings for Steam release".to_string(),
                expected_output: self.create_mock_scene_for_prompt("Steam config"),
                category: "platform".to_string(),
                difficulty: DifficultyLevel::Intermediate,
                metadata: HashMap::new(),
            },
            // Add more platform examples...
        ]
    }

    fn create_marketing_examples(&self) -> Vec<TrainingExample> {
        vec![
            TrainingExample {
                prompt: "Write a compelling Steam store description for an indie platformer".to_string(),
                expected_output: self.create_mock_scene_for_prompt("Steam description"),
                category: "marketing".to_string(),
                difficulty: DifficultyLevel::Intermediate,
                metadata: HashMap::new(),
            },
            // Add more marketing examples...
        ]
    }

    /// Create a mock scene for testing (replace with actual SDL generation)
    fn create_mock_scene_for_prompt(&self, prompt: &str) -> crate::scene_description::SceneDescription {
        use crate::scene_description::*;
        
        SceneDescription::new(format!("Mock scene for: {}", prompt))
    }
}

/// Default configuration for specialized training
impl Default for SpecializedTrainingConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("./specialized_models"),
            models: vec![
                ModelConfig {
                    specialty: ModelSpecialty::Design,
                    base_model: "llama3.1:8b".to_string(),
                    model_name: "lumina-design".to_string(),
                    training_params: ModelTrainingParams {
                        learning_rate: 2e-5,
                        epochs: 5,
                        temperature: 0.8,
                        max_sequence_length: 4096,
                        target_dataset_size: 2000,
                    },
                    system_prompt: include_str!("../prompts/design_system.txt").to_string(),
                },
                ModelConfig {
                    specialty: ModelSpecialty::Scene,
                    base_model: "codellama:7b".to_string(),
                    model_name: "lumina-scene".to_string(),
                    training_params: ModelTrainingParams {
                        learning_rate: 1e-5,
                        epochs: 8,
                        temperature: 0.3,
                        max_sequence_length: 6144,
                        target_dataset_size: 5000,
                    },
                    system_prompt: include_str!("../prompts/scene_system.txt").to_string(),
                },
                ModelConfig {
                    specialty: ModelSpecialty::Assets,
                    base_model: "mistral:7b".to_string(),
                    model_name: "lumina-assets".to_string(),
                    training_params: ModelTrainingParams {
                        learning_rate: 3e-5,
                        epochs: 4,
                        temperature: 0.7,
                        max_sequence_length: 2048,
                        target_dataset_size: 3000,
                    },
                    system_prompt: include_str!("../prompts/assets_system.txt").to_string(),
                },
                ModelConfig {
                    specialty: ModelSpecialty::Scripts,
                    base_model: "codellama:13b".to_string(),
                    model_name: "lumina-scripts".to_string(),
                    training_params: ModelTrainingParams {
                        learning_rate: 1.5e-5,
                        epochs: 6,
                        temperature: 0.4,
                        max_sequence_length: 4096,
                        target_dataset_size: 4000,
                    },
                    system_prompt: include_str!("../prompts/scripts_system.txt").to_string(),
                },
                ModelConfig {
                    specialty: ModelSpecialty::Performance,
                    base_model: "llama3:8b".to_string(),
                    model_name: "lumina-perf".to_string(),
                    training_params: ModelTrainingParams {
                        learning_rate: 2e-5,
                        epochs: 4,
                        temperature: 0.5,
                        max_sequence_length: 3072,
                        target_dataset_size: 1500,
                    },
                    system_prompt: include_str!("../prompts/performance_system.txt").to_string(),
                },
                ModelConfig {
                    specialty: ModelSpecialty::Deployment,
                    base_model: "llama3.1:8b".to_string(),
                    model_name: "lumina-deploy".to_string(),
                    training_params: ModelTrainingParams {
                        learning_rate: 2.5e-5,
                        epochs: 3,
                        temperature: 0.6,
                        max_sequence_length: 2048,
                        target_dataset_size: 1000,
                    },
                    system_prompt: include_str!("../prompts/deployment_system.txt").to_string(),
                },
            ],
            global_settings: GlobalTrainingSettings {
                enable_augmentation: true,
                augmentation_factor: 3,
                validation_split: 0.2,
                parallel_training: false, // Set to true if you have multiple GPUs
            },
        }
    }
}