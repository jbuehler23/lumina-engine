//! Enhanced training with parallelization, monitoring, and validation
//! 
//! This module provides advanced training capabilities with:
//! - Parallel model training
//! - Real-time progress monitoring
//! - Dataset validation
//! - ETA calculation
//! - Comprehensive logging

use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use log::{info, warn, error, debug};
use tokio::sync::Semaphore;

use crate::training::{TrainingExample, DifficultyLevel, ModelTrainer, TrainingConfig};
use crate::specialized_training::{
    SpecializedTrainingConfig, ModelSpecialty, ModelConfig
};

/// Enhanced training orchestrator with monitoring and parallelization
pub struct EnhancedTrainingOrchestrator {
    config: SpecializedTrainingConfig,
    datasets: HashMap<ModelSpecialty, Vec<TrainingExample>>,
    progress_tracker: Arc<Mutex<TrainingProgressTracker>>,
}

/// Training progress tracking and monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingProgressTracker {
    pub total_models: usize,
    pub completed_models: usize,
    pub current_model: Option<ModelSpecialty>,
    pub model_progress: HashMap<ModelSpecialty, ModelProgress>,
    pub start_time: Option<std::time::SystemTime>,
    pub estimated_completion: Option<std::time::SystemTime>,
    pub dataset_validation_results: HashMap<ModelSpecialty, DatasetValidationResult>,
}

/// Progress for individual model training
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProgress {
    pub stage: TrainingStage,
    pub current_batch: usize,
    pub total_batches: usize,
    pub examples_processed: usize,
    pub total_examples: usize,
    pub start_time: std::time::SystemTime,
    pub estimated_completion: Option<std::time::SystemTime>,
    pub errors: Vec<String>,
}

/// Training stages for detailed progress tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrainingStage {
    NotStarted,
    DatasetValidation,
    DatasetPreparation,
    ModelCreation,
    BatchProcessing,
    ModelFinalization,
    Completed,
    Failed(String),
}

/// Dataset validation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetValidationResult {
    pub is_valid: bool,
    pub total_examples: usize,
    pub valid_examples: usize,
    pub validation_errors: Vec<ValidationError>,
    pub structure_analysis: DatasetStructureAnalysis,
}

/// Individual validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub example_index: usize,
    pub error_type: ValidationErrorType,
    pub message: String,
}

/// Types of validation errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationErrorType {
    InvalidJson,
    MissingField,
    InvalidFieldType,
    EmptyContent,
    TooLarge,
    InvalidStructure,
}

/// Analysis of dataset structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetStructureAnalysis {
    pub categories: HashMap<String, usize>,
    pub difficulty_distribution: HashMap<String, usize>,
    pub average_prompt_length: f64,
    pub average_output_length: f64,
    pub min_output_length: usize,
    pub max_output_length: usize,
    pub json_structure_consistency: f64,
}

impl EnhancedTrainingOrchestrator {
    /// Create a new enhanced training orchestrator
    pub fn new(config: SpecializedTrainingConfig) -> Self {
        let progress_tracker = Arc::new(Mutex::new(TrainingProgressTracker {
            total_models: config.models.len(),
            completed_models: 0,
            current_model: None,
            model_progress: HashMap::new(),
            start_time: None,
            estimated_completion: None,
            dataset_validation_results: HashMap::new(),
        }));

        Self {
            config,
            datasets: HashMap::new(),
            progress_tracker,
        }
    }

    /// Generate and validate all datasets
    pub async fn generate_and_validate_datasets(&mut self) -> Result<()> {
        info!("🔍 Starting dataset generation and validation");
        self.update_progress(|tracker| {
            tracker.start_time = Some(std::time::SystemTime::now());
        });

        // Generate datasets for each model
        for model_config in &self.config.models {
            info!("📊 Generating dataset for {:?} model", model_config.specialty);
            
            self.update_model_progress(&model_config.specialty, |progress| {
                progress.stage = TrainingStage::DatasetValidation;
                progress.start_time = std::time::SystemTime::now();
            });

            let dataset = self.generate_dataset_for_specialty(&model_config.specialty).await?;
            
            // Validate dataset
            info!("✅ Validating dataset for {:?} model", model_config.specialty);
            let validation_result = self.validate_dataset(&dataset, &model_config.specialty).await?;
            
            self.update_progress(|tracker| {
                tracker.dataset_validation_results.insert(
                    model_config.specialty.clone(), 
                    validation_result.clone()
                );
            });

            if !validation_result.is_valid {
                warn!("⚠️  Dataset validation failed for {:?}: {} errors", 
                      model_config.specialty, validation_result.validation_errors.len());
                let max_errors_to_show = std::cmp::min(5, validation_result.validation_errors.len());
                for error in &validation_result.validation_errors[..max_errors_to_show] {
                    warn!("   Error #{}: {} - {}", error.example_index, 
                          format!("{:?}", error.error_type), error.message);
                }
                if validation_result.validation_errors.len() > 5 {
                    warn!("   ... and {} more errors", validation_result.validation_errors.len() - 5);
                }
            } else {
                info!("✅ Dataset validation passed for {:?}", model_config.specialty);
            }

            // Save dataset to file
            let dataset_path = self.config.output_dir.join(format!("{}_dataset.jsonl", model_config.model_name));
            self.save_dataset_jsonl(&dataset, &dataset_path).await?;
            info!("💾 Saved {} examples to {:?}", dataset.len(), dataset_path);

            self.datasets.insert(model_config.specialty.clone(), dataset);
        }

        self.print_validation_summary();
        Ok(())
    }

    /// Train all models with parallelization and monitoring
    pub async fn train_all_models_parallel(&self) -> Result<HashMap<ModelSpecialty, String>> {
        info!("🚀 Starting parallel model training");
        
        let max_concurrent = if self.config.global_settings.parallel_training {
            std::cmp::min(self.config.models.len(), 3) // Max 3 concurrent to avoid resource issues
        } else {
            1
        };

        info!("⚡ Training up to {} models concurrently", max_concurrent);
        let semaphore = Arc::new(Semaphore::new(max_concurrent));
        
        let mut tasks = Vec::new();
        let trained_models = Arc::new(Mutex::new(HashMap::new()));

        for model_config in &self.config.models {
            let semaphore = semaphore.clone();
            let trained_models = trained_models.clone();
            let progress_tracker = self.progress_tracker.clone();
            let model_config = model_config.clone();
            let dataset = self.datasets.get(&model_config.specialty).unwrap().clone();
            let output_dir = self.config.output_dir.clone();

            let task = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                
                info!("🏋️  Starting training for {:?} model", model_config.specialty);
                
                // Update progress
                {
                    let mut tracker = progress_tracker.lock().unwrap();
                    tracker.current_model = Some(model_config.specialty.clone());
                    tracker.model_progress.insert(
                        model_config.specialty.clone(),
                        ModelProgress {
                            stage: TrainingStage::DatasetPreparation,
                            current_batch: 0,
                            total_batches: (dataset.len() + 9) / 10, // Batches of 10
                            examples_processed: 0,
                            total_examples: dataset.len(),
                            start_time: std::time::SystemTime::now(),
                            estimated_completion: None,
                            errors: Vec::new(),
                        }
                    );
                }

                match Self::train_single_model_enhanced(
                    &model_config, 
                    &dataset, 
                    &output_dir,
                    progress_tracker.clone()
                ).await {
                    Ok(model_name) => {
                        info!("✅ Completed training for {:?} model: {}", model_config.specialty, model_name);
                        
                        // Update completion
                        {
                            let mut tracker = progress_tracker.lock().unwrap();
                            tracker.completed_models += 1;
                            if let Some(progress) = tracker.model_progress.get_mut(&model_config.specialty) {
                                progress.stage = TrainingStage::Completed;
                            }
                        }

                        {
                            let mut models = trained_models.lock().unwrap();
                            models.insert(model_config.specialty.clone(), model_name);
                        }
                    },
                    Err(e) => {
                        error!("❌ Failed to train {:?} model: {}", model_config.specialty, e);
                        
                        // Update error status
                        {
                            let mut tracker = progress_tracker.lock().unwrap();
                            if let Some(progress) = tracker.model_progress.get_mut(&model_config.specialty) {
                                progress.stage = TrainingStage::Failed(e.to_string());
                                progress.errors.push(e.to_string());
                            }
                        }
                    }
                }
            });

            tasks.push(task);
        }

        // Start progress monitoring task
        let progress_monitor = self.start_progress_monitor();

        // Wait for all training tasks to complete
        for task in tasks {
            task.await.context("Training task failed")?;
        }

        // Stop progress monitoring
        progress_monitor.abort();

        let final_models = trained_models.lock().unwrap().clone();
        
        self.update_progress(|tracker| {
            tracker.estimated_completion = Some(std::time::SystemTime::now());
        });

        self.print_training_summary(&final_models);
        
        Ok(final_models)
    }

    /// Train a single model with enhanced monitoring
    async fn train_single_model_enhanced(
        model_config: &ModelConfig,
        dataset: &[TrainingExample],
        output_dir: &PathBuf,
        progress_tracker: Arc<Mutex<TrainingProgressTracker>>,
    ) -> Result<String> {
        // Update stage
        {
            let mut tracker = progress_tracker.lock().unwrap();
            if let Some(progress) = tracker.model_progress.get_mut(&model_config.specialty) {
                progress.stage = TrainingStage::ModelCreation;
            }
        }

        let training_config = TrainingConfig {
            base_model: model_config.base_model.clone(),
            dataset_path: output_dir.join(format!("{}_dataset.json", model_config.model_name)),
            output_dir: output_dir.join(&model_config.model_name),
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

        // Create trainer with enhanced monitoring
        let trainer = ModelTrainer::new_with_name(
            training_config, 
            dataset.to_vec(), 
            model_config.model_name.clone()
        );

        // Update stage to batch processing
        {
            let mut tracker = progress_tracker.lock().unwrap();
            if let Some(progress) = tracker.model_progress.get_mut(&model_config.specialty) {
                progress.stage = TrainingStage::BatchProcessing;
            }
        }

        let trained_model = trainer.train().await?;

        // Update stage to finalization
        {
            let mut tracker = progress_tracker.lock().unwrap();
            if let Some(progress) = tracker.model_progress.get_mut(&model_config.specialty) {
                progress.stage = TrainingStage::ModelFinalization;
            }
        }

        Ok(trained_model.model_name)
    }

    /// Validate a dataset
    async fn validate_dataset(
        &self, 
        dataset: &[TrainingExample], 
        specialty: &ModelSpecialty
    ) -> Result<DatasetValidationResult> {
        debug!("🔍 Validating dataset for {:?}", specialty);
        
        let mut validation_errors = Vec::new();
        let mut valid_examples = 0;
        let mut categories = HashMap::new();
        let mut difficulty_distribution = HashMap::new();
        let mut prompt_lengths = Vec::new();
        let mut output_lengths = Vec::new();
        let mut json_structures = Vec::new();

        for (index, example) in dataset.iter().enumerate() {
            let mut example_valid = true;

            // Check if prompt is not empty
            if example.prompt.trim().is_empty() {
                validation_errors.push(ValidationError {
                    example_index: index,
                    error_type: ValidationErrorType::EmptyContent,
                    message: "Prompt is empty".to_string(),
                });
                example_valid = false;
            }

            // Check if expected_output is valid JSON
            match serde_json::from_str::<serde_json::Value>(&example.expected_output) {
                Ok(json_value) => {
                    json_structures.push(self.analyze_json_structure(&json_value));
                    output_lengths.push(example.expected_output.len());
                },
                Err(e) => {
                    validation_errors.push(ValidationError {
                        example_index: index,
                        error_type: ValidationErrorType::InvalidJson,
                        message: format!("Invalid JSON: {}", e),
                    });
                    example_valid = false;
                }
            }

            // Check output size (not too large)
            if example.expected_output.len() > 50000 { // 50KB limit
                validation_errors.push(ValidationError {
                    example_index: index,
                    error_type: ValidationErrorType::TooLarge,
                    message: format!("Output too large: {} bytes", example.expected_output.len()),
                });
                example_valid = false;
            }

            if example_valid {
                valid_examples += 1;
            }

            // Collect statistics
            *categories.entry(example.category.clone()).or_insert(0) += 1;
            *difficulty_distribution.entry(format!("{:?}", example.difficulty)).or_insert(0) += 1;
            prompt_lengths.push(example.prompt.len());
        }

        // Calculate structure consistency
        let json_structure_consistency = if json_structures.is_empty() {
            0.0
        } else {
            self.calculate_structure_consistency(&json_structures)
        };

        let structure_analysis = DatasetStructureAnalysis {
            categories,
            difficulty_distribution,
            average_prompt_length: prompt_lengths.iter().sum::<usize>() as f64 / prompt_lengths.len() as f64,
            average_output_length: if output_lengths.is_empty() { 0.0 } else { 
                output_lengths.iter().sum::<usize>() as f64 / output_lengths.len() as f64 
            },
            min_output_length: output_lengths.iter().min().copied().unwrap_or(0),
            max_output_length: output_lengths.iter().max().copied().unwrap_or(0),
            json_structure_consistency,
        };

        Ok(DatasetValidationResult {
            is_valid: validation_errors.is_empty(),
            total_examples: dataset.len(),
            valid_examples,
            validation_errors,
            structure_analysis,
        })
    }

    /// Generate dataset for a specific specialty using comprehensive training data
    async fn generate_dataset_for_specialty(&self, specialty: &ModelSpecialty) -> Result<Vec<TrainingExample>> {
        info!("Generating comprehensive dataset for {:?} specialty", specialty);
        
        // Use the comprehensive dataset builder from training.rs
        let dummy_config = crate::training::TrainingConfig {
            base_model: "dummy".to_string(),
            dataset_path: std::path::PathBuf::new(),
            output_dir: std::path::PathBuf::new(),
            hyperparameters: crate::training::TrainingHyperparameters::default(),
            specialization: crate::training::ModelSpecialization::default(),
        };
        
        let mut builder = crate::training::TrainingDatasetBuilder::new(dummy_config);
        
        // Generate comprehensive dataset based on specialty
        match specialty {
            ModelSpecialty::Design => {
                builder.add_game_design_patterns()?;
                builder.add_comprehensive_game_mechanics()?;
            },
            ModelSpecialty::Scene => {
                builder.add_platformer_examples()?;
                builder.add_shooter_examples()?;
                builder.add_puzzle_examples()?;
                builder.add_rpg_examples()?;
                builder.add_racing_examples()?;
                builder.add_advanced_sdl_patterns()?;
            },
            ModelSpecialty::Assets => {
                builder.add_asset_specification_examples()?;
            },
            ModelSpecialty::Scripts => {
                builder.add_scripting_patterns_examples()?;
            },
            ModelSpecialty::Performance => {
                builder.add_performance_optimization_examples()?;
            },
            ModelSpecialty::Deployment => {
                builder.add_deployment_strategies_examples()?;
            },
        }
        
        let examples = builder.get_examples().clone();
        info!("Generated {} examples for {:?} specialty", examples.len(), specialty);
        Ok(examples)
    }

    /// Save dataset in JSONL format with proper formatting
    async fn save_dataset_jsonl(&self, dataset: &[TrainingExample], path: &PathBuf) -> Result<()> {
        std::fs::create_dir_all(path.parent().unwrap())?;
        
        let mut content = String::new();
        for example in dataset {
            let json_line = serde_json::to_string(example)
                .context("Failed to serialize training example")?;
            content.push_str(&json_line);
            content.push('\n');
        }
        
        std::fs::write(path, content)
            .context("Failed to write dataset file")?;
        
        Ok(())
    }

    /// Start progress monitoring task
    fn start_progress_monitor(&self) -> tokio::task::JoinHandle<()> {
        let progress_tracker = self.progress_tracker.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(300)); // 5 minutes
            let mut last_completed = 0;
            let mut last_current_stage = String::new();
            
            loop {
                interval.tick().await;
                
                let tracker = progress_tracker.lock().unwrap();
                
                // Exit if all models are completed
                if tracker.completed_models >= tracker.total_models {
                    break;
                }
                
                // Only print if there's been progress
                let current_stage = if let Some(current) = &tracker.current_model {
                    if let Some(progress) = tracker.model_progress.get(current) {
                        format!("{:?}-{:?}", current, progress.stage)
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };
                
                if tracker.completed_models != last_completed || current_stage != last_current_stage {
                    info!("📊 Training Progress: {}/{} models completed", 
                          tracker.completed_models, tracker.total_models);
                    
                    if let Some(current) = &tracker.current_model {
                        if let Some(progress) = tracker.model_progress.get(current) {
                            info!("   Current: {:?} - Stage: {:?} - {}/{} batches", 
                                  current, progress.stage, progress.current_batch, progress.total_batches);
                        }
                    }
                    
                    // Calculate ETA
                    if let Some(start_time) = tracker.start_time {
                        if tracker.completed_models > 0 {
                            let elapsed = start_time.elapsed().unwrap_or(Duration::ZERO);
                            let avg_time_per_model = elapsed.as_secs() / tracker.completed_models as u64;
                            let remaining_models = tracker.total_models - tracker.completed_models;
                            let eta_seconds = avg_time_per_model * remaining_models as u64;
                            
                            info!("   ETA: {} minutes remaining", eta_seconds / 60);
                        }
                    }
                    
                    last_completed = tracker.completed_models;
                    last_current_stage = current_stage;
                }
                
                drop(tracker);
            }
        })
    }

    /// Update overall progress
    fn update_progress<F>(&self, update_fn: F) 
    where 
        F: FnOnce(&mut TrainingProgressTracker)
    {
        let mut tracker = self.progress_tracker.lock().unwrap();
        update_fn(&mut tracker);
    }

    /// Update progress for a specific model
    fn update_model_progress<F>(&self, specialty: &ModelSpecialty, update_fn: F)
    where
        F: FnOnce(&mut ModelProgress)
    {
        let mut tracker = self.progress_tracker.lock().unwrap();
        let progress = tracker.model_progress.entry(specialty.clone()).or_insert_with(|| {
            ModelProgress {
                stage: TrainingStage::NotStarted,
                current_batch: 0,
                total_batches: 0,
                examples_processed: 0,
                total_examples: 0,
                start_time: std::time::SystemTime::now(),
                estimated_completion: None,
                errors: Vec::new(),
            }
        });
        update_fn(progress);
    }

    /// Print validation summary
    fn print_validation_summary(&self) {
        info!("📋 Dataset Validation Summary");
        info!("==============================");
        
        let tracker = self.progress_tracker.lock().unwrap();
        for (specialty, result) in &tracker.dataset_validation_results {
            let status = if result.is_valid { "✅ VALID" } else { "❌ INVALID" };
            info!("  {:?}: {} ({}/{} examples valid)", 
                  specialty, status, result.valid_examples, result.total_examples);
            
            if !result.validation_errors.is_empty() {
                info!("    {} validation errors", result.validation_errors.len());
            }
            
            info!("    Avg prompt length: {:.0} chars", result.structure_analysis.average_prompt_length);
            info!("    Avg output length: {:.0} chars", result.structure_analysis.average_output_length);
            info!("    JSON consistency: {:.1}%", result.structure_analysis.json_structure_consistency * 100.0);
        }
    }

    /// Print training summary
    fn print_training_summary(&self, trained_models: &HashMap<ModelSpecialty, String>) {
        info!("🎉 Training Complete!");
        info!("====================");
        
        let tracker = self.progress_tracker.lock().unwrap();
        if let Some(start_time) = tracker.start_time {
            let total_time = start_time.elapsed().unwrap_or(Duration::ZERO);
            info!("Total training time: {:.2} hours", total_time.as_secs_f64() / 3600.0);
        }
        
        info!("Trained models:");
        for (specialty, model_name) in trained_models {
            info!("  • {:?}: {}", specialty, model_name);
        }
        
        // Show any failed models
        let failed_models: Vec<_> = tracker.model_progress.iter()
            .filter(|(_, progress)| matches!(progress.stage, TrainingStage::Failed(_)))
            .collect();
            
        if !failed_models.is_empty() {
            warn!("Failed models:");
            for (specialty, progress) in failed_models {
                if let TrainingStage::Failed(error) = &progress.stage {
                    warn!("  • {:?}: {}", specialty, error);
                }
            }
        }
    }


    /// Analyze JSON structure for consistency checking
    fn analyze_json_structure(&self, json: &serde_json::Value) -> Vec<String> {
        let mut keys = Vec::new();
        self.collect_json_keys(json, "", &mut keys);
        keys
    }

    /// Recursively collect JSON keys for structure analysis
    fn collect_json_keys(&self, value: &serde_json::Value, prefix: &str, keys: &mut Vec<String>) {
        match value {
            serde_json::Value::Object(map) => {
                for (key, val) in map {
                    let full_key = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    keys.push(full_key.clone());
                    self.collect_json_keys(val, &full_key, keys);
                }
            },
            serde_json::Value::Array(arr) => {
                if !arr.is_empty() {
                    let array_key = format!("{}[]", prefix);
                    keys.push(array_key.clone());
                    self.collect_json_keys(&arr[0], &format!("{}[0]", prefix), keys);
                }
            },
            _ => {}
        }
    }

    /// Calculate structure consistency across examples
    fn calculate_structure_consistency(&self, structures: &[Vec<String>]) -> f64 {
        if structures.len() < 2 {
            return 1.0;
        }

        let mut all_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
        for structure in structures {
            for key in structure {
                all_keys.insert(key.clone());
            }
        }

        let mut consistency_scores = Vec::new();
        for key in &all_keys {
            let count = structures.iter().filter(|s| s.contains(key)).count();
            consistency_scores.push(count as f64 / structures.len() as f64);
        }

        consistency_scores.iter().sum::<f64>() / consistency_scores.len() as f64
    }
}