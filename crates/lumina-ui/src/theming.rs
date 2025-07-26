//! Theming and styling system for the UI framework

use glam::Vec4;
use serde::{Deserialize, Serialize};

/// Complete theme definition for the UI framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Color palette
    pub colors: ColorPalette,
    /// Typography settings
    pub typography: Typography,
    /// Spacing scale
    pub spacing: SpacingScale,
    /// Animation settings
    pub animations: AnimationSettings,
    /// Component-specific styles
    pub components: ComponentStyles,
}

/// Color palette for the theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    /// Primary brand color
    pub primary: Vec4,
    /// Secondary brand color
    pub secondary: Vec4,
    /// Accent color for highlights
    pub accent: Vec4,
    /// Background colors
    pub background: BackgroundColors,
    /// Surface colors (cards, panels)
    pub surface: SurfaceColors,
    /// Text colors
    pub text: TextColors,
    /// Border colors
    pub border: BorderColors,
    /// Status colors
    pub status: StatusColors,
}

/// Background color variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundColors {
    /// Primary background
    pub primary: Vec4,
    /// Secondary background
    pub secondary: Vec4,
    /// Tertiary background
    pub tertiary: Vec4,
}

/// Surface color variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceColors {
    /// Default surface
    pub default: Vec4,
    /// Elevated surface
    pub elevated: Vec4,
    /// Overlay surface
    pub overlay: Vec4,
}

/// Text color variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextColors {
    /// Primary text
    pub primary: Vec4,
    /// Secondary text
    pub secondary: Vec4,
    /// Disabled text
    pub disabled: Vec4,
    /// Inverse text (on dark backgrounds)
    pub inverse: Vec4,
}

/// Border color variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderColors {
    /// Default border
    pub default: Vec4,
    /// Focused border
    pub focused: Vec4,
    /// Error border
    pub error: Vec4,
}

/// Status color variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusColors {
    /// Success/positive status
    pub success: Vec4,
    /// Warning status
    pub warning: Vec4,
    /// Error/danger status
    pub error: Vec4,
    /// Info status
    pub info: Vec4,
}

/// Typography settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    /// Font families
    pub font_families: FontFamilies,
    /// Font sizes
    pub font_sizes: FontSizes,
    /// Font weights
    pub font_weights: FontWeights,
    /// Line heights
    pub line_heights: LineHeights,
}

/// Font family definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontFamilies {
    /// Primary font (for body text)
    pub primary: String,
    /// Secondary font (for headings)
    pub secondary: String,
    /// Monospace font (for code)
    pub monospace: String,
}

/// Font size scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSizes {
    /// Extra small text
    pub xs: f32,
    /// Small text
    pub sm: f32,
    /// Base text size
    pub base: f32,
    /// Large text
    pub lg: f32,
    /// Extra large text
    pub xl: f32,
    /// Display heading 1
    pub display1: f32,
    /// Display heading 2
    pub display2: f32,
    /// Display heading 3
    pub display3: f32,
}

/// Font weight definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontWeights {
    /// Light weight
    pub light: u16,
    /// Normal weight
    pub normal: u16,
    /// Medium weight
    pub medium: u16,
    /// Bold weight
    pub bold: u16,
}

/// Line height scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineHeights {
    /// Tight line height
    pub tight: f32,
    /// Normal line height
    pub normal: f32,
    /// Loose line height
    pub loose: f32,
}

/// Spacing scale for layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingScale {
    /// Available spacing values
    pub values: Vec<f32>,
}

/// Animation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationSettings {
    /// Duration presets
    pub durations: AnimationDurations,
    /// Easing functions
    pub easing: AnimationEasing,
}

/// Animation duration presets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationDurations {
    /// Instant (no animation)
    pub instant: f32,
    /// Fast animation
    pub fast: f32,
    /// Normal animation
    pub normal: f32,
    /// Slow animation
    pub slow: f32,
}

/// Animation easing curves
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationEasing {
    /// Linear easing
    pub linear: EasingCurve,
    /// Ease in
    pub ease_in: EasingCurve,
    /// Ease out
    pub ease_out: EasingCurve,
    /// Ease in-out
    pub ease_in_out: EasingCurve,
}

/// Easing curve definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EasingCurve {
    /// Control points for cubic bezier curve
    pub control_points: [f32; 4],
}

/// Component-specific style definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStyles {
    /// Button styles
    pub button: ButtonTheme,
    /// Text input styles
    pub text_input: TextInputTheme,
    /// Panel styles
    pub panel: PanelTheme,
}

/// Button theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonTheme {
    /// Primary button variant
    pub primary: ButtonVariant,
    /// Secondary button variant
    pub secondary: ButtonVariant,
    /// Ghost button variant
    pub ghost: ButtonVariant,
    /// Danger button variant
    pub danger: ButtonVariant,
}

/// Button variant styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonVariant {
    /// Default state colors
    pub default: ButtonStateColors,
    /// Hovered state colors
    pub hovered: ButtonStateColors,
    /// Pressed state colors
    pub pressed: ButtonStateColors,
    /// Disabled state colors
    pub disabled: ButtonStateColors,
    /// Border radius
    pub border_radius: f32,
    /// Padding
    pub padding: [f32; 4], // [top, right, bottom, left]
}

/// Button state colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonStateColors {
    /// Background color
    pub background: Vec4,
    /// Text color
    pub text: Vec4,
    /// Border color
    pub border: Vec4,
}

/// Text input theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextInputTheme {
    /// Default state
    pub default: TextInputState,
    /// Focused state
    pub focused: TextInputState,
    /// Error state
    pub error: TextInputState,
    /// Disabled state
    pub disabled: TextInputState,
    /// Border radius
    pub border_radius: f32,
    /// Padding
    pub padding: [f32; 4],
}

/// Text input state styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextInputState {
    /// Background color
    pub background: Vec4,
    /// Border color
    pub border: Vec4,
    /// Text color
    pub text: Vec4,
    /// Placeholder text color
    pub placeholder: Vec4,
}

/// Panel theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelTheme {
    /// Default panel style
    pub default: PanelStyle,
    /// Elevated panel style
    pub elevated: PanelStyle,
    /// Outlined panel style
    pub outlined: PanelStyle,
}

/// Panel style definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelStyle {
    /// Background color
    pub background: Vec4,
    /// Border color
    pub border: Vec4,
    /// Border width
    pub border_width: f32,
    /// Border radius
    pub border_radius: f32,
    /// Drop shadow
    pub shadow: Option<ShadowStyle>,
}

/// Drop shadow style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowStyle {
    /// Shadow offset
    pub offset: [f32; 2],
    /// Shadow blur radius
    pub blur_radius: f32,
    /// Shadow color
    pub color: Vec4,
}

impl Theme {
    /// Create a modern dark theme
    pub fn dark() -> Self {
        Self {
            colors: ColorPalette {
                primary: Vec4::new(0.4, 0.5, 0.9, 1.0),     // #667eea
                secondary: Vec4::new(0.46, 0.29, 0.64, 1.0), // #764ba2
                accent: Vec4::new(0.0, 0.8, 0.6, 1.0),       // #00cc99
                background: BackgroundColors {
                    primary: Vec4::new(0.06, 0.06, 0.14, 1.0),   // #0f0f23
                    secondary: Vec4::new(0.1, 0.1, 0.18, 1.0),    // #1a1a2e
                    tertiary: Vec4::new(0.15, 0.15, 0.25, 1.0),   // #26263f
                },
                surface: SurfaceColors {
                    default: Vec4::new(0.1, 0.1, 0.18, 1.0),      // #1a1a2e
                    elevated: Vec4::new(0.15, 0.15, 0.25, 1.0),   // #26263f
                    overlay: Vec4::new(0.2, 0.2, 0.3, 0.95),      // #333344 with alpha
                },
                text: TextColors {
                    primary: Vec4::new(1.0, 1.0, 1.0, 1.0),       // #ffffff
                    secondary: Vec4::new(0.8, 0.8, 0.8, 1.0),     // #cccccc
                    disabled: Vec4::new(0.5, 0.5, 0.5, 1.0),      // #808080
                    inverse: Vec4::new(0.1, 0.1, 0.1, 1.0),       // #1a1a1a
                },
                border: BorderColors {
                    default: Vec4::new(0.3, 0.3, 0.4, 1.0),       // #4d4d66
                    focused: Vec4::new(0.4, 0.5, 0.9, 1.0),       // #667eea
                    error: Vec4::new(0.9, 0.3, 0.3, 1.0),         // #e64d4d
                },
                status: StatusColors {
                    success: Vec4::new(0.0, 0.8, 0.4, 1.0),       // #00cc66
                    warning: Vec4::new(1.0, 0.6, 0.0, 1.0),       // #ff9900
                    error: Vec4::new(0.9, 0.3, 0.3, 1.0),         // #e64d4d
                    info: Vec4::new(0.0, 0.6, 0.9, 1.0),          // #0099e6
                },
            },
            typography: Typography {
                font_families: FontFamilies {
                    primary: "Inter".to_string(),
                    secondary: "Inter".to_string(),
                    monospace: "JetBrains Mono".to_string(),
                },
                font_sizes: FontSizes {
                    xs: 10.0,
                    sm: 12.0,
                    base: 14.0,
                    lg: 16.0,
                    xl: 18.0,
                    display1: 32.0,
                    display2: 24.0,
                    display3: 20.0,
                },
                font_weights: FontWeights {
                    light: 300,
                    normal: 400,
                    medium: 500,
                    bold: 700,
                },
                line_heights: LineHeights {
                    tight: 1.2,
                    normal: 1.5,
                    loose: 1.8,
                },
            },
            spacing: SpacingScale {
                values: vec![0.0, 4.0, 8.0, 12.0, 16.0, 24.0, 32.0, 48.0, 64.0, 96.0],
            },
            animations: AnimationSettings {
                durations: AnimationDurations {
                    instant: 0.0,
                    fast: 0.15,
                    normal: 0.3,
                    slow: 0.5,
                },
                easing: AnimationEasing {
                    linear: EasingCurve { control_points: [0.0, 0.0, 1.0, 1.0] },
                    ease_in: EasingCurve { control_points: [0.42, 0.0, 1.0, 1.0] },
                    ease_out: EasingCurve { control_points: [0.0, 0.0, 0.58, 1.0] },
                    ease_in_out: EasingCurve { control_points: [0.42, 0.0, 0.58, 1.0] },
                },
            },
            components: ComponentStyles {
                button: ButtonTheme {
                    primary: ButtonVariant {
                        default: ButtonStateColors {
                            background: Vec4::new(0.4, 0.5, 0.9, 1.0),
                            text: Vec4::new(1.0, 1.0, 1.0, 1.0),
                            border: Vec4::new(0.4, 0.5, 0.9, 1.0),
                        },
                        hovered: ButtonStateColors {
                            background: Vec4::new(0.35, 0.45, 0.85, 1.0),
                            text: Vec4::new(1.0, 1.0, 1.0, 1.0),
                            border: Vec4::new(0.35, 0.45, 0.85, 1.0),
                        },
                        pressed: ButtonStateColors {
                            background: Vec4::new(0.3, 0.4, 0.8, 1.0),
                            text: Vec4::new(1.0, 1.0, 1.0, 1.0),
                            border: Vec4::new(0.3, 0.4, 0.8, 1.0),
                        },
                        disabled: ButtonStateColors {
                            background: Vec4::new(0.2, 0.2, 0.3, 1.0),
                            text: Vec4::new(0.5, 0.5, 0.5, 1.0),
                            border: Vec4::new(0.2, 0.2, 0.3, 1.0),
                        },
                        border_radius: 6.0,
                        padding: [8.0, 16.0, 8.0, 16.0],
                    },
                    secondary: ButtonVariant {
                        default: ButtonStateColors {
                            background: Vec4::new(0.15, 0.15, 0.25, 1.0),
                            text: Vec4::new(0.8, 0.8, 0.8, 1.0),
                            border: Vec4::new(0.3, 0.3, 0.4, 1.0),
                        },
                        hovered: ButtonStateColors {
                            background: Vec4::new(0.2, 0.2, 0.3, 1.0),
                            text: Vec4::new(1.0, 1.0, 1.0, 1.0),
                            border: Vec4::new(0.4, 0.4, 0.5, 1.0),
                        },
                        pressed: ButtonStateColors {
                            background: Vec4::new(0.1, 0.1, 0.2, 1.0),
                            text: Vec4::new(0.8, 0.8, 0.8, 1.0),
                            border: Vec4::new(0.3, 0.3, 0.4, 1.0),
                        },
                        disabled: ButtonStateColors {
                            background: Vec4::new(0.1, 0.1, 0.15, 1.0),
                            text: Vec4::new(0.3, 0.3, 0.3, 1.0),
                            border: Vec4::new(0.2, 0.2, 0.25, 1.0),
                        },
                        border_radius: 6.0,
                        padding: [8.0, 16.0, 8.0, 16.0],
                    },
                    ghost: ButtonVariant {
                        default: ButtonStateColors {
                            background: Vec4::new(0.0, 0.0, 0.0, 0.0),
                            text: Vec4::new(0.8, 0.8, 0.8, 1.0),
                            border: Vec4::new(0.0, 0.0, 0.0, 0.0),
                        },
                        hovered: ButtonStateColors {
                            background: Vec4::new(0.15, 0.15, 0.25, 0.5),
                            text: Vec4::new(1.0, 1.0, 1.0, 1.0),
                            border: Vec4::new(0.0, 0.0, 0.0, 0.0),
                        },
                        pressed: ButtonStateColors {
                            background: Vec4::new(0.1, 0.1, 0.2, 0.7),
                            text: Vec4::new(0.8, 0.8, 0.8, 1.0),
                            border: Vec4::new(0.0, 0.0, 0.0, 0.0),
                        },
                        disabled: ButtonStateColors {
                            background: Vec4::new(0.0, 0.0, 0.0, 0.0),
                            text: Vec4::new(0.3, 0.3, 0.3, 1.0),
                            border: Vec4::new(0.0, 0.0, 0.0, 0.0),
                        },
                        border_radius: 6.0,
                        padding: [8.0, 16.0, 8.0, 16.0],
                    },
                    danger: ButtonVariant {
                        default: ButtonStateColors {
                            background: Vec4::new(0.9, 0.3, 0.3, 1.0),
                            text: Vec4::new(1.0, 1.0, 1.0, 1.0),
                            border: Vec4::new(0.9, 0.3, 0.3, 1.0),
                        },
                        hovered: ButtonStateColors {
                            background: Vec4::new(0.8, 0.25, 0.25, 1.0),
                            text: Vec4::new(1.0, 1.0, 1.0, 1.0),
                            border: Vec4::new(0.8, 0.25, 0.25, 1.0),
                        },
                        pressed: ButtonStateColors {
                            background: Vec4::new(0.7, 0.2, 0.2, 1.0),
                            text: Vec4::new(1.0, 1.0, 1.0, 1.0),
                            border: Vec4::new(0.7, 0.2, 0.2, 1.0),
                        },
                        disabled: ButtonStateColors {
                            background: Vec4::new(0.3, 0.15, 0.15, 1.0),
                            text: Vec4::new(0.5, 0.3, 0.3, 1.0),
                            border: Vec4::new(0.3, 0.15, 0.15, 1.0),
                        },
                        border_radius: 6.0,
                        padding: [8.0, 16.0, 8.0, 16.0],
                    },
                },
                text_input: TextInputTheme {
                    default: TextInputState {
                        background: Vec4::new(0.1, 0.1, 0.18, 1.0),
                        border: Vec4::new(0.3, 0.3, 0.4, 1.0),
                        text: Vec4::new(1.0, 1.0, 1.0, 1.0),
                        placeholder: Vec4::new(0.5, 0.5, 0.5, 1.0),
                    },
                    focused: TextInputState {
                        background: Vec4::new(0.15, 0.15, 0.25, 1.0),
                        border: Vec4::new(0.4, 0.5, 0.9, 1.0),
                        text: Vec4::new(1.0, 1.0, 1.0, 1.0),
                        placeholder: Vec4::new(0.6, 0.6, 0.6, 1.0),
                    },
                    error: TextInputState {
                        background: Vec4::new(0.2, 0.1, 0.1, 1.0),
                        border: Vec4::new(0.9, 0.3, 0.3, 1.0),
                        text: Vec4::new(1.0, 1.0, 1.0, 1.0),
                        placeholder: Vec4::new(0.7, 0.4, 0.4, 1.0),
                    },
                    disabled: TextInputState {
                        background: Vec4::new(0.08, 0.08, 0.12, 1.0),
                        border: Vec4::new(0.2, 0.2, 0.25, 1.0),
                        text: Vec4::new(0.3, 0.3, 0.3, 1.0),
                        placeholder: Vec4::new(0.2, 0.2, 0.2, 1.0),
                    },
                    border_radius: 4.0,
                    padding: [8.0, 12.0, 8.0, 12.0],
                },
                panel: PanelTheme {
                    default: PanelStyle {
                        background: Vec4::new(0.1, 0.1, 0.18, 1.0),
                        border: Vec4::new(0.0, 0.0, 0.0, 0.0),
                        border_width: 0.0,
                        border_radius: 8.0,
                        shadow: None,
                    },
                    elevated: PanelStyle {
                        background: Vec4::new(0.15, 0.15, 0.25, 1.0),
                        border: Vec4::new(0.0, 0.0, 0.0, 0.0),
                        border_width: 0.0,
                        border_radius: 8.0,
                        shadow: Some(ShadowStyle {
                            offset: [0.0, 4.0],
                            blur_radius: 16.0,
                            color: Vec4::new(0.0, 0.0, 0.0, 0.25),
                        }),
                    },
                    outlined: PanelStyle {
                        background: Vec4::new(0.0, 0.0, 0.0, 0.0),
                        border: Vec4::new(0.3, 0.3, 0.4, 1.0),
                        border_width: 1.0,
                        border_radius: 8.0,
                        shadow: None,
                    },
                },
            },
        }
    }
    
    /// Create a modern light theme
    pub fn light() -> Self {
        // For now, return dark theme - implement light theme later
        Self::dark()
    }
    
    /// Get a spacing value by index
    pub fn spacing(&self, index: usize) -> f32 {
        self.spacing.values.get(index).copied().unwrap_or(0.0)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}