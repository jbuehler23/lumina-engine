//! Editor toolbar with common tools and actions

use anyhow::Result;
use glam::Vec2;
use lumina_ui::{UiFramework, Button, Panel, Text, WidgetId};
use lumina_ui::widgets::button::ButtonVariant;
use glam::Vec4;
use log::debug;

use crate::layout::types::Rect;

/// Editor toolbar with quick access tools and actions
pub struct EditorToolbar {
    widgets: Vec<WidgetId>,
    selected_tool: ToolType,
    bounds: Rect,
    /// Actions triggered by toolbar buttons that need to be processed
    pending_actions: Vec<ToolbarAction>,
}

/// Types of tools available in the toolbar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolType {
    /// Selection tool for picking objects
    Select,
    /// Move tool for repositioning objects  
    Move,
    /// Rotate tool for rotating objects
    Rotate,
    /// Scale tool for resizing objects
    Scale,
    /// Brush tool for painting/drawing
    Brush,
    /// Eraser tool for removing objects
    Eraser,
}

impl ToolType {
    /// Get the display name for the tool
    pub fn display_name(self) -> &'static str {
        match self {
            ToolType::Select => "Select",
            ToolType::Move => "Move", 
            ToolType::Rotate => "Rotate",
            ToolType::Scale => "Scale",
            ToolType::Brush => "Brush",
            ToolType::Eraser => "Eraser",
        }
    }

    /// Get the icon for the tool
    pub fn icon(self) -> &'static str {
        match self {
            ToolType::Select => "🔍",
            ToolType::Move => "↔️",
            ToolType::Rotate => "🔄", 
            ToolType::Scale => "📏",
            ToolType::Brush => "🖌️",
            ToolType::Eraser => "🧽",
        }
    }

    /// Get the keyboard shortcut for the tool
    pub fn shortcut(self) -> &'static str {
        match self {
            ToolType::Select => "V",
            ToolType::Move => "G",
            ToolType::Rotate => "R", 
            ToolType::Scale => "S",
            ToolType::Brush => "B",
            ToolType::Eraser => "E",
        }
    }
}

/// Actions that can be triggered from the toolbar
#[derive(Debug, Clone)]
pub enum ToolbarAction {
    /// Tool selection changed
    ToolSelected(ToolType),
    /// New project requested
    NewProject,
    /// Open project requested  
    OpenProject,
    /// Save project requested
    SaveProject,
    /// Undo last action
    Undo,
    /// Redo last action
    Redo,
    /// Play/test the game
    Play,
    /// Pause the game
    Pause, 
    /// Stop the game
    Stop,
    /// No action
    None,
}

impl EditorToolbar {
    /// Create a new editor toolbar
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            selected_tool: ToolType::Select,
            bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
            pending_actions: Vec::new(),
        }
    }

    /// Set the toolbar bounds
    pub fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    /// Get the currently selected tool
    pub fn selected_tool(&self) -> ToolType {
        self.selected_tool
    }

    /// Set the selected tool
    pub fn set_selected_tool(&mut self, tool: ToolType) {
        if self.selected_tool != tool {
            self.selected_tool = tool;
            debug!("Selected tool changed to: {:?}", tool);
        }
    }

    /// Render the toolbar
    pub fn render(&mut self, ui: &mut UiFramework) -> Result<()> {
        // Clear existing widgets
        self.widgets.clear();

        if self.bounds.width <= 0.0 || self.bounds.height <= 0.0 {
            return Ok(());
        }

        // Create toolbar background
        let toolbar_panel = Panel::new()
            .style(lumina_ui::widgets::WidgetStyle {
                background_color: Some([0.08, 0.08, 0.16, 1.0]), // Slightly darker than panels
                border_radius: Some(4.0),
                ..Default::default()
            });

        let panel_id = ui.add_root_widget(Box::new(toolbar_panel));
        self.widgets.push(panel_id);

        // File operations section
        self.render_file_section(ui, panel_id)?;

        // Tool separator  
        let separator1 = Text::new("|")
            .font_size(16.0)
            .color(Vec4::new(0.4, 0.4, 0.4, 1.0));
        let sep1_id = ui.add_widget(Box::new(separator1));
        ui.add_child_to_parent(panel_id, sep1_id);
        self.widgets.push(sep1_id);

        // Edit operations section
        self.render_edit_section(ui, panel_id)?;

        // Tool separator
        let separator2 = Text::new("|")
            .font_size(16.0)
            .color(Vec4::new(0.4, 0.4, 0.4, 1.0));
        let sep2_id = ui.add_widget(Box::new(separator2));
        ui.add_child_to_parent(panel_id, sep2_id);
        self.widgets.push(sep2_id);

        // Tools section
        self.render_tools_section(ui, panel_id)?;

        // Tool separator
        let separator3 = Text::new("|")
            .font_size(16.0)
            .color(Vec4::new(0.4, 0.4, 0.4, 1.0));
        let sep3_id = ui.add_widget(Box::new(separator3));
        ui.add_child_to_parent(panel_id, sep3_id);
        self.widgets.push(sep3_id);

        // Playback controls section
        self.render_playback_section(ui, panel_id)?;

        Ok(())
    }

    fn render_file_section(&mut self, ui: &mut UiFramework, parent_id: WidgetId) -> Result<()> {
        // New project button
        let new_btn = Button::new("📄 New")
            .variant(ButtonVariant::Ghost)
            .action("new_project");
        let new_id = ui.add_widget(Box::new(new_btn));
        ui.add_child_to_parent(parent_id, new_id);
        self.widgets.push(new_id);

        // Open project button
        let open_btn = Button::new("📂 Open")
            .variant(ButtonVariant::Ghost)
            .action("open_project");
        let open_id = ui.add_widget(Box::new(open_btn));
        ui.add_child_to_parent(parent_id, open_id);
        self.widgets.push(open_id);

        // Save project button
        let save_btn = Button::new("💾 Save")
            .variant(ButtonVariant::Ghost)
            .action("save_project");
        let save_id = ui.add_widget(Box::new(save_btn));
        ui.add_child_to_parent(parent_id, save_id);
        self.widgets.push(save_id);

        Ok(())
    }

    fn render_edit_section(&mut self, ui: &mut UiFramework, parent_id: WidgetId) -> Result<()> {
        // Undo button
        let undo_btn = Button::new("↶ Undo")
            .variant(ButtonVariant::Ghost)
            .action("undo");
        let undo_id = ui.add_widget(Box::new(undo_btn));
        ui.add_child_to_parent(parent_id, undo_id);
        self.widgets.push(undo_id);

        // Redo button  
        let redo_btn = Button::new("↷ Redo")
            .variant(ButtonVariant::Ghost)
            .action("redo");
        let redo_id = ui.add_widget(Box::new(redo_btn));
        ui.add_child_to_parent(parent_id, redo_id);
        self.widgets.push(redo_id);

        Ok(())
    }

    fn render_tools_section(&mut self, ui: &mut UiFramework, parent_id: WidgetId) -> Result<()> {
        let tools = [
            ToolType::Select,
            ToolType::Move,
            ToolType::Rotate,
            ToolType::Scale,
            ToolType::Brush,
            ToolType::Eraser,
        ];

        for tool in &tools {
            let is_selected = *tool == self.selected_tool;
            let variant = if is_selected {
                ButtonVariant::Primary
            } else {
                ButtonVariant::Ghost
            };

            let button_text = format!("{} {}", tool.icon(), tool.display_name());
            let tool_copy = *tool; // Copy for closure
            
            let tool_btn = Button::new(&button_text)
                .variant(variant)
                .action(format!("select_tool_{:?}", tool_copy));

            let tool_id = ui.add_widget(Box::new(tool_btn));
            ui.add_child_to_parent(parent_id, tool_id);
            self.widgets.push(tool_id);
        }

        Ok(())
    }

    fn render_playback_section(&mut self, ui: &mut UiFramework, parent_id: WidgetId) -> Result<()> {
        // Play button
        let play_btn = Button::new("▶️ Play")
            .variant(ButtonVariant::Primary)
            .action("play");
        let play_id = ui.add_widget(Box::new(play_btn));
        ui.add_child_to_parent(parent_id, play_id);
        self.widgets.push(play_id);

        // Pause button
        let pause_btn = Button::new("⏸️ Pause")
            .variant(ButtonVariant::Secondary)
            .action("pause");
        let pause_id = ui.add_widget(Box::new(pause_btn));
        ui.add_child_to_parent(parent_id, pause_id);
        self.widgets.push(pause_id);

        // Stop button
        let stop_btn = Button::new("⏹️ Stop") 
            .variant(ButtonVariant::Ghost)
            .action("stop");
        let stop_id = ui.add_widget(Box::new(stop_btn));
        ui.add_child_to_parent(parent_id, stop_id);
        self.widgets.push(stop_id);

        Ok(())
    }

    /// Get and clear pending actions from toolbar button clicks
    pub fn take_pending_actions(&mut self) -> Vec<ToolbarAction> {
        std::mem::take(&mut self.pending_actions)
    }
    
    /// Add an action to the pending actions queue
    fn add_action(&mut self, action: ToolbarAction) {
        debug!("Toolbar action queued: {:?}", action);
        self.pending_actions.push(action);
    }

    /// Handle keyboard shortcuts for tools
    pub fn handle_keyboard_shortcut(&mut self, key: &str) -> ToolbarAction {
        let tools = [
            ToolType::Select,
            ToolType::Move,
            ToolType::Rotate,
            ToolType::Scale,
            ToolType::Brush,
            ToolType::Eraser,
        ];

        for tool in &tools {
            if tool.shortcut().to_lowercase() == key.to_lowercase() {
                self.set_selected_tool(*tool);
                return ToolbarAction::ToolSelected(*tool);
            }
        }

        // Handle other shortcuts
        match key.to_lowercase().as_str() {
            "ctrl+n" => ToolbarAction::NewProject,
            "ctrl+o" => ToolbarAction::OpenProject,
            "ctrl+s" => ToolbarAction::SaveProject,
            "ctrl+z" => ToolbarAction::Undo,
            "ctrl+y" | "ctrl+shift+z" => ToolbarAction::Redo,
            "space" => ToolbarAction::Play,
            _ => ToolbarAction::None,
        }
    }

    /// Get the bounds of the toolbar
    pub fn bounds(&self) -> Rect {
        self.bounds
    }

    /// Update toolbar state (called each frame)
    pub fn update(&mut self) {
        // Update toolbar state, animations, etc.
        // For now, this is empty but could be used for:
        // - Tool state animations
        // - Button hover effects  
        // - Status updates
    }
}

impl Default for EditorToolbar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolbar_creation() {
        let toolbar = EditorToolbar::new();
        assert_eq!(toolbar.selected_tool(), ToolType::Select);
        assert_eq!(toolbar.widgets.len(), 0);
    }

    #[test]
    fn test_tool_selection() {
        let mut toolbar = EditorToolbar::new();
        assert_eq!(toolbar.selected_tool(), ToolType::Select);

        toolbar.set_selected_tool(ToolType::Move);
        assert_eq!(toolbar.selected_tool(), ToolType::Move);
    }

    #[test]
    fn test_keyboard_shortcuts() {
        let mut toolbar = EditorToolbar::new();
        
        // Test tool shortcuts
        let action = toolbar.handle_keyboard_shortcut("g");
        match action {
            ToolbarAction::ToolSelected(ToolType::Move) => {
                assert_eq!(toolbar.selected_tool(), ToolType::Move);
            }
            _ => panic!("Expected ToolSelected(Move) action"),
        }

        // Test file shortcuts
        let action = toolbar.handle_keyboard_shortcut("ctrl+n");
        match action {
            ToolbarAction::NewProject => {},
            _ => panic!("Expected NewProject action"),
        }
    }

    #[test]
    fn test_tool_properties() {
        assert_eq!(ToolType::Select.display_name(), "Select");
        assert_eq!(ToolType::Move.icon(), "↔️");
        assert_eq!(ToolType::Scale.shortcut(), "S");
    }
}