use crate::{Component, Entity};
use parking_lot::RwLock;
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub trait ComponentStorage: Send + Sync {
    fn remove(&mut self, entity: Entity) -> bool;
    fn clear(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct TypedComponentStorage<T: Component> {
    components: RwLock<HashMap<Entity, T>>,
}

impl<T: Component> TypedComponentStorage<T> {
    pub fn new() -> Self {
        Self {
            components: RwLock::new(HashMap::new()),
        }
    }

    pub fn insert(&self, entity: Entity, component: T) {
        self.components.write().insert(entity, component);
    }

    pub fn get<R>(&self, entity: Entity, f: impl FnOnce(Option<&T>) -> R) -> R {
        let components = self.components.read();
        f(components.get(&entity))
    }

    pub fn get_mut<R>(&self, entity: Entity, f: impl FnOnce(Option<&mut T>) -> R) -> R {
        let mut components = self.components.write();
        f(components.get_mut(&entity))
    }

    pub fn remove(&self, entity: Entity) -> Option<T> {
        self.components.write().remove(&entity)
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.components.read().contains_key(&entity)
    }

    pub fn with_iter<R>(&self, f: impl FnOnce(std::collections::hash_map::Iter<Entity, T>) -> R) -> R {
        let components = self.components.read();
        f(components.iter())
    }

    pub fn len(&self) -> usize {
        self.components.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.components.read().is_empty()
    }
}

impl<T: Component> Default for TypedComponentStorage<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Component> ComponentStorage for TypedComponentStorage<T> {
    fn remove(&mut self, entity: Entity) -> bool {
        self.components.write().remove(&entity).is_some()
    }

    fn clear(&mut self) {
        self.components.write().clear();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct ComponentManager {
    storages: RwLock<HashMap<TypeId, Box<dyn ComponentStorage>>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            storages: RwLock::new(HashMap::new()),
        }
    }

    pub fn register<T: Component>(&self) {
        let type_id = TypeId::of::<T>();
        let mut storages = self.storages.write();
        
        if !storages.contains_key(&type_id) {
            storages.insert(type_id, Box::new(TypedComponentStorage::<T>::new()));
        }
    }

    pub fn add_component<T: Component>(&self, entity: Entity, component: T) {
        self.register::<T>();
        let type_id = TypeId::of::<T>();
        let storages = self.storages.read();
        
        if let Some(storage) = storages.get(&type_id) {
            if let Some(typed_storage) = storage.as_any().downcast_ref::<TypedComponentStorage<T>>() {
                typed_storage.insert(entity, component);
            }
        }
    }

    pub fn get_component<T: Component + Clone>(&self, entity: Entity) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let storages = self.storages.read();
        
        storages.get(&type_id)
            .and_then(|storage| storage.as_any().downcast_ref::<TypedComponentStorage<T>>())
            .map(|typed_storage| typed_storage.get(entity, |opt| opt.cloned()))
            .flatten()
    }

    pub fn with_component<T: Component, R>(&self, entity: Entity, f: impl FnOnce(Option<&T>) -> R) -> R {
        let type_id = TypeId::of::<T>();
        let storages = self.storages.read();
        
        match storages.get(&type_id)
            .and_then(|storage| storage.as_any().downcast_ref::<TypedComponentStorage<T>>()) {
            Some(typed_storage) => typed_storage.get(entity, f),
            None => f(None),
        }
    }

    pub fn with_component_mut<T: Component, R>(&self, entity: Entity, f: impl FnOnce(Option<&mut T>) -> R) -> R {
        let type_id = TypeId::of::<T>();
        let storages = self.storages.read();
        
        match storages.get(&type_id)
            .and_then(|storage| storage.as_any().downcast_ref::<TypedComponentStorage<T>>()) {
            Some(typed_storage) => typed_storage.get_mut(entity, f),
            None => f(None),
        }
    }

    pub fn remove_component<T: Component>(&self, entity: Entity) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let storages = self.storages.read();
        
        storages.get(&type_id)
            .and_then(|storage| storage.as_any().downcast_ref::<TypedComponentStorage<T>>())
            .and_then(|typed_storage| typed_storage.remove(entity))
    }

    pub fn has_component<T: Component>(&self, entity: Entity) -> bool {
        let type_id = TypeId::of::<T>();
        let storages = self.storages.read();
        
        storages.get(&type_id)
            .and_then(|storage| storage.as_any().downcast_ref::<TypedComponentStorage<T>>())
            .map(|typed_storage| typed_storage.contains(entity))
            .unwrap_or(false)
    }

    pub fn remove_all_components(&self, entity: Entity) {
        let mut storages = self.storages.write();
        for storage in storages.values_mut() {
            storage.remove(entity);
        }
    }

    pub fn with_storage<T: Component, R>(&self, f: impl FnOnce(Option<&TypedComponentStorage<T>>) -> R) -> R {
        let type_id = TypeId::of::<T>();
        let storages = self.storages.read();
        
        let storage = storages.get(&type_id)
            .and_then(|storage| storage.as_any().downcast_ref::<TypedComponentStorage<T>>());
        f(storage)
    }

    pub fn clear(&self) {
        let mut storages = self.storages.write();
        for storage in storages.values_mut() {
            storage.clear();
        }
    }
}

impl Default for ComponentManager {
    fn default() -> Self {
        Self::new()
    }
}