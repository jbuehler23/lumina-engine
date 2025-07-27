//! Text rendering implementation using cosmic-text
//!
//! Provides font loading, glyph caching, and text rendering capabilities using cosmic-text,
//! following Bevy's design patterns for optimal performance.

use crate::{RenderResult, FontHandle};
use glam::Vec2;
use cosmic_text::{FontSystem, SwashCache, Buffer, Metrics, Family, Attrs, Shaping, Wrap, CacheKey};
use std::collections::HashMap;

/// Text rendering system using cosmic-text with buffer reuse
pub struct TextRenderer {
    /// Cosmic text font system (singleton)
    font_system: FontSystem,
    /// Swash cache for glyph rasterization (singleton)
    swash_cache: SwashCache,
    /// Text buffers cached by content hash for reuse
    text_buffers: HashMap<String, Buffer>,
    /// Glyph atlas texture for GPU rendering
    glyph_atlas: Option<wgpu::Texture>,
    /// Current atlas dimensions
    atlas_size: (u32, u32),
    /// Next position in atlas for new glyphs
    atlas_cursor: (u32, u32),
    /// Row height tracking for proper atlas packing
    current_row_height: u32,
    /// Cached glyph data
    glyph_cache: HashMap<GlyphKey, CachedGlyph>,
    /// Default font handle
    default_font: Option<FontHandle>,
}

/// Key for glyph cache lookup
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct GlyphKey {
    cache_key: CacheKey,
    font_size: u32, // Size in fixed point (size * 100)
}

/// A cached glyph with rendering data
pub struct CachedGlyph {
    /// Glyph metrics
    pub metrics: GlyphMetrics,
    /// Glyph bitmap data (grayscale)
    pub bitmap: Vec<u8>,
    /// Texture coordinates if uploaded to GPU
    pub texture_coords: Option<[f32; 4]>,
}

/// Glyph metrics for layout
#[derive(Debug, Clone)]
pub struct GlyphMetrics {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub advance_width: f32,
}

/// Positioned glyph ready for rendering
#[derive(Debug)]
pub struct PositionedGlyph {
    pub position: Vec2,
    pub atlas_coords: [f32; 4],
    pub size: Vec2,
}

impl TextRenderer {
    /// Create a new text renderer
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _format: wgpu::TextureFormat,
    ) -> RenderResult<Self> {
        // Create cosmic-text singletons
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        
        // Create glyph atlas texture (1024x1024 grayscale)
        let atlas_size = (1024, 1024);
        let glyph_atlas = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Glyph Atlas"),
            size: wgpu::Extent3d {
                width: atlas_size.0,
                height: atlas_size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm, // Grayscale
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        
        let mut renderer = Self {
            font_system,
            swash_cache,
            text_buffers: HashMap::new(),
            glyph_atlas: Some(glyph_atlas),
            atlas_size,
            atlas_cursor: (1, 1), // Reserve (0,0) for white pixel
            current_row_height: 0,
            glyph_cache: HashMap::new(),
            default_font: None,
        };
        
        // Initialize the atlas with a white pixel at (0,0) for solid color rendering
        renderer.init_white_pixel(queue);
        
        // Load default font
        let default_handle = renderer.load_default_font()?;
        renderer.default_font = Some(default_handle);
        
        Ok(renderer)
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

    /// Get or create a text buffer for the given text and font size (Bevy pattern)
    fn get_text_layout(&mut self, text: &str, font_size: f32) -> &Buffer {
        let text_key = format!("{}_{}", text, (font_size * 100.0) as u32);
        
        self.text_buffers.entry(text_key).or_insert_with(|| {
            log::debug!("Creating new text buffer for: '{}' at size {}", text, font_size);
            let mut buffer = Buffer::new(&mut self.font_system, Metrics::new(font_size, font_size));
            buffer.set_size(&mut self.font_system, Some(1000.0), Some(1000.0)); // Large enough for measurement
            buffer.set_text(&mut self.font_system, text, Attrs::new().family(Family::SansSerif), Shaping::Advanced);
            buffer.set_wrap(&mut self.font_system, Wrap::None);
            buffer.shape_until_scroll(&mut self.font_system, false);
            buffer
        })
    }

    /// Get or rasterize a glyph using Bevy patterns
    fn get_or_rasterize_glyph(&mut self, 
        cache_key: CacheKey, 
        font_size: f32,
        queue: Option<&wgpu::Queue>
    ) -> Option<&CachedGlyph> {
        let glyph_key = GlyphKey {
            cache_key,
            font_size: (font_size * 100.0) as u32,
        };
        
        if !self.glyph_cache.contains_key(&glyph_key) {
            // Rasterize the glyph using swash
            if let Some(image) = self.swash_cache.get_image(&mut self.font_system, cache_key) {
                log::debug!("Rasterized glyph at size {} ({}x{})", font_size, image.placement.width, image.placement.height);
                
                // Extract image data before calling methods that need mutable self
                let glyph_width = image.placement.width;
                let glyph_height = image.placement.height;
                let placement_left = image.placement.left;
                let placement_top = image.placement.top;
                
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
                
                // Calculate texture coordinates in the atlas
                let texture_coords = if glyph_width > 0 && glyph_height > 0 {
                    self.allocate_atlas_space(glyph_width, glyph_height, &bitmap, queue)
                } else {
                    // For empty glyphs (like space), still provide valid texture coords
                    Some([0.0, 0.0, 0.0, 0.0])
                };
                
                let cached_glyph = CachedGlyph {
                    metrics: GlyphMetrics {
                        width: glyph_width,
                        height: glyph_height,
                        x: placement_left,
                        y: placement_top,
                        advance_width: 0.0, // Will be set from layout glyph
                    },
                    bitmap,
                    texture_coords,
                };
                
                self.glyph_cache.insert(glyph_key.clone(), cached_glyph);
                log::debug!("Cached glyph (size: {}) with {}x{} bitmap", font_size, glyph_width, glyph_height);
            } else {
                log::warn!("Failed to rasterize glyph with cache key {:?}", cache_key);
                return None;
            }
        }
        
        self.glyph_cache.get(&glyph_key)
    }

    /// Allocate space in the atlas and upload glyph bitmap
    fn allocate_atlas_space(&mut self, width: u32, height: u32, bitmap: &[u8], queue: Option<&wgpu::Queue>) -> Option<[f32; 4]> {
        // Check if we need to move to next row
        if self.atlas_cursor.0 + width > self.atlas_size.0 {
            // Move to next row
            self.atlas_cursor.0 = 1; // Skip white pixel
            self.atlas_cursor.1 += self.current_row_height + 2; // Add padding
            self.current_row_height = 0;
        }
        
        // Track row height
        self.current_row_height = self.current_row_height.max(height);
        
        // Check if we have vertical space
        if self.atlas_cursor.1 + height <= self.atlas_size.1 {
            let coords = [
                self.atlas_cursor.0 as f32 / self.atlas_size.0 as f32,
                self.atlas_cursor.1 as f32 / self.atlas_size.1 as f32,
                (self.atlas_cursor.0 + width) as f32 / self.atlas_size.0 as f32,
                (self.atlas_cursor.1 + height) as f32 / self.atlas_size.1 as f32,
            ];
            
            // Upload to GPU if queue is available
            if let Some(queue) = queue {
                if let Some(atlas) = &self.glyph_atlas {
                    queue.write_texture(
                        wgpu::ImageCopyTexture {
                            texture: atlas,
                            mip_level: 0,
                            origin: wgpu::Origin3d {
                                x: self.atlas_cursor.0,
                                y: self.atlas_cursor.1,
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
                    log::debug!("Uploaded glyph to GPU atlas at ({}, {})", self.atlas_cursor.0, self.atlas_cursor.1);
                }
            }
            
            // Advance cursor
            self.atlas_cursor.0 += width + 1; // Add padding
            
            Some(coords)
        } else {
            log::warn!("Glyph atlas is full ({}x{} at cursor {:?})", width, height, self.atlas_cursor);
            None // Atlas is full
        }
    }

    /// Render text string and return positioned glyphs (Bevy pattern)
    pub fn render_text_string(&mut self, 
        text: &str, 
        font_size: f32, 
        position: Vec2,
        queue: &wgpu::Queue
    ) -> Vec<PositionedGlyph> {
        // Collect glyph info first to avoid borrowing issues
        let mut glyph_infos = Vec::new();
        {
            // Get or create text buffer (reuse existing)
            let buffer = self.get_text_layout(text, font_size);
            
            // Process layout runs from the buffer
            for run in buffer.layout_runs() {
                for glyph in run.glyphs {
                    let cache_key = CacheKey::new(glyph.font_id, glyph.glyph_id, font_size, (0.0, 0.0), cosmic_text::CacheKeyFlags::empty()).0;
                    glyph_infos.push((cache_key, glyph.x, glyph.y));
                }
            }
        }
        
        let mut positioned_glyphs = Vec::new();
        
        // Now rasterize glyphs
        for (cache_key, glyph_x, glyph_y) in glyph_infos {
            if let Some(cached_glyph) = self.get_or_rasterize_glyph(
                cache_key,
                font_size, 
                Some(queue)
            ) {
                positioned_glyphs.push(PositionedGlyph {
                    position: Vec2::new(
                        position.x + glyph_x, 
                        position.y + glyph_y
                    ),
                    atlas_coords: cached_glyph.texture_coords.unwrap_or([0.0, 0.0, 0.0, 0.0]),
                    size: Vec2::new(cached_glyph.metrics.width as f32, cached_glyph.metrics.height as f32),
                });
            }
        }
        
        positioned_glyphs
    }

    /// Legacy method for compatibility with existing UI code
    pub fn get_glyph(&mut self, _font_handle: FontHandle, character: char, size: f32) -> Option<&CachedGlyph> {
        self.get_glyph_with_queue(_font_handle, character, size, None)
    }
    
    /// Legacy method for compatibility - now uses proper buffer caching
    pub fn get_glyph_with_queue(&mut self, _font_handle: FontHandle, character: char, size: f32, queue: Option<&wgpu::Queue>) -> Option<&CachedGlyph> {
        // Collect glyph info first to avoid borrowing issues
        let glyph_info = {
            // Use single character as text for buffer lookup
            let buffer = self.get_text_layout(&character.to_string(), size);
            
            // Get the first glyph from the buffer
            let mut result = None;
            for run in buffer.layout_runs() {
                for glyph in run.glyphs {
                    let cache_key = CacheKey::new(glyph.font_id, glyph.glyph_id, size, (0.0, 0.0), cosmic_text::CacheKeyFlags::empty()).0;
                    result = Some((cache_key, glyph.w));
                    break;
                }
                if result.is_some() {
                    break;
                }
            }
            result
        };
        
        if let Some((cache_key, advance_width)) = glyph_info {
            // Rasterize the glyph
            if let Some(_) = self.get_or_rasterize_glyph(cache_key, size, queue) {
                // Update advance width
                let glyph_key = GlyphKey {
                    cache_key,
                    font_size: (size * 100.0) as u32,
                };
                
                if let Some(cached) = self.glyph_cache.get_mut(&glyph_key) {
                    cached.metrics.advance_width = advance_width;
                }
                
                return self.glyph_cache.get(&glyph_key);
            }
        }
        
        None
    }

    /// Get text dimensions for layout calculations
    pub fn measure_text(&mut self, text: &str, _font_handle: FontHandle, size: f32) -> Vec2 {
        let buffer = self.get_text_layout(text, size);
        
        let mut width = 0.0;
        let mut height = size;
        
        for run in buffer.layout_runs() {
            width += run.line_w;
            height = height.max(run.line_height);
        }
        
        Vec2::new(width, height)
    }

    /// Get the default font handle
    pub fn default_font(&self) -> Option<FontHandle> {
        log::debug!("default_font() called, returning: {:?}", self.default_font);
        self.default_font
    }
    
    /// Upload all pending glyphs to the GPU atlas (no-op since we upload immediately)
    pub fn upload_pending_glyphs(&mut self, _queue: &wgpu::Queue) {
        // cosmic-text uploads glyphs immediately during rasterization
    }
    
    /// Get the glyph atlas texture for binding in shaders
    pub fn glyph_atlas(&self) -> Option<&wgpu::Texture> {
        self.glyph_atlas.as_ref()
    }
    
    /// Get the atlas size
    pub fn atlas_size(&self) -> (u32, u32) {
        self.atlas_size
    }
    
    /// Initialize a white pixel at (0,0) in the atlas for solid color rendering
    fn init_white_pixel(&self, queue: &wgpu::Queue) {
        if let Some(atlas) = &self.glyph_atlas {
            // Create a single white pixel
            let white_pixel = [255u8]; // Single grayscale white pixel
            
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: atlas,
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
            
            log::debug!("Initialized white pixel at (0,0) in glyph atlas for solid rendering");
        }
    }
}