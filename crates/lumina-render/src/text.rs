//! Text rendering pipeline - TEMPORARILY DISABLED due to glyphon version conflicts
//!
//! This module will be re-enabled once glyphon version compatibility is resolved.

// ENTIRE MODULE TEMPORARILY COMMENTED OUT - glyphon version conflicts with wgpu 0.20
// This will be restored once we find a compatible glyphon version or upgrade everything together

// Placeholder structs to prevent compilation errors in dependent code
use glam::Vec2;

/// Text layout information - placeholder while text rendering is disabled
#[derive(Debug)]
pub struct TextLayoutInfo {
    pub position: Vec2,
    pub size: Vec2,
}

/// Text measurement information - placeholder 
#[derive(Debug, Clone)]
pub struct TextMeasurement {
    pub size: Vec2,
    pub ascent: f32,
    pub descent: f32,
    pub baseline_offset: f32,
}

/// Text area information - placeholder
#[derive(Debug)]
pub struct TextAreaInfo {
    pub size: Vec2,
}

/// Text rendering errors - placeholder
#[derive(Debug)]
pub enum TextError {
    /// Temporarily disabled
    Disabled,
    /// Render error wrapper
    RenderError(crate::RenderError),
}

impl std::fmt::Display for TextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextError::Disabled => write!(f, "Text rendering temporarily disabled"),
            TextError::RenderError(e) => write!(f, "Render error: {}", e),
        }
    }
}

impl std::error::Error for TextError {}

impl From<crate::RenderError> for TextError {
    fn from(err: crate::RenderError) -> Self {
        TextError::RenderError(err)
    }
}

/// Text renderer - placeholder while disabled
pub struct TextRenderer {
    _placeholder: (),
}

impl TextRenderer {
    pub fn new(_device: &wgpu::Device, _queue: &wgpu::Queue, _format: wgpu::TextureFormat) -> crate::RenderResult<Self> {
        Ok(Self { _placeholder: () })
    }
    
    pub fn add_text(&mut self, _text: &str, _font_size: f32, _position: Vec2, _color: glam::Vec4) -> crate::RenderResult<TextLayoutInfo> {
        Ok(TextLayoutInfo {
            position: _position,
            size: Vec2::new(100.0, 20.0), // Placeholder size
        })
    }
    
    pub fn measure_text(&mut self, _text: &str, _font_size: f32) -> crate::RenderResult<TextMeasurement> {
        Ok(TextMeasurement {
            size: Vec2::new(100.0, 20.0),
            ascent: 15.0,
            descent: 5.0,
            baseline_offset: 15.0,
        })
    }
    
    pub fn prepare(&mut self, _device: &wgpu::Device, _queue: &wgpu::Queue) -> crate::RenderResult<()> {
        Ok(())
    }
    
    pub fn render(&mut self, _render_pass: &mut wgpu::RenderPass) -> crate::RenderResult<()> {
        // No-op while disabled
        Ok(())
    }
}

/// Text pipeline - placeholder while disabled
pub struct TextPipeline {
    _placeholder: (),
}

impl TextPipeline {
    pub fn new(_device: &wgpu::Device, _queue: &wgpu::Queue, _format: wgpu::TextureFormat) -> crate::RenderResult<Self> {
        Ok(Self { _placeholder: () })
    }
    
    pub fn set_resolution(&mut self, _width: u32, _height: u32) {
        // No-op while disabled
    }
    
    pub fn prepare_text_layouts(&mut self, _device: &wgpu::Device, _queue: &wgpu::Queue, _layouts: &[TextLayoutInfo]) -> crate::RenderResult<()> {
        // No-op while disabled
        Ok(())
    }
    
    pub fn render_text_areas(&mut self, _render_pass: &mut wgpu::RenderPass) -> crate::RenderResult<()> {
        // No-op while disabled
        Ok(())
    }
    
    pub fn measure_text(&mut self, _text: &str, _font: crate::FontHandle, _size: f32) -> Result<TextMeasurement, TextError> {
        Ok(TextMeasurement {
            size: Vec2::new(100.0, 20.0),
            ascent: 15.0,
            descent: 5.0,
            baseline_offset: 15.0,
        })
    }
    
    pub fn queue_text(&mut self, _text: &str, _font: crate::FontHandle, _size: f32, _position: Vec2, _color_array: [f32; 4], _queue: &wgpu::Queue) -> crate::RenderResult<TextLayoutInfo> {
        Ok(TextLayoutInfo {
            position: _position,
            size: Vec2::new(100.0, 20.0),
        })
    }
    
    pub fn default_font(&self) -> Option<crate::FontHandle> {
        Some(crate::FontHandle(0))
    }
}