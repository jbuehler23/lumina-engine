//! Basic example demonstrating the dockable panel system

use lumina_editor::layout::{DockingManager, types::BuiltinPanelId};
use lumina_editor::dockable_scene_panel::DockableScenePanel;

fn main() {
    println!("🎯 Basic Docking System Example");
    println!("================================");
    
    // Create a docking manager with default layout
    let mut docking_manager = DockingManager::with_default_layout();
    
    // Create and register a scene panel
    let scene_panel = Box::new(DockableScenePanel::new());
    docking_manager.add_panel(scene_panel);
    
    // Display information about the docking system
    println!("✅ DockingManager created successfully");
    println!("📋 Registered panels: {:?}", docking_manager.get_all_panels());
    println!("🎬 Active panel: {:?}", docking_manager.get_active_panel());
    
    // Test layout serialization
    match docking_manager.save_layout() {
        Ok(layout_json) => {
            println!("💾 Layout serialization successful");
            println!("📝 Layout JSON: {}", layout_json);
            
            // Test loading the layout back
            let mut new_docking_manager = DockingManager::new();
            match new_docking_manager.load_layout(&layout_json) {
                Ok(()) => println!("📂 Layout deserialization successful"),
                Err(e) => println!("❌ Layout deserialization failed: {}", e),
            }
        }
        Err(e) => println!("❌ Layout serialization failed: {}", e),
    }
    
    // Test built-in panel IDs
    println!("🏷️ Built-in Panel IDs:");
    println!("  - Scene Editor: {:?}", BuiltinPanelId::SceneEditor.panel_id());
    println!("  - Properties: {:?}", BuiltinPanelId::PropertyInspector.panel_id());
    println!("  - Assets: {:?}", BuiltinPanelId::AssetBrowser.panel_id());
    
    println!("\n🚀 Dockable panel system is ready for integration!");
    println!("   Next steps: Run the full editor to see the UI in action");
}