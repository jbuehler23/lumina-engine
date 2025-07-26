use crate::{engine::{Engine, EngineConfig, System, SystemContext}, Result};

pub trait App {
    fn initialize(&mut self, engine: &mut Engine) -> Result<()>;
    fn update(&mut self, engine: &mut Engine) -> Result<()>;
    fn shutdown(&mut self, engine: &mut Engine) -> Result<()>;
}

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

    fn update(&mut self, engine: &mut Engine) -> Result<()> {
        use crate::input::Key;
        use crate::event::MouseButton;
        
        let should_stop = engine.context().input.is_key_just_pressed(&Key::Escape);
        let mouse_clicked = engine.context().input.is_mouse_button_just_pressed(MouseButton::Left);
        let mouse_pos = engine.context().input.mouse_position();
        
        if should_stop {
            log::info!("Escape pressed, stopping engine");
            engine.stop()?;
        }

        if mouse_clicked {
            log::info!("Mouse clicked at ({}, {})", mouse_pos.x, mouse_pos.y);
        }

        Ok(())
    }

    fn shutdown(&mut self, _engine: &mut Engine) -> Result<()> {
        log::info!("BasicApp shutdown");
        self.initialized = false;
        Ok(())
    }
}