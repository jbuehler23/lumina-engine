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
    components: HashMap<Entity, T>,
}

impl<T: Component> TypedComponentStorage<T> {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, component: T) {
        self.components.insert(entity, component);
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.components.get(&entity)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.components.get_mut(&entity)
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        self.components.remove(&entity)
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.components.contains_key(&entity)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.components.iter().map(|(&entity, component)| (entity, component))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.components.iter_mut().map(|(&entity, component)| (entity, component))
    }

    pub fn len(&self) -> usize {
        self.components.len()
    }

    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }
}

impl<T: Component> Default for TypedComponentStorage<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Component> ComponentStorage for TypedComponentStorage<T> {
    fn remove(&mut self, entity: Entity) -> bool {
        self.components.remove(&entity).is_some()
    }

    fn clear(&mut self) {
        self.components.clear();
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
        let mut storages = self.storages.write();
        
        if let Some(storage) = storages.get_mut(&type_id) {
            if let Some(typed_storage) = storage.as_any_mut().downcast_mut::<TypedComponentStorage<T>>() {
                typed_storage.insert(entity, component);
            }
        }
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        let storages = self.storages.read();
        
        storages.get(&type_id)
            .and_then(|storage| storage.as_any().downcast_ref::<TypedComponentStorage<T>>())
            .and_then(|typed_storage| typed_storage.get(entity))
    }

    pub fn get_component_mut<T: Component>(&self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        let mut storages = self.storages.write();
        
        storages.get_mut(&type_id)
            .and_then(|storage| storage.as_any_mut().downcast_mut::<TypedComponentStorage<T>>())
            .and_then(|typed_storage| typed_storage.get_mut(entity))
    }

    pub fn remove_component<T: Component>(&self, entity: Entity) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let mut storages = self.storages.write();
        
        storages.get_mut(&type_id)
            .and_then(|storage| storage.as_any_mut().downcast_mut::<TypedComponentStorage<T>>())
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

    pub fn get_storage<T: Component>(&self) -> Option<&TypedComponentStorage<T>> {
        let type_id = TypeId::of::<T>();
        let storages = self.storages.read();
        
        storages.get(&type_id)
            .and_then(|storage| storage.as_any().downcast_ref::<TypedComponentStorage<T>>())
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