//! Main game generation orchestrator
//! 
//! This module coordinates the entire text-to-game pipeline, combining
//! prompt processing, AI generation, and scene creation.

use std::collections::HashMap;
use anyhow::{Result, Context, anyhow};
use serde::{Deserialize, Serialize};
use log::{info, warn, error};

use crate::{
    scene_description::{SceneDescription, EntityDescription, ComponentDescription},
    prompt_engine::{PromptEngine, GamePrompt},
    ai_client::{AiClient, GenerationRequest},
    asset_generation::{AssetGenerator, AssetRequest, AssetType},
};

/// Main game generator that orchestrates the text-to-game pipeline
pub struct GameGenerator {
    ai_client: AiClient,
    prompt_engine: PromptEngine,
    asset_generator: AssetGenerator,
}

/// Request for generating a complete game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameGenerationRequest {
    /// User's game description
    pub prompt: GamePrompt,
    /// Whether to generate assets automatically
    pub generate_assets: bool,
    /// Maximum number of entities to create
    pub max_entities: Option<usize>,
    /// Whether to include audio
    pub include_audio: bool,
    /// Target platform optimizations
    pub target_platform: TargetPlatform,
}

/// Target platforms for game generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetPlatform {
    Desktop,
    Web,
    Mobile,
    Universal,
}

/// Result of game generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameGenerationResult {
    /// Generated scene description
    pub scene: SceneDescription,
    /// Generated assets
    pub assets: Vec<crate::asset_generation::GeneratedAsset>,
    /// Generation statistics
    pub stats: GenerationStats,
    /// Any warnings or issues encountered
    pub warnings: Vec<String>,
}

/// Statistics about the generation process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationStats {
    /// Total generation time in seconds
    pub total_time: f32,
    /// Number of AI API calls made
    pub api_calls: u32,
    /// Total tokens used
    pub tokens_used: u32,
    /// Number of entities created
    pub entities_created: usize,
    /// Number of assets generated
    pub assets_generated: usize,
}

impl GameGenerator {
    /// Create a new game generator
    pub fn new(
        ai_client: AiClient, 
        prompt_engine: PromptEngine, 
        asset_generator: AssetGenerator
    ) -> Self {
        Self {
            ai_client,
            prompt_engine,
            asset_generator,
        }
    }

    /// Generate a complete game from a text prompt
    pub async fn generate_game(&mut self, request: GameGenerationRequest) -> Result<GameGenerationResult> {
        let start_time = std::time::Instant::now();
        let mut stats = GenerationStats::default();
        let mut warnings = Vec::new();

        info!("Starting game generation: {}", request.prompt.description);

        // Step 1: Generate scene structure from prompt
        let scene = self.generate_scene_from_prompt(&request.prompt, &mut stats).await?;
        
        // Step 2: Generate assets if requested
        let mut assets = Vec::new();
        if request.generate_assets {
            assets = self.generate_scene_assets(&scene, &request, &mut stats).await?;
        }

        // Step 3: Validate and optimize the generated content
        self.validate_scene(&scene, &mut warnings)?;

        // Step 4: Apply platform-specific optimizations
        let optimized_scene = self.optimize_for_platform(scene, &request.target_platform)?;

        stats.total_time = start_time.elapsed().as_secs_f32();
        stats.entities_created = optimized_scene.entities.len();
        stats.assets_generated = assets.len();

        info!("Game generation completed in {:.2}s", stats.total_time);

        Ok(GameGenerationResult {
            scene: optimized_scene,
            assets,
            stats,
            warnings,
        })
    }

    /// Refine an existing game based on user feedback
    pub async fn refine_game(&mut self, 
                            current_scene: SceneDescription,
                            refinement_request: &str) -> Result<SceneDescription> {
        info!("Refining game: {}", refinement_request);

        // Generate refinement prompt
        let prompt = self.prompt_engine.generate_refinement_prompt(
            refinement_request, 
            &current_scene
        )?;

        // Get AI response
        let request = GenerationRequest {
            prompt,
            max_tokens: 4096,
            temperature: 0.7,
            stream: false,
            system_message: Some(
                "You are an expert game designer. Modify the provided game scene based on the user's request. \
                 Return only valid JSON in the Scene Description Language format.".to_string()
            ),
        };

        let response = self.ai_client.generate_text(request).await?;

        // Parse the refined scene
        let refined_scene = self.parse_scene_from_response(&response.content)?;

        info!("Game refinement completed");
        Ok(refined_scene)
    }

    /// Generate scene structure from prompt
    async fn generate_scene_from_prompt(&mut self, 
                                      prompt: &GamePrompt, 
                                      stats: &mut GenerationStats) -> Result<SceneDescription> {
        // Generate the AI prompt
        let ai_prompt = self.prompt_engine.generate_scene_prompt(prompt)?;

        // Make AI request
        let request = GenerationRequest {
            prompt: ai_prompt,
            max_tokens: 6000,
            temperature: 0.7,
            stream: false,
            system_message: Some(
                "You are an expert game designer and programmer. Generate complete, playable games \
                 using the Scene Description Language. Focus on fun gameplay mechanics and clear objectives. \
                 Return only valid JSON - no explanations or additional text.".to_string()
            ),
        };

        let response = self.ai_client.generate_text(request).await
            .context("Failed to generate scene from AI")?;

        stats.api_calls += 1;
        stats.tokens_used += response.tokens_used;

        // Parse the AI response into a scene
        let scene = self.parse_scene_from_response(&response.content)?;

        info!("Generated scene with {} entities", scene.entities.len());
        Ok(scene)
    }

    /// Generate assets for the scene entities
    async fn generate_scene_assets(&mut self, 
                                 scene: &SceneDescription,
                                 request: &GameGenerationRequest,
                                 stats: &mut GenerationStats) -> Result<Vec<crate::asset_generation::GeneratedAsset>> {
        let mut assets = Vec::new();
        let game_context = format!("{} - {}", scene.metadata.name, scene.metadata.description);

        // Collect all asset references from the scene
        let mut asset_requests = Vec::new();
        
        for entity in &scene.entities {
            for (component_name, component) in &entity.components {
                match component {
                    ComponentDescription::Sprite { texture, .. } => {
                        if !texture.is_empty() && !texture.starts_with("generated:") {
                            asset_requests.push(AssetRequest::new(
                                AssetType::Sprite, 
                                format!("{} sprite for {}", texture, entity.name)
                            )
                            .with_context(&game_context)
                            .with_style("pixel art"));
                        }
                    }
                    ComponentDescription::AudioSource { clip, .. } => {
                        if !clip.is_empty() && !clip.starts_with("generated:") {
                            let asset_type = if clip.contains("music") || clip.contains("bgm") {
                                AssetType::Music
                            } else {
                                AssetType::SoundEffect
                            };
                            
                            asset_requests.push(AssetRequest::new(
                                asset_type,
                                format!("{} audio for {}", clip, entity.name)
                            )
                            .with_context(&game_context)
                            .with_style("8-bit chiptune"));
                        }
                    }
                    _ => {}
                }
            }
        }

        // Generate background if needed
        if scene.metadata.background_color == [0.2, 0.3, 0.4, 1.0] { // Default background
            asset_requests.push(AssetRequest::new(
                AssetType::Background,
                format!("Background for {}", scene.metadata.description)
            )
            .with_context(&game_context)
            .with_dimensions(1280, 720));
        }

        // Limit asset generation to avoid overwhelming the system
        let max_assets = 10;
        asset_requests.truncate(max_assets);

        // Generate each asset
        for asset_request in asset_requests {
            match self.asset_generator.generate_asset(asset_request).await {
                Ok(asset) => {
                    info!("Generated asset: {}", asset.description);
                    assets.push(asset);
                }
                Err(e) => {
                    warn!("Failed to generate asset: {}", e);
                }
            }
        }

        info!("Generated {} assets", assets.len());
        Ok(assets)
    }

    /// Parse scene description from AI response
    fn parse_scene_from_response(&self, response: &str) -> Result<SceneDescription> {
        // Try to extract JSON from the response
        let json_content = self.extract_json_from_response(response)?;
        
        // Parse the JSON into a SceneDescription
        SceneDescription::from_json(&json_content)
            .context("Failed to parse scene description from AI response")
    }

    /// Extract JSON content from AI response (handles cases where AI adds explanations)
    fn extract_json_from_response(&self, response: &str) -> Result<String> {
        let trimmed = response.trim();
        
        // Look for JSON object boundaries
        if let Some(start) = trimmed.find('{') {
            if let Some(end) = trimmed.rfind('}') {
                if end > start {
                    return Ok(trimmed[start..=end].to_string());
                }
            }
        }

        // If no clear JSON boundaries, try to parse the entire response
        Ok(trimmed.to_string())
    }

    /// Validate the generated scene for common issues
    fn validate_scene(&self, scene: &SceneDescription, warnings: &mut Vec<String>) -> Result<()> {
        // Check for empty scene
        if scene.entities.is_empty() {
            warnings.push("Generated scene has no entities".to_string());
        }

        // Check for entities without essential components
        for entity in &scene.entities {
            let has_transform = entity.components.contains_key("Transform");
            let has_visual = entity.components.contains_key("Sprite") || 
                           entity.components.contains_key("Text");

            if !has_transform {
                warnings.push(format!("Entity '{}' missing Transform component", entity.name));
            }

            if !has_visual && !entity.tags.contains(&"system".to_string()) {
                warnings.push(format!("Entity '{}' has no visual representation", entity.name));
            }
        }

        // Check for player entity
        let has_player = scene.entities.iter().any(|e| 
            e.components.contains_key("Player") || 
            e.tags.contains(&"player".to_string())
        );

        if !has_player {
            warnings.push("No player entity found in scene".to_string());
        }

        // Check for game objectives/goals
        let has_objectives = scene.scripts.iter().any(|s| 
            s.name.to_lowercase().contains("goal") ||
            s.name.to_lowercase().contains("win") ||
            s.name.to_lowercase().contains("score")
        );

        if !has_objectives {
            warnings.push("No clear win condition or objectives defined".to_string());
        }

        Ok(())
    }

    /// Apply platform-specific optimizations
    fn optimize_for_platform(&self, 
                            mut scene: SceneDescription, 
                            platform: &TargetPlatform) -> Result<SceneDescription> {
        match platform {
            TargetPlatform::Web => {
                // Optimize for web deployment
                scene.metadata.resolution = [800, 600]; // Smaller resolution for web
                
                // Reduce physics complexity
                scene.metadata.physics.timestep = 1.0 / 30.0; // 30 FPS for web
            }
            TargetPlatform::Mobile => {
                // Optimize for mobile devices
                scene.metadata.resolution = [720, 1280]; // Portrait orientation
                
                // Reduce asset complexity
                for entity in &mut scene.entities {
                    if let Some(ComponentDescription::Sprite { layer, .. }) = 
                        entity.components.get_mut("Sprite") {
                        if *layer > 5 {
                            *layer = 5; // Limit render layers for mobile
                        }
                    }
                }
            }
            TargetPlatform::Desktop => {
                // Keep full quality for desktop
                scene.metadata.resolution = [1920, 1080];
            }
            TargetPlatform::Universal => {
                // Balanced settings for all platforms
                scene.metadata.resolution = [1280, 720];
            }
        }

        Ok(scene)
    }
}

impl Default for GenerationStats {
    fn default() -> Self {
        Self {
            total_time: 0.0,
            api_calls: 0,
            tokens_used: 0,
            entities_created: 0,
            assets_generated: 0,
        }
    }
}

impl Default for TargetPlatform {
    fn default() -> Self {
        TargetPlatform::Universal
    }
}

impl GameGenerationRequest {
    /// Create a new game generation request
    pub fn new(prompt: GamePrompt) -> Self {
        Self {
            prompt,
            generate_assets: true,
            max_entities: Some(20),
            include_audio: true,
            target_platform: TargetPlatform::default(),
        }
    }

    /// Disable asset generation
    pub fn without_assets(mut self) -> Self {
        self.generate_assets = false;
        self
    }

    /// Set maximum entities
    pub fn with_max_entities(mut self, max: usize) -> Self {
        self.max_entities = Some(max);
        self
    }

    /// Set target platform
    pub fn for_platform(mut self, platform: TargetPlatform) -> Self {
        self.target_platform = platform;
        self
    }

    /// Disable audio generation
    pub fn without_audio(mut self) -> Self {
        self.include_audio = false;
        self
    }
}