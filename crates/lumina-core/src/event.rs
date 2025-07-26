use std::any::{Any, TypeId};
use std::collections::HashMap;
use parking_lot::RwLock;
use smallvec::SmallVec;

pub trait Event: Any + Send + Sync + 'static {}

pub type EventHandler<T> = Box<dyn Fn(&T) + Send + Sync>;
pub type EventHandlerDyn = Box<dyn Fn(&dyn Any) + Send + Sync>;

pub struct EventBus {
    handlers: RwLock<HashMap<TypeId, SmallVec<[EventHandlerDyn; 4]>>>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
        }
    }
}

impl EventBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe<T: Event>(&self, handler: impl Fn(&T) + Send + Sync + 'static) {
        let type_id = TypeId::of::<T>();
        let handler: EventHandlerDyn = Box::new(move |event| {
            if let Some(event) = event.downcast_ref::<T>() {
                handler(event);
            }
        });

        let mut handlers = self.handlers.write();
        handlers.entry(type_id).or_default().push(handler);
    }

    pub fn publish<T: Event>(&self, event: T) {
        let type_id = TypeId::of::<T>();
        let handlers = self.handlers.read();
        
        if let Some(handlers) = handlers.get(&type_id) {
            for handler in handlers.iter() {
                handler(&event as &dyn Any);
            }
        }
    }

    pub fn clear<T: Event>(&self) {
        let type_id = TypeId::of::<T>();
        let mut handlers = self.handlers.write();
        handlers.remove(&type_id);
    }

    pub fn clear_all(&self) {
        let mut handlers = self.handlers.write();
        handlers.clear();
    }
}

#[derive(Debug, Clone)]
pub struct WindowResizeEvent {
    pub width: u32,
    pub height: u32,
}

impl Event for WindowResizeEvent {}

#[derive(Debug, Clone)]
pub struct WindowCloseEvent;

impl Event for WindowCloseEvent {}

#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    pub key: String,
    pub pressed: bool,
    pub repeat: bool,
}

impl Event for KeyboardEvent {}

#[derive(Debug, Clone)]
pub struct MouseButtonEvent {
    pub button: MouseButton,
    pub pressed: bool,
    pub x: f32,
    pub y: f32,
}

impl Event for MouseButtonEvent {}

#[derive(Debug, Clone)]
pub struct MouseMoveEvent {
    pub x: f32,
    pub y: f32,
    pub delta_x: f32,
    pub delta_y: f32,
}

impl Event for MouseMoveEvent {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}