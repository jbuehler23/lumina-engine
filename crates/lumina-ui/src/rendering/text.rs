//! Text rendering for the UI framework

use crate::{error::RenderError, rendering::{FontHandle, TextureHandle}};
use glam::{Vec2, Vec4};
use fontdue::{Font, FontSettings};
use std::collections::HashMap;

/// Text renderer that handles font rasterization and text layout
#[derive(Debug)]
pub struct TextRenderer {
    /// Loaded fonts
    fonts: HashMap<FontHandle, Font>,
    /// Font texture atlas
    texture_atlas: TextureAtlas,
    /// Glyph cache
    glyph_cache: HashMap<GlyphKey, GlyphInfo>,
    /// Next font handle ID
    next_font_id: u32,
}

/// Key for caching rendered glyphs
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct GlyphKey {
    font_id: FontHandle,
    character: char,
    size: u32, // Size in pixels, rounded
}

/// Information about a cached glyph
#[derive(Debug, Clone)]
struct GlyphInfo {
    /// Position in texture atlas
    atlas_position: Vec2,
    /// Size of glyph in pixels
    size: Vec2,
    /// Bearing (offset from baseline)
    bearing: Vec2,
    /// Advance width
    advance: f32,
}

/// Texture atlas for storing font glyphs
#[derive(Debug)]
struct TextureAtlas {
    /// Current atlas texture
    texture: Option<wgpu::Texture>,
    /// Atlas size
    size: Vec2,
    /// Current packing position
    cursor: Vec2,
    /// Row height for current row
    row_height: f32,
}

impl TextRenderer {
    /// Create a new text renderer
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
    ) -> Result<Self, RenderError> {
        Ok(Self {
            fonts: HashMap::new(),
            texture_atlas: TextureAtlas::new(device, 1024, 1024),
            glyph_cache: HashMap::new(),
            next_font_id: 0,
        })
    }
    
    /// Load a font from bytes
    pub fn load_font(&mut self, font_data: &[u8]) -> Result<FontHandle, RenderError> {
        let font = Font::from_bytes(font_data, FontSettings::default())
            .map_err(|e| RenderError::Texture(format!("Failed to load font: {}", e)))?;
        
        let handle = FontHandle(self.next_font_id);
        self.next_font_id += 1;
        
        self.fonts.insert(handle, font);
        Ok(handle)
    }
    
    /// Draw text at the specified position
    pub fn draw_text(&mut self, text: &str, position: Vec2, font: FontHandle, size: f32, color: Vec4) {
        // TODO: Implement text rendering
        // This would:
        // 1. Get the font from the handle
        // 2. Rasterize each character
        // 3. Pack glyphs into texture atlas
        // 4. Generate quads for each character
        // 5. Submit draw commands
        
        log::warn!("Text rendering not yet implemented");
    }
    
    /// Measure the size of text
    pub fn measure_text(&self, text: &str, font: FontHandle, size: f32) -> Vec2 {
        if let Some(font_data) = self.fonts.get(&font) {
            let mut width = 0.0;
            let mut height = size;
            
            for ch in text.chars() {
                let metrics = font_data.metrics(ch, size);
                width += metrics.advance_width;
                height = height.max(metrics.height);
            }
            
            Vec2::new(width, height)
        } else {
            Vec2::ZERO
        }
    }
}

impl TextureAtlas {
    /// Create a new texture atlas
    fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Font Texture Atlas"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        
        Self {
            texture: Some(texture),
            size: Vec2::new(width as f32, height as f32),
            cursor: Vec2::ZERO,
            row_height: 0.0,
        }
    }
}