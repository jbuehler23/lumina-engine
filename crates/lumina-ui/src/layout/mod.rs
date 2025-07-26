//! Layout system for the Lumina UI framework
//! 
//! Provides a flexbox-inspired layout engine that can handle complex
//! UI layouts with automatic sizing and positioning.

use crate::{Rect, error::LayoutError};
use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Layout engine responsible for calculating widget positions and sizes
#[derive(Debug, Default)]
pub struct LayoutEngine {
    /// Cache for expensive layout calculations
    cache: std::collections::HashMap<LayoutKey, LayoutResult>,
}

/// Key used for caching layout calculations
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct LayoutKey {
    constraints: LayoutConstraints,
    available_space: (u32, u32), // Rounded to avoid floating point precision issues
}

/// Constraints that define how a widget should be laid out
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConstraints {
    /// Minimum width
    pub min_width: Option<f32>,
    /// Maximum width
    pub max_width: Option<f32>,
    /// Minimum height
    pub min_height: Option<f32>,
    /// Maximum height
    pub max_height: Option<f32>,
    /// Fixed width (overrides min/max if set)
    pub width: Option<f32>,
    /// Fixed height (overrides min/max if set)
    pub height: Option<f32>,
    /// How the widget should grow to fill available space
    pub flex: FlexConstraints,
    /// Alignment within the parent container
    pub alignment: Alignment,
}

impl PartialEq for LayoutConstraints {
    fn eq(&self, other: &Self) -> bool {
        self.min_width == other.min_width &&
        self.max_width == other.max_width &&
        self.min_height == other.min_height &&
        self.max_height == other.max_height &&
        self.width == other.width &&
        self.height == other.height &&
        self.flex == other.flex &&
        self.alignment == other.alignment
    }
}

impl Eq for LayoutConstraints {}

impl std::hash::Hash for LayoutConstraints {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // For f32 values, we'll use their bit representation for hashing
        self.min_width.map(f32::to_bits).hash(state);
        self.max_width.map(f32::to_bits).hash(state);
        self.min_height.map(f32::to_bits).hash(state);
        self.max_height.map(f32::to_bits).hash(state);
        self.width.map(f32::to_bits).hash(state);
        self.height.map(f32::to_bits).hash(state);
        self.flex.hash(state);
        self.alignment.hash(state);
    }
}

/// Flexbox-style constraints for dynamic sizing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlexConstraints {
    /// Flex direction (row or column)
    pub direction: FlexDirection,
    /// How to wrap flex items
    pub wrap: FlexWrap,
    /// How to align items along the main axis
    pub justify_content: JustifyContent,
    /// How to align items along the cross axis
    pub align_items: AlignItems,
    /// How to align content when there are multiple lines
    pub align_content: AlignContent,
    /// Flex grow factor (how much to grow)
    pub grow: f32,
    /// Flex shrink factor (how much to shrink)
    pub shrink: f32,
    /// Flex basis (initial size before growing/shrinking)
    pub basis: FlexBasis,
}

impl std::hash::Hash for FlexConstraints {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.direction.hash(state);
        self.wrap.hash(state);
        self.justify_content.hash(state);
        self.align_items.hash(state);
        self.align_content.hash(state);
        self.grow.to_bits().hash(state);
        self.shrink.to_bits().hash(state);
        self.basis.hash(state);
    }
}

/// Direction of the flex container
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexDirection {
    /// Items are arranged horizontally (left to right)
    Row,
    /// Items are arranged horizontally (right to left)
    RowReverse,
    /// Items are arranged vertically (top to bottom)
    Column,
    /// Items are arranged vertically (bottom to top)
    ColumnReverse,
}

/// How to wrap flex items
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexWrap {
    /// Don't wrap items
    NoWrap,
    /// Wrap items to new lines
    Wrap,
    /// Wrap items to new lines in reverse order
    WrapReverse,
}

/// How to justify content along the main axis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JustifyContent {
    /// Pack items at the start
    FlexStart,
    /// Pack items at the end
    FlexEnd,
    /// Center items
    Center,
    /// Distribute items evenly with space between
    SpaceBetween,
    /// Distribute items evenly with space around
    SpaceAround,
    /// Distribute items evenly with equal space around
    SpaceEvenly,
}

/// How to align items along the cross axis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlignItems {
    /// Stretch items to fill the container
    Stretch,
    /// Align items at the start
    FlexStart,
    /// Align items at the end
    FlexEnd,
    /// Center items
    Center,
    /// Align items along their baseline
    Baseline,
}

/// How to align content when there are multiple lines
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlignContent {
    /// Stretch lines to fill the container
    Stretch,
    /// Pack lines at the start
    FlexStart,
    /// Pack lines at the end
    FlexEnd,
    /// Center lines
    Center,
    /// Distribute lines evenly with space between
    SpaceBetween,
    /// Distribute lines evenly with space around
    SpaceAround,
}

/// Flex basis (initial size)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlexBasis {
    /// Use the content size
    Auto,
    /// Use a specific size
    Length(f32),
    /// Use a percentage of the parent
    Percentage(f32),
}

impl std::hash::Hash for FlexBasis {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            FlexBasis::Auto => 0.hash(state),
            FlexBasis::Length(v) => {
                1.hash(state);
                v.to_bits().hash(state);
            },
            FlexBasis::Percentage(v) => {
                2.hash(state);
                v.to_bits().hash(state);
            },
        }
    }
}

/// Widget alignment within its allocated space
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Alignment {
    /// Horizontal alignment
    pub horizontal: HorizontalAlign,
    /// Vertical alignment
    pub vertical: VerticalAlign,
}

/// Horizontal alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HorizontalAlign {
    /// Align to the left
    Left,
    /// Center horizontally
    Center,
    /// Align to the right
    Right,
    /// Stretch to fill width
    Stretch,
}

/// Vertical alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerticalAlign {
    /// Align to the top
    Top,
    /// Center vertically
    Center,
    /// Align to the bottom
    Bottom,
    /// Stretch to fill height
    Stretch,
}

/// Result of a layout calculation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutResult {
    /// Final bounds of the widget
    pub bounds: Rect,
    /// Whether the widget needs more space than available
    pub overflow: bool,
    /// Content size (may be larger than bounds if overflow)
    pub content_size: Vec2,
}

impl Default for LayoutConstraints {
    fn default() -> Self {
        Self {
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            width: None,
            height: None,
            flex: FlexConstraints::default(),
            alignment: Alignment::default(),
        }
    }
}

impl Default for FlexConstraints {
    fn default() -> Self {
        Self {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            align_content: AlignContent::Stretch,
            grow: 0.0,
            shrink: 1.0,
            basis: FlexBasis::Auto,
        }
    }
}

impl Default for Alignment {
    fn default() -> Self {
        Self {
            horizontal: HorizontalAlign::Left,
            vertical: VerticalAlign::Top,
        }
    }
}

impl LayoutEngine {
    /// Create a new layout engine
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
        }
    }
    
    /// Calculate layout for a widget with given constraints and available space
    pub fn calculate_layout(
        &mut self,
        constraints: &LayoutConstraints,
        available_space: Vec2,
    ) -> Result<LayoutResult, LayoutError> {
        // Create cache key
        let key = LayoutKey {
            constraints: constraints.clone(),
            available_space: (available_space.x.round() as u32, available_space.y.round() as u32),
        };
        
        // Check cache first
        if let Some(cached_result) = self.cache.get(&key) {
            return Ok(cached_result.clone());
        }
        
        // Calculate new layout
        let result = self.calculate_layout_internal(constraints, available_space)?;
        
        // Cache result
        self.cache.insert(key, result.clone());
        
        Ok(result)
    }
    
    /// Clear the layout cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    
    /// Internal layout calculation
    fn calculate_layout_internal(
        &self,
        constraints: &LayoutConstraints,
        available_space: Vec2,
    ) -> Result<LayoutResult, LayoutError> {
        let mut width = available_space.x;
        let mut height = available_space.y;
        
        // Apply fixed dimensions first
        if let Some(fixed_width) = constraints.width {
            width = fixed_width;
        }
        if let Some(fixed_height) = constraints.height {
            height = fixed_height;
        }
        
        // Apply minimum constraints
        if let Some(min_width) = constraints.min_width {
            width = width.max(min_width);
        }
        if let Some(min_height) = constraints.min_height {
            height = height.max(min_height);
        }
        
        // Apply maximum constraints
        if let Some(max_width) = constraints.max_width {
            width = width.min(max_width);
        }
        if let Some(max_height) = constraints.max_height {
            height = height.min(max_height);
        }
        
        // Check for overflow
        let overflow = width > available_space.x || height > available_space.y;
        
        // Calculate position based on alignment
        let (x, y) = self.calculate_alignment_position(
            Vec2::new(width, height),
            available_space,
            &constraints.alignment,
        );
        
        Ok(LayoutResult {
            bounds: Rect::new(x, y, width, height),
            overflow,
            content_size: Vec2::new(width, height),
        })
    }
    
    /// Calculate position based on alignment
    fn calculate_alignment_position(
        &self,
        size: Vec2,
        available_space: Vec2,
        alignment: &Alignment,
    ) -> (f32, f32) {
        let x = match alignment.horizontal {
            HorizontalAlign::Left => 0.0,
            HorizontalAlign::Center => (available_space.x - size.x) * 0.5,
            HorizontalAlign::Right => available_space.x - size.x,
            HorizontalAlign::Stretch => 0.0, // Position doesn't matter for stretch
        };
        
        let y = match alignment.vertical {
            VerticalAlign::Top => 0.0,
            VerticalAlign::Center => (available_space.y - size.y) * 0.5,
            VerticalAlign::Bottom => available_space.y - size.y,
            VerticalAlign::Stretch => 0.0, // Position doesn't matter for stretch
        };
        
        (x, y)
    }
}

/// Builder for creating layout constraints
pub struct LayoutConstraintsBuilder {
    constraints: LayoutConstraints,
}

impl LayoutConstraintsBuilder {
    /// Create a new builder with default constraints
    pub fn new() -> Self {
        Self {
            constraints: LayoutConstraints::default(),
        }
    }
    
    /// Set minimum width
    pub fn min_width(mut self, width: f32) -> Self {
        self.constraints.min_width = Some(width);
        self
    }
    
    /// Set maximum width
    pub fn max_width(mut self, width: f32) -> Self {
        self.constraints.max_width = Some(width);
        self
    }
    
    /// Set minimum height
    pub fn min_height(mut self, height: f32) -> Self {
        self.constraints.min_height = Some(height);
        self
    }
    
    /// Set maximum height
    pub fn max_height(mut self, height: f32) -> Self {
        self.constraints.max_height = Some(height);
        self
    }
    
    /// Set fixed width
    pub fn width(mut self, width: f32) -> Self {
        self.constraints.width = Some(width);
        self
    }
    
    /// Set fixed height
    pub fn height(mut self, height: f32) -> Self {
        self.constraints.height = Some(height);
        self
    }
    
    /// Set flex grow factor
    pub fn flex_grow(mut self, grow: f32) -> Self {
        self.constraints.flex.grow = grow;
        self
    }
    
    /// Set flex shrink factor
    pub fn flex_shrink(mut self, shrink: f32) -> Self {
        self.constraints.flex.shrink = shrink;
        self
    }
    
    /// Set alignment
    pub fn align(mut self, horizontal: HorizontalAlign, vertical: VerticalAlign) -> Self {
        self.constraints.alignment = Alignment { horizontal, vertical };
        self
    }
    
    /// Build the constraints
    pub fn build(self) -> LayoutConstraints {
        self.constraints
    }
}

impl Default for LayoutConstraintsBuilder {
    fn default() -> Self {
        Self::new()
    }
}