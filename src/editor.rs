use lumina_editor::EditorRunner;
use anyhow::Result;
use log::{info, debug, warn, error};

fn main() -> Result<()> {
    // Initialize enhanced logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_secs()
        .init();
    
    info!("🎮 Lumina Engine - Visual Editor v0.1.0");
    info!("======================================");
    info!("Starting visual editor built with Lumina UI framework...");
    
    // Log system information
    debug!("Rust version: {}", std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string()));
    debug!("Target: {}", std::env::var("TARGET").unwrap_or_else(|_| "unknown".to_string()));
    debug!("Environment: {}", if cfg!(debug_assertions) { "DEBUG" } else { "RELEASE" });
    
    // Create and run the editor
    debug!("Initializing editor runner...");
    let editor = EditorRunner::new()?;
    
    info!("Editor initialized successfully!");
    info!("Use Ctrl+C or close the window to exit.");
    
    // Run the editor with async runtime
    debug!("Starting async runtime...");
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        info!("Editor runtime started");
        let result = editor.run().await;
        match &result {
            Ok(_) => info!("Editor shut down gracefully"),
            Err(e) => error!("Editor error: {}", e),
        }
        result
    })
}