//! AI integration layer for Lumina Engine
//! 
//! This crate provides text-to-game generation capabilities, allowing users to create
//! games through natural language prompts. It integrates with various AI services
//! and translates prompts into Lumina Engine's ECS representation.

pub mod scene_description;
pub mod scene_loader;
// Temporarily disabled during refactoring
// pub mod prompt_engine;
// pub mod ai_client;
// pub mod asset_generation;
// pub mod game_generator;
// pub mod live_preview;

#[cfg(feature = "training")]
pub mod training;

#[cfg(feature = "training")]
pub mod specialized_training;

#[cfg(feature = "training")]
pub mod enhanced_training;

pub use scene_description::{SceneDescription, Entity as EntityDescription, ComponentData as ComponentDescription};
pub use scene_loader::{SceneLoader, SceneLoadResult};
// Temporarily disabled exports
// pub use prompt_engine::{PromptEngine, PromptTemplate, GamePrompt};
// pub use ai_client::{AiClient, AiProvider, GenerationRequest, GenerationResponse};
// pub use asset_generation::{AssetGenerator, AssetRequest, GeneratedAsset};
// pub use game_generator::{GameGenerator, GameGenerationRequest, GameGenerationResult};
// pub use live_preview::{LivePreviewSystem, PreviewConfig, PreviewResult, PreviewState, PreviewStatus};

#[cfg(feature = "training")]
pub use training::{
    TrainingConfig, ModelTrainer, TrainingDatasetBuilder, 
    TrainingExample, TrainingMetrics
};

#[cfg(feature = "training")]
pub use specialized_training::{
    SpecializedTrainer, SpecializedModelConfig, TrainingPipeline,
    ModelSpecialization, ModelType, TrainingResults
};

#[cfg(feature = "training")]
pub use enhanced_training::{
    EnhancedTrainer, ModelRegistry, ExperimentConfig,
    PerformanceMetrics, TrainingMonitor
};

// Re-export anyhow Result for convenience
pub use anyhow::Result;