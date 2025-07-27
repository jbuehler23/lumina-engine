//! Text rendering pipeline using cosmic-text following Bevy's patterns
//!
//! This module provides a clean separation between text layout and rendering,
//! following Bevy's design patterns for optimal performance and maintainability.
//! No fallback rendering - fails fast on errors for better debugging.

use crate::{RenderResult, FontHandle};
use glam::Vec2;
use cosmic_text::{FontSystem, SwashCache, Buffer, Metrics, Family, Attrs, Shaping, Wrap, CacheKey};
use std::collections::HashMap;
use thiserror::Error;

/// Text layout information containing all positioned glyphs (Bevy pattern)
#[derive(Debug, Clone)]
pub struct TextLayoutInfo {
    /// All positioned glyphs ready for rendering
    pub glyphs: Vec<PositionedGlyph>,
    /// Total text bounds
    pub size: Vec2,
}

/// A positioned glyph ready for rendering (Bevy pattern)
#[derive(Debug, Clone)]
pub struct PositionedGlyph {
    /// Position in world space
    pub position: Vec2,
    /// Size of the glyph
    pub size: Vec2,
    /// Atlas coordinates for texture sampling
    pub atlas_coords: [f32; 4],
    /// Color multiplier
    pub color: [f32; 4],
}

/// Key for glyph cache lookup
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct GlyphKey {
    cache_key: CacheKey,
    font_size: u32, // Size in fixed point (size * 100)
}

/// Text rendering errors
#[derive(Error, Debug)]
pub enum TextError {
    #[error("Failed to load font: {0}")]
    FontLoadError(String),
    #[error("Failed to shape text: {0}")]
    ShapingError(String),
    #[error("Failed to rasterize glyph: {0}")]
    RasterizationError(String),
    #[error("Font atlas is full")]
    AtlasFull,
}

/// Text pipeline for layout and rendering using cosmic-text (Bevy pattern)
pub struct TextPipeline {
    /// Cosmic text font system (singleton)
    font_system: FontSystem,
    /// Swash cache for glyph rasterization (singleton)
    swash_cache: SwashCache,
    /// Font atlas for efficient glyph storage
    font_atlas: FontAtlas,
    /// Default font handle
    default_font: Option<FontHandle>,
}

/// Font atlas for efficient glyph storage and retrieval
pub struct FontAtlas {
    /// Glyph atlas texture for GPU rendering
    texture: wgpu::Texture,
    /// Current atlas dimensions
    size: (u32, u32),
    /// Next position in atlas for new glyphs
    cursor: (u32, u32),
    /// Row height tracking for proper atlas packing
    current_row_height: u32,
    /// Cached glyph data
    glyph_cache: HashMap<GlyphKey, PositionedGlyph>,
}

impl FontAtlas {
    /// Create a new font atlas
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> RenderResult<Self> {
        let size = (1024, 1024);
        
        // Create glyph atlas texture (1024x1024 grayscale)
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Font Atlas"),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm, // Grayscale
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        
        let atlas = Self {
            texture,
            size,
            cursor: (1, 1), // Reserve (0,0) for white pixel
            current_row_height: 0,
            glyph_cache: HashMap::new(),
        };
        
        // Initialize the atlas with a white pixel at (0,0) for solid color rendering
        atlas.init_white_pixel(queue);
        
        Ok(atlas)
    }
    
    /// Get or rasterize a glyph using cosmic-text
    pub fn get_or_rasterize_glyph(
        &mut self,
        cache_key: CacheKey,
        font_size: f32,
        font_system: &mut FontSystem,
        swash_cache: &mut SwashCache,
        queue: &wgpu::Queue,
    ) -> Result<&PositionedGlyph, TextError> {
        let glyph_key = GlyphKey {
            cache_key,
            font_size: (font_size * 100.0) as u32,
        };
        
        if !self.glyph_cache.contains_key(&glyph_key) {
            // Rasterize the glyph using swash
            let image = if let Some(image) = swash_cache.get_image(font_system, cache_key) {
                image
            } else {
                return Err(TextError::RasterizationError(format!("Failed to rasterize glyph with cache key {:?}", cache_key)));
            };
            
            log::debug!("Rasterized glyph at size {} ({}x{})", font_size, image.placement.width, image.placement.height);
            
            // Extract image data
            let glyph_width = image.placement.width;
            let glyph_height = image.placement.height;
            
            // Convert to grayscale bitmap
            let bitmap = match image.content {
                cosmic_text::SwashContent::Mask => {
                    // Already grayscale
                    image.data.to_vec()
                },
                cosmic_text::SwashContent::Color => {
                    // Convert RGBA to grayscale
                    let mut grayscale = Vec::with_capacity(image.data.len() / 4);
                    for chunk in image.data.chunks_exact(4) {
                        let r = chunk[0] as f32;
                        let g = chunk[1] as f32;
                        let b = chunk[2] as f32;
                        let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
                        grayscale.push(gray);
                    }
                    grayscale
                },
                cosmic_text::SwashContent::SubpixelMask => {
                    // Take only one channel for simplicity
                    image.data.iter().step_by(3).copied().collect()
                },
            };
            
            // Allocate space in atlas and get coordinates
            let atlas_coords = self.allocate_atlas_space(glyph_width, glyph_height, &bitmap, queue)?;
            
            let positioned_glyph = PositionedGlyph {
                position: Vec2::ZERO, // Will be set by caller
                size: Vec2::new(glyph_width as f32, glyph_height as f32),
                atlas_coords,
                color: [1.0, 1.0, 1.0, 1.0], // Default white
            };
            
            self.glyph_cache.insert(glyph_key.clone(), positioned_glyph);
            log::debug!("Cached glyph (size: {}) with {}x{} bitmap", font_size, glyph_width, glyph_height);
        }
        
        self.glyph_cache.get(&glyph_key)
            .ok_or_else(|| TextError::RasterizationError("Failed to retrieve cached glyph".to_string()))
    }
    
    /// Allocate space in the atlas and upload glyph bitmap
    fn allocate_atlas_space(&mut self, width: u32, height: u32, bitmap: &[u8], queue: &wgpu::Queue) -> Result<[f32; 4], TextError> {
        // Check if we need to move to next row
        if self.cursor.0 + width > self.size.0 {
            // Move to next row
            self.cursor.0 = 1; // Skip white pixel
            self.cursor.1 += self.current_row_height + 2; // Add padding
            self.current_row_height = 0;
        }
        
        // Track row height
        self.current_row_height = self.current_row_height.max(height);
        
        // Check if we have vertical space
        if self.cursor.1 + height > self.size.1 {
            return Err(TextError::AtlasFull);
        }
        
        let coords = [
            self.cursor.0 as f32 / self.size.0 as f32,
            self.cursor.1 as f32 / self.size.1 as f32,
            (self.cursor.0 + width) as f32 / self.size.0 as f32,
            (self.cursor.1 + height) as f32 / self.size.1 as f32,
        ];
        
        // Upload to GPU
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: self.cursor.0,
                    y: self.cursor.1,
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            },
            bitmap,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
        
        log::debug!("Uploaded glyph to GPU atlas at ({}, {})", self.cursor.0, self.cursor.1);
        
        // Advance cursor
        self.cursor.0 += width + 1; // Add padding
        
        Ok(coords)
    }
    
    /// Initialize a white pixel at (0,0) in the atlas for solid color rendering
    fn init_white_pixel(&self, queue: &wgpu::Queue) {
        // Create a single white pixel
        let white_pixel = [255u8]; // Single grayscale white pixel
        
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &white_pixel,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(1),
                rows_per_image: Some(1),
            },
            wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        
        log::debug!("Initialized white pixel at (0,0) in font atlas for solid rendering");
    }
}

impl TextPipeline {
    /// Create a new text pipeline
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _format: wgpu::TextureFormat,
    ) -> RenderResult<Self> {
        // Create cosmic-text singletons
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        
        // Create font atlas
        let font_atlas = FontAtlas::new(device, queue)?;
        
        let mut pipeline = Self {
            font_system,
            swash_cache,
            font_atlas,
            default_font: None,
        };
        
        // Load default font
        let default_handle = pipeline.load_default_font()?;
        pipeline.default_font = Some(default_handle);
        
        Ok(pipeline)
    }
    
    /// Queue text for layout and rendering (Bevy pattern)
    pub fn queue_text(
        &mut self,
        text: &str,
        _font_handle: FontHandle,
        font_size: f32,
        position: Vec2,
        color: [f32; 4],
        queue: &wgpu::Queue,
    ) -> Result<TextLayoutInfo, TextError> {
        log::debug!("Queueing text: '{}' at size {} at position {:?}", text, font_size, position);
        
        // Create buffer for text shaping
        let metrics = Metrics::new(font_size, font_size * 1.2); // 1.2 line height
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        
        // Configure buffer
        buffer.set_size(&mut self.font_system, Some(1000.0), Some(1000.0));
        buffer.set_text(
            &mut self.font_system,
            text,
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );
        buffer.set_wrap(&mut self.font_system, Wrap::None);
        
        // Shape the text
        buffer.shape_until_scroll(&mut self.font_system, false);
        
        // Extract positioned glyphs
        let mut glyphs = Vec::new();
        let mut max_x = 0.0f32;
        let mut max_y = 0.0f32;
        
        for run in buffer.layout_runs() {
            for glyph in run.glyphs {
                let cache_key = CacheKey::new(
                    glyph.font_id,
                    glyph.glyph_id,
                    font_size,
                    (0.0, 0.0), // No subpixel positioning for now
                    cosmic_text::CacheKeyFlags::empty(),
                ).0;
                
                // Get or create glyph in atlas
                let positioned_glyph = self.font_atlas.get_or_rasterize_glyph(
                    cache_key,
                    font_size,
                    &mut self.font_system,
                    &mut self.swash_cache,
                    queue,
                )?;
                
                let glyph_position = Vec2::new(
                    position.x + glyph.x,
                    position.y + glyph.y,
                );
                
                glyphs.push(PositionedGlyph {
                    position: glyph_position,
                    size: positioned_glyph.size,
                    atlas_coords: positioned_glyph.atlas_coords,
                    color,
                });
                
                max_x = max_x.max(glyph_position.x + positioned_glyph.size.x);
                max_y = max_y.max(glyph_position.y + positioned_glyph.size.y);
            }
        }
        
        let layout_size = Vec2::new(max_x - position.x, max_y - position.y);
        
        log::debug!("Generated {} positioned glyphs for text '{}'", glyphs.len(), text);
        
        Ok(TextLayoutInfo {
            glyphs,
            size: layout_size,
        })
    }

    /// Load default system font
    pub fn load_default_font(&mut self) -> RenderResult<FontHandle> {
        // Try to load a font from the assets directory
        let font_paths = [
            "assets/fonts/Inter-Regular.ttf",
            "assets/fonts/Roboto-Regular.ttf",
            "assets/fonts/SourceSansPro-Regular.ttf",
            "assets/fonts/DejaVuSans.ttf",
        ];
        
        for font_path in &font_paths {
            if let Ok(font_data) = std::fs::read(font_path) {
                log::info!("Loading default font from: {} ({} bytes)", font_path, font_data.len());
                return self.load_font("Default".to_string(), &font_data);
            } else {
                log::warn!("Could not read font file: {}", font_path);
            }
        }
        
        log::info!("No custom font found, using system default font with cosmic-text");
        // cosmic-text will automatically use system fonts
        Ok(FontHandle(0))
    }
    
    /// Load a font from bytes
    pub fn load_font(&mut self, name: String, font_data: &[u8]) -> RenderResult<FontHandle> {
        log::info!("Attempting to load font '{}' from {} bytes", name, font_data.len());
        
        // Register font with cosmic-text
        self.font_system.db_mut().load_font_data(font_data.to_vec());
        
        log::info!("Successfully loaded font '{}' with cosmic-text", name);
        Ok(FontHandle(0)) // cosmic-text manages font IDs internally
    }
    
    /// Get the default font handle
    pub fn default_font(&self) -> Option<FontHandle> {
        self.default_font
    }
    
    /// Get the font atlas texture for binding in shaders
    pub fn glyph_atlas(&self) -> &wgpu::Texture {
        &self.font_atlas.texture
    }
    
    /// Get the atlas size
    pub fn atlas_size(&self) -> (u32, u32) {
        self.font_atlas.size
    }
}