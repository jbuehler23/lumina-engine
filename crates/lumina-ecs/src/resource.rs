use parking_lot::RwLock;
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct ResourceManager {
    resources: RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            resources: RwLock::new(HashMap::new()),
        }
    }

    pub fn add<T: Send + Sync + 'static>(&self, resource: T) {
        let type_id = TypeId::of::<T>();
        let mut resources = self.resources.write();
        resources.insert(type_id, Box::new(RwLock::new(resource)));
    }

    pub fn with_resource<T: Send + Sync + 'static, R>(&self, f: impl FnOnce(Option<&T>) -> R) -> R {
        let type_id = TypeId::of::<T>();
        let resources = self.resources.read();
        
        match resources.get(&type_id)
            .and_then(|resource| resource.downcast_ref::<RwLock<T>>()) {
            Some(lock) => {
                let guard = lock.read();
                f(Some(&*guard))
            }
            None => f(None),
        }
    }

    pub fn with_resource_mut<T: Send + Sync + 'static, R>(&self, f: impl FnOnce(Option<&mut T>) -> R) -> R {
        let type_id = TypeId::of::<T>();
        let resources = self.resources.read();
        
        match resources.get(&type_id)
            .and_then(|resource| resource.downcast_ref::<RwLock<T>>()) {
            Some(lock) => {
                let mut guard = lock.write();
                f(Some(&mut *guard))
            }
            None => f(None),
        }
    }

    pub fn remove<T: Send + Sync + 'static>(&self) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let mut resources = self.resources.write();
        
        resources.remove(&type_id)
            .and_then(|resource| resource.downcast::<RwLock<T>>().ok())
            .map(|lock| lock.into_inner())
    }

    pub fn has<T: Send + Sync + 'static>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        let resources = self.resources.read();
        resources.contains_key(&type_id)
    }

    pub fn clear(&self) {
        let mut resources = self.resources.write();
        resources.clear();
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}