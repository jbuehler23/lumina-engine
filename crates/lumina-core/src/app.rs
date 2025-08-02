use crate::{engine::{Engine, EngineConfig, System}, Result};

/// DEPRECATED: Use EcsAppRunner for new projects
#[deprecated(note = "Use EcsAppRunner for new projects - see examples/pong")]
pub trait App {
    fn initialize(&mut self, engine: &mut Engine) -> Result<()>;
    fn update(&mut self, engine: &mut Engine) -> Result<()>;
    fn shutdown(&mut self, engine: &mut Engine) -> Result<()>;
}

/// DEPRECATED: Use EcsAppRunner for new projects
#[deprecated(note = "Use EcsAppRunner for new projects - see examples/pong")]
pub struct AppRunner {
    app: Box<dyn App>,
    engine: Engine,
    config: EngineConfig,
}

impl AppRunner {
    pub fn new<A: App + 'static>(app: A) -> Self {
        Self {
            app: Box::new(app),
            engine: Engine::new(),
            config: EngineConfig::default(),
        }
    }

    pub fn with_config<A: App + 'static>(app: A, config: EngineConfig) -> Self {
        Self {
            app: Box::new(app),
            engine: Engine::new(),
            config,
        }
    }

    pub fn add_system<S: System + 'static>(&mut self, system: S) -> Result<()> {
        self.engine.add_system(system)
    }

    pub fn run(mut self) -> Result<()> {
        log::info!("Initializing application");
        self.app.initialize(&mut self.engine)?;

        log::info!("Starting application loop");
        self.engine.start()?;

        while self.engine.is_running() {
            self.engine.update()?;
            self.app.update(&mut self.engine)?;
        }

        log::info!("Shutting down application");
        self.app.shutdown(&mut self.engine)?;
        self.engine.stop()?;

        Ok(())
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn engine_mut(&mut self) -> &mut Engine {
        &mut self.engine
    }

    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut EngineConfig {
        &mut self.config
    }
}

pub struct BasicApp {
    initialized: bool,
}

impl BasicApp {
    pub fn new() -> Self {
        Self {
            initialized: false,
        }
    }
}

impl Default for BasicApp {
    fn default() -> Self {
        Self::new()
    }
}

impl App for BasicApp {
    fn initialize(&mut self, _engine: &mut Engine) -> Result<()> {
        log::info!("BasicApp initialized");
        self.initialized = true;
        Ok(())
    }

    fn update(&mut self, _engine: &mut Engine) -> Result<()> {
        // DEPRECATED: Use lumina-input with EcsAppRunner instead
        // This is a stub to make compilation work
        Ok(())
    }

    fn shutdown(&mut self, _engine: &mut Engine) -> Result<()> {
        log::info!("BasicApp shutdown");
        self.initialized = false;
        Ok(())
    }
}