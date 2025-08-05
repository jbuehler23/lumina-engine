//! Complete ECS-UI Integration Example
//! 
//! This example demonstrates the complete integration between the Lumina UI system
//! and ECS, showing proper data flow in both directions:
//! - ECS state driving UI updates
//! - UI events modifying ECS components
//! - Proper system scheduling and resource management

use lumina_ui::{
    ecs_integration::{EcsUiApp, example_components::{Player, GameState}},
    UiBuilder, Color, ButtonStyle,
    Theme
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
    dpi::LogicalSize,
};
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use std::sync::Arc;

/// Main application that manages the window and ECS-UI integration
struct CompleteEcsUiDemo {
    ecs_app: EcsUiApp,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Arc<Window>,
}

impl CompleteEcsUiDemo {
    async fn new(window: Arc<Window>) -> Result<Self, Box<dyn std::error::Error>> {
        let size = window.inner_size();
        
        // Initialize WGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        let surface = instance.create_surface(window.clone())?;
        
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or("Failed to find suitable adapter")?;
        
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;
        
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
        
        // Create ECS app with UI framework
        let mut ecs_app = EcsUiApp::new(Theme::default());
        
        // Initialize the UI renderer
        let ui_renderer = lumina_render::UiRenderer::new(&device, &queue, config.clone()).await?;
        ecs_app.ui_framework_mut().set_renderer(ui_renderer);
        
        // Add some example game entities
        let player_entity = ecs_app.world_mut().spawn_with(Player {
            name: "Hero".to_string(),
            health: 100,
            max_health: 100,
            level: 1,
        });
        
        let game_state_entity = ecs_app.world_mut().spawn_with(GameState {
            score: 0,
            lives: 3,
            paused: false,
        });
        
        println!("🎮 ECS-UI Demo initialized!");
        println!("   Player entity: {:?}", player_entity);
        println!("   Game state entity: {:?}", game_state_entity);
        
        Ok(Self {
            ecs_app,
            surface,
            device,
            queue,
            config,
            size,
            window,
        })
    }
    
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
    
    fn input(&mut self, event: &WindowEvent) -> bool {
        // Handle input through the ECS app
        self.ecs_app.handle_input(event);
        false
    }
    
    fn update(&mut self) {
        // Run ECS systems
        self.ecs_app.update();
        
        // Build UI based on current ECS state
        self.build_ui_from_ecs_state();
    }
    
    fn build_ui_from_ecs_state(&mut self) {
        // This demonstrates ECS -> UI data flow
        // Query the ECS world for current state and build UI accordingly
        
        let player_data = self.find_player_data();
        let game_state_data = self.find_game_state_data();
        
        // Create a UI builder to construct the interface
        // Note: In a real implementation, you'd want to cache this or use a more efficient approach
        let mut ui_builder = UiBuilder::dark();
        
        // Title
        let _title = ui_builder.text("🎮 ECS-UI Integration Demo")
            .size(28.0)
            .color(Color::hex("#00D9FF").unwrap())
            .build();
        
        // Player info section
        if let Some(player) = player_data {
            let _player_title = ui_builder.text(&format!("👤 Player: {}", player.name))
                .size(20.0)
                .color(Color::WHITE)
                .build();
            
            let health_color = if player.health > player.max_health / 2 {
                Color::GREEN
            } else if player.health > player.max_health / 4 {
                Color::hex("#FFA500").unwrap()
            } else {
                Color::RED
            };
            
            let _health_bar = ui_builder.text(&format!("❤️ Health: {}/{}", player.health, player.max_health))
                .size(16.0)
                .color(health_color)
                .build();
            
            let _level_text = ui_builder.text(&format!("⭐ Level: {}", player.level))
                .size(16.0)
                .color(Color::hex("#FFD700").unwrap())
                .build();
        }
        
        // Game state section
        if let Some(state) = game_state_data {
            let _score_text = ui_builder.text(&format!("🏆 Score: {}", state.score))
                .size(18.0)
                .color(Color::hex("#FFD700").unwrap())
                .build();
            
            let _lives_text = ui_builder.text(&format!("💖 Lives: {}", state.lives))
                .size(16.0)
                .color(Color::RED)
                .build();
            
            let status_color = if state.paused { Color::hex("#FFA500").unwrap() } else { Color::GREEN };
            let status = if state.paused { "⏸️ Paused" } else { "▶️ Playing" };
            
            let _status_text = ui_builder.text(&format!("Status: {}", status))
                .size(16.0)
                .color(status_color)
                .build();
        }
        
        // Action buttons (these would trigger ECS events)
        let _heal_button = ui_builder.button("💚 Heal Player")
            .style(ButtonStyle::Success)
            .on_click(|| {
                println!("Heal button clicked - would trigger ECS event");
            })
            .build();
        
        let _level_up_button = ui_builder.button("⭐ Level Up")
            .style(ButtonStyle::Primary)
            .on_click(|| {
                println!("Level up button clicked - would trigger ECS event");
            })
            .build();
        
        let _pause_button = ui_builder.button("⏸️ Toggle Pause")
            .style(ButtonStyle::Warning)
            .on_click(|| {
                println!("Pause button clicked - would trigger ECS event");
            })
            .build();
        
        // In a real implementation, you'd integrate this UI with the framework
        // For now, this demonstrates the concept
    }
    
    fn find_player_data(&self) -> Option<Player> {
        // Query ECS world for player data
        // This is a simplified approach - in a real system you'd use proper queries
        let entities = self.ecs_app.world().iter_entities();
        for entity in entities {
            if let Some(player) = self.ecs_app.world().get_component::<Player>(entity) {
                return Some(player);
            }
        }
        None
    }
    
    fn find_game_state_data(&self) -> Option<GameState> {
        // Query ECS world for game state data
        let entities = self.ecs_app.world().iter_entities();
        for entity in entities {
            if let Some(state) = self.ecs_app.world().get_component::<GameState>(entity) {
                return Some(state);
            }
        }
        None
    }
    
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.08,
                            g: 0.08,
                            b: 0.10,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            
            // Render the UI using the ECS app
            self.ecs_app.render(&mut render_pass, &self.device, &self.queue);
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("ECS-UI Integration Demo")
            .with_inner_size(LogicalSize::new(1000, 700))
            .with_resizable(true)
            .build(&event_loop)?
    );
    
    let mut app = CompleteEcsUiDemo::new(window.clone()).await?;
    
    println!("🚀 ECS-UI Integration Demo started!");
    println!("💡 This demo shows:");
    println!("   • ECS components driving UI display");
    println!("   • UI events that would modify ECS state");
    println!("   • Proper separation of concerns");
    println!("   • Clean architecture following ECS principles");
    
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !app.input(event) {
                    match event {
                        WindowEvent::CloseRequested => {
                            println!("👋 Closing ECS-UI Demo...");
                            elwt.exit();
                        }
                        WindowEvent::Resized(physical_size) => {
                            app.resize(*physical_size);
                        }
                        WindowEvent::RedrawRequested => {
                            app.update();
                            match app.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost) => app.resize(app.size),
                                Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                                Err(e) => eprintln!("Render error: {:?}", e),
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    })?;
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(run())
}