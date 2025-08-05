//! Layout containers for flexible UI organization
//!
//! This module provides modern layout containers inspired by popular UI frameworks
//! like Iced, egui, and CSS Flexbox. These containers automatically manage the
//! positioning and sizing of their child widgets.

use crate::{
    Widget, WidgetId, LayoutConstraints, InputEvent, InputResponse, 
    UiRenderer, Rect, Theme, widgets::BaseWidget,
    layout::LayoutResult,
};
use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Direction for flex layouts
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FlexDirection {
    /// Layout children horizontally (left to right)
    Row,
    /// Layout children vertically (top to bottom)
    Column,
}

/// Alignment along the main axis (direction of layout)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MainAxisAlignment {
    /// Pack children at the start
    Start,
    /// Pack children at the end
    End,
    /// Center children
    Center,
    /// Distribute remaining space evenly between children
    SpaceBetween,
    /// Distribute remaining space evenly around children
    SpaceAround,
    /// Distribute remaining space evenly, including edges
    SpaceEvenly,
}

/// Alignment along the cross axis (perpendicular to layout direction)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CrossAxisAlignment {
    /// Align to the start of the cross axis
    Start,
    /// Align to the end of the cross axis
    End,
    /// Center along the cross axis
    Center,
    /// Stretch to fill the cross axis
    Stretch,
}

/// Spacing configuration for layouts
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Spacing {
    /// Space between children
    pub gap: f32,
    /// Padding inside the container
    pub padding: Padding,
}

/// Padding configuration
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
    
    /// Create symmetric padding (vertical, horizontal)
    pub fn symmetric(vertical: f32, horizontal: f32) -> Self {
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
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            gap: 8.0,
            padding: Padding::uniform(8.0),
        }
    }
}

/// Flexible layout container (Row/Column)
pub struct Flex {
    /// Base widget properties
    base: BaseWidget,
    /// Layout direction
    direction: FlexDirection,
    /// Main axis alignment
    main_axis_alignment: MainAxisAlignment,
    /// Cross axis alignment
    cross_axis_alignment: CrossAxisAlignment,
    /// Spacing configuration
    spacing: Spacing,
    /// Child widgets
    children: Vec<WidgetId>,
}

impl std::fmt::Debug for Flex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Flex")
            .field("base", &self.base)
            .field("direction", &self.direction)
            .field("main_axis_alignment", &self.main_axis_alignment)
            .field("cross_axis_alignment", &self.cross_axis_alignment)
            .field("spacing", &self.spacing)
            .field("children_count", &self.children.len())
            .finish()
    }
}

impl Flex {
    /// Create a new flex container
    pub fn new(direction: FlexDirection) -> Self {
        Self {
            base: BaseWidget::default(),
            direction,
            main_axis_alignment: MainAxisAlignment::Start,
            cross_axis_alignment: CrossAxisAlignment::Start,
            spacing: Spacing::default(),
            children: Vec::new(),
        }
    }
    
    /// Create a horizontal row
    pub fn row() -> Self {
        Self::new(FlexDirection::Row)
    }
    
    /// Create a vertical column
    pub fn column() -> Self {
        Self::new(FlexDirection::Column)
    }
    
    /// Set main axis alignment
    pub fn main_axis_alignment(mut self, alignment: MainAxisAlignment) -> Self {
        self.main_axis_alignment = alignment;
        self
    }
    
    /// Set cross axis alignment
    pub fn cross_axis_alignment(mut self, alignment: CrossAxisAlignment) -> Self {
        self.cross_axis_alignment = alignment;
        self
    }
    
    /// Set spacing
    pub fn spacing(mut self, spacing: Spacing) -> Self {
        self.spacing = spacing;
        self
    }
    
    /// Set gap between children
    pub fn gap(mut self, gap: f32) -> Self {
        self.spacing.gap = gap;
        self
    }
    
    /// Set padding
    pub fn padding(mut self, padding: Padding) -> Self {
        self.spacing.padding = padding;
        self
    }
    
    /// Add a child widget
    pub fn add_child(&mut self, child_id: WidgetId) {
        self.children.push(child_id);
    }
    
    /// Get children
    pub fn children(&self) -> &[WidgetId] {
        &self.children
    }
}

impl Widget for Flex {
    fn id(&self) -> WidgetId {
        self.base.id
    }
    
    fn layout_constraints(&self) -> LayoutConstraints {
        self.base.constraints.clone()
    }
    
    fn layout(&mut self, available_space: Vec2) -> LayoutResult {
        let _content_area = Vec2::new(
            available_space.x - self.spacing.padding.left - self.spacing.padding.right,
            available_space.y - self.spacing.padding.top - self.spacing.padding.bottom,
        );
        
        // For now, return a simple bounds calculation
        // In a full implementation, this would calculate child layouts
        let bounds = Rect::new(0.0, 0.0, available_space.x, available_space.y);
        
        let result = LayoutResult {
            bounds,
            overflow: false,
            content_size: available_space,
        };
        
        self.base.layout_cache = Some(result.clone());
        result
    }
    
    fn handle_input(&mut self, _input: &InputEvent) -> InputResponse {
        // Flex containers typically pass input through to children
        InputResponse::NotHandled
    }
    
    fn render(&self, renderer: &mut UiRenderer, bounds: Rect, _queue: &wgpu::Queue, _theme: &Theme) {
        if !self.base.visible {
            return;
        }
        
        // Flex containers typically don't render themselves, just their children
        // Optionally draw a debug border in development mode
        if cfg!(debug_assertions) {
            let debug_color = [0.2, 0.8, 0.2, 0.3].into(); // Semi-transparent green
            renderer.draw_rect(bounds, debug_color);
        }
    }
    
    fn can_focus(&self) -> bool {
        false
    }
    
    fn on_focus_gained(&mut self) {}
    
    fn on_focus_lost(&mut self) {}
}

/// Grid layout container for organized layouts
pub struct Grid {
    /// Base widget properties
    base: BaseWidget,
    /// Number of columns
    columns: usize,
    /// Column widths (None = auto-size)
    column_widths: Vec<Option<f32>>,
    /// Row heights (None = auto-size)
    row_heights: Vec<Option<f32>>,
    /// Spacing configuration
    spacing: Spacing,
    /// Child widgets with their grid positions
    children: Vec<(WidgetId, usize, usize)>, // (widget, row, column)
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Grid")
            .field("base", &self.base)
            .field("columns", &self.columns)
            .field("spacing", &self.spacing)
            .field("children_count", &self.children.len())
            .finish()
    }
}

impl Grid {
    /// Create a new grid with specified columns
    pub fn new(columns: usize) -> Self {
        Self {
            base: BaseWidget::default(),
            columns,
            column_widths: vec![None; columns],
            row_heights: Vec::new(),
            spacing: Spacing::default(),
            children: Vec::new(),
        }
    }
    
    /// Set column width
    pub fn column_width(mut self, column: usize, width: f32) -> Self {
        if column < self.column_widths.len() {
            self.column_widths[column] = Some(width);
        }
        self
    }
    
    /// Set spacing
    pub fn spacing(mut self, spacing: Spacing) -> Self {
        self.spacing = spacing;
        self
    }
    
    /// Add a child at the specified grid position
    pub fn add_child(&mut self, child_id: WidgetId, row: usize, column: usize) {
        self.children.push((child_id, row, column));
        
        // Ensure we have enough row heights
        while self.row_heights.len() <= row {
            self.row_heights.push(None);
        }
    }
    
    /// Get children with their positions
    pub fn children(&self) -> &[(WidgetId, usize, usize)] {
        &self.children
    }
}

impl Widget for Grid {
    fn id(&self) -> WidgetId {
        self.base.id
    }
    
    fn layout_constraints(&self) -> LayoutConstraints {
        self.base.constraints.clone()
    }
    
    fn layout(&mut self, available_space: Vec2) -> LayoutResult {
        let bounds = Rect::new(0.0, 0.0, available_space.x, available_space.y);
        
        let result = LayoutResult {
            bounds,
            overflow: false,
            content_size: available_space,
        };
        
        self.base.layout_cache = Some(result.clone());
        result
    }
    
    fn handle_input(&mut self, _input: &InputEvent) -> InputResponse {
        InputResponse::NotHandled
    }
    
    fn render(&self, renderer: &mut UiRenderer, bounds: Rect, _queue: &wgpu::Queue, _theme: &Theme) {
        if !self.base.visible {
            return;
        }
        
        // Optionally draw grid lines in development mode
        if cfg!(debug_assertions) {
            let debug_color = [0.8, 0.2, 0.2, 0.3].into(); // Semi-transparent red
            renderer.draw_rect(bounds, debug_color);
        }
    }
    
    fn can_focus(&self) -> bool {
        false
    }
    
    fn on_focus_gained(&mut self) {}
    
    fn on_focus_lost(&mut self) {}
}

/// Convenient constructor functions
pub fn row() -> Flex {
    Flex::row()
}

/// Create a new column flex container
pub fn column() -> Flex {
    Flex::column()
}

/// Create a new grid container with the specified number of columns
pub fn grid(columns: usize) -> Grid {
    Grid::new(columns)
}