//! Error types for the Lumina UI framework

use thiserror::Error;

/// Errors that can occur in the UI framework
#[derive(Error, Debug)]
pub enum UiError {
    /// Rendering-related errors
    #[error("Rendering error: {0}")]
    Rendering(#[from] RenderError),
    
    /// Layout-related errors
    #[error("Layout error: {0}")]
    Layout(#[from] LayoutError),
    
    /// Input handling errors
    #[error("Input error: {0}")]
    Input(#[from] InputError),
    
    /// Widget not found
    #[error("Widget not found: {id:?}")]
    WidgetNotFound { 
        /// The ID of the widget that was not found
        id: crate::WidgetId 
    },
    
    /// Invalid widget hierarchy
    #[error("Invalid widget hierarchy: {reason}")]
    InvalidHierarchy { 
        /// The reason for the hierarchy error
        reason: String 
    },
    
    /// Font loading errors
    #[error("Font error: {0}")]
    Font(String),
    
    /// Theme errors
    #[error("Theme error: {0}")]
    Theme(String),
}

/// Rendering-specific errors
#[derive(Error, Debug)]
pub enum RenderError {
    /// WGPU device errors
    #[error("WGPU device error: {0}")]
    DeviceError(String),
    
    /// Shader compilation errors
    #[error("Shader compilation failed: {0}")]
    ShaderCompilation(String),
    
    /// Buffer creation errors
    #[error("Buffer creation failed: {0}")]
    BufferCreation(String),
    
    /// Texture errors
    #[error("Texture error: {0}")]
    Texture(String),
    
    /// Surface configuration errors
    #[error("Surface configuration error: {0}")]
    SurfaceConfig(String),
}

/// Layout calculation errors
#[derive(Error, Debug, Clone)]
pub enum LayoutError {
    /// Circular dependency in layout
    #[error("Circular dependency detected in layout")]
    CircularDependency,
    
    /// Invalid constraints
    #[error("Invalid layout constraints: {reason}")]
    InvalidConstraints { 
        /// The reason for the constraint error
        reason: String 
    },
    
    /// Layout calculation overflow
    #[error("Layout calculation overflow")]
    Overflow,
    
    /// Insufficient space for required content
    #[error("Insufficient space: required {required}, available {available}")]
    InsufficientSpace { 
        /// Required space amount
        required: f32, 
        /// Available space amount
        available: f32 
    },
}

/// Input handling errors
#[derive(Error, Debug)]
pub enum InputError {
    /// Invalid input event
    #[error("Invalid input event: {0}")]
    InvalidEvent(String),
    
    /// Input handler not initialized
    #[error("Input handler not initialized")]
    NotInitialized,
    
    /// Input mapping errors
    #[error("Input mapping error: {0}")]
    Mapping(String),
}

/// Result type for UI operations
pub type UiResult<T> = Result<T, UiError>;

/// Result type for rendering operations
pub type RenderResult<T> = Result<T, RenderError>;

/// Result type for layout operations
pub type LayoutOperationResult<T> = Result<T, LayoutError>;

/// Result type for input operations
pub type InputResult<T> = Result<T, InputError>;