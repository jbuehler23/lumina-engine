use crate::{Component, ComponentManager, Entity, EntityBuilder, EntityManager, ResourceManager};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::Arc;

pub struct World {
    entities: Arc<EntityManager>,
    components: Arc<ComponentManager>,
    resources: Arc<ResourceManager>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Arc::new(EntityManager::new()),
            components: Arc::new(ComponentManager::new()),
            resources: Arc::new(ResourceManager::new()),
        }
    }

    pub fn spawn(&self) -> EntityBuilder {
        let entity = self.entities.create();
        EntityBuilder::new(entity)
    }

    pub fn spawn_with<T: Component>(&self, component: T) -> Entity {
        let entity = self.entities.create();
        self.components.add_component(entity, component);
        entity
    }

    pub fn despawn(&self, entity: Entity) -> bool {
        if self.entities.destroy(entity) {
            self.components.remove_all_components(entity);
            true
        } else {
            false
        }
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.entities.is_alive(entity)
    }

    pub fn add_component<T: Component>(&self, entity: Entity, component: T) {
        if self.entities.is_alive(entity) {
            self.components.add_component(entity, component);
        }
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        if self.entities.is_alive(entity) {
            self.components.get_component(entity)
        } else {
            None
        }
    }

    pub fn get_component_mut<T: Component>(&self, entity: Entity) -> Option<&mut T> {
        if self.entities.is_alive(entity) {
            self.components.get_component_mut(entity)
        } else {
            None
        }
    }

    pub fn remove_component<T: Component>(&self, entity: Entity) -> Option<T> {
        if self.entities.is_alive(entity) {
            self.components.remove_component(entity)
        } else {
            None
        }
    }

    pub fn has_component<T: Component>(&self, entity: Entity) -> bool {
        self.entities.is_alive(entity) && self.components.has_component::<T>(entity)
    }

    pub fn add_resource<T: Send + Sync + 'static>(&self, resource: T) {
        self.resources.add(resource);
    }

    pub fn get_resource<T: Send + Sync + 'static>(&self) -> Option<RwLockReadGuard<T>> {
        self.resources.get()
    }

    pub fn get_resource_mut<T: Send + Sync + 'static>(&self) -> Option<RwLockWriteGuard<T>> {
        self.resources.get_mut()
    }

    pub fn remove_resource<T: Send + Sync + 'static>(&self) -> Option<T> {
        self.resources.remove()
    }

    pub fn has_resource<T: Send + Sync + 'static>(&self) -> bool {
        self.resources.has::<T>()
    }

    pub fn query<T: Component>(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.components.get_storage::<T>()
            .into_iter()
            .flat_map(|storage| storage.iter())
            .filter(|(entity, _)| self.entities.is_alive(*entity))
    }

    pub fn query_mut<T: Component>(&self) -> QueryMut<T> {
        QueryMut {
            world: self,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn entity_count(&self) -> usize {
        self.entities.alive_count()
    }

    pub fn iter_entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entities.iter_alive()
    }

    pub fn clear(&self) {
        self.entities.clear();
        self.components.clear();
        self.resources.clear();
    }

    pub fn entities(&self) -> &EntityManager {
        &self.entities
    }

    pub fn components(&self) -> &ComponentManager {
        &self.components
    }

    pub fn resources(&self) -> &ResourceManager {
        &self.resources
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

pub struct QueryMut<'a, T: Component> {
    world: &'a World,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: Component> QueryMut<'a, T> {
    pub fn iter_mut(&self) -> impl Iterator<Item = (Entity, &mut T)> {
        unsafe {
            let components = &*self.world.components as *const ComponentManager as *mut ComponentManager;
            (*components).get_storage::<T>()
                .unwrap()
                .iter_mut()
                .filter(|(entity, _)| self.world.entities.is_alive(*entity))
        }
    }
}