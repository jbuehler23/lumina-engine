//! AI-powered asset generation for games
//! 
//! This module handles the generation of game assets (sprites, sounds, textures)
//! using AI services like DALL-E, Midjourney, or local models.

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context, anyhow};
use uuid::Uuid;

use crate::{AiConfig, ai_client::AiClient};

/// Asset generation service
pub struct AssetGenerator {
    config: AiConfig,
    ai_client: AiClient,
    cache: AssetCache,
}

/// Cache for generated assets
#[derive(Debug, Default)]
pub struct AssetCache {
    /// Mapping from description hash to asset path
    assets: HashMap<String, GeneratedAsset>,
    /// Base directory for cached assets
    cache_dir: PathBuf,
}

/// Request for asset generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRequest {
    /// Type of asset to generate
    pub asset_type: AssetType,
    /// Text description of the asset
    pub description: String,
    /// Visual style preference
    pub style: Option<String>,
    /// Game context for better generation
    pub game_context: Option<String>,
    /// Asset dimensions (for images)
    pub dimensions: Option<(u32, u32)>,
    /// Additional parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Types of assets that can be generated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    /// 2D sprite or texture
    Sprite,
    /// Background image
    Background,
    /// UI element
    UiElement,
    /// Sound effect
    SoundEffect,
    /// Music track
    Music,
    /// 3D model (future)
    Model3D,
}

/// Generated asset information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedAsset {
    /// Unique identifier for the asset
    pub id: String,
    /// Type of asset
    pub asset_type: AssetType,
    /// Original description used to generate
    pub description: String,
    /// Local file path where asset is stored
    pub file_path: PathBuf,
    /// Asset metadata
    pub metadata: AssetMetadata,
    /// Timestamp when generated
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Metadata for generated assets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMetadata {
    /// Asset dimensions (for images)
    pub dimensions: Option<(u32, u32)>,
    /// File size in bytes
    pub file_size: Option<u64>,
    /// Format/extension
    pub format: String,
    /// Generation parameters used
    pub generation_params: HashMap<String, serde_json::Value>,
    /// AI model used for generation
    pub model_used: Option<String>,
}

impl AssetGenerator {
    /// Create a new asset generator
    pub async fn new(config: AiConfig) -> Result<Self> {
        let ai_client = AiClient::new(config.clone()).await?;
        let cache = AssetCache::new()?;
        
        Ok(Self {
            config,
            ai_client,
            cache,
        })
    }

    /// Generate an asset from a request
    pub async fn generate_asset(&mut self, request: AssetRequest) -> Result<GeneratedAsset> {
        // Check cache first
        let cache_key = self.compute_cache_key(&request);
        if let Some(cached_asset) = self.cache.get(&cache_key) {
            log::info!("Using cached asset for: {}", request.description);
            return Ok(cached_asset.clone());
        }

        log::info!("Generating new {} asset: {}", 
                  format!("{:?}", request.asset_type).to_lowercase(), 
                  request.description);

        let asset = match request.asset_type {
            AssetType::Sprite | AssetType::Background | AssetType::UiElement => {
                self.generate_image_asset(request).await?
            }
            AssetType::SoundEffect | AssetType::Music => {
                self.generate_audio_asset(request).await?
            }
            AssetType::Model3D => {
                return Err(anyhow!("3D model generation not yet implemented"));
            }
        };

        // Cache the generated asset
        self.cache.store(&cache_key, &asset)?;
        
        Ok(asset)
    }

    /// Generate image assets (sprites, backgrounds, UI elements)
    async fn generate_image_asset(&self, request: AssetRequest) -> Result<GeneratedAsset> {
        // Generate detailed prompt for image generation
        let enhanced_prompt = self.enhance_image_prompt(&request).await?;
        
        log::debug!("Enhanced prompt: {}", enhanced_prompt);

        // For now, we'll create placeholder assets
        // In a real implementation, this would call DALL-E, Midjourney, or Stable Diffusion
        let asset_id = Uuid::new_v4().to_string();
        let file_path = self.cache.cache_dir.join(format!("{}.png", asset_id));
        
        // Create a simple colored rectangle as placeholder
        self.create_placeholder_image(&file_path, &request)?;

        let metadata = AssetMetadata {
            dimensions: request.dimensions,
            file_size: std::fs::metadata(&file_path).ok().map(|m| m.len()),
            format: "png".to_string(),
            generation_params: request.parameters.clone(),
            model_used: self.config.image_model.clone(),
        };

        Ok(GeneratedAsset {
            id: asset_id,
            asset_type: request.asset_type,
            description: request.description,
            file_path,
            metadata,
            created_at: chrono::Utc::now(),
        })
    }

    /// Generate audio assets (sound effects, music)
    async fn generate_audio_asset(&self, request: AssetRequest) -> Result<GeneratedAsset> {
        // Generate detailed prompt for audio generation
        let enhanced_prompt = self.enhance_audio_prompt(&request).await?;
        
        log::debug!("Enhanced audio prompt: {}", enhanced_prompt);

        // For now, we'll create placeholder audio files
        // In a real implementation, this would call audio generation APIs
        let asset_id = Uuid::new_v4().to_string();
        let file_path = self.cache.cache_dir.join(format!("{}.wav", asset_id));
        
        // Create a simple placeholder audio file
        self.create_placeholder_audio(&file_path, &request)?;

        let metadata = AssetMetadata {
            dimensions: None,
            file_size: std::fs::metadata(&file_path).ok().map(|m| m.len()),
            format: "wav".to_string(),
            generation_params: request.parameters.clone(),
            model_used: Some("placeholder".to_string()),
        };

        Ok(GeneratedAsset {
            id: asset_id,
            asset_type: request.asset_type,
            description: request.description,
            file_path,
            metadata,
            created_at: chrono::Utc::now(),
        })
    }

    /// Enhance image generation prompt using AI
    async fn enhance_image_prompt(&self, request: &AssetRequest) -> Result<String> {
        let base_prompt = &request.description;
        let style = request.style.as_deref().unwrap_or("pixel art");
        let context = request.game_context.as_deref().unwrap_or("retro game");

        self.ai_client.generate_asset_description(
            &format!("{:?}", request.asset_type).to_lowercase(),
            &format!("{} in {}", base_prompt, context),
            Some(style)
        ).await
    }

    /// Enhance audio generation prompt using AI
    async fn enhance_audio_prompt(&self, request: &AssetRequest) -> Result<String> {
        let prompt = format!(
            "Generate a detailed description for a {} audio asset: {}\n\
             Game context: {}\n\
             Style: {}\n\
             Focus on tempo, instruments, mood, and audio characteristics.",
            format!("{:?}", request.asset_type).to_lowercase(),
            request.description,
            request.game_context.as_deref().unwrap_or("retro game"),
            request.style.as_deref().unwrap_or("8-bit chiptune")
        );

        let generation_request = crate::ai_client::GenerationRequest {
            prompt,
            max_tokens: 150,
            temperature: 0.7,
            stream: false,
            system_message: Some("You are an expert game audio designer.".to_string()),
        };

        let response = self.ai_client.generate_text(generation_request).await?;
        Ok(response.content)
    }

    /// Create a placeholder image (for development/testing)
    fn create_placeholder_image(&self, path: &PathBuf, request: &AssetRequest) -> Result<()> {
        use std::fs;
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // For now, just create an empty file with metadata comment
        let placeholder_content = format!(
            "# Placeholder for: {}\n# Type: {:?}\n# Style: {}\n",
            request.description,
            request.asset_type,
            request.style.as_deref().unwrap_or("default")
        );

        fs::write(path, placeholder_content)?;
        
        log::info!("Created placeholder image at: {:?}", path);
        Ok(())
    }

    /// Create a placeholder audio file (for development/testing)
    fn create_placeholder_audio(&self, path: &PathBuf, request: &AssetRequest) -> Result<()> {
        use std::fs;
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Create a simple text file as placeholder
        let placeholder_content = format!(
            "# Placeholder audio for: {}\n# Type: {:?}\n# Duration: 3s\n",
            request.description,
            request.asset_type
        );

        fs::write(path, placeholder_content)?;
        
        log::info!("Created placeholder audio at: {:?}", path);
        Ok(())
    }

    /// Compute cache key for an asset request
    fn compute_cache_key(&self, request: &AssetRequest) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        request.description.hash(&mut hasher);
        format!("{:?}", request.asset_type).hash(&mut hasher);
        request.style.hash(&mut hasher);
        request.game_context.hash(&mut hasher);
        
        format!("{:x}", hasher.finish())
    }
}

impl AssetCache {
    /// Create a new asset cache
    pub fn new() -> Result<Self> {
        let cache_dir = std::env::current_dir()
            .context("Failed to get current directory")?
            .join("generated_assets");
        
        std::fs::create_dir_all(&cache_dir)
            .context("Failed to create cache directory")?;

        Ok(Self {
            assets: HashMap::new(),
            cache_dir,
        })
    }

    /// Get an asset from cache
    pub fn get(&self, key: &str) -> Option<&GeneratedAsset> {
        self.assets.get(key)
    }

    /// Store an asset in cache
    pub fn store(&mut self, key: &str, asset: &GeneratedAsset) -> Result<()> {
        self.assets.insert(key.to_string(), asset.clone());
        
        // Also save cache index to disk for persistence
        self.save_cache_index()?;
        
        Ok(())
    }

    /// Save cache index to disk
    fn save_cache_index(&self) -> Result<()> {
        let index_path = self.cache_dir.join("cache_index.json");
        let index_data = serde_json::to_string_pretty(&self.assets)?;
        std::fs::write(index_path, index_data)?;
        Ok(())
    }

    /// Load cache index from disk
    pub fn load_cache_index(&mut self) -> Result<()> {
        let index_path = self.cache_dir.join("cache_index.json");
        if index_path.exists() {
            let index_data = std::fs::read_to_string(index_path)?;
            self.assets = serde_json::from_str(&index_data)?;
        }
        Ok(())
    }
}

impl AssetRequest {
    /// Create a new asset request
    pub fn new(asset_type: AssetType, description: impl Into<String>) -> Self {
        Self {
            asset_type,
            description: description.into(),
            style: None,
            game_context: None,
            dimensions: None,
            parameters: HashMap::new(),
        }
    }

    /// Set the visual style
    pub fn with_style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Set the game context
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.game_context = Some(context.into());
        self
    }

    /// Set dimensions for image assets
    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.dimensions = Some((width, height));
        self
    }

    /// Add a parameter
    pub fn with_parameter(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.parameters.insert(key.into(), value);
        self
    }
}