//! Text rendering pipeline using glyphon for optimized WGPU integration
//!
//! This module provides a clean separation between text layout and rendering,
//! using glyphon's specialized WGPU text rendering capabilities for better
//! performance and simplified management.

use crate::{RenderResult, FontHandle};
use glam::Vec2;
use glyphon::{
    FontSystem, SwashCache, TextAtlas, TextRenderer, TextArea, TextBounds,
    Buffer, Metrics, Family, Attrs, Shaping, Resolution, Color as GlyphonColor
};
// HashMap no longer needed with glyphon
use thiserror::Error;

/// Text layout information for glyphon-based rendering
#[derive(Debug)]
pub struct TextLayoutInfo {
    /// Buffer for text rendering (to be used with glyphon)
    pub buffer: Buffer,
    /// Position for the text
    pub position: Vec2,
    /// Color for the text  
    pub color: GlyphonColor,
    /// Total text bounds
    pub size: Vec2,
}

/// Text measurement information for layout calculations
#[derive(Debug, Clone)]
pub struct TextMeasurement {
    /// Total size of the text
    pub size: Vec2,
    /// Distance from baseline to top of tallest character
    pub ascent: f32,
    /// Distance from baseline to bottom of lowest character
    pub descent: f32,
    /// Distance from top of text bounds to baseline
    pub baseline_offset: f32,
}

/// A text area ready for glyphon rendering
#[derive(Debug)]
pub struct TextAreaInfo {
    /// Text area bounds
    pub bounds: TextBounds,
    /// Text color
    pub color: GlyphonColor,
}

// Glyphon handles glyph caching internally - no manual cache management needed

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
    #[error("Rendering error: {0}")]
    RenderingError(String),
}

/// Text pipeline for layout and rendering using glyphon
pub struct TextPipeline {
    /// Glyphon font system
    font_system: FontSystem,
    /// Swash cache for glyph rasterization
    swash_cache: SwashCache,
    /// Glyphon text atlas for glyph management
    text_atlas: TextAtlas,
    /// Glyphon text renderer
    text_renderer: TextRenderer,
    /// Default font handle
    default_font: Option<FontHandle>,
    /// Screen resolution for proper scaling
    resolution: Resolution,
}

// Glyphon manages font atlas internally - no manual atlas management needed

impl TextPipeline {
    /// Create a new text pipeline using glyphon
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
    ) -> RenderResult<Self> {
        // Create glyphon components
        let mut font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        
        // Create text atlas with proper format
        let mut text_atlas = TextAtlas::new(device, queue, format);
        
        // Create text renderer
        let text_renderer = TextRenderer::new(
            &mut text_atlas,
            device,
            wgpu::MultisampleState::default(),
            None, // No depth stencil for UI text
        );
        
        // Set up screen resolution (will be updated on resize)
        let resolution = Resolution {
            width: 1920,
            height: 1080,
        };
        
        let mut pipeline = Self {
            font_system,
            swash_cache,
            text_atlas,
            text_renderer,
            default_font: None,
            resolution,
        };
        
        // Load default font
        let default_handle = pipeline.load_default_font()?;
        pipeline.default_font = Some(default_handle);
        
        Ok(pipeline)
    }
    
    /// Measure text dimensions without rendering (for layout calculations)
    pub fn measure_text(
        &mut self,
        text: &str,
        _font_handle: FontHandle,
        font_size: f32,
    ) -> Result<TextMeasurement, TextError> {
        let metrics = Metrics::new(font_size, font_size * 1.2);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        
        // Configure buffer for measurement
        buffer.set_size(&mut self.font_system, f32::MAX, f32::MAX); // Large size for measurement
        buffer.set_text(
            &mut self.font_system,
            text,
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );
        
        // Shape the text
        buffer.shape_until_scroll(&mut self.font_system);
        
        // Calculate text bounds using glyphon's layout information
        let mut width = 0.0f32;
        let mut height = 0.0f32;
        
        for run in buffer.layout_runs() {
            width = width.max(run.line_w);
            height += metrics.line_height;
        }
        
        // Handle empty text
        if text.is_empty() {
            return Ok(TextMeasurement {
                size: Vec2::ZERO,
                ascent: 0.0,
                descent: 0.0,
                baseline_offset: 0.0,
            });
        }
        
        // Calculate font metrics for proper centering
        let ascent = metrics.font_size * 0.8; // Approximate ascent
        let descent = metrics.font_size * 0.2; // Approximate descent
        let baseline_offset = ascent; // Distance from top to baseline
        
        Ok(TextMeasurement {
            size: Vec2::new(width, height),
            ascent,
            descent,
            baseline_offset,
        })
    }

    /// Queue text for layout and rendering using glyphon
    pub fn queue_text(
        &mut self,
        text: &str,
        _font_handle: FontHandle,
        font_size: f32,
        position: Vec2,
        color: [f32; 4],
        _queue: &wgpu::Queue,
    ) -> Result<TextLayoutInfo, TextError> {
        log::debug!("Queueing text: '{}' at size {} at position {:?}", text, font_size, position);
        
        // Create buffer for text shaping
        let metrics = Metrics::new(font_size, font_size * 1.2); // 1.2 line height
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        
        // Configure buffer
        buffer.set_size(&mut self.font_system, f32::MAX, f32::MAX); // Large size for layout
        buffer.set_text(
            &mut self.font_system,
            text,
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );
        
        // Shape the text
        buffer.shape_until_scroll(&mut self.font_system);
        
        // Calculate text bounds
        let mut width = 0.0f32;
        let mut height = 0.0f32;
        
        for run in buffer.layout_runs() {
            width = width.max(run.line_w);
            height += metrics.line_height;
        }
        
        let glyphon_color = GlyphonColor::rgba(
            (color[0] * 255.0) as u8,
            (color[1] * 255.0) as u8,
            (color[2] * 255.0) as u8,
            (color[3] * 255.0) as u8,
        );
        
        log::debug!("Generated text buffer for text '{}' with bounds {}x{}", text, width, height);
        
        Ok(TextLayoutInfo {
            buffer,
            position,
            color: glyphon_color,
            size: Vec2::new(width, height),
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
    
    /// Get the glyphon text atlas
    pub fn text_atlas(&self) -> &TextAtlas {
        &self.text_atlas
    }
    
    /// Get mutable reference to text atlas
    pub fn text_atlas_mut(&mut self) -> &mut TextAtlas {
        &mut self.text_atlas
    }
    
    /// Get the glyphon text renderer
    pub fn text_renderer(&self) -> &TextRenderer {
        &self.text_renderer
    }
    
    /// Get mutable reference to text renderer
    pub fn text_renderer_mut(&mut self) -> &mut TextRenderer {
        &mut self.text_renderer
    }
    
    /// Update screen resolution for proper text scaling
    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.resolution = Resolution { width, height };
    }
    
    /// Prepare text layout infos for rendering
    pub fn prepare_text_layouts(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        text_layouts: &[TextLayoutInfo],
    ) -> Result<(), TextError> {
        // Convert TextLayoutInfo to TextArea for glyphon
        let text_areas: Vec<TextArea> = text_layouts.iter().map(|layout| {
            TextArea {
                buffer: &layout.buffer,
                left: layout.position.x,
                top: layout.position.y,
                scale: 1.0,
                bounds: TextBounds {
                    left: layout.position.x as i32,
                    top: layout.position.y as i32,
                    right: (layout.position.x + layout.size.x) as i32,
                    bottom: (layout.position.y + layout.size.y) as i32,
                },
                default_color: layout.color,
            }
        }).collect();
        
        self.text_renderer
            .prepare(
                device,
                queue,
                &mut self.font_system,
                &mut self.text_atlas,
                self.resolution,
                text_areas,
                &mut self.swash_cache,
            )
            .map_err(|e| TextError::RenderingError(format!("Failed to prepare text areas: {:?}", e)))
    }
    
    /// Render text areas to a render pass
    pub fn render_text_areas<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) -> Result<(), TextError> {
        self.text_renderer
            .render(&self.text_atlas, render_pass)
            .map_err(|e| TextError::RenderingError(format!("Failed to render text areas: {:?}", e)))
    }
}