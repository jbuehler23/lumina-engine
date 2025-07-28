//! WGPU-based UI rendering system
//!
//! This module provides specialized rendering capabilities for immediate-mode UI interfaces.
//! It handles batching, clipping, and efficient rendering of UI primitives like rectangles,
//! text, and textured quads.

use crate::{Rect, RenderResult};
use crate::text::{TextPipeline, TextLayoutInfo};
use glam::{Vec2, Vec4, Mat4};
use bytemuck::{Pod, Zeroable};

/// Main UI renderer using WGPU
pub struct UiRenderer {
    /// Surface configuration
    config: wgpu::SurfaceConfiguration,
    /// Vertex buffer for UI quads
    vertex_buffer: wgpu::Buffer,
    /// Index buffer for UI quads
    index_buffer: wgpu::Buffer,
    /// Uniform buffer for view/projection matrices
    uniform_buffer: wgpu::Buffer,
    /// Bind group for uniforms
    uniform_bind_group: wgpu::BindGroup,
    /// Render pipeline for textured quads
    texture_pipeline: wgpu::RenderPipeline,
    /// Text pipeline for proper font-based text rendering
    text_pipeline: TextPipeline,
    /// Texture bind group for solid color rendering
    solid_texture_bind_group: wgpu::BindGroup,
    /// Current frame's text layouts for glyphon rendering
    text_layouts: Vec<TextLayoutInfo>,
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
        /// The bounds of the rectangle
        bounds: Rect,
        /// The color as RGBA components
        color: Vec4,
        /// The border radius for rounded corners
        border_radius: f32,
    },
    /// Draw a textured rectangle
    TexturedRect {
        /// The bounds of the rectangle
        bounds: Rect,
        /// Handle to the texture to render
        texture: TextureHandle,
        /// Tint color as RGBA components
        color: Vec4,
    },
    /// Draw text
    Text {
        /// The text string to render
        text: String,
        /// Position where to render the text
        position: Vec2,
        /// Handle to the font to use
        font: FontHandle,
        /// Font size in pixels
        size: f32,
        /// Text color as RGBA components
        color: Vec4,
    },
    /// Push a clip rectangle
    PushClip { 
        /// The clipping bounds
        bounds: Rect 
    },
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
        let _solid_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
        
        // Create text pipeline using glyphon
        let text_pipeline = TextPipeline::new(device, queue, config.format)?;
        
        // Create a simple white texture for solid color rendering
        let white_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("White Texture"),
            size: wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        
        // Write white pixel data
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &white_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &[255u8, 255u8, 255u8, 255u8], // White RGBA
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: Some(1),
            },
            wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        
        // Create texture sampler
        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Solid Texture Sampler"),
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
        
        // Create texture view
        let white_texture_view = white_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Create bind group for solid texture
        let solid_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Solid Texture Bind Group"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&texture_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&white_texture_view),
                },
            ],
        });
        
        Ok(Self {
            config,
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            uniform_bind_group,
            texture_pipeline,
            text_pipeline,
            solid_texture_bind_group,
            text_layouts: Vec::new(),
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
        self.text_layouts.clear();
        self.clip_stack.clear();
        
        // Update text pipeline resolution
        self.text_pipeline.set_resolution(self.screen_size.x as u32, self.screen_size.y as u32);
        
        // Update uniforms
        let uniforms = UiUniforms {
            view_proj: (self.projection_matrix * self.view_matrix).to_cols_array_2d(),
            screen_size: [self.screen_size.x, self.screen_size.y],
            _padding: [0.0, 0.0],
        };
        
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }
    
    /// End the current frame and submit all draw commands
    pub fn end_frame(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<(), crate::text::TextError> {
        // Prepare text layouts for glyphon rendering
        if !self.text_layouts.is_empty() {
            self.text_pipeline.prepare_text_layouts(device, queue, &self.text_layouts)?;
        }
        
        // Update vertex and index buffers for solid/textured quads
        if !self.vertices.is_empty() {
            queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
        }
        if !self.indices.is_empty() {
            queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&self.indices));
        }
        
        Ok(())
    }
    
    /// Submit the rendered UI to a render pass
    pub fn submit_to_render_pass<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) -> Result<(), crate::text::TextError> {
        // Render solid/textured quads first
        if !self.vertices.is_empty() && !self.indices.is_empty() {
            // Set vertex and index buffers
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            
            // Use texture pipeline for solid quads with white texture
            render_pass.set_pipeline(&self.texture_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_bind_group(1, &self.solid_texture_bind_group, &[]);
            
            // Draw all vertices
            let num_indices = self.indices.len() as u32;
            if num_indices > 0 {
                render_pass.draw_indexed(0..num_indices, 0, 0..1);
            }
        }
        
        // Render text using glyphon
        if !self.text_layouts.is_empty() {
            self.text_pipeline.render_text_areas(render_pass)?;
        }
        
        Ok(())
    }
    
    /// Draw a colored rectangle
    pub fn draw_rect(&mut self, bounds: Rect, color: Vec4) {
        // For solid color rendering, we'll use the center of the texture (0.5, 0.5)
        // This should give us a white pixel in most texture atlases
        self.add_quad(bounds, color, [0.5, 0.5], [0.5, 0.5]);
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
    
    /// Measure text dimensions without rendering (for layout calculations)
    pub fn measure_text(&mut self, text: &str, font: FontHandle, size: f32) -> Result<crate::text::TextMeasurement, crate::text::TextError> {
        self.text_pipeline.measure_text(text, font, size)
    }

    /// Draw text using glyphon TextPipeline
    pub fn draw_text(&mut self, text: &str, position: Vec2, font: FontHandle, size: f32, color: Vec4, _queue: &wgpu::Queue) -> Result<(), crate::text::TextError> {
        // Queue text for layout using the glyphon-based TextPipeline
        let color_array = [color.x, color.y, color.z, color.w];
        let layout_info = self.text_pipeline.queue_text(
            text,
            font,
            size,
            position,
            color_array,
            _queue,
        )?;
        
        // Add text layout to our frame's text layouts for glyphon rendering
        self.text_layouts.push(layout_info);
        
        log::debug!("Queued text '{}' for glyphon rendering at position {:?}", text, position);
        Ok(())
    }
    
    
    // Glyphon handles glyph rendering internally - no manual glyph drawing needed
    
    
    
    
    
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
        
        // Update text pipeline resolution
        self.text_pipeline.set_resolution(new_size.x as u32, new_size.y as u32);
    }
    
    /// Get the default font handle
    pub fn get_default_font(&self) -> FontHandle {
        let handle = self.text_pipeline.default_font().unwrap_or(FontHandle(0));
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