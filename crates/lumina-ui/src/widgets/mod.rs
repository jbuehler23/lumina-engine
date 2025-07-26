//! Core UI widgets for the Lumina UI framework

pub mod button;
pub mod panel;
pub mod text;
pub mod text_input;
pub mod canvas;
pub mod container;

// Re-export all widgets
pub use button::*;
pub use panel::*;
pub use text::*;
pub use text_input::*;
pub use canvas::*;
pub use container::*;

use crate::{WidgetId, InputEvent, InputResponse, LayoutConstraints, LayoutResult, UiRenderer, Rect};
use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Base widget properties that all widgets share
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseWidget {
    /// Unique identifier for this widget
    pub id: WidgetId,
    /// Whether the widget is visible
    pub visible: bool,
    /// Whether the widget is enabled (can receive input)
    pub enabled: bool,
    /// Custom style overrides
    pub style: WidgetStyle,
    /// Layout constraints
    pub constraints: LayoutConstraints,
    /// Cached layout result
    pub layout_cache: Option<LayoutResult>,
}

/// Style properties that can be applied to widgets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetStyle {
    /// Background color (RGBA)
    pub background_color: Option<[f32; 4]>,
    /// Border color (RGBA)
    pub border_color: Option<[f32; 4]>,
    /// Border width
    pub border_width: Option<f32>,
    /// Border radius for rounded corners
    pub border_radius: Option<f32>,
    /// Padding inside the widget
    pub padding: Option<Padding>,
    /// Margin around the widget
    pub margin: Option<Margin>,
    /// Text color (RGBA)
    pub text_color: Option<[f32; 4]>,
    /// Font size
    pub font_size: Option<f32>,
    /// Font family
    pub font_family: Option<String>,
    /// Opacity (0.0 to 1.0)
    pub opacity: Option<f32>,
    /// Drop shadow
    pub shadow: Option<Shadow>,
}

/// Padding values for a widget
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Padding {
    /// Top padding
    pub top: f32,
    /// Right padding
    pub right: f32,
    /// Bottom padding
    pub bottom: f32,
    /// Left padding
    pub left: f32,
}

/// Margin values for a widget
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Margin {
    /// Top margin
    pub top: f32,
    /// Right margin
    pub right: f32,
    /// Bottom margin
    pub bottom: f32,
    /// Left margin
    pub left: f32,
}

/// Drop shadow configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shadow {
    /// Shadow offset
    pub offset: Vec2,
    /// Shadow blur radius
    pub blur_radius: f32,
    /// Shadow color (RGBA)
    pub color: [f32; 4],
}

impl Default for BaseWidget {
    fn default() -> Self {
        Self {
            id: WidgetId::new(),
            visible: true,
            enabled: true,
            style: WidgetStyle::default(),
            constraints: LayoutConstraints::default(),
            layout_cache: None,
        }
    }
}

impl Default for WidgetStyle {
    fn default() -> Self {
        Self {
            background_color: None,
            border_color: None,
            border_width: None,
            border_radius: None,
            padding: None,
            margin: None,
            text_color: None,
            font_size: None,
            font_family: None,
            opacity: None,
            shadow: None,
        }
    }
}

impl Padding {
    /// Create uniform padding
    pub fn uniform(value: f32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }
    
    /// Create horizontal and vertical padding
    pub fn symmetric(horizontal: f32, vertical: f32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }
    
    /// Create padding with individual values
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top, right, bottom, left }
    }
    
    /// Get total horizontal padding
    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }
    
    /// Get total vertical padding
    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

impl Margin {
    /// Create uniform margin
    pub fn uniform(value: f32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }
    
    /// Create horizontal and vertical margin
    pub fn symmetric(horizontal: f32, vertical: f32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }
    
    /// Create margin with individual values
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top, right, bottom, left }
    }
    
    /// Get total horizontal margin
    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }
    
    /// Get total vertical margin
    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

/// Widget animation state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationState {
    /// Widget is in its normal state
    Normal,
    /// Widget is being hovered
    Hovered,
    /// Widget is being pressed/clicked
    Pressed,
    /// Widget is focused
    Focused,
    /// Widget is disabled
    Disabled,
}

/// Trait for widgets that can be styled
pub trait Styleable {
    /// Apply a style to the widget
    fn apply_style(&mut self, style: WidgetStyle);
    
    /// Get the current style
    fn get_style(&self) -> &WidgetStyle;
    
    /// Set background color
    fn set_background_color(&mut self, color: [f32; 4]) {
        self.apply_style(WidgetStyle {
            background_color: Some(color),
            ..Default::default()
        });
    }
    
    /// Set text color
    fn set_text_color(&mut self, color: [f32; 4]) {
        self.apply_style(WidgetStyle {
            text_color: Some(color),
            ..Default::default()
        });
    }
    
    /// Set padding
    fn set_padding(&mut self, padding: Padding) {
        self.apply_style(WidgetStyle {
            padding: Some(padding),
            ..Default::default()
        });
    }
    
    /// Set margin
    fn set_margin(&mut self, margin: Margin) {
        self.apply_style(WidgetStyle {
            margin: Some(margin),
            ..Default::default()
        });
    }
}