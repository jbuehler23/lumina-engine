//! Demonstration of the editor toolbar functionality

use lumina_editor::toolbar::{EditorToolbar, ToolType, ToolbarAction};
use lumina_editor::layout::types::Rect;

fn main() {
    println!("🔧 Editor Toolbar Demo");
    println!("======================");
    
    // Create a new toolbar
    let mut toolbar = EditorToolbar::new();
    
    // Display initial state
    println!("✅ Toolbar created successfully");
    println!("🎯 Default tool: {:?}", toolbar.selected_tool());
    
    // Demonstrate tool selection
    println!("\n🛠️ Tool Selection Demo:");
    let tools = [
        ToolType::Select,
        ToolType::Move,
        ToolType::Rotate,
        ToolType::Scale,
        ToolType::Brush,
        ToolType::Eraser,
    ];
    
    for tool in &tools {
        toolbar.set_selected_tool(*tool);
        println!("  {} {} - {} ({})", 
                tool.icon(), 
                tool.display_name(),
                tool.shortcut(),
                if toolbar.selected_tool() == *tool { "SELECTED" } else { "" });
    }
    
    // Demonstrate keyboard shortcuts
    println!("\n⌨️ Keyboard Shortcuts Demo:");
    let shortcuts = [
        ("v", "Select Tool"),
        ("g", "Move Tool"), 
        ("r", "Rotate Tool"),
        ("s", "Scale Tool"),
        ("b", "Brush Tool"),
        ("e", "Eraser Tool"),
        ("ctrl+n", "New Project"),
        ("ctrl+s", "Save Project"),
        ("ctrl+z", "Undo"),
        ("space", "Play/Pause"),
    ];
    
    for (shortcut, description) in &shortcuts {
        let action = toolbar.handle_keyboard_shortcut(shortcut);
        match action {
            ToolbarAction::ToolSelected(tool) => {
                println!("  {} -> Tool changed to: {:?}", shortcut, tool);
            }
            ToolbarAction::NewProject => println!("  {} -> {}", shortcut, description),
            ToolbarAction::SaveProject => println!("  {} -> {}", shortcut, description),
            ToolbarAction::Undo => println!("  {} -> {}", shortcut, description),
            ToolbarAction::Play => println!("  {} -> {}", shortcut, description),
            ToolbarAction::None => println!("  {} -> No action", shortcut),
            _ => println!("  {} -> {:?}", shortcut, action),
        }
    }
    
    // Demonstrate toolbar bounds and layout
    println!("\n📐 Layout Demo:");
    let bounds = Rect::new(0.0, 0.0, 1200.0, 40.0);
    toolbar.set_bounds(bounds);
    println!("  Toolbar bounds: {:?}", toolbar.bounds());
    
    // Tool properties demonstration
    println!("\n🏷️ Tool Properties:");
    for tool in &tools {
        println!("  {}: {} - Key: {} - Name: {}", 
                tool.icon(),
                tool.display_name(), 
                tool.shortcut(),
                tool.display_name());
    }
    
    // State management demo
    println!("\n🔄 State Management Demo:");
    println!("  Current tool: {:?}", toolbar.selected_tool());
    
    // Simulate tool switching
    toolbar.set_selected_tool(ToolType::Move);
    println!("  After switching to Move: {:?}", toolbar.selected_tool());
    
    toolbar.set_selected_tool(ToolType::Brush);
    println!("  After switching to Brush: {:?}", toolbar.selected_tool());
    
    // Reset to select
    toolbar.set_selected_tool(ToolType::Select);
    println!("  Reset to Select: {:?}", toolbar.selected_tool());
    
    println!("\n🎉 Toolbar Demo Complete!");
    println!("   The toolbar system is ready for UI integration!");
    
    // Show integration points
    println!("\n🔗 Integration Points:");
    println!("  - Keyboard shortcuts: Handled via handle_keyboard_shortcut()");
    println!("  - Mouse clicks: Handled via handle_click()");
    println!("  - Tool state: Access via selected_tool() and set_selected_tool()");
    println!("  - Rendering: Call render() within UI framework");
    println!("  - Layout: Set bounds via set_bounds() before rendering");
}