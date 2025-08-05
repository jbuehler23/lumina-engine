use crate::World;

/// Simple ECS system trait
pub trait EcsSystem: Send + Sync {
    fn run(&mut self, world: &mut World) -> Result<(), Box<dyn std::error::Error>>;
}

/// Function-based system wrapper
pub struct FunctionSystem<F>
where
    F: Fn(&mut World) -> Result<(), Box<dyn std::error::Error>> + Send + Sync + 'static,
{
    func: F,
}

impl<F> EcsSystem for FunctionSystem<F>
where
    F: Fn(&mut World) -> Result<(), Box<dyn std::error::Error>> + Send + Sync + 'static,
{
    fn run(&mut self, world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        (self.func)(world)
    }
}

/// Create a system from a function
pub fn make_system<F>(func: F) -> FunctionSystem<F>
where
    F: Fn(&mut World) -> Result<(), Box<dyn std::error::Error>> + Send + Sync + 'static,
{
    FunctionSystem { func }
}