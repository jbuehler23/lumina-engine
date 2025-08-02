//! Text rendering pipeline using glyphon

use crate::{RenderResult, Rect};
use glam::{Vec2, Vec4};
use glyphon::{
    Attrs,
    Buffer,
    Color,
    FontSystem,
    Metrics,
    Resolution,
    SwashCache,
    TextArea,
    TextAtlas,
    TextBounds,
    TextRenderer,
};

/// Text pipeline for rendering text with glyphon
pub struct TextPipeline {
    font_system: FontSystem,
    cache: SwashCache,
    atlas: TextAtlas,
    text_renderer: TextRenderer,
    buffer: Buffer,
    resolution: Resolution,
}

impl TextPipeline {
    /// Create a new text pipeline
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
    ) -> RenderResult<Self> {
        let font_system = FontSystem::new();
        let cache = SwashCache::new();
        let mut atlas = TextAtlas::new(device, queue, format);
        let text_renderer = TextRenderer::new(&mut atlas, device, wgpu::MultisampleState::default(), None)?;
        let buffer = Buffer::new(&font_system, Metrics::new(30.0, 42.0));

        Ok(Self {
            font_system,
            cache,
            atlas,
            text_renderer,
            buffer,
            resolution: Resolution { width: 0, height: 0 },
        })
    }

    /// Set the screen resolution
    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.resolution = Resolution { width, height };
    }

    /// Prepare text for rendering
    pub fn prepare_text(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        text_areas: &[TextArea],
    ) -> RenderResult<()> {
        self.text_renderer.prepare(
            device,
            queue,
            &mut self.font_system,
            &mut self.atlas,
            self.resolution,
            text_areas,
            &mut self.cache,
        )?;
        Ok(())
    }

    /// Render the prepared text
    pub fn render_text<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) -> RenderResult<()> {
        self.text_renderer.render(&self.atlas, render_pass)?;
        Ok(())
    }

    /// Create a text area for rendering
    pub fn create_text_area<'a>(
        &'a mut self,
        text: &'a str,
        position: Vec2,
        size: Vec2,
        color: Color,
    ) -> TextArea<'a> {
        self.buffer.set_text(&mut self.font_system, text, Attrs::new().color(color), glyphon::Shaping::Advanced);
        self.buffer.set_size(&mut self.font_system, size.x, size.y);
        TextArea {
            buffer: &self.buffer,
            left: position.x,
            top: position.y,
            scale: 1.0,
            bounds: TextBounds {
                left: position.x as i32,
                top: position.y as i32,
                right: (position.x + size.x) as i32,
                bottom: (position.y + size.y) as i32,
            },
            default_color: color,
        }
    }
}
