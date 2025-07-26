use crate::{Component, Entity, World};
use std::marker::PhantomData;

pub trait Query {
    type Item;
    
    fn query(world: &World) -> Self;
    fn get(&self, entity: Entity) -> Option<Self::Item>;
}

pub struct With<T: Component> {
    _phantom: PhantomData<T>,
}

impl<T: Component> Query for With<T> {
    type Item = &'static T;
    
    fn query(_world: &World) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
    
    fn get(&self, entity: Entity) -> Option<Self::Item> {
        None
    }
}

pub struct WithMut<T: Component> {
    _phantom: PhantomData<T>,
}

impl<T: Component> Query for WithMut<T> {
    type Item = &'static mut T;
    
    fn query(_world: &World) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
    
    fn get(&self, entity: Entity) -> Option<Self::Item> {
        None
    }
}

pub struct QueryBuilder<Q> {
    query: Q,
}

impl<Q: Query> QueryBuilder<Q> {
    pub fn new(world: &World) -> Self {
        Self {
            query: Q::query(world),
        }
    }
    
    pub fn get(&self, entity: Entity) -> Option<Q::Item> {
        self.query.get(entity)
    }
}

impl World {
    pub fn query_builder<Q: Query>(&self) -> QueryBuilder<Q> {
        QueryBuilder::new(self)
    }
}