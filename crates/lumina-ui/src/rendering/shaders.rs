//! Shader management for the UI renderer

use crate::error::RenderError;

/// Shader manager for handling UI shaders
#[derive(Debug)]
pub struct ShaderManager {
    /// Device reference
    device: wgpu::Device,
    /// Loaded shaders
    shaders: std::collections::HashMap<String, wgpu::ShaderModule>,
}

impl ShaderManager {
    /// Create a new shader manager
    pub fn new(device: wgpu::Device) -> Self {
        Self {
            device,
            shaders: std::collections::HashMap::new(),
        }
    }
    
    /// Load a shader from WGSL source
    pub fn load_shader(&mut self, name: &str, source: &str) -> Result<&wgpu::ShaderModule, RenderError> {
        let shader = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(name),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        
        self.shaders.insert(name.to_string(), shader);
        self.shaders.get(name).ok_or_else(|| {
            RenderError::ShaderCompilation(format!("Failed to store shader: {}", name))
        })
    }
    
    /// Get a loaded shader
    pub fn get_shader(&self, name: &str) -> Option<&wgpu::ShaderModule> {
        self.shaders.get(name)
    }
}