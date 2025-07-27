//! Graphics pipeline management
//!
//! Provides utilities for creating and managing WGPU render pipelines.

use crate::{RenderResult, RenderError};

/// Graphics pipeline wrapper
pub struct Pipeline {
    /// The WGPU render pipeline
    pub render_pipeline: wgpu::RenderPipeline,
    /// Pipeline layout
    pub layout: wgpu::PipelineLayout,
}

impl Pipeline {
    /// Create a new graphics pipeline
    pub fn new(
        device: &wgpu::Device,
        layout: wgpu::PipelineLayout,
        shader: &wgpu::ShaderModule,
        vertex_entry: &str,
        fragment_entry: &str,
        vertex_buffers: &[wgpu::VertexBufferLayout],
        targets: &[Option<wgpu::ColorTargetState>],
    ) -> RenderResult<Self> {
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: vertex_entry,
                buffers: vertex_buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: fragment_entry,
                targets,
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

        Ok(Self {
            render_pipeline,
            layout,
        })
    }
}