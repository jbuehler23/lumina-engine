//! Text rendering implementation
//!
//! Provides font loading, glyph caching, and text rendering capabilities.

use crate::{RenderResult, FontHandle};
use glam::{Vec2, Vec4};
use fontdue::{Font as FontdueFont, FontSettings};

/// Text rendering system
pub struct TextRenderer {
    /// Font cache
    fonts: Vec<Font>,
    /// Glyph cache
    glyph_cache: GlyphCache,
    /// Default font handle
    default_font: Option<FontHandle>,
}

/// Font data and fontdue font instance
pub struct Font {
    /// Font name
    pub name: String,
    /// Fontdue font instance
    pub font: FontdueFont,
}

/// Glyph cache for efficient text rendering
pub struct GlyphCache {
    /// Cached glyphs
    glyphs: std::collections::HashMap<(FontHandle, char, u32), CachedGlyph>,
}

/// A cached glyph with rendering data
pub struct CachedGlyph {
    /// Glyph metrics
    pub metrics: fontdue::Metrics,
    /// Glyph bitmap data (grayscale)
    pub bitmap: Vec<u8>,
    /// Texture coordinates if uploaded to GPU
    pub texture_coords: Option<[f32; 4]>,
}

// For now, we'll use a simple fallback approach without embedding fonts
// TODO: Add proper font embedding in the future

impl TextRenderer {
    /// Create a new text renderer
    pub fn new(
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _format: wgpu::TextureFormat,
    ) -> RenderResult<Self> {
        let mut renderer = Self {
            fonts: Vec::new(),
            glyph_cache: GlyphCache {
                glyphs: std::collections::HashMap::new(),
            },
            default_font: None,
        };
        
        // Load default font
        let default_handle = renderer.load_default_font()?;
        renderer.default_font = Some(default_handle);
        
        Ok(renderer)
    }

    /// Load a font from bytes
    pub fn load_font(&mut self, name: String, font_data: &[u8]) -> RenderResult<FontHandle> {
        let font = FontdueFont::from_bytes(font_data, FontSettings::default())
            .map_err(|e| crate::RenderError::FontLoad(format!("Failed to load font: {}", e)))?;
        
        self.fonts.push(Font {
            name,
            font,
        });

        Ok(FontHandle((self.fonts.len() - 1) as u32))
    }

    /// Load default system font (simplified approach)
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
                log::info!("Loading default font from: {}", font_path);
                return self.load_font("Default".to_string(), &font_data);
            }
        }
        
        log::warn!("No font file found in assets/fonts/, using placeholder");
        // Create a placeholder font entry for development
        // This will allow the system to work without crashing while we develop the UI
        Ok(FontHandle(0))
    }

    /// Get or rasterize a glyph
    pub fn get_glyph(&mut self, font_handle: FontHandle, character: char, size: f32) -> Option<&CachedGlyph> {
        let size_key = (size * 10.0) as u32; // Round to nearest 0.1
        let cache_key = (font_handle, character, size_key);
        
        if !self.glyph_cache.glyphs.contains_key(&cache_key) {
            // Try to rasterize the glyph if we have fonts loaded
            if let Some(font) = self.fonts.get(font_handle.0 as usize) {
                let (metrics, bitmap) = font.font.rasterize(character, size);
                
                let cached_glyph = CachedGlyph {
                    metrics,
                    bitmap,
                    texture_coords: None,
                };
                
                self.glyph_cache.glyphs.insert(cache_key, cached_glyph);
            } else {
                // Fallback: create a simple placeholder glyph for development
                // Since we don't have proper fontdue metrics, we'll create minimal ones
                return None; // For now, just return None to fallback to simpler rendering
            }
        }
        
        self.glyph_cache.glyphs.get(&cache_key)
    }

    /// Get text dimensions for layout calculations
    pub fn measure_text(&mut self, text: &str, font_handle: FontHandle, size: f32) -> Vec2 {
        if let Some(font) = self.fonts.get(font_handle.0 as usize) {
            let mut width = 0.0;
            let mut height = size;
            
            for ch in text.chars() {
                let (metrics, _) = font.font.rasterize(ch, size);
                width += metrics.advance_width;
                height = height.max(metrics.height as f32);
            }
            
            Vec2::new(width, height)
        } else {
            // Fallback measurement for development
            let char_width = size * 0.6;
            let char_count = text.chars().count();
            Vec2::new(char_count as f32 * char_width, size)
        }
    }

    /// Get font by handle
    pub fn get_font(&self, handle: FontHandle) -> Option<&Font> {
        self.fonts.get(handle.0 as usize)
    }
    
    /// Get the default font handle
    pub fn default_font(&self) -> Option<FontHandle> {
        self.default_font
    }
}