use crate::World;
use lumina_core::{engine::SystemContext, Result};
use std::sync::Arc;

pub trait EcsSystem: Send + Sync {
    fn run(&mut self, world: &World, context: &SystemContext) -> Result<()>;
}

pub struct EcsSystemRunner {
    world: Arc<World>,
    systems: Vec<Box<dyn EcsSystem>>,
}

impl EcsSystemRunner {
    pub fn new() -> Self {
        Self {
            world: Arc::new(World::new()),
            systems: Vec::new(),
        }
    }

    pub fn add_system<S: EcsSystem + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub fn world(&self) -> &Arc<World> {
        &self.world
    }

    pub fn run_systems(&mut self, context: &SystemContext) -> Result<()> {
        for system in &mut self.systems {
            system.run(&self.world, context)?;
        }
        Ok(())
    }
}

impl Default for EcsSystemRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl lumina_core::engine::System for EcsSystemRunner {
    fn initialize(&mut self, _context: &mut SystemContext) -> Result<()> {
        log::info!("ECS system initialized");
        Ok(())
    }

    fn update(&mut self, context: &mut SystemContext) -> Result<()> {
        self.run_systems(context)
    }

    fn shutdown(&mut self, _context: &mut SystemContext) -> Result<()> {
        log::info!("ECS system shutdown");
        Ok(())
    }
}

pub fn make_system<F>(func: F) -> impl EcsSystem
where
    F: Fn(&World, &SystemContext) -> Result<()> + Send + Sync + 'static,
{
    FunctionSystem { func }
}

struct FunctionSystem<F>
where
    F: Fn(&World, &SystemContext) -> Result<()> + Send + Sync + 'static,
{
    func: F,
}

impl<F> EcsSystem for FunctionSystem<F>
where
    F: Fn(&World, &SystemContext) -> Result<()> + Send + Sync + 'static,
{
    fn run(&mut self, world: &World, context: &SystemContext) -> Result<()> {
        (self.func)(world, context)
    }
}