//! Buffer management for UI rendering

use crate::error::RenderError;
use bytemuck::Pod;

/// Buffer manager for handling vertex and index buffers
#[derive(Debug)]
pub struct BufferManager {
    /// Device reference
    device: wgpu::Device,
    /// Queue reference
    queue: wgpu::Queue,
}

impl BufferManager {
    /// Create a new buffer manager
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self { device, queue }
    }
    
    /// Create a vertex buffer
    pub fn create_vertex_buffer<T: Pod>(
        &self,
        label: &str,
        data: &[T],
        dynamic: bool,
    ) -> Result<wgpu::Buffer, RenderError> {
        let usage = if dynamic {
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
        } else {
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
        };
        
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: (std::mem::size_of::<T>() * data.len()) as u64,
            usage,
            mapped_at_creation: false,
        });
        
        // Write data to buffer
        self.queue.write_buffer(&buffer, 0, bytemuck::cast_slice(data));
        
        Ok(buffer)
    }
    
    /// Create an index buffer
    pub fn create_index_buffer(
        &self,
        label: &str,
        data: &[u16],
        dynamic: bool,
    ) -> Result<wgpu::Buffer, RenderError> {
        let usage = if dynamic {
            wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST
        } else {
            wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST
        };
        
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: (std::mem::size_of::<u16>() * data.len()) as u64,
            usage,
            mapped_at_creation: false,
        });
        
        // Write data to buffer
        self.queue.write_buffer(&buffer, 0, bytemuck::cast_slice(data));
        
        Ok(buffer)
    }
    
    /// Update a buffer with new data
    pub fn update_buffer<T: Pod>(&self, buffer: &wgpu::Buffer, data: &[T]) {
        self.queue.write_buffer(buffer, 0, bytemuck::cast_slice(data));
    }
}