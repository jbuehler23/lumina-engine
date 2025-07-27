//! GPU buffer management utilities
//!
//! Provides helper functions for creating and managing WGPU buffers.

use wgpu::util::DeviceExt;

/// Buffer creation utilities
pub struct BufferUtils;

impl BufferUtils {
    /// Create a vertex buffer
    pub fn create_vertex_buffer(
        device: &wgpu::Device,
        data: &[u8],
        label: Option<&str>,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents: data,
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    /// Create an index buffer
    pub fn create_index_buffer(
        device: &wgpu::Device,
        data: &[u8],
        label: Option<&str>,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents: data,
            usage: wgpu::BufferUsages::INDEX,
        })
    }

    /// Create a uniform buffer
    pub fn create_uniform_buffer(
        device: &wgpu::Device,
        size: u64,
        label: Option<&str>,
    ) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    /// Create a storage buffer
    pub fn create_storage_buffer(
        device: &wgpu::Device,
        size: u64,
        label: Option<&str>,
    ) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }
}