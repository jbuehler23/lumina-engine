use anyhow::Result;
use lumina_editor::EditorRunner;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    println!("🚀 Starting Lumina Engine Visual Editor");
    println!("Building games with visual scripting and drag-and-drop ease");
    
    let editor = EditorRunner::new()?;
    editor.run().await
}