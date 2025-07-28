//! Input handling for the Lumina UI framework

#![allow(missing_docs)]

use glam::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Data that can be dragged and dropped
#[derive(Debug, Clone, PartialEq)]
pub enum DragData {
    /// Text data
    Text(String),
    /// Widget ID being dragged
    Widget(crate::WidgetId),
    /// Visual scripting node type
    NodeType(String),
    /// Custom data with type identifier
    Custom { data_type: String, data: Vec<u8> },
}

/// Current drag operation state
#[derive(Debug, Clone)]
pub struct DragState {
    /// Position where drag started
    pub start_position: Vec2,
    /// Current drag position
    pub current_position: Vec2,
    /// Data being dragged
    pub data: DragData,
    /// Mouse button used for dragging
    pub button: MouseButton,
    /// Whether drag threshold has been exceeded
    pub is_active: bool,
}

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
    /// Current drag state
    drag_state: Option<DragState>,
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
    /// Drag start event
    DragStart { position: Vec2, button: MouseButton },
    /// Drag update event
    DragUpdate { position: Vec2, delta: Vec2 },
    /// Drag end event
    DragEnd { position: Vec2 },
    /// Drop event
    Drop { position: Vec2, data: DragData },
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
    
    /// Start a drag operation
    pub fn start_drag(&mut self, data: DragData, button: MouseButton) -> bool {
        if self.drag_state.is_none() {
            self.drag_state = Some(DragState {
                start_position: self.mouse_position,
                current_position: self.mouse_position,
                data,
                button,
                is_active: false,
            });
            true
        } else {
            false
        }
    }
    
    /// Update drag state (called during mouse movement)
    pub fn update_drag(&mut self) -> Option<InputEvent> {
        if let Some(ref mut drag_state) = self.drag_state {
            let previous_position = drag_state.current_position;
            drag_state.current_position = self.mouse_position;
            
            // Check if drag threshold has been exceeded
            const DRAG_THRESHOLD: f32 = 3.0; // pixels
            if !drag_state.is_active {
                let distance = (drag_state.current_position - drag_state.start_position).length();
                if distance > DRAG_THRESHOLD {
                    drag_state.is_active = true;
                    return Some(InputEvent::DragStart {
                        position: drag_state.current_position,
                        button: drag_state.button,
                    });
                }
            } else {
                let delta = drag_state.current_position - previous_position;
                return Some(InputEvent::DragUpdate {
                    position: drag_state.current_position,
                    delta,
                });
            }
        }
        None
    }
    
    /// End drag operation
    pub fn end_drag(&mut self) -> Option<InputEvent> {
        if let Some(drag_state) = self.drag_state.take() {
            if drag_state.is_active {
                Some(InputEvent::DragEnd {
                    position: drag_state.current_position,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Get current drag state
    pub fn drag_state(&self) -> Option<&DragState> {
        self.drag_state.as_ref()
    }
    
    /// Check if currently dragging
    pub fn is_dragging(&self) -> bool {
        self.drag_state.as_ref().map_or(false, |state| state.is_active)
    }
    
    /// Create a drop event at the current position
    pub fn create_drop_event(&self, data: DragData) -> InputEvent {
        InputEvent::Drop {
            position: self.mouse_position,
            data,
        }
    }
    
    /// Handle potential drag operations during mouse events
    pub fn handle_drag_events(&mut self, raw_input: &RawInputEvent) -> Vec<InputEvent> {
        let mut events = Vec::new();
        
        match raw_input {
            RawInputEvent::MouseMove { .. } => {
                if let Some(drag_event) = self.update_drag() {
                    events.push(drag_event);
                }
            }
            RawInputEvent::MouseUp { button, .. } => {
                // Take drag_state out to avoid double borrow
                let drag_state = if let Some(drag_state) = self.drag_state.as_ref() {
                    if drag_state.button == *button {
                        Some(drag_state.data.clone())
                    } else {
                        None
                    }
                } else {
                    None
                };

                if drag_state.is_some() {
                    if let Some(drag_end_event) = self.end_drag() {
                        events.push(drag_end_event);

                        // Also create a drop event with the dragged data
                        events.push(InputEvent::Drop {
                            position: self.mouse_position,
                            data: drag_state.unwrap(),
                        });
                    }
                }
            }
            _ => {}
        }
        
        events
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