//! Button widget implementation

use crate::{
    Widget, WidgetId, LayoutConstraints, InputEvent, InputResponse, 
    UiRenderer, Rect, Theme, widgets::{BaseWidget, WidgetStyle, AnimationState},
    layout::LayoutResult,
};
use glam::{Vec2, Vec4};
use serde::{Deserialize, Serialize};

/// Button widget for user interactions
pub struct Button {
    /// Base widget properties
    base: BaseWidget,
    /// Button text
    text: String,
    /// Button variant style
    variant: ButtonVariant,
    /// Current animation state
    state: AnimationState,
    /// Click callback
    on_click: Option<Box<dyn Fn() + Send + Sync>>,
    /// Whether the button is currently pressed
    is_pressed: bool,
    /// Whether the button is currently hovered
    is_hovered: bool,
}

impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Button")
            .field("base", &self.base)
            .field("text", &self.text)
            .field("variant", &self.variant)
            .field("state", &self.state)
            .field("on_click", &"<callback>")
            .field("is_pressed", &self.is_pressed)
            .field("is_hovered", &self.is_hovered)
            .finish()
    }
}

/// Button style variants
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ButtonVariant {
    /// Primary button (most important action)
    Primary,
    /// Secondary button (less important action)
    Secondary,
    /// Ghost button (subtle action)
    Ghost,
    /// Danger button (destructive action)
    Danger,
}

impl Button {
    /// Create a new button with the given text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            base: BaseWidget::default(),
            text: text.into(),
            variant: ButtonVariant::Primary,
            state: AnimationState::Normal,
            on_click: None,
            is_pressed: false,
            is_hovered: false,
        }
    }
    
    /// Set the button variant
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    
    /// Set the button text
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }
    
    /// Set the click callback
    pub fn on_click<F>(mut self, callback: F) -> Self 
    where F: Fn() + Send + Sync + 'static {
        self.on_click = Some(Box::new(callback));
        self
    }
    
    /// Set the button style
    pub fn style(mut self, style: WidgetStyle) -> Self {
        self.base.style = style;
        self
    }
    
    /// Enable or disable the button
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.base.enabled = enabled;
        if !enabled {
            self.state = AnimationState::Disabled;
        }
        self
    }
    
    /// Get the current text
    pub fn get_text(&self) -> &str {
        &self.text
    }
    
    /// Set text (for dynamic updates)
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }
    
    /// Check if button is currently pressed
    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }
    
    /// Check if button is currently hovered
    pub fn is_hovered(&self) -> bool {
        self.is_hovered
    }
    
    /// Get the current button colors based on theme and state
    pub fn get_current_colors(&self, theme: &Theme) -> (Vec4, Vec4, Vec4) {
        let button_theme = &theme.components.button;
        let variant_theme = match self.variant {
            ButtonVariant::Primary => &button_theme.primary,
            ButtonVariant::Secondary => &button_theme.secondary,
            ButtonVariant::Ghost => &button_theme.ghost,
            ButtonVariant::Danger => &button_theme.danger,
        };
        
        let colors = match self.state {
            AnimationState::Normal => &variant_theme.default,
            AnimationState::Hovered => &variant_theme.hovered,
            AnimationState::Pressed => &variant_theme.pressed,
            AnimationState::Disabled => &variant_theme.disabled,
            AnimationState::Focused => &variant_theme.hovered, // Use hovered colors for focus
        };
        
        (colors.background, colors.text, colors.border)
    }
    
    /// Calculate the minimum size needed for the button
    pub fn calculate_min_size(&self, theme: &Theme) -> Vec2 {
        let font_size = theme.typography.font_sizes.base;
        let padding = match self.variant {
            ButtonVariant::Primary => &theme.components.button.primary.padding,
            ButtonVariant::Secondary => &theme.components.button.secondary.padding,
            ButtonVariant::Ghost => &theme.components.button.ghost.padding,
            ButtonVariant::Danger => &theme.components.button.danger.padding,
        };
        
        // Estimate text size (in a real implementation, this would measure actual text)
        let text_width = self.text.len() as f32 * font_size * 0.6; // Rough estimate
        let text_height = font_size;
        
        Vec2::new(
            text_width + padding[1] + padding[3], // left + right padding
            text_height + padding[0] + padding[2], // top + bottom padding
        )
    }
}

impl Widget for Button {
    fn id(&self) -> WidgetId {
        self.base.id
    }
    
    fn layout_constraints(&self) -> LayoutConstraints {
        self.base.constraints.clone()
    }
    
    fn layout(&mut self, available_space: Vec2) -> LayoutResult {
        // Calculate button size based on text content
        let text_width = self.text.len() as f32 * 12.0; // Approximate character width
        let button_width = (text_width + 20.0).min(available_space.x); // Add padding
        let button_height = 30.0_f32.min(available_space.y); // Fixed height
        
        let bounds = Rect::new(0.0, 0.0, button_width, button_height);
        
        let result = LayoutResult {
            bounds,
            overflow: button_width > available_space.x || button_height > available_space.y,
            content_size: Vec2::new(button_width, button_height),
        };
        
        self.base.layout_cache = Some(result.clone());
        result
    }
    
    fn handle_input(&mut self, input: &InputEvent) -> InputResponse {
        if !self.base.enabled {
            return InputResponse::NotHandled;
        }
        
        match input {
            InputEvent::MouseEnter => {
                self.is_hovered = true;
                self.state = AnimationState::Hovered;
                InputResponse::Handled
            }
            
            InputEvent::MouseExit => {
                self.is_hovered = false;
                self.is_pressed = false;
                self.state = AnimationState::Normal;
                InputResponse::Handled
            }
            
            InputEvent::MouseDown { .. } => {
                self.is_pressed = true;
                self.state = AnimationState::Pressed;
                InputResponse::Handled
            }
            
            InputEvent::MouseUp { .. } => {
                if self.is_pressed && self.is_hovered {
                    // Button was clicked
                    if let Some(callback) = &self.on_click {
                        callback();
                    }
                }
                self.is_pressed = false;
                self.state = if self.is_hovered {
                    AnimationState::Hovered
                } else {
                    AnimationState::Normal
                };
                InputResponse::Handled
            }
            
            InputEvent::MouseClick { .. } => {
                if self.is_hovered {
                    if let Some(callback) = &self.on_click {
                        callback();
                    }
                    InputResponse::Handled
                } else {
                    InputResponse::NotHandled
                }
            }
            
            InputEvent::FocusGained => {
                self.state = AnimationState::Focused;
                InputResponse::Handled
            }
            
            InputEvent::FocusLost => {
                self.state = AnimationState::Normal;
                InputResponse::Handled
            }
            
            InputEvent::KeyDown { key, .. } => {
                // Handle Enter/Space key presses when focused
                match key {
                    crate::input::KeyCode::Enter | crate::input::KeyCode::Space => {
                        if let Some(callback) = &self.on_click {
                            callback();
                        }
                        InputResponse::Handled
                    }
                    _ => InputResponse::NotHandled
                }
            }
            
            _ => InputResponse::NotHandled
        }
    }
    
    fn render(&self, renderer: &mut UiRenderer, bounds: Rect) {
        if !self.base.visible {
            return;
        }
        
        // Get theme (in a real implementation, this would be passed in or accessed differently)
        let theme = Theme::default();
        let (bg_color, text_color, border_color) = self.get_current_colors(&theme);
        
        // Get border radius
        let border_radius = match self.variant {
            ButtonVariant::Primary => theme.components.button.primary.border_radius,
            ButtonVariant::Secondary => theme.components.button.secondary.border_radius,
            ButtonVariant::Ghost => theme.components.button.ghost.border_radius,
            ButtonVariant::Danger => theme.components.button.danger.border_radius,
        };
        
        // Draw background
        if bg_color.w > 0.0 { // Only draw if not transparent
            renderer.draw_rounded_rect(bounds, bg_color, border_radius);
        }
        
        // Draw border (if visible)
        if border_color.w > 0.0 && border_color != bg_color {
            // TODO: Implement border rendering
            // For now, just draw a slightly smaller rectangle on top
        }
        
        // Draw text
        if !self.text.is_empty() {
            let font_size = theme.typography.font_sizes.base;
            let text_pos = Vec2::new(
                bounds.position.x + bounds.size.x * 0.5, // Center horizontally
                bounds.position.y + bounds.size.y * 0.5, // Center vertically
            );
            
            // TODO: Use actual font handle
            let font_handle = lumina_render::FontHandle(0);
            renderer.draw_text(&self.text, text_pos, font_handle, font_size, text_color);
        }
    }
    
    fn can_focus(&self) -> bool {
        self.base.enabled
    }
    
    fn on_focus_gained(&mut self) {
        if self.base.enabled && self.state == AnimationState::Normal {
            self.state = AnimationState::Focused;
        }
    }
    
    fn on_focus_lost(&mut self) {
        if self.state == AnimationState::Focused {
            self.state = AnimationState::Normal;
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new("Button")
    }
}

/// Builder for creating buttons with a fluent API
pub struct ButtonBuilder {
    button: Button,
}

impl ButtonBuilder {
    /// Create a new button builder
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            button: Button::new(text),
        }
    }
    
    /// Set the button variant
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.button = self.button.variant(variant);
        self
    }
    
    /// Set the click callback
    pub fn on_click<F>(mut self, callback: F) -> Self 
    where F: Fn() + Send + Sync + 'static {
        self.button = self.button.on_click(callback);
        self
    }
    
    /// Set the button style
    pub fn style(mut self, style: WidgetStyle) -> Self {
        self.button = self.button.style(style);
        self
    }
    
    /// Enable or disable the button
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.button = self.button.enabled(enabled);
        self
    }
    
    /// Build the button
    pub fn build(self) -> Button {
        self.button
    }
}

/// Convenience function for creating a primary button
pub fn button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).variant(ButtonVariant::Primary)
}

/// Convenience function for creating a secondary button
pub fn secondary_button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).variant(ButtonVariant::Secondary)
}

/// Convenience function for creating a ghost button
pub fn ghost_button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).variant(ButtonVariant::Ghost)
}

/// Convenience function for creating a danger button
pub fn danger_button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).variant(ButtonVariant::Danger)
}