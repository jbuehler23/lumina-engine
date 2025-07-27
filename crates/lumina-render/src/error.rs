//! Error types for the Lumina rendering system

use thiserror::Error;

/// Errors that can occur during rendering operations
#[derive(Error, Debug, Clone)]
pub enum RenderError {
    /// WGPU initialization failed
    #[error("Graphics initialization failed: {0}")]
    GraphicsInit(String),
    
    /// Shader compilation failed
    #[error("Shader compilation failed: {0}")]
    ShaderCompilation(String),
    
    /// Buffer creation failed
    #[error("Buffer creation failed: {0}")]
    BufferCreation(String),
    
    /// Texture loading failed
    #[error("Texture loading failed: {0}")]
    TextureLoad(String),
    
    /// Font loading failed
    #[error("Font loading failed: {0}")]
    FontLoad(String),
    
    /// Surface creation failed
    #[error("Surface creation failed: {0}")]
    SurfaceCreation(String),
    
    /// Adapter not found
    #[error("Graphics adapter not found")]
    AdapterNotFound,
    
    /// Device creation failed
    #[error("Graphics device creation failed: {0}")]
    DeviceCreation(String),
    
    /// Resource not found
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
    
    /// Invalid operation
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    /// Out of memory
    #[error("Out of graphics memory")]
    OutOfMemory,
}

/// Result type for rendering operations
pub type RenderResult<T> = Result<T, RenderError>;

impl From<wgpu::CreateSurfaceError> for RenderError {
    fn from(err: wgpu::CreateSurfaceError) -> Self {
        RenderError::SurfaceCreation(err.to_string())
    }
}

impl From<wgpu::RequestDeviceError> for RenderError {
    fn from(err: wgpu::RequestDeviceError) -> Self {
        RenderError::DeviceCreation(err.to_string())
    }
}