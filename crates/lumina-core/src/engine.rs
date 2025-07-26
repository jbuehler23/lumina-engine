use crate::{event::EventBus, input::Input, time::Time, Result, LuminaError};
use parking_lot::RwLock;
use std::sync::Arc;

pub trait System: Send + Sync {
    fn initialize(&mut self, context: &mut SystemContext) -> Result<()>;
    fn update(&mut self, context: &mut SystemContext) -> Result<()>;
    fn shutdown(&mut self, context: &mut SystemContext) -> Result<()>;
}

pub struct SystemContext {
    pub event_bus: Arc<EventBus>,
    pub input: Arc<Input>,
    pub time: Arc<RwLock<Time>>,
}

impl SystemContext {
    pub fn new() -> Self {
        let event_bus = Arc::new(EventBus::new());
        let input = Arc::new(Input::new());
        
        input.setup_event_handlers(&event_bus);
        
        Self {
            event_bus,
            input,
            time: Arc::new(RwLock::new(Time::new())),
        }
    }
}

pub struct Engine {
    context: SystemContext,
    systems: Vec<Box<dyn System>>,
    running: bool,
}

impl Engine {
    pub fn new() -> Self {
        crate::init_logging();
        log::info!("Initializing Lumina Engine");
        
        Self {
            context: SystemContext::new(),
            systems: Vec::new(),
            running: false,
        }
    }

    pub fn add_system<S: System + 'static>(&mut self, mut system: S) -> Result<()> {
        system.initialize(&mut self.context)?;
        self.systems.push(Box::new(system));
        Ok(())
    }

    pub fn context(&self) -> &SystemContext {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut SystemContext {
        &mut self.context
    }

    pub fn start(&mut self) -> Result<()> {
        if self.running {
            return Err(LuminaError::RuntimeError("Engine is already running".to_string()).into());
        }

        log::info!("Starting Lumina Engine");
        self.running = true;
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        if !self.running {
            return Ok(());
        }

        self.context.time.write().update();
        
        for system in &mut self.systems {
            system.update(&mut self.context)?;
        }

        self.context.input.update();
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        if !self.running {
            return Ok(());
        }

        log::info!("Stopping Lumina Engine");
        self.running = false;

        for system in &mut self.systems {
            system.shutdown(&mut self.context)?;
        }

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        if self.running {
            let _ = self.stop();
        }
        log::info!("Lumina Engine destroyed");
    }
}

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub vsync: bool,
    pub max_fps: Option<u32>,
    pub enable_audio: bool,
    pub enable_physics: bool,
    pub enable_scripting: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window_title: "Lumina Engine Game".to_string(),
            window_width: 1280,
            window_height: 720,
            vsync: true,
            max_fps: None,
            enable_audio: true,
            enable_physics: true,
            enable_scripting: true,
        }
    }
}