pub mod component;
pub mod entity;
pub mod query;
pub mod resource;
pub mod system;
pub mod world;

pub use component::*;
pub use entity::*;
pub use query::*;
pub use resource::*;
pub use system::*;
pub use world::*;

pub use lumina_core::{define_handle, Id};

define_handle!(Entity);

pub trait Component: Send + Sync + 'static {}

impl<T: Send + Sync + 'static> Component for T {}
