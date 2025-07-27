use lumina_editor::EditorRunner;
use anyhow::Result;

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    println!("🎮 Lumina Engine - Visual Editor v0.1.0");
    println!("======================================");
    println!("Starting visual editor built with Lumina UI framework...");
    
    // Create and run the editor
    let editor = EditorRunner::new()?;
    
    println!("Editor initialized successfully!");
    println!("Use Ctrl+C or close the window to exit.");
    
    // Run the editor with async runtime
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        editor.run().await
    })
}