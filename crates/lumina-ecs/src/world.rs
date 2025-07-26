use crate::{Component, ComponentManager, Entity, EntityBuilder, EntityManager, ResourceManager};
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

    pub fn get_component<T: Component + Clone>(&self, entity: Entity) -> Option<T> {
        if self.entities.is_alive(entity) {
            self.components.get_component(entity)
        } else {
            None
        }
    }

    pub fn with_component<T: Component, R>(&self, entity: Entity, f: impl FnOnce(Option<&T>) -> R) -> R {
        if self.entities.is_alive(entity) {
            self.components.with_component(entity, f)
        } else {
            f(None)
        }
    }

    pub fn with_component_mut<T: Component, R>(&self, entity: Entity, f: impl FnOnce(Option<&mut T>) -> R) -> R {
        if self.entities.is_alive(entity) {
            self.components.with_component_mut(entity, f)
        } else {
            f(None)
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

    pub fn with_resource<T: Send + Sync + 'static, R>(&self, f: impl FnOnce(Option<&T>) -> R) -> R {
        self.resources.with_resource(f)
    }

    pub fn with_resource_mut<T: Send + Sync + 'static, R>(&self, f: impl FnOnce(Option<&mut T>) -> R) -> R {
        self.resources.with_resource_mut(f)
    }

    pub fn remove_resource<T: Send + Sync + 'static>(&self) -> Option<T> {
        self.resources.remove()
    }

    pub fn has_resource<T: Send + Sync + 'static>(&self) -> bool {
        self.resources.has::<T>()
    }

    pub fn query<T: Component + Clone>(&self) -> Vec<(Entity, T)> {
        let mut results = Vec::new();
        self.components.with_storage::<T, _>(|storage_opt| {
            if let Some(storage) = storage_opt {
                storage.with_iter(|iter| {
                    for (&entity, component) in iter {
                        if self.entities.is_alive(entity) {
                            results.push((entity, component.clone()));
                        }
                    }
                });
            }
        });
        results
    }

    pub fn entity_count(&self) -> usize {
        self.entities.alive_count()
    }

    pub fn iter_entities(&self) -> Vec<Entity> {
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

