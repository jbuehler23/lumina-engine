//! Easy-to-use UI API for Lumina Engine
//! 
//! This module provides a simple, declarative API for creating UIs that's
//! accessible to non-technical game developers. Inspired by modern UI frameworks
//! but optimized for game development workflows.

use crate::{
    UiFramework, WidgetId, Theme, 
    Button, Text, Panel,
    layout::containers::{Flex, FlexDirection, MainAxisAlignment, CrossAxisAlignment, Spacing, Padding},
    widgets::button::ButtonVariant,
};
use glam::Vec2;
use std::collections::HashMap;

/// Easy UI builder that provides a declarative API
pub struct UiBuilder {
    /// The underlying UI framework
    framework: UiFramework,
    /// Widget stack for building hierarchies
    widget_stack: Vec<WidgetId>,
    /// Named widgets for easy access
    named_widgets: HashMap<String, WidgetId>,
}

/// Color helper for easy color specification
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Create a new color from RGBA values (0.0 to 1.0)
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    /// Create a new color from RGB values (0.0 to 1.0), alpha = 1.0
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
    
    /// Create color from hex string (e.g., "#FF0000" or "#FF0000FF")
    pub fn hex(hex: &str) -> Result<Self, &'static str> {
        let hex = hex.trim_start_matches('#');
        
        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex color")?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex color")?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex color")?;
                Ok(Self::rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0))
            },
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex color")?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex color")?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex color")?;
                let a = u8::from_str_radix(&hex[6..8], 16).map_err(|_| "Invalid hex color")?;
                Ok(Self::rgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0))
            },
            _ => Err("Hex color must be 6 or 8 characters"),
        }
    }
    
    /// Convert to array format used by widgets
    pub fn to_array(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
    
    // Common colors
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
}

impl From<[f32; 4]> for Color {
    fn from(array: [f32; 4]) -> Self {
        Self { r: array[0], g: array[1], b: array[2], a: array[3] }
    }
}

impl From<Color> for [f32; 4] {
    fn from(color: Color) -> Self {
        color.to_array()
    }
}

/// Button style options
#[derive(Debug, Clone)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Ghost,
}

impl From<ButtonStyle> for ButtonVariant {
    fn from(style: ButtonStyle) -> Self {
        match style {
            ButtonStyle::Primary => ButtonVariant::Primary,
            ButtonStyle::Secondary => ButtonVariant::Secondary,
            ButtonStyle::Success => ButtonVariant::Primary, // Map to primary for now
            ButtonStyle::Warning => ButtonVariant::Secondary, // Map to secondary for now
            ButtonStyle::Danger => ButtonVariant::Danger,
            ButtonStyle::Ghost => ButtonVariant::Ghost,
        }
    }
}

/// Layout direction for containers
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Row,
    Column,
}

/// Alignment options
#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
}

impl UiBuilder {
    /// Create a new UI builder with the specified theme
    pub fn new(theme: Theme) -> Self {
        Self {
            framework: UiFramework::new(theme),
            widget_stack: Vec::new(),
            named_widgets: HashMap::new(),
        }
    }
    
    /// Create a new UI builder with dark theme
    pub fn dark() -> Self {
        Self::new(Theme::dark())
    }
    
    /// Create a new UI builder with light theme  
    pub fn light() -> Self {
        Self::new(Theme::light())
    }
    
    /// Get mutable access to the underlying framework (for advanced use)
    pub fn framework_mut(&mut self) -> &mut UiFramework {
        &mut self.framework
    }
    
    /// Get immutable access to the underlying framework
    pub fn framework(&self) -> &UiFramework {
        &self.framework
    }
    
    /// Build and return the UI framework
    pub fn build(self) -> UiFramework {
        self.framework
    }
    
    /// Create a text widget
    pub fn text(&mut self, content: &str) -> TextBuilder {
        TextBuilder::new(self, content)
    }
    
    /// Create a button widget
    pub fn button(&mut self, text: &str) -> ButtonBuilder {
        ButtonBuilder::new(self, text)
    }
    
    /// Create a container (panel)
    pub fn container(&mut self) -> ContainerBuilder {
        ContainerBuilder::new(self)
    }
    
    /// Create a row layout
    pub fn row(&mut self) -> LayoutBuilder {
        LayoutBuilder::new(self, Direction::Row)
    }
    
    /// Create a column layout
    pub fn column(&mut self) -> LayoutBuilder {
        LayoutBuilder::new(self, Direction::Column)
    }
    
    /// Get a widget by name
    pub fn get_widget(&self, name: &str) -> Option<WidgetId> {
        self.named_widgets.get(name).copied()
    }
    
    /// Update the UI layout
    pub fn update_layout(&mut self, screen_size: Vec2) {
        self.framework.update_layout(screen_size);
    }
    
    /// Render the UI
    pub fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.framework.render(render_pass, device, queue);
    }
}

/// Builder for text widgets
pub struct TextBuilder<'a> {
    ui: &'a mut UiBuilder,
    text: Text,
    name: Option<String>,
}

impl<'a> TextBuilder<'a> {
    fn new(ui: &'a mut UiBuilder, content: &str) -> Self {
        Self {
            ui,
            text: Text::new(content),
            name: None,
        }
    }
    
    /// Set the font size
    pub fn size(mut self, size: f32) -> Self {
        self.text = self.text.font_size(size);
        self
    }
    
    /// Set the text color
    pub fn color(mut self, color: Color) -> Self {
        self.text = self.text.color(color.to_array().into());
        self
    }
    
    /// Give this widget a name for later reference
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    
    /// Build and add the text widget
    pub fn build(self) -> WidgetId {
        let widget_id = self.ui.framework.add_root_widget(Box::new(self.text));
        
        if let Some(name) = self.name {
            self.ui.named_widgets.insert(name, widget_id);
        }
        
        widget_id
    }
}

/// Builder for button widgets
pub struct ButtonBuilder<'a> {
    ui: &'a mut UiBuilder,
    button: Button,
    name: Option<String>,
}

impl<'a> ButtonBuilder<'a> {
    fn new(ui: &'a mut UiBuilder, text: &str) -> Self {
        Self {
            ui,
            button: Button::new(text),
            name: None,
        }
    }
    
    /// Set the button style
    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.button = self.button.variant(style.into());
        self
    }
    
    /// Set click handler (for simple cases)
    pub fn on_click<F>(self, _handler: F) -> Self 
    where 
        F: Fn() + 'static
    {
        // In a real implementation, we'd store the handler
        // For now, this is a placeholder for the API design
        self
    }
    
    /// Give this widget a name for later reference
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    
    /// Build and add the button widget
    pub fn build(self) -> WidgetId {
        let widget_id = self.ui.framework.add_root_widget(Box::new(self.button));
        
        if let Some(name) = self.name {
            self.ui.named_widgets.insert(name, widget_id);
        }
        
        widget_id
    }
}

/// Builder for container widgets
pub struct ContainerBuilder<'a> {
    ui: &'a mut UiBuilder,
    container: Panel,
    name: Option<String>,
}

impl<'a> ContainerBuilder<'a> {
    fn new(ui: &'a mut UiBuilder) -> Self {
        Self {
            ui,
            container: Panel::new(),
            name: None,
        }
    }
    
    /// Set the background color
    pub fn background(self, _color: Color) -> Self {
        // In a real implementation, we'd set the background color
        // For now, this is a placeholder for the API design
        self
    }
    
    /// Set padding
    pub fn padding(self, _padding: f32) -> Self {
        // In a real implementation, we'd set padding on the container
        self
    }
    
    /// Give this widget a name for later reference
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    
    /// Build and add the container widget
    pub fn build(self) -> WidgetId {
        let widget_id = self.ui.framework.add_root_widget(Box::new(self.container));
        
        if let Some(name) = self.name {
            self.ui.named_widgets.insert(name, widget_id);
        }
        
        widget_id
    }
}

/// Builder for layout containers (Row/Column)
pub struct LayoutBuilder<'a> {
    ui: &'a mut UiBuilder,
    direction: Direction,
    main_alignment: Alignment,
    cross_alignment: Alignment,
    gap: f32,
    padding: f32,
    name: Option<String>,
    children: Vec<WidgetId>,
}

impl<'a> LayoutBuilder<'a> {
    fn new(ui: &'a mut UiBuilder, direction: Direction) -> Self {
        Self {
            ui,
            direction,
            main_alignment: Alignment::Start,
            cross_alignment: Alignment::Start,
            gap: 8.0,
            padding: 8.0,
            name: None,
            children: Vec::new(),
        }
    }
    
    /// Set main axis alignment (along the layout direction)
    pub fn main_alignment(mut self, alignment: Alignment) -> Self {
        self.main_alignment = alignment;
        self
    }
    
    /// Set cross axis alignment (perpendicular to layout direction)
    pub fn cross_alignment(mut self, alignment: Alignment) -> Self {
        self.cross_alignment = alignment;
        self
    }
    
    /// Set gap between children
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }
    
    /// Set padding around the container
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }
    
    /// Add a child widget
    pub fn child(mut self, child_id: WidgetId) -> Self {
        self.children.push(child_id);
        self
    }
    
    /// Give this widget a name for later reference
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    
    /// Build and add the layout widget
    pub fn build(self) -> WidgetId {
        let flex_direction = match self.direction {
            Direction::Row => FlexDirection::Row,
            Direction::Column => FlexDirection::Column,
        };
        
        let main_axis_alignment = match self.main_alignment {
            Alignment::Start => MainAxisAlignment::Start,
            Alignment::Center => MainAxisAlignment::Center,
            Alignment::End => MainAxisAlignment::End,
            Alignment::Stretch => MainAxisAlignment::SpaceBetween, // Approximate
        };
        
        let cross_axis_alignment = match self.cross_alignment {
            Alignment::Start => CrossAxisAlignment::Start,
            Alignment::Center => CrossAxisAlignment::Center,
            Alignment::End => CrossAxisAlignment::End,
            Alignment::Stretch => CrossAxisAlignment::Stretch,
        };
        
        let mut flex = Flex::new(flex_direction)
            .main_axis_alignment(main_axis_alignment)
            .cross_axis_alignment(cross_axis_alignment)
            .spacing(Spacing {
                gap: self.gap,
                padding: Padding::uniform(self.padding),
            });
        
        // Add children to the flex container
        for child_id in self.children {
            flex.add_child(child_id);
        }
        
        let widget_id = self.ui.framework.add_root_widget(Box::new(flex));
        
        if let Some(name) = self.name {
            self.ui.named_widgets.insert(name, widget_id);
        }
        
        widget_id
    }
}

/// Convenience macros for common UI patterns
#[macro_export]
macro_rules! ui {
    // Simple text
    (text $content:expr) => {
        |ui: &mut UiBuilder| ui.text($content).build()
    };
    
    // Text with size
    (text $content:expr, size: $size:expr) => {
        |ui: &mut UiBuilder| ui.text($content).size($size).build()
    };
    
    // Button
    (button $text:expr) => {
        |ui: &mut UiBuilder| ui.button($text).build()
    };
    
    // Button with style
    (button $text:expr, style: $style:expr) => {
        |ui: &mut UiBuilder| ui.button($text).style($style).build()
    };
}

/// Example usage functions to demonstrate the API
pub fn example_simple_ui() -> UiFramework {
    let mut ui = UiBuilder::dark();
    
    // Create a simple UI with text and buttons
    ui.text("Welcome to Lumina Engine!")
        .size(24.0)
        .color(Color::WHITE)
        .name("title")
        .build();
    
    ui.button("Start Game")
        .style(ButtonStyle::Primary)
        .name("start_button")
        .build();
    
    ui.button("Settings")
        .style(ButtonStyle::Secondary)
        .name("settings_button")
        .build();
    
    ui.button("Quit")
        .style(ButtonStyle::Danger)
        .name("quit_button")
        .build();
    
    ui.build()
}

pub fn example_layout_ui() -> UiFramework {
    let mut ui = UiBuilder::dark();
    
    // Create a more complex layout
    let title = ui.text("Game Menu")
        .size(32.0)
        .color(Color::WHITE)
        .build();
    
    let start_btn = ui.button("Start Game")
        .style(ButtonStyle::Primary)
        .build();
    
    let settings_btn = ui.button("Settings")
        .style(ButtonStyle::Secondary)
        .build();
    
    let quit_btn = ui.button("Quit")
        .style(ButtonStyle::Danger)
        .build();
    
    // Arrange in a column
    ui.column()
        .main_alignment(Alignment::Center)
        .cross_alignment(Alignment::Center)
        .gap(16.0)
        .child(title)
        .child(start_btn)
        .child(settings_btn)
        .child(quit_btn)
        .name("main_menu")
        .build();
    
    ui.build()
}