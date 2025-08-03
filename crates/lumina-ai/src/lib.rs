//! AI integration layer for Lumina Engine
//! 
//! This crate provides text-to-game generation capabilities, allowing users to create
//! games through natural language prompts. It integrates with various AI services
//! and translates prompts into Lumina Engine's ECS representation.

pub mod scene_description;
pub mod prompt_engine;
pub mod ai_client;
pub mod asset_generation;
pub mod game_generator;
pub mod scene_loader;
pub mod live_preview;

#[cfg(feature = "training")]
pub mod training;

#[cfg(feature = "training")]
pub mod specialized_training;

pub use scene_description::{SceneDescription, EntityDescription, ComponentDescription};
pub use prompt_engine::{PromptEngine, PromptTemplate, GamePrompt};
pub use ai_client::{AiClient, AiProvider, GenerationRequest, GenerationResponse};
pub use asset_generation::{AssetGenerator, AssetRequest, GeneratedAsset};
pub use game_generator::{GameGenerator, GameGenerationRequest, GameGenerationResult};
pub use scene_loader::{SceneLoader, SceneLoadResult};
pub use live_preview::{LivePreviewSystem, PreviewConfig, PreviewResult, PreviewState, PreviewStatus};

#[cfg(feature = "training")]
pub use training::{
    TrainingConfig, ModelTrainer, TrainingDatasetBuilder, 
    TrainingExample, DifficultyLevel, TrainedModel
};

#[cfg(feature = "training")]
pub use specialized_training::{
    SpecializedModelTrainer, SpecializedTrainingConfig, ModelSpecialty,
    ModelConfig, ModelTrainingParams, GlobalTrainingSettings
};

use anyhow::Result;

/// AI configuration for the Lumina Engine
#[derive(Debug, Clone)]
pub struct AiConfig {
    /// AI provider to use (OpenAI, Anthropic, Local)
    pub provider: AiProvider,
    /// API key for the AI service
    pub api_key: Option<String>,
    /// Base URL for custom AI endpoints
    pub base_url: Option<String>,
    /// Model name to use for text generation
    pub text_model: String,
    /// Model name to use for image generation
    pub image_model: Option<String>,
    /// Maximum tokens per generation request
    pub max_tokens: u32,
    /// Temperature for text generation (0.0-1.0)
    pub temperature: f32,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            provider: AiProvider::Ollama,
            api_key: None,
            base_url: Some("http://localhost:11434".to_string()),
            text_model: "lumina-gamedev:latest".to_string(),
            image_model: None,
            max_tokens: 4096,
            temperature: 0.7,
        }
    }
}

/// Initialize the AI system with the given configuration
pub async fn initialize_ai_system(config: AiConfig) -> Result<GameGenerator> {
    log::info!("Initializing AI system with provider: {:?}", config.provider);
    
    let ai_client = AiClient::new(config.clone()).await?;
    let prompt_engine = PromptEngine::new();
    let asset_generator = AssetGenerator::new(config.clone()).await?;
    
    Ok(GameGenerator::new(ai_client, prompt_engine, asset_generator))
}