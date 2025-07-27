//! Input handling for the Lumina UI framework

use glam::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Input handler that processes and distributes input events
#[derive(Debug, Default)]
pub struct InputHandler {
    /// Currently pressed keys
    pressed_keys: HashSet<KeyCode>,
    /// Previously pressed keys (for detecting key releases)
    previous_keys: HashSet<KeyCode>,
    /// Current mouse position
    mouse_position: Vec2,
    /// Previous mouse position
    previous_mouse_position: Vec2,
    /// Currently pressed mouse buttons
    pressed_mouse_buttons: HashSet<MouseButton>,
    /// Previous mouse buttons (for detecting releases)
    previous_mouse_buttons: HashSet<MouseButton>,
    /// Mouse wheel delta for this frame
    mouse_wheel_delta: Vec2,
    /// Text input buffer for this frame
    text_input: String,
    /// Whether input has been captured by a widget (prevents propagation)
    input_captured: bool,
}

/// Input events that can be sent to widgets
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    /// Key was pressed down
    KeyDown { key: KeyCode, modifiers: Modifiers },
    /// Key was released
    KeyUp { key: KeyCode, modifiers: Modifiers },
    /// Key is being held down (sent every frame while held)
    KeyHeld { key: KeyCode, modifiers: Modifiers },
    /// Text input (for typing)
    TextInput { text: String },
    /// Mouse button was pressed
    MouseDown { button: MouseButton, position: Vec2, modifiers: Modifiers },
    /// Mouse button was released
    MouseUp { button: MouseButton, position: Vec2, modifiers: Modifiers },
    /// Mouse button was clicked (down then up quickly)
    MouseClick { button: MouseButton, position: Vec2, modifiers: Modifiers },
    /// Mouse moved
    MouseMove { position: Vec2, delta: Vec2 },
    /// Mouse wheel scrolled
    MouseWheel { delta: Vec2, position: Vec2 },
    /// Mouse entered widget area
    MouseEnter,
    /// Mouse exited widget area
    MouseExit,
    /// Widget gained focus
    FocusGained,
    /// Widget lost focus
    FocusLost,
}

/// Response that widgets can return to input events
#[derive(Debug, Clone, PartialEq)]
pub enum InputResponse {
    /// Input was handled, don't propagate to other widgets
    Handled,
    /// Input was not handled, continue propagation
    NotHandled,
    /// Request focus for this widget
    RequestFocus,
    /// Request to capture all input (modal behavior)
    CaptureInput,
    /// Release input capture
    ReleaseCapture,
}

/// Keyboard key codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCode {
    // Letters
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    
    // Numbers
    Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9,
    
    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    
    // Arrow keys
    ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
    
    // Special keys
    Space, Enter, Escape, Tab, Backspace, Delete,
    Home, End, PageUp, PageDown, Insert,
    
    // Modifier keys
    Shift, Control, Alt, Meta,
    
    // Punctuation
    Comma, Period, Semicolon, Quote, Backtick,
    Slash, Backslash, Minus, Equal,
    LeftBracket, RightBracket,
    
    // Numpad
    Numpad0, Numpad1, Numpad2, Numpad3, Numpad4,
    Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    NumpadAdd, NumpadSubtract, NumpadMultiply, NumpadDivide,
    NumpadEnter, NumpadDecimal,
    
    // Other
    PrintScreen, ScrollLock, Pause,
    CapsLock, NumLock,
}

/// Mouse button types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button (wheel click)
    Middle,
    /// Additional mouse button (back)
    Back,
    /// Additional mouse button (forward)
    Forward,
}

/// Keyboard modifier keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Modifiers {
    /// Shift key is pressed
    pub shift: bool,
    /// Control key is pressed (Cmd on macOS)
    pub ctrl: bool,
    /// Alt key is pressed (Option on macOS)
    pub alt: bool,
    /// Meta key is pressed (Windows key on Windows, Cmd on macOS)
    pub meta: bool,
}

impl InputHandler {
    /// Create a new input handler
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Begin a new input frame
    pub fn begin_frame(&mut self) {
        // Move current state to previous
        self.previous_keys = self.pressed_keys.clone();
        self.previous_mouse_buttons = self.pressed_mouse_buttons.clone();
        self.previous_mouse_position = self.mouse_position;
        
        // Reset frame-specific state
        self.mouse_wheel_delta = Vec2::ZERO;
        self.text_input.clear();
        self.input_captured = false;
    }
    
    /// Process a raw input event and convert to UI events
    pub fn process_input(&mut self, raw_input: &RawInputEvent) -> Vec<InputEvent> {
        let mut events = Vec::new();
        
        match raw_input {
            RawInputEvent::KeyDown { key, modifiers } => {
                if self.pressed_keys.insert(*key) {
                    events.push(InputEvent::KeyDown {
                        key: *key,
                        modifiers: *modifiers,
                    });
                }
            }
            
            RawInputEvent::KeyUp { key, modifiers } => {
                if self.pressed_keys.remove(key) {
                    events.push(InputEvent::KeyUp {
                        key: *key,
                        modifiers: *modifiers,
                    });
                }
            }
            
            RawInputEvent::TextInput { text } => {
                self.text_input.push_str(text);
                events.push(InputEvent::TextInput { text: text.clone() });
            }
            
            RawInputEvent::MouseDown { button, position, modifiers } => {
                self.mouse_position = *position;
                if self.pressed_mouse_buttons.insert(*button) {
                    events.push(InputEvent::MouseDown {
                        button: *button,
                        position: *position,
                        modifiers: *modifiers,
                    });
                }
            }
            
            RawInputEvent::MouseUp { button, position, modifiers } => {
                self.mouse_position = *position;
                if self.pressed_mouse_buttons.remove(button) {
                    events.push(InputEvent::MouseUp {
                        button: *button,
                        position: *position,
                        modifiers: *modifiers,
                    });
                    
                    // Generate click event if button was pressed and released quickly
                    events.push(InputEvent::MouseClick {
                        button: *button,
                        position: *position,
                        modifiers: *modifiers,
                    });
                }
            }
            
            RawInputEvent::MouseMove { position } => {
                let delta = *position - self.mouse_position;
                self.mouse_position = *position;
                events.push(InputEvent::MouseMove {
                    position: *position,
                    delta,
                });
            }
            
            RawInputEvent::MouseWheel { delta, position } => {
                self.mouse_wheel_delta += *delta;
                events.push(InputEvent::MouseWheel {
                    delta: *delta,
                    position: *position,
                });
            }
        }
        
        events
    }
    
    /// Generate held key events for currently pressed keys
    pub fn generate_held_events(&self, modifiers: Modifiers) -> Vec<InputEvent> {
        self.pressed_keys
            .iter()
            .map(|&key| InputEvent::KeyHeld { key, modifiers })
            .collect()
    }
    
    /// Check if a key is currently pressed
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }
    
    /// Check if a key was just pressed this frame
    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key) && !self.previous_keys.contains(&key)
    }
    
    /// Check if a key was just released this frame
    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        !self.pressed_keys.contains(&key) && self.previous_keys.contains(&key)
    }
    
    /// Check if a mouse button is currently pressed
    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(&button)
    }
    
    /// Check if a mouse button was just pressed this frame
    pub fn is_mouse_just_pressed(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(&button) && !self.previous_mouse_buttons.contains(&button)
    }
    
    /// Check if a mouse button was just released this frame
    pub fn is_mouse_just_released(&self, button: MouseButton) -> bool {
        !self.pressed_mouse_buttons.contains(&button) && self.previous_mouse_buttons.contains(&button)
    }
    
    /// Get current mouse position
    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }
    
    /// Get mouse movement delta for this frame
    pub fn mouse_delta(&self) -> Vec2 {
        self.mouse_position - self.previous_mouse_position
    }
    
    /// Get mouse wheel delta for this frame
    pub fn mouse_wheel_delta(&self) -> Vec2 {
        self.mouse_wheel_delta
    }
    
    /// Get text input for this frame
    pub fn text_input(&self) -> &str {
        &self.text_input
    }
    
    /// Set input capture state
    pub fn set_input_captured(&mut self, captured: bool) {
        self.input_captured = captured;
    }
    
    /// Check if input is currently captured
    pub fn is_input_captured(&self) -> bool {
        self.input_captured
    }
}

/// Raw input events from the windowing system
#[derive(Debug, Clone)]
pub enum RawInputEvent {
    /// Key was pressed
    KeyDown { key: KeyCode, modifiers: Modifiers },
    /// Key was released
    KeyUp { key: KeyCode, modifiers: Modifiers },
    /// Text was input
    TextInput { text: String },
    /// Mouse button was pressed
    MouseDown { button: MouseButton, position: Vec2, modifiers: Modifiers },
    /// Mouse button was released
    MouseUp { button: MouseButton, position: Vec2, modifiers: Modifiers },
    /// Mouse moved
    MouseMove { position: Vec2 },
    /// Mouse wheel was scrolled
    MouseWheel { delta: Vec2, position: Vec2 },
}

impl Default for Modifiers {
    fn default() -> Self {
        Self {
            shift: false,
            ctrl: false,
            alt: false,
            meta: false,
        }
    }
}

impl Modifiers {
    /// Create modifiers with no keys pressed
    pub fn empty() -> Self {
        Self::default()
    }
    
    /// Create modifiers with only shift pressed
    pub fn shift() -> Self {
        Self { shift: true, ..Default::default() }
    }
    
    /// Create modifiers with only ctrl pressed
    pub fn ctrl() -> Self {
        Self { ctrl: true, ..Default::default() }
    }
    
    /// Create modifiers with only alt pressed
    pub fn alt() -> Self {
        Self { alt: true, ..Default::default() }
    }
    
    /// Create modifiers with only meta pressed
    pub fn meta() -> Self {
        Self { meta: true, ..Default::default() }
    }
    
    /// Check if any modifier keys are pressed
    pub fn any(&self) -> bool {
        self.shift || self.ctrl || self.alt || self.meta
    }
    
    /// Check if only the specified modifiers are pressed
    pub fn only(&self, other: &Modifiers) -> bool {
        self == other
    }
}