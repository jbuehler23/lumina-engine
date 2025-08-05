use crate::{Entity, BitSet};
use parking_lot::RwLock;
use smallvec::SmallVec;

pub struct EntityManager {
    next_id: RwLock<u32>,
    free_entities: RwLock<SmallVec<[Entity; 16]>>,
    alive_entities: RwLock<BitSet>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            next_id: RwLock::new(0),
            free_entities: RwLock::new(SmallVec::new()),
            alive_entities: RwLock::new(BitSet::new()),
        }
    }

    pub fn create(&self) -> Entity {
        let mut free_entities = self.free_entities.write();
        
        if let Some(entity) = free_entities.pop() {
            let mut alive = self.alive_entities.write();
            alive.set(entity.index() as usize);
            entity
        } else {
            let mut next_id = self.next_id.write();
            let entity = Entity::new(*next_id);
            *next_id += 1;
            
            let mut alive = self.alive_entities.write();
            alive.set(entity.index() as usize);
            
            entity
        }
    }

    pub fn destroy(&self, entity: Entity) -> bool {
        let mut alive = self.alive_entities.write();
        
        if alive.get(entity.index() as usize) {
            alive.clear(entity.index() as usize);
            
            let mut free_entities = self.free_entities.write();
            free_entities.push(entity);
            
            true
        } else {
            false
        }
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        let alive = self.alive_entities.read();
        alive.get(entity.index() as usize)
    }

    pub fn alive_count(&self) -> usize {
        let alive = self.alive_entities.read();
        alive.iter_set_bits().count()
    }

    pub fn iter_alive(&self) -> Vec<Entity> {
        let alive = self.alive_entities.read();
        alive.iter_set_bits().map(|index| Entity::new(index as u32)).collect()
    }

    pub fn clear(&self) {
        let mut next_id = self.next_id.write();
        let mut free_entities = self.free_entities.write();
        let mut alive = self.alive_entities.write();
        
        *next_id = 0;
        free_entities.clear();
        alive.clear_all();
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct EntityBuilder {
    entity: Entity,
    components: Vec<Box<dyn FnOnce(&crate::World) + Send>>,
}

impl EntityBuilder {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            components: Vec::new(),
        }
    }

    pub fn with<T: crate::Component>(mut self, component: T) -> Self {
        let entity = self.entity; // Capture entity before move
        self.components.push(Box::new(move |world| {
            world.add_component(entity, component);
        }));
        self
    }

    pub fn build(self, world: &crate::World) -> Entity {
        for add_component in self.components {
            add_component(world);
        }
        self.entity
    }
}