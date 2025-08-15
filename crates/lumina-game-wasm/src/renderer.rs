//! Web renderer for WASM games
//! 
//! Provides visual rendering of ECS entities using WGPU and WebGL2.

use web_sys::{HtmlCanvasElement, console};
use wgpu::{self, Device, Queue, Surface, SurfaceConfiguration};
use lumina_ecs::{World, Transform2D, Collider2D, CharacterController2D, Sprite};
use glam::{Vec2, Vec4};
use anyhow::Result;

use crate::game::EntityType;

/// Web renderer using WGPU
pub struct WebRenderer {
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    // Texture rendering
    default_texture: wgpu::Texture,
    texture_bind_group: wgpu::BindGroup,
    canvas: HtmlCanvasElement,
    camera_position: Vec2,
}

/// Vertex data for rendering sprites with UV coordinates
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    uv: [f32; 2],
    color: [f32; 4],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        0 => Float32x2,  // position
        1 => Float32x2,  // uv
        2 => Float32x4,  // color
    ];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// Camera uniform data
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl WebRenderer {
    /// Create a new web renderer
    pub async fn new(canvas: &HtmlCanvasElement) -> Result<Self> {
        console::log_1(&"🎨 Initializing WGPU renderer...".into());
        
        // Create WGPU instance
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL, // Use WebGL2
            ..Default::default()
        });
        
        // Create surface from canvas using direct canvas reference
        let surface = instance.create_surface(canvas.clone())
            .expect("Failed to create surface");
        
        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find appropriate adapter");
        
        // Request device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    memory_hints: wgpu::MemoryHints::default(),
                    trace: wgpu::Trace::Off,
                },
                None,
            )
            .await?;
        
        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        
        let size = (
            canvas.client_width() as u32,
            canvas.client_height() as u32,
        );
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0.max(1),
            height: size.1.max(1),
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
        
        // Create shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Game Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/game.wgsl").into()),
        });
        
        // Create camera buffer
        let camera_uniform = CameraUniform {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        };
        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Buffer"),
            size: std::mem::size_of::<CameraUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Create camera bind group layout
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });
        
        // Create texture bind group layout
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                label: Some("texture_bind_group_layout"),
            });
        
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });
        
        // Create default texture (white pixel for sprites without textures)
        let default_texture_data: [u8; 4] = [255, 255, 255, 255]; // RGBA white
        let default_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("default_texture"),
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
        
        queue.write_texture(
            default_texture.as_image_copy(),
            &default_texture_data,
            wgpu::TextureDataLayout {
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
        
        let default_texture_view = default_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Create sampler
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        
        // Create texture bind group
        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&default_texture_view),
                },
            ],
            label: Some("texture_bind_group"),
        });
        
        // Create render pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout],
                push_constant_ranges: &[],
            });
        
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            cache: None,
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
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
        
        // Create vertex buffer
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: 10000 * std::mem::size_of::<Vertex>() as u64, // Space for many vertices
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        console::log_1(&"✅ WGPU renderer initialized successfully".into());
        
        Ok(Self {
            device,
            queue,
            surface,
            config,
            render_pipeline,
            vertex_buffer,
            camera_buffer,
            camera_bind_group,
            default_texture,
            texture_bind_group,
            canvas: canvas.clone(),
            camera_position: Vec2::ZERO,
        })
    }
    
    /// Render one frame
    pub fn render_frame(&mut self, world: &World) -> Result<()> {
        // Get current texture
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Update camera to follow player
        self.update_camera(world);
        
        // Generate vertices for all renderable entities
        let vertices = self.generate_vertices(world);
        
        // Update vertex buffer
        if !vertices.is_empty() {
            self.queue.write_buffer(
                &self.vertex_buffer,
                0,
                bytemuck::cast_slice(&vertices),
            );
        }
        
        // Create command encoder
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        // Render pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.texture_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            
            if !vertices.is_empty() {
                render_pass.draw(0..vertices.len() as u32, 0..1);
            }
        }
        
        // Submit commands
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
    
    /// Update camera to follow the player
    fn update_camera(&mut self, world: &World) {
        // Find player position
        let mut player_pos = None;
        for entity in world.iter_entities() {
            if world.has_component::<CharacterController2D>(entity) {
                if let Some(transform) = world.get_component::<Transform2D>(entity) {
                    player_pos = Some(transform.pos);
                    break;
                }
            }
        }
        
        // Update camera position with smooth following
        if let Some(target_pos) = player_pos {
            let lerp_factor = 0.1; // Smooth camera following
            self.camera_position = self.camera_position.lerp(target_pos, lerp_factor);
        }
        
        // Create view-projection matrix
        let aspect = self.config.width as f32 / self.config.height as f32;
        let zoom = 0.5; // Zoom level
        
        let projection = glam::Mat4::orthographic_rh(
            -aspect * zoom, aspect * zoom,
            -zoom, zoom,
            -1.0, 1.0,
        );
        
        let view = glam::Mat4::from_translation((-self.camera_position, 0.0).into());
        let view_proj = projection * view;
        
        // Update camera uniform
        let camera_uniform = CameraUniform {
            view_proj: view_proj.to_cols_array_2d(),
        };
        
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }
    
    /// Generate vertices for all renderable entities
    fn generate_vertices(&self, world: &World) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        
        for entity in world.iter_entities() {
            if let Some(transform) = world.get_component::<Transform2D>(entity) {
                // Check if entity has a Sprite component
                let color = if let Some(sprite) = world.get_component::<Sprite>(entity) {
                    // Use sprite color
                    Vec4::from(sprite.color)
                } else {
                    // Fallback to entity type color
                    let entity_type = self.determine_entity_type(world, entity);
                    self.get_entity_color(entity_type)
                };
                
                let size = self.get_entity_size(world, entity);
                
                // Generate quad vertices
                let quad_vertices = self.create_quad_vertices(
                    transform.pos,
                    size * transform.scale,
                    color,
                );
                vertices.extend_from_slice(&quad_vertices);
            }
        }
        
        vertices
    }
    
    /// Determine entity type for rendering
    fn determine_entity_type(&self, world: &World, entity: lumina_ecs::Entity) -> EntityType {
        if world.has_component::<CharacterController2D>(entity) {
            EntityType::Player
        } else if let Some(collider) = world.get_component::<Collider2D>(entity) {
            if collider.is_sensor {
                EntityType::Coin
            } else {
                EntityType::Platform
            }
        } else {
            EntityType::Platform
        }
    }
    
    /// Get color for entity type
    fn get_entity_color(&self, entity_type: EntityType) -> Vec4 {
        match entity_type {
            EntityType::Player => Vec4::new(0.2, 0.8, 0.2, 1.0),    // Green
            EntityType::Platform => Vec4::new(0.6, 0.4, 0.2, 1.0),  // Brown
            EntityType::Coin => Vec4::new(1.0, 0.8, 0.0, 1.0),      // Gold
            EntityType::Dynamic => Vec4::new(0.8, 0.2, 0.2, 1.0),   // Red
        }
    }
    
    /// Get size for entity
    fn get_entity_size(&self, world: &World, entity: lumina_ecs::Entity) -> Vec2 {
        // Try to get size from collider
        if let Some(collider) = world.get_component::<Collider2D>(entity) {
            match collider.shape {
                lumina_ecs::CollisionShape::AABB { width, height } => {
                    return Vec2::new(width / 100.0, height / 100.0); // Scale down for rendering
                }
                lumina_ecs::CollisionShape::Circle { radius } => {
                    let size = radius / 50.0; // Scale down
                    return Vec2::new(size, size);
                }
                lumina_ecs::CollisionShape::Capsule { width, height } => {
                    return Vec2::new(width / 100.0, height / 100.0);
                }
            }
        }
        
        // Default size
        Vec2::new(0.32, 0.32)
    }
    
    /// Create vertices for a quad with texture coordinates
    fn create_quad_vertices(&self, position: Vec2, size: Vec2, color: Vec4) -> [Vertex; 6] {
        let half_size = size * 0.5;
        let pos = position / 100.0; // Scale down world coordinates
        
        [
            // Triangle 1
            Vertex { 
                position: [pos.x - half_size.x, pos.y - half_size.y], 
                uv: [0.0, 1.0], // Bottom-left
                color: color.into() 
            },
            Vertex { 
                position: [pos.x + half_size.x, pos.y - half_size.y], 
                uv: [1.0, 1.0], // Bottom-right
                color: color.into() 
            },
            Vertex { 
                position: [pos.x + half_size.x, pos.y + half_size.y], 
                uv: [1.0, 0.0], // Top-right
                color: color.into() 
            },
            
            // Triangle 2
            Vertex { 
                position: [pos.x - half_size.x, pos.y - half_size.y], 
                uv: [0.0, 1.0], // Bottom-left
                color: color.into() 
            },
            Vertex { 
                position: [pos.x + half_size.x, pos.y + half_size.y], 
                uv: [1.0, 0.0], // Top-right
                color: color.into() 
            },
            Vertex { 
                position: [pos.x - half_size.x, pos.y + half_size.y], 
                uv: [0.0, 0.0], // Top-left
                color: color.into() 
            },
        ]
    }
    
    /// Resize the renderer
    pub fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
        Ok(())
    }
}