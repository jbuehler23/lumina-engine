//! WGPU-based UI rendering system
//!
//! This module provides specialized rendering capabilities for immediate-mode UI interfaces.
//! It handles batching, clipping, and efficient rendering of UI primitives like rectangles,
//! text, and textured quads.

use crate::{Rect, RenderError, RenderResult, TextRenderer};
use glam::{Vec2, Vec4, Mat4};
use bytemuck::{Pod, Zeroable};

/// Main UI renderer using WGPU
pub struct UiRenderer {
    /// Surface configuration
    config: wgpu::SurfaceConfiguration,
    /// Current render pass
    current_pass: Option<wgpu::RenderPass<'static>>,
    /// Vertex buffer for UI quads
    vertex_buffer: wgpu::Buffer,
    /// Index buffer for UI quads
    index_buffer: wgpu::Buffer,
    /// Uniform buffer for view/projection matrices
    uniform_buffer: wgpu::Buffer,
    /// Bind group for uniforms
    uniform_bind_group: wgpu::BindGroup,
    /// Render pipeline for solid colors
    solid_pipeline: wgpu::RenderPipeline,
    /// Render pipeline for textured quads
    texture_pipeline: wgpu::RenderPipeline,
    /// Text renderer for proper font-based text rendering
    text_renderer: TextRenderer,
    /// Texture bind group for glyph atlas
    glyph_atlas_bind_group: wgpu::BindGroup,
    /// Sampler for texture filtering
    texture_sampler: wgpu::Sampler,
    /// Current frame's vertices
    vertices: Vec<UiVertex>,
    /// Current frame's indices
    indices: Vec<u16>,
    /// Screen size
    screen_size: Vec2,
    /// View matrix
    view_matrix: Mat4,
    /// Projection matrix
    projection_matrix: Mat4,
    /// Current clip stack
    clip_stack: Vec<Rect>,
}

/// Vertex data for UI rendering
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct UiVertex {
    /// Position in screen space
    pub position: [f32; 2],
    /// Texture coordinates
    pub tex_coords: [f32; 2],
    /// Color (RGBA)
    pub color: [f32; 4],
}

/// Uniform data for shaders
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct UiUniforms {
    /// View-projection matrix
    pub view_proj: [[f32; 4]; 4],
    /// Screen size
    pub screen_size: [f32; 2],
    /// Padding for alignment
    pub _padding: [f32; 2],
}

/// Draw command for rendering
#[derive(Debug, Clone)]
pub enum DrawCommand {
    /// Draw a colored rectangle
    Rect {
        bounds: Rect,
        color: Vec4,
        border_radius: f32,
    },
    /// Draw a textured rectangle
    TexturedRect {
        bounds: Rect,
        texture: TextureHandle,
        color: Vec4,
    },
    /// Draw text
    Text {
        text: String,
        position: Vec2,
        font: FontHandle,
        size: f32,
        color: Vec4,
    },
    /// Push a clip rectangle
    PushClip { bounds: Rect },
    /// Pop the last clip rectangle
    PopClip,
}

/// Handle to a texture resource
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureHandle(pub u32);

/// Handle to a font resource
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontHandle(pub u32);

impl UiRenderer {
    /// Create a new UI renderer
    pub async fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: wgpu::SurfaceConfiguration,
    ) -> RenderResult<Self> {
        let screen_size = Vec2::new(config.width as f32, config.height as f32);
        
        // Create buffers with larger capacity for text rendering
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("UI Vertex Buffer"),
            size: std::mem::size_of::<UiVertex>() as u64 * 100000, // Increased capacity for bitmap text
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("UI Index Buffer"),
            size: std::mem::size_of::<u16>() as u64 * 150000, // Increased capacity for bitmap text
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("UI Uniform Buffer"),
            size: std::mem::size_of::<UiUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Create bind group layout for uniforms
        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("UI Uniform Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        
        // Create bind group
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("UI Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
        });
        
        // Create shaders
        let solid_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("UI Solid Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/solid.wgsl").into()),
        });
        
        let texture_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("UI Texture Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/texture.wgsl").into()),
        });
        
        // Create texture bind group layout
        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("UI Texture Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
            ],
        });

        // Create pipeline layouts
        let solid_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("UI Solid Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });
        
        let texture_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("UI Texture Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });
        
        // Create render pipelines
        let solid_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("UI Solid Pipeline"),
            layout: Some(&solid_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &solid_shader,
                entry_point: "vs_main",
                buffers: &[UiVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &solid_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        
        let texture_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("UI Texture Pipeline"),
            layout: Some(&texture_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &texture_shader,
                entry_point: "vs_main",
                buffers: &[UiVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &texture_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        
        // Create matrices
        let view_matrix = Mat4::IDENTITY;
        let projection_matrix = Mat4::orthographic_rh(
            0.0, screen_size.x,
            screen_size.y, 0.0,
            -1.0, 1.0,
        );
        
        // Create text renderer
        let text_renderer = TextRenderer::new(device, queue, config.format)?;
        
        // Create texture sampler for font atlas
        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Font Atlas Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: None,
            lod_min_clamp: 0.0,
            lod_max_clamp: 32.0,
            border_color: None,
            anisotropy_clamp: 1,
        });
        
        // Create texture view for the glyph atlas
        let glyph_atlas_texture = text_renderer.glyph_atlas().ok_or_else(|| {
            crate::RenderError::GraphicsInit("Text renderer missing glyph atlas".to_string())
        })?;
        let glyph_atlas_view = glyph_atlas_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Create bind group for glyph atlas
        let glyph_atlas_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Glyph Atlas Bind Group"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&texture_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&glyph_atlas_view),
                },
            ],
        });
        
        Ok(Self {
            config,
            current_pass: None,
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            uniform_bind_group,
            solid_pipeline,
            texture_pipeline,
            text_renderer,
            glyph_atlas_bind_group,
            texture_sampler,
            vertices: Vec::new(),
            indices: Vec::new(),
            screen_size,
            view_matrix,
            projection_matrix,
            clip_stack: Vec::new(),
        })
    }
    
    /// Begin a new frame
    pub fn begin_frame(&mut self, queue: &wgpu::Queue) {
        self.vertices.clear();
        self.indices.clear();
        self.clip_stack.clear();
        
        // Update uniforms
        let uniforms = UiUniforms {
            view_proj: (self.projection_matrix * self.view_matrix).to_cols_array_2d(),
            screen_size: [self.screen_size.x, self.screen_size.y],
            _padding: [0.0, 0.0],
        };
        
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }
    
    /// End the current frame and submit all draw commands
    pub fn end_frame(&mut self, queue: &wgpu::Queue) {
        // Upload any pending glyphs to the GPU atlas
        self.upload_pending_glyphs(queue);
        
        // Update vertex and index buffers
        if !self.vertices.is_empty() {
            queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
        }
        if !self.indices.is_empty() {
            queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&self.indices));
        }
    }
    
    /// Submit the rendered UI to a render pass
    pub fn submit_to_render_pass<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.vertices.is_empty() || self.indices.is_empty() {
            return;
        }
        
        // Set vertex and index buffers (same for both pipelines)
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        
        // For now, render everything with the texture pipeline and glyph atlas
        // This handles both textured glyphs and solid quads (solid quads will use white texture)
        render_pass.set_pipeline(&self.texture_pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_bind_group(1, &self.glyph_atlas_bind_group, &[]);
        
        // Draw all vertices with texture pipeline
        let num_indices = self.indices.len() as u32;
        if num_indices > 0 {
            render_pass.draw_indexed(0..num_indices, 0, 0..1);
        }
    }
    
    /// Draw a colored rectangle
    pub fn draw_rect(&mut self, bounds: Rect, color: Vec4) {
        // Use a tiny white pixel at (0,0) in the atlas for solid color rendering
        let white_pixel_uv = 0.5 / self.text_renderer.atlas_size().0 as f32; // Half pixel size
        self.add_quad(bounds, color, [0.0, 0.0], [white_pixel_uv, white_pixel_uv]);
    }
    
    /// Draw a rectangle with rounded corners
    pub fn draw_rounded_rect(&mut self, bounds: Rect, color: Vec4, _border_radius: f32) {
        // For now, just draw a regular rectangle
        // TODO: Implement proper rounded rectangle rendering
        self.draw_rect(bounds, color);
    }
    
    /// Draw a textured rectangle
    pub fn draw_textured_rect(&mut self, bounds: Rect, color: Vec4) {
        self.add_quad(bounds, color, [0.0, 0.0], [1.0, 1.0]);
    }
    
    /// Draw text using TTF font glyph bitmaps with GPU atlas optimization
    pub fn draw_text(&mut self, text: &str, position: Vec2, font: FontHandle, size: f32, color: Vec4) {
        let mut cursor_x = position.x;
        let cursor_y = position.y;
        
        for ch in text.chars() {
            if ch == ' ' {
                // Space character - just advance cursor
                let space_advance = size * 0.25; // Standard space width
                cursor_x += space_advance;
                continue;
            }
            
            // Get the glyph from the text renderer and extract data
            let (glyph_bitmap, glyph_metrics, texture_coords, advance_width) = if let Some(glyph) = self.text_renderer.get_glyph(font, ch, size) {
                // Extract the data we need before any potential mutable borrows
                let bitmap = glyph.bitmap.clone();
                let metrics = glyph.metrics.clone();
                let coords = glyph.texture_coords;
                let advance = metrics.advance_width;
                (Some(bitmap), Some(metrics), coords, advance)
            } else {
                (None, None, None, size * 0.5) // Fallback advance width
            };
            
            if let (Some(bitmap), Some(metrics)) = (glyph_bitmap, glyph_metrics) {
                // Use actual glyph metrics and bitmap
                let glyph_width = metrics.width as f32;
                let glyph_height = metrics.height as f32;
                
                if !bitmap.is_empty() && glyph_width > 0.0 && glyph_height > 0.0 {
                    // Calculate glyph position based on baseline and bearing
                    let glyph_x = cursor_x + metrics.x as f32;
                    let glyph_y = cursor_y + (size * 0.8) - metrics.y as f32; // Adjust for baseline
                    
                    let glyph_bounds = Rect::new(
                        glyph_x,
                        glyph_y,
                        glyph_width,
                        glyph_height,
                    );
                    
                    // Check if we have texture coordinates for GPU atlas rendering
                    if let Some(coords) = texture_coords {
                        // Use optimized GPU atlas rendering
                        log::debug!("Rendering glyph '{}' using GPU atlas at coords {:?}", ch, coords);
                        self.draw_textured_glyph(glyph_bounds, color, coords);
                    } else {
                        // Fallback to pixel-by-pixel rendering for now
                        log::debug!("Rendering glyph '{}' using pixel bitmap ({}x{})", ch, metrics.width, metrics.height);
                        self.draw_glyph_bitmap(&bitmap, glyph_bounds, color, 
                                             metrics.width as usize, metrics.height as usize);
                    }
                }
                
                // Advance cursor by the glyph's advance width
                cursor_x += advance_width;
            } else {
                // Fallback: use readable character representation if no glyph available
                log::warn!("No glyph found for character '{}', using fallback shape", ch);
                let char_width = size * 0.5;
                self.draw_readable_char(ch, Vec2::new(cursor_x, cursor_y), size, color);
                cursor_x += char_width;
            }
        }
    }
    
    /// Draw a readable character using simple but recognizable shapes
    fn draw_readable_char(&mut self, ch: char, position: Vec2, size: f32, color: Vec4) {
        let char_width = size * 0.5;
        let char_height = size * 0.8;
        
        // Base character rectangle
        let base_rect = Rect::new(
            position.x + char_width * 0.1,
            position.y + size * 0.1,
            char_width * 0.8,
            char_height * 0.6,
        );
        
        match ch {
            // Draw distinctive shapes for different character types
            'A'..='Z' => {
                // Uppercase: full height rectangle with small top accent
                self.draw_rect(base_rect, color);
                let top_accent = Rect::new(
                    position.x + char_width * 0.3,
                    position.y,
                    char_width * 0.4,
                    size * 0.15,
                );
                self.draw_rect(top_accent, color);
            },
            'a'..='z' => {
                // Lowercase: smaller rectangle
                let lower_rect = Rect::new(
                    position.x + char_width * 0.1,
                    position.y + size * 0.25,
                    char_width * 0.8,
                    char_height * 0.5,
                );
                self.draw_rect(lower_rect, color);
            },
            '0'..='9' => {
                // Numbers: distinctive square shape
                let num_rect = Rect::new(
                    position.x + char_width * 0.15,
                    position.y + size * 0.1,
                    char_width * 0.7,
                    char_height * 0.7,
                );
                self.draw_rect(num_rect, color);
                
                // Add number-specific details
                match ch {
                    '1' => {
                        let thin_rect = Rect::new(
                            position.x + char_width * 0.4,
                            position.y + size * 0.05,
                            char_width * 0.2,
                            char_height * 0.8,
                        );
                        self.draw_rect(thin_rect, color);
                    },
                    '0' => {
                        // Draw as hollow rectangle
                        let inner_rect = Rect::new(
                            position.x + char_width * 0.25,
                            position.y + size * 0.2,
                            char_width * 0.5,
                            char_height * 0.5,
                        );
                        // Note: We'd need a hollow rect function for this, using solid for now
                        self.draw_rect(num_rect, color);
                    },
                    _ => {
                        self.draw_rect(num_rect, color);
                    }
                }
            },
            '.' => {
                // Period: small dot at bottom
                let dot = Rect::new(
                    position.x + char_width * 0.4,
                    position.y + size * 0.7,
                    char_width * 0.2,
                    size * 0.15,
                );
                self.draw_rect(dot, color);
            },
            ',' => {
                // Comma: small dot with tail
                let dot = Rect::new(
                    position.x + char_width * 0.4,
                    position.y + size * 0.7,
                    char_width * 0.2,
                    size * 0.2,
                );
                self.draw_rect(dot, color);
            },
            ':' => {
                // Colon: two dots
                let top_dot = Rect::new(
                    position.x + char_width * 0.4,
                    position.y + size * 0.2,
                    char_width * 0.2,
                    size * 0.15,
                );
                let bottom_dot = Rect::new(
                    position.x + char_width * 0.4,
                    position.y + size * 0.6,
                    char_width * 0.2,
                    size * 0.15,
                );
                self.draw_rect(top_dot, color);
                self.draw_rect(bottom_dot, color);
            },
            _ => {
                // Default: draw base rectangle for other characters
                self.draw_rect(base_rect, color);
            }
        }
    }
    
    /// Draw a textured glyph using the font atlas texture for optimal performance
    fn draw_textured_glyph(&mut self, bounds: Rect, color: Vec4, texture_coords: [f32; 4]) {
        // Add a textured quad using the atlas coordinates
        // texture_coords = [u_min, v_min, u_max, v_max] in atlas space
        self.add_quad(bounds, color, [texture_coords[0], texture_coords[1]], [texture_coords[2], texture_coords[3]]);
    }
    
    /// Upload any pending glyphs to the GPU atlas
    fn upload_pending_glyphs(&mut self, queue: &wgpu::Queue) {
        self.text_renderer.upload_pending_glyphs(queue);
    }
    
    /// Draw glyph bitmap as individual pixels for readable text
    fn draw_glyph_bitmap(&mut self, bitmap: &[u8], bounds: Rect, color: Vec4, width: usize, height: usize) {
        if bitmap.is_empty() || width == 0 || height == 0 {
            return;
        }
        
        let pixel_width = bounds.size.x / width as f32;
        let pixel_height = bounds.size.y / height as f32;
        
        for y in 0..height {
            for x in 0..width {
                let pixel_index = y * width + x;
                if pixel_index < bitmap.len() {
                    let alpha = bitmap[pixel_index] as f32 / 255.0;
                    
                    if alpha > 0.1 { // Only draw visible pixels
                        let pixel_rect = Rect::new(
                            bounds.position.x + x as f32 * pixel_width,
                            bounds.position.y + y as f32 * pixel_height,
                            pixel_width,
                            pixel_height,
                        );
                        
                        // Apply alpha to color for anti-aliasing
                        let pixel_color = Vec4::new(color.x, color.y, color.z, color.w * alpha);
                        self.draw_rect(pixel_rect, pixel_color);
                    }
                }
            }
        }
    }
    
    /// Get a simple 8x8 bitmap pattern for a character
    fn get_char_bitmap(&self, ch: char) -> [u8; 8] {
        match ch {
            // Uppercase letters
            'A' => [0x18, 0x3C, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x00],
            'B' => [0x7C, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x7C, 0x00],
            'C' => [0x3C, 0x66, 0x60, 0x60, 0x60, 0x66, 0x3C, 0x00],
            'D' => [0x78, 0x6C, 0x66, 0x66, 0x66, 0x6C, 0x78, 0x00],
            'E' => [0x7E, 0x60, 0x60, 0x78, 0x60, 0x60, 0x7E, 0x00],
            'F' => [0x7E, 0x60, 0x60, 0x78, 0x60, 0x60, 0x60, 0x00],
            'G' => [0x3C, 0x66, 0x60, 0x6E, 0x66, 0x66, 0x3C, 0x00],
            'H' => [0x66, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00],
            'I' => [0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00],
            'J' => [0x1E, 0x0C, 0x0C, 0x0C, 0x0C, 0x6C, 0x38, 0x00],
            'K' => [0x66, 0x6C, 0x78, 0x70, 0x78, 0x6C, 0x66, 0x00],
            'L' => [0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x7E, 0x00],
            'M' => [0x63, 0x77, 0x7F, 0x6B, 0x63, 0x63, 0x63, 0x00],
            'N' => [0x66, 0x76, 0x7E, 0x7E, 0x6E, 0x66, 0x66, 0x00],
            'O' => [0x3C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
            'P' => [0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60, 0x60, 0x00],
            'Q' => [0x3C, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x0E, 0x00],
            'R' => [0x7C, 0x66, 0x66, 0x7C, 0x78, 0x6C, 0x66, 0x00],
            'S' => [0x3C, 0x66, 0x60, 0x3C, 0x06, 0x66, 0x3C, 0x00],
            'T' => [0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00],
            'U' => [0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
            'V' => [0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00],
            'W' => [0x63, 0x63, 0x63, 0x6B, 0x7F, 0x77, 0x63, 0x00],
            'X' => [0x66, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x66, 0x00],
            'Y' => [0x66, 0x66, 0x66, 0x3C, 0x18, 0x18, 0x18, 0x00],
            'Z' => [0x7E, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x7E, 0x00],
            
            // Lowercase letters
            'a' => [0x00, 0x00, 0x3C, 0x06, 0x3E, 0x66, 0x3E, 0x00],
            'b' => [0x60, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x7C, 0x00],
            'c' => [0x00, 0x00, 0x3C, 0x60, 0x60, 0x60, 0x3C, 0x00],
            'd' => [0x06, 0x06, 0x3E, 0x66, 0x66, 0x66, 0x3E, 0x00],
            'e' => [0x00, 0x00, 0x3C, 0x66, 0x7E, 0x60, 0x3C, 0x00],
            'f' => [0x0E, 0x18, 0x18, 0x7E, 0x18, 0x18, 0x18, 0x00],
            'g' => [0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x7C],
            'h' => [0x60, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x66, 0x00],
            'i' => [0x18, 0x00, 0x38, 0x18, 0x18, 0x18, 0x3C, 0x00],
            'j' => [0x06, 0x00, 0x0E, 0x06, 0x06, 0x66, 0x66, 0x3C],
            'k' => [0x60, 0x60, 0x6C, 0x78, 0x70, 0x78, 0x6C, 0x00],
            'l' => [0x38, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00],
            'm' => [0x00, 0x00, 0x66, 0x7F, 0x7F, 0x6B, 0x63, 0x00],
            'n' => [0x00, 0x00, 0x7C, 0x66, 0x66, 0x66, 0x66, 0x00],
            'o' => [0x00, 0x00, 0x3C, 0x66, 0x66, 0x66, 0x3C, 0x00],
            'p' => [0x00, 0x00, 0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60],
            'q' => [0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x06],
            'r' => [0x00, 0x00, 0x7C, 0x66, 0x60, 0x60, 0x60, 0x00],
            's' => [0x00, 0x00, 0x3E, 0x60, 0x3C, 0x06, 0x7C, 0x00],
            't' => [0x18, 0x18, 0x7E, 0x18, 0x18, 0x18, 0x0E, 0x00],
            'u' => [0x00, 0x00, 0x66, 0x66, 0x66, 0x66, 0x3E, 0x00],
            'v' => [0x00, 0x00, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00],
            'w' => [0x00, 0x00, 0x63, 0x6B, 0x7F, 0x3E, 0x36, 0x00],
            'x' => [0x00, 0x00, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x00],
            'y' => [0x00, 0x00, 0x66, 0x66, 0x66, 0x3E, 0x0C, 0x78],
            'z' => [0x00, 0x00, 0x7E, 0x0C, 0x18, 0x30, 0x7E, 0x00],
            
            // Numbers
            '0' => [0x3C, 0x66, 0x6E, 0x76, 0x66, 0x66, 0x3C, 0x00],
            '1' => [0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00],
            '2' => [0x3C, 0x66, 0x06, 0x0C, 0x30, 0x60, 0x7E, 0x00],
            '3' => [0x3C, 0x66, 0x06, 0x1C, 0x06, 0x66, 0x3C, 0x00],
            '4' => [0x06, 0x0E, 0x1E, 0x66, 0x7F, 0x06, 0x06, 0x00],
            '5' => [0x7E, 0x60, 0x7C, 0x06, 0x06, 0x66, 0x3C, 0x00],
            '6' => [0x3C, 0x66, 0x60, 0x7C, 0x66, 0x66, 0x3C, 0x00],
            '7' => [0x7E, 0x66, 0x0C, 0x18, 0x18, 0x18, 0x18, 0x00],
            '8' => [0x3C, 0x66, 0x66, 0x3C, 0x66, 0x66, 0x3C, 0x00],
            '9' => [0x3C, 0x66, 0x66, 0x3E, 0x06, 0x66, 0x3C, 0x00],
            
            // Special characters
            ' ' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            '.' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00],
            ',' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x30],
            ':' => [0x00, 0x00, 0x18, 0x18, 0x00, 0x18, 0x18, 0x00],
            ';' => [0x00, 0x00, 0x18, 0x18, 0x00, 0x18, 0x18, 0x30],
            '!' => [0x18, 0x18, 0x18, 0x18, 0x00, 0x00, 0x18, 0x00],
            '?' => [0x3C, 0x66, 0x06, 0x0C, 0x18, 0x00, 0x18, 0x00],
            '-' => [0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00],
            '+' => [0x00, 0x18, 0x18, 0x7E, 0x18, 0x18, 0x00, 0x00],
            '=' => [0x00, 0x00, 0x7E, 0x00, 0x7E, 0x00, 0x00, 0x00],
            '(' => [0x0E, 0x18, 0x30, 0x30, 0x30, 0x18, 0x0E, 0x00],
            ')' => [0x70, 0x18, 0x0C, 0x0C, 0x0C, 0x18, 0x70, 0x00],
            '[' => [0x3E, 0x30, 0x30, 0x30, 0x30, 0x30, 0x3E, 0x00],
            ']' => [0x7C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x7C, 0x00],
            '/' => [0x00, 0x03, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x00],
            '\\' => [0x00, 0x60, 0x30, 0x18, 0x0C, 0x06, 0x03, 0x00],
            '"' => [0x66, 0x66, 0x66, 0x00, 0x00, 0x00, 0x00, 0x00],
            '\'' => [0x18, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00],
            
            // Default fallback for unknown characters
            _ => [0x3C, 0x42, 0x99, 0xA1, 0xA1, 0x99, 0x42, 0x3C],
        }
    }
    
    
    /// Set clip rectangle
    pub fn push_clip(&mut self, bounds: Rect) {
        self.clip_stack.push(bounds);
    }
    
    /// Remove the last clip rectangle
    pub fn pop_clip(&mut self) {
        self.clip_stack.pop();
    }
    
    /// Get the current clip rectangle
    pub fn current_clip(&self) -> Option<Rect> {
        self.clip_stack.last().copied()
    }
    
    /// Resize the renderer
    pub fn resize(&mut self, new_size: Vec2) {
        self.screen_size = new_size;
        self.projection_matrix = Mat4::orthographic_rh(
            0.0, new_size.x,
            new_size.y, 0.0,
            -1.0, 1.0,
        );
        
        // Update configuration
        self.config.width = new_size.x as u32;
        self.config.height = new_size.y as u32;
    }
    
    /// Get the default font handle
    pub fn get_default_font(&self) -> FontHandle {
        let handle = self.text_renderer.default_font().unwrap_or(FontHandle(0));
        log::debug!("get_default_font() returning handle: {:?}", handle);
        handle
    }
    
    /// Add a quad to the vertex buffer
    fn add_quad(&mut self, bounds: Rect, color: Vec4, uv_min: [f32; 2], uv_max: [f32; 2]) {
        let base_index = self.vertices.len() as u16;
        
        // Add vertices for the quad
        self.vertices.extend_from_slice(&[
            UiVertex {
                position: [bounds.position.x, bounds.position.y],
                tex_coords: [uv_min[0], uv_min[1]],
                color: color.to_array(),
            },
            UiVertex {
                position: [bounds.position.x + bounds.size.x, bounds.position.y],
                tex_coords: [uv_max[0], uv_min[1]],
                color: color.to_array(),
            },
            UiVertex {
                position: [bounds.position.x + bounds.size.x, bounds.position.y + bounds.size.y],
                tex_coords: [uv_max[0], uv_max[1]],
                color: color.to_array(),
            },
            UiVertex {
                position: [bounds.position.x, bounds.position.y + bounds.size.y],
                tex_coords: [uv_min[0], uv_max[1]],
                color: color.to_array(),
            },
        ]);
        
        // Add indices for two triangles
        self.indices.extend_from_slice(&[
            base_index, base_index + 1, base_index + 2,
            base_index + 2, base_index + 3, base_index,
        ]);
    }
}

impl UiVertex {
    /// Get the vertex buffer layout descriptor
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<UiVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Texture coordinates
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}