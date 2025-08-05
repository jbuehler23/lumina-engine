//! AI service client for communicating with various AI providers
//! 
//! This module provides a unified interface for different AI services
//! including OpenAI, Anthropic, and local models.

use std::time::Duration;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use anyhow::{Result, Context, anyhow};

use crate::AiConfig;

/// AI service providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiProvider {
    OpenAI,
    Anthropic,
    Ollama,
    Local,
}

/// Request for text generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRequest {
    /// The prompt to send to the AI
    pub prompt: String,
    /// Maximum tokens to generate
    pub max_tokens: u32,
    /// Temperature for generation (0.0-1.0)
    pub temperature: f32,
    /// Whether to stream the response
    pub stream: bool,
    /// System message (for supported providers)
    pub system_message: Option<String>,
}

/// Response from text generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResponse {
    /// Generated text content
    pub content: String,
    /// Tokens used in the request
    pub tokens_used: u32,
    /// Whether the response was truncated
    pub truncated: bool,
    /// Additional metadata from the provider
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

/// AI client for making requests to various providers
pub struct AiClient {
    config: AiConfig,
    http_client: Client,
}

impl AiClient {
    /// Create a new AI client
    pub async fn new(config: AiConfig) -> Result<Self> {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .context("Failed to create HTTP client")?;

        // Validate configuration
        match config.provider {
            AiProvider::OpenAI | AiProvider::Anthropic => {
                if config.api_key.is_none() {
                    return Err(anyhow!("API key required for {:?} provider", config.provider));
                }
            }
            AiProvider::Ollama => {
                // Ollama typically runs on localhost:11434, set default if not specified
                if config.base_url.is_none() {
                    log::info!("Using default Ollama URL: http://localhost:11434");
                }
            }
            AiProvider::Local => {
                if config.base_url.is_none() {
                    return Err(anyhow!("Base URL required for local provider"));
                }
            }
        }

        Ok(Self {
            config,
            http_client,
        })
    }

    /// Generate text using the configured AI provider
    pub async fn generate_text(&self, request: GenerationRequest) -> Result<GenerationResponse> {
        match self.config.provider {
            AiProvider::OpenAI => self.generate_openai(request).await,
            AiProvider::Anthropic => self.generate_anthropic(request).await,
            AiProvider::Ollama => self.generate_ollama(request).await,
            AiProvider::Local => self.generate_local(request).await,
        }
    }

    /// Generate image description/prompt for asset generation
    pub async fn generate_asset_description(&self, 
                                          asset_type: &str, 
                                          game_context: &str,
                                          style: Option<&str>) -> Result<String> {
        let prompt = format!(
            "Generate a detailed visual description for a {} asset in a game context: {}\n\
             Style preference: {}\n\
             Focus on visual details that would help an AI image generator create the asset.\n\
             Keep the description concise but specific about colors, shapes, and style.",
            asset_type,
            game_context,
            style.unwrap_or("pixel art")
        );

        let request = GenerationRequest {
            prompt,
            max_tokens: 200,
            temperature: 0.7,
            stream: false,
            system_message: Some("You are an expert game asset designer. Generate clear, specific visual descriptions.".to_string()),
        };

        let response = self.generate_text(request).await?;
        Ok(response.content)
    }

    /// OpenAI API implementation
    async fn generate_openai(&self, request: GenerationRequest) -> Result<GenerationResponse> {
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| anyhow!("OpenAI API key not configured"))?;

        let url = "https://api.openai.com/v1/chat/completions";
        
        let mut messages = Vec::new();
        
        if let Some(system_msg) = request.system_message {
            messages.push(serde_json::json!({
                "role": "system",
                "content": system_msg
            }));
        }
        
        messages.push(serde_json::json!({
            "role": "user", 
            "content": request.prompt
        }));

        let payload = serde_json::json!({
            "model": self.config.text_model,
            "messages": messages,
            "max_tokens": request.max_tokens,
            "temperature": request.temperature,
            "stream": request.stream
        });

        let response = self.http_client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .context("Failed to send OpenAI request")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("OpenAI API error: {}", error_text));
        }

        let response_data: serde_json::Value = response.json().await
            .context("Failed to parse OpenAI response")?;

        let content = response_data["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Invalid OpenAI response format"))?
            .to_string();

        let tokens_used = response_data["usage"]["total_tokens"]
            .as_u64()
            .unwrap_or(0) as u32;

        let truncated = response_data["choices"][0]["finish_reason"]
            .as_str() == Some("length");

        Ok(GenerationResponse {
            content,
            tokens_used,
            truncated,
            metadata: std::collections::HashMap::new(),
        })
    }

    /// Anthropic API implementation
    async fn generate_anthropic(&self, request: GenerationRequest) -> Result<GenerationResponse> {
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| anyhow!("Anthropic API key not configured"))?;

        let url = "https://api.anthropic.com/v1/messages";
        
        let mut messages = Vec::new();
        messages.push(serde_json::json!({
            "role": "user",
            "content": request.prompt
        }));

        let payload = serde_json::json!({
            "model": self.config.text_model,
            "max_tokens": request.max_tokens,
            "temperature": request.temperature,
            "messages": messages,
            "system": request.system_message.unwrap_or_default()
        });

        let response = self.http_client
            .post(url)
            .header("x-api-key", api_key)
            .header("Content-Type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .json(&payload)
            .send()
            .await
            .context("Failed to send Anthropic request")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Anthropic API error: {}", error_text));
        }

        let response_data: serde_json::Value = response.json().await
            .context("Failed to parse Anthropic response")?;

        let content = response_data["content"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow!("Invalid Anthropic response format"))?
            .to_string();

        let tokens_used = response_data["usage"]["output_tokens"]
            .as_u64()
            .unwrap_or(0) as u32;

        let truncated = response_data["stop_reason"]
            .as_str() == Some("max_tokens");

        Ok(GenerationResponse {
            content,
            tokens_used,
            truncated,
            metadata: std::collections::HashMap::new(),
        })
    }

    /// Ollama model implementation
    async fn generate_ollama(&self, request: GenerationRequest) -> Result<GenerationResponse> {
        let base_url = self.config.base_url.as_deref().unwrap_or("http://localhost:11434");
        let url = format!("{}/api/generate", base_url);

        let payload = serde_json::json!({
            "model": self.config.text_model,
            "prompt": request.prompt,
            "stream": false,
            "options": {
                "temperature": request.temperature,
                "num_predict": request.max_tokens
            }
        });

        let response = self.http_client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .context("Failed to send Ollama request")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Ollama API error: {}", error_text));
        }

        let response_data: serde_json::Value = response.json().await
            .context("Failed to parse Ollama response")?;

        let content = response_data["response"]
            .as_str()
            .ok_or_else(|| anyhow!("Invalid Ollama response format"))?
            .to_string();

        let tokens_used = response_data["eval_count"]
            .as_u64()
            .unwrap_or(0) as u32;

        let truncated = response_data["done"]
            .as_bool()
            .map(|done| !done)
            .unwrap_or(false);

        Ok(GenerationResponse {
            content,
            tokens_used,
            truncated,
            metadata: std::collections::HashMap::new(),
        })
    }

    /// Local model implementation (e.g., custom models, local OpenAI-compatible API)
    async fn generate_local(&self, request: GenerationRequest) -> Result<GenerationResponse> {
        let base_url = self.config.base_url.as_ref()
            .ok_or_else(|| anyhow!("Base URL not configured for local provider"))?;

        let url = format!("{}/v1/chat/completions", base_url);
        
        let mut messages = Vec::new();
        
        if let Some(system_msg) = request.system_message {
            messages.push(serde_json::json!({
                "role": "system",
                "content": system_msg
            }));
        }
        
        messages.push(serde_json::json!({
            "role": "user",
            "content": request.prompt
        }));

        let payload = serde_json::json!({
            "model": self.config.text_model,
            "messages": messages,
            "max_tokens": request.max_tokens,
            "temperature": request.temperature,
            "stream": request.stream
        });

        let mut req_builder = self.http_client
            .post(&url)
            .header("Content-Type", "application/json");

        // Add authorization if API key is provided
        if let Some(api_key) = &self.config.api_key {
            req_builder = req_builder.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = req_builder
            .json(&payload)
            .send()
            .await
            .context("Failed to send local API request")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Local API error: {}", error_text));
        }

        let response_data: serde_json::Value = response.json().await
            .context("Failed to parse local API response")?;

        let content = response_data["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Invalid local API response format"))?
            .to_string();

        let tokens_used = response_data["usage"]["total_tokens"]
            .as_u64()
            .unwrap_or(0) as u32;

        let truncated = response_data["choices"][0]["finish_reason"]
            .as_str() == Some("length");

        Ok(GenerationResponse {
            content,
            tokens_used,
            truncated,
            metadata: std::collections::HashMap::new(),
        })
    }
}

impl Default for GenerationRequest {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            max_tokens: 4096,
            temperature: 0.7,
            stream: false,
            system_message: None,
        }
    }
}