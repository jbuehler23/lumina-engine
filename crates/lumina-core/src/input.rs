use crate::event::{EventBus, KeyboardEvent, MouseButtonEvent, MouseMoveEvent, MouseButton};
use crate::math::Vec2;
use std::collections::HashSet;
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Key {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Escape, Enter, Space, Tab, Backspace, Delete,
    ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
    Shift, Control, Alt, Meta,
    Unknown(String),
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "a" => Key::A, "b" => Key::B, "c" => Key::C, "d" => Key::D,
            "e" => Key::E, "f" => Key::F, "g" => Key::G, "h" => Key::H,
            "i" => Key::I, "j" => Key::J, "k" => Key::K, "l" => Key::L,
            "m" => Key::M, "n" => Key::N, "o" => Key::O, "p" => Key::P,
            "q" => Key::Q, "r" => Key::R, "s" => Key::S, "t" => Key::T,
            "u" => Key::U, "v" => Key::V, "w" => Key::W, "x" => Key::X,
            "y" => Key::Y, "z" => Key::Z,
            "0" => Key::Key0, "1" => Key::Key1, "2" => Key::Key2, "3" => Key::Key3,
            "4" => Key::Key4, "5" => Key::Key5, "6" => Key::Key6, "7" => Key::Key7,
            "8" => Key::Key8, "9" => Key::Key9,
            "f1" => Key::F1, "f2" => Key::F2, "f3" => Key::F3, "f4" => Key::F4,
            "f5" => Key::F5, "f6" => Key::F6, "f7" => Key::F7, "f8" => Key::F8,
            "f9" => Key::F9, "f10" => Key::F10, "f11" => Key::F11, "f12" => Key::F12,
            "escape" => Key::Escape, "enter" => Key::Enter, "space" => Key::Space,
            "tab" => Key::Tab, "backspace" => Key::Backspace, "delete" => Key::Delete,
            "arrowup" => Key::ArrowUp, "arrowdown" => Key::ArrowDown,
            "arrowleft" => Key::ArrowLeft, "arrowright" => Key::ArrowRight,
            "shift" => Key::Shift, "control" => Key::Control, "alt" => Key::Alt,
            "meta" => Key::Meta,
            _ => Key::Unknown(s.to_string()),
        }
    }
}

pub struct Input {
    pressed_keys: Arc<RwLock<HashSet<Key>>>,
    just_pressed_keys: Arc<RwLock<HashSet<Key>>>,
    just_released_keys: Arc<RwLock<HashSet<Key>>>,
    pressed_mouse_buttons: Arc<RwLock<HashSet<MouseButton>>>,
    just_pressed_mouse_buttons: Arc<RwLock<HashSet<MouseButton>>>,
    just_released_mouse_buttons: Arc<RwLock<HashSet<MouseButton>>>,
    mouse_position: Arc<RwLock<Vec2>>,
    mouse_delta: Arc<RwLock<Vec2>>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            pressed_keys: Arc::new(RwLock::new(HashSet::new())),
            just_pressed_keys: Arc::new(RwLock::new(HashSet::new())),
            just_released_keys: Arc::new(RwLock::new(HashSet::new())),
            pressed_mouse_buttons: Arc::new(RwLock::new(HashSet::new())),
            just_pressed_mouse_buttons: Arc::new(RwLock::new(HashSet::new())),
            just_released_mouse_buttons: Arc::new(RwLock::new(HashSet::new())),
            mouse_position: Arc::new(RwLock::new(Vec2::ZERO)),
            mouse_delta: Arc::new(RwLock::new(Vec2::ZERO)),
        }
    }
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setup_event_handlers(&self, event_bus: &EventBus) {
        let pressed_keys = self.pressed_keys.clone();
        let just_pressed_keys = self.just_pressed_keys.clone();
        let just_released_keys = self.just_released_keys.clone();
        
        event_bus.subscribe(move |event: &KeyboardEvent| {
            let key = Key::from(event.key.as_str());
            
            if event.pressed {
                let mut pressed = pressed_keys.write();
                let mut just_pressed = just_pressed_keys.write();
                
                if !pressed.contains(&key) {
                    just_pressed.insert(key.clone());
                }
                pressed.insert(key);
            } else {
                let mut pressed = pressed_keys.write();
                let mut just_released = just_released_keys.write();
                
                pressed.remove(&key);
                just_released.insert(key);
            }
        });

        let pressed_mouse_buttons = self.pressed_mouse_buttons.clone();
        let just_pressed_mouse_buttons = self.just_pressed_mouse_buttons.clone();
        let just_released_mouse_buttons = self.just_released_mouse_buttons.clone();
        let mouse_position = self.mouse_position.clone();
        
        event_bus.subscribe(move |event: &MouseButtonEvent| {
            if event.pressed {
                let mut pressed = pressed_mouse_buttons.write();
                let mut just_pressed = just_pressed_mouse_buttons.write();
                
                if !pressed.contains(&event.button) {
                    just_pressed.insert(event.button);
                }
                pressed.insert(event.button);
            } else {
                let mut pressed = pressed_mouse_buttons.write();
                let mut just_released = just_released_mouse_buttons.write();
                
                pressed.remove(&event.button);
                just_released.insert(event.button);
            }
            
            *mouse_position.write() = Vec2::new(event.x, event.y);
        });

        let mouse_position_move = self.mouse_position.clone();
        let mouse_delta = self.mouse_delta.clone();
        
        event_bus.subscribe(move |event: &MouseMoveEvent| {
            *mouse_position_move.write() = Vec2::new(event.x, event.y);
            *mouse_delta.write() = Vec2::new(event.delta_x, event.delta_y);
        });
    }

    pub fn update(&self) {
        self.just_pressed_keys.write().clear();
        self.just_released_keys.write().clear();
        self.just_pressed_mouse_buttons.write().clear();
        self.just_released_mouse_buttons.write().clear();
        *self.mouse_delta.write() = Vec2::ZERO;
    }

    pub fn is_key_pressed(&self, key: &Key) -> bool {
        self.pressed_keys.read().contains(key)
    }

    pub fn is_key_just_pressed(&self, key: &Key) -> bool {
        self.just_pressed_keys.read().contains(key)
    }

    pub fn is_key_just_released(&self, key: &Key) -> bool {
        self.just_released_keys.read().contains(key)
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.read().contains(&button)
    }

    pub fn is_mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        self.just_pressed_mouse_buttons.read().contains(&button)
    }

    pub fn is_mouse_button_just_released(&self, button: MouseButton) -> bool {
        self.just_released_mouse_buttons.read().contains(&button)
    }

    pub fn mouse_position(&self) -> Vec2 {
        *self.mouse_position.read()
    }

    pub fn mouse_delta(&self) -> Vec2 {
        *self.mouse_delta.read()
    }
}