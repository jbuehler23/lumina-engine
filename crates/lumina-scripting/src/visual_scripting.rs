use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Visual scripting system for non-programmer game creation
/// This allows users to create game logic using node-based visual interfaces

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualScript {
    pub name: String,
    pub nodes: Vec<ScriptNode>,
    pub connections: Vec<NodeConnection>,
    pub variables: HashMap<String, ScriptValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptNode {
    pub id: String,
    pub node_type: NodeType,
    pub position: (f32, f32), // For editor positioning
    pub properties: HashMap<String, ScriptValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConnection {
    pub from_node: String,
    pub from_output: String,
    pub to_node: String,
    pub to_input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    // Event Nodes (Blue - Start execution)
    OnStart,
    OnUpdate,
    OnInput(InputType),
    OnCollision(String), // collision tag
    OnTimer(f32),
    
    // Action Nodes (Red - Do something)
    MoveTowards { target: String, speed: f32 },
    PlaySound(String),
    PlayAnimation(String),
    ChangeScene(String),
    SetProperty { target: String, property: String, value: ScriptValue },
    SpawnObject(String),
    DestroyObject(String),
    
    // Logic Nodes (Yellow - Make decisions)
    If { condition: Condition },
    While { condition: Condition },
    Switch { variable: String },
    
    // Data Nodes (Green - Store/retrieve data)
    GetProperty { target: String, property: String },
    SetVariable { name: String, value: ScriptValue },
    GetVariable(String),
    Random { min: f32, max: f32 },
    
    // Math Nodes (Purple - Calculations)
    Add, Subtract, Multiply, Divide,
    Compare { operator: CompareOp },
    
    // Utility Nodes (Gray - Helper functions)
    Print(String),
    Wait(f32),
    Comment(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputType {
    KeyPressed(String),
    KeyHeld(String),
    KeyReleased(String),
    MouseClick,
    MouseHover,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptValue {
    Number(f32),
    Text(String),
    Boolean(bool),
    Vector2(f32, f32),
    Color(f32, f32, f32, f32),
    Reference(String), // Reference to game object
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    Equals(ScriptValue, ScriptValue),
    Greater(ScriptValue, ScriptValue),
    Less(ScriptValue, ScriptValue),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompareOp {
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

// Visual scripts would be components in the ECS system
// impl Component for VisualScript {}

/// Example: Create a simple "Player Movement" script
pub fn create_player_movement_script() -> VisualScript {
    VisualScript {
        name: "Player Movement".to_string(),
        nodes: vec![
            // Input detection
            ScriptNode {
                id: "input_left".to_string(),
                node_type: NodeType::OnInput(InputType::KeyHeld("A".to_string())),
                position: (100.0, 100.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "input_right".to_string(),
                node_type: NodeType::OnInput(InputType::KeyHeld("D".to_string())),
                position: (100.0, 200.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "input_jump".to_string(),
                node_type: NodeType::OnInput(InputType::KeyPressed("Space".to_string())),
                position: (100.0, 300.0),
                properties: HashMap::new(),
            },
            
            // Movement actions
            ScriptNode {
                id: "move_left".to_string(),
                node_type: NodeType::MoveTowards { 
                    target: "self".to_string(), 
                    speed: -200.0 
                },
                position: (400.0, 100.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "move_right".to_string(),
                node_type: NodeType::MoveTowards { 
                    target: "self".to_string(), 
                    speed: 200.0 
                },
                position: (400.0, 200.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "jump".to_string(),
                node_type: NodeType::SetProperty {
                    target: "self".to_string(),
                    property: "velocity_y".to_string(),
                    value: ScriptValue::Number(300.0),
                },
                position: (400.0, 300.0),
                properties: HashMap::new(),
            },
        ],
        connections: vec![
            NodeConnection {
                from_node: "input_left".to_string(),
                from_output: "triggered".to_string(),
                to_node: "move_left".to_string(),
                to_input: "execute".to_string(),
            },
            NodeConnection {
                from_node: "input_right".to_string(),
                from_output: "triggered".to_string(),
                to_node: "move_right".to_string(),
                to_input: "execute".to_string(),
            },
            NodeConnection {
                from_node: "input_jump".to_string(),
                from_output: "triggered".to_string(),
                to_node: "jump".to_string(),
                to_input: "execute".to_string(),
            },
        ],
        variables: HashMap::new(),
    }
}

/// Example: Create a "Coin Collection" script
pub fn create_coin_collection_script() -> VisualScript {
    VisualScript {
        name: "Coin Collection".to_string(),
        nodes: vec![
            // Collision detection
            ScriptNode {
                id: "player_collision".to_string(),
                node_type: NodeType::OnCollision("Player".to_string()),
                position: (100.0, 100.0),
                properties: HashMap::new(),
            },
            
            // Actions when collected
            ScriptNode {
                id: "play_sound".to_string(),
                node_type: NodeType::PlaySound("coin_pickup.wav".to_string()),
                position: (400.0, 50.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "add_score".to_string(),
                node_type: NodeType::SetVariable {
                    name: "score".to_string(),
                    value: ScriptValue::Number(10.0), // This would be score + 10
                },
                position: (400.0, 150.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "destroy_coin".to_string(),
                node_type: NodeType::DestroyObject("self".to_string()),
                position: (400.0, 250.0),
                properties: HashMap::new(),
            },
        ],
        connections: vec![
            NodeConnection {
                from_node: "player_collision".to_string(),
                from_output: "triggered".to_string(),
                to_node: "play_sound".to_string(),
                to_input: "execute".to_string(),
            },
            NodeConnection {
                from_node: "play_sound".to_string(),
                from_output: "finished".to_string(),
                to_node: "add_score".to_string(),
                to_input: "execute".to_string(),
            },
            NodeConnection {
                from_node: "add_score".to_string(),
                from_output: "finished".to_string(),
                to_node: "destroy_coin".to_string(),
                to_input: "execute".to_string(),
            },
        ],
        variables: HashMap::new(),
    }
}

/// System that executes visual scripts
pub struct VisualScriptExecutor {
    // This would contain the runtime state for executing scripts
    // In a real implementation, this would interface with the ECS
    // to read inputs, modify components, etc.
}

impl VisualScriptExecutor {
    pub fn new() -> Self {
        Self {}
    }
    
    /// Execute a visual script node
    /// In a full implementation, this would take a reference to the ECS World
    pub fn execute_node(&mut self, node: &ScriptNode) {
        match &node.node_type {
            NodeType::OnStart => {
                // Script execution starts here
            },
            NodeType::MoveTowards { target, speed } => {
                // Move the target object towards a direction at given speed
                // This would interface with the transform component
            },
            NodeType::PlaySound(sound_path) => {
                // Play audio file
                // This would interface with the audio system
            },
            NodeType::If { condition } => {
                // Evaluate condition and branch execution
                // This would control which connected nodes execute next
            },
            // ... implement other node types
            _ => {
                log::warn!("Unimplemented node type: {:?}", node.node_type);
            }
        }
    }
}

/// Web editor would generate JavaScript like this to represent the visual script:
/// ```javascript
/// const playerMovementScript = {
///   name: "Player Movement",
///   nodes: [
///     {
///       id: "input_left",
///       type: "OnInput",
///       inputType: "KeyHeld",
///       key: "A",
///       position: [100, 100]
///     },
///     {
///       id: "move_left", 
///       type: "MoveTowards",
///       target: "self",
///       speed: -200,
///       position: [400, 100]
///     }
///   ],
///   connections: [
///     { from: "input_left", to: "move_left" }
///   ]
/// }
/// ```

/// Create a top-down movement script for adventure games
pub fn create_topdown_movement_script() -> VisualScript {
    VisualScript {
        name: "Top-Down Movement".to_string(),
        nodes: vec![
            ScriptNode {
                id: "input_up".to_string(),
                node_type: NodeType::OnInput(InputType::KeyHeld("W".to_string())),
                position: (50.0, 50.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "input_down".to_string(),
                node_type: NodeType::OnInput(InputType::KeyHeld("S".to_string())),
                position: (50.0, 150.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "input_left".to_string(),
                node_type: NodeType::OnInput(InputType::KeyHeld("A".to_string())),
                position: (50.0, 250.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "input_right".to_string(),
                node_type: NodeType::OnInput(InputType::KeyHeld("D".to_string())),
                position: (50.0, 350.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "move_up".to_string(),
                node_type: NodeType::SetProperty {
                    target: "self".to_string(),
                    property: "velocity".to_string(),
                    value: ScriptValue::Vector2(0.0, 150.0),
                },
                position: (300.0, 50.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "move_down".to_string(),
                node_type: NodeType::SetProperty {
                    target: "self".to_string(),
                    property: "velocity".to_string(),
                    value: ScriptValue::Vector2(0.0, -150.0),
                },
                position: (300.0, 150.0),
                properties: HashMap::new(),
            },
        ],
        connections: vec![
            NodeConnection {
                from_node: "input_up".to_string(),
                from_output: "triggered".to_string(),
                to_node: "move_up".to_string(),
                to_input: "execute".to_string(),
            },
            NodeConnection {
                from_node: "input_down".to_string(),
                from_output: "triggered".to_string(),
                to_node: "move_down".to_string(),
                to_input: "execute".to_string(),
            },
        ],
        variables: HashMap::new(),
    }
}

/// Create an NPC dialogue script
pub fn create_npc_dialogue_script() -> VisualScript {
    VisualScript {
        name: "NPC Dialogue".to_string(),
        nodes: vec![
            ScriptNode {
                id: "player_interaction".to_string(),
                node_type: NodeType::OnCollision("Player".to_string()),
                position: (100.0, 100.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "show_dialogue".to_string(),
                node_type: NodeType::SetProperty {
                    target: "UI_DialogueBox".to_string(),
                    property: "text".to_string(),
                    value: ScriptValue::Text("Hello, traveler! Welcome to our village.".to_string()),
                },
                position: (400.0, 100.0),
                properties: HashMap::new(),
            },
        ],
        connections: vec![
            NodeConnection {
                from_node: "player_interaction".to_string(),
                from_output: "triggered".to_string(),
                to_node: "show_dialogue".to_string(),
                to_input: "execute".to_string(),
            },
        ],
        variables: HashMap::new(),
    }
}

/// Create a puzzle manager script
pub fn create_puzzle_manager_script() -> VisualScript {
    VisualScript {
        name: "Puzzle Manager".to_string(),
        nodes: vec![
            ScriptNode {
                id: "check_solution".to_string(),
                node_type: NodeType::OnUpdate,
                position: (100.0, 100.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "puzzle_solved".to_string(),
                node_type: NodeType::If {
                    condition: Condition::Equals(
                        ScriptValue::Text("pieces_in_place".to_string()),
                        ScriptValue::Number(3.0),
                    ),
                },
                position: (400.0, 100.0),
                properties: HashMap::new(),
            },
        ],
        connections: vec![
            NodeConnection {
                from_node: "check_solution".to_string(),
                from_output: "tick".to_string(),
                to_node: "puzzle_solved".to_string(),
                to_input: "execute".to_string(),
            },
        ],
        variables: HashMap::new(),
    }
}

/// Create a draggable piece script
pub fn create_draggable_piece_script() -> VisualScript {
    VisualScript {
        name: "Draggable Piece".to_string(),
        nodes: vec![
            ScriptNode {
                id: "mouse_click".to_string(),
                node_type: NodeType::OnInput(InputType::MouseClick),
                position: (100.0, 100.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "follow_mouse".to_string(),
                node_type: NodeType::SetProperty {
                    target: "self".to_string(),
                    property: "follow_mouse".to_string(),
                    value: ScriptValue::Boolean(true),
                },
                position: (400.0, 100.0),
                properties: HashMap::new(),
            },
        ],
        connections: vec![
            NodeConnection {
                from_node: "mouse_click".to_string(),
                from_output: "triggered".to_string(),
                to_node: "follow_mouse".to_string(),
                to_input: "execute".to_string(),
            },
        ],
        variables: HashMap::new(),
    }
}

/// Create a ship control script for arcade shooter
pub fn create_ship_control_script() -> VisualScript {
    VisualScript {
        name: "Ship Control".to_string(),
        nodes: vec![
            ScriptNode {
                id: "input_left".to_string(),
                node_type: NodeType::OnInput(InputType::KeyHeld("A".to_string())),
                position: (50.0, 100.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "input_right".to_string(),
                node_type: NodeType::OnInput(InputType::KeyHeld("D".to_string())),
                position: (50.0, 200.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "move_left".to_string(),
                node_type: NodeType::SetProperty {
                    target: "self".to_string(),
                    property: "velocity_x".to_string(),
                    value: ScriptValue::Number(-250.0),
                },
                position: (300.0, 100.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "move_right".to_string(),
                node_type: NodeType::SetProperty {
                    target: "self".to_string(),
                    property: "velocity_x".to_string(),
                    value: ScriptValue::Number(250.0),
                },
                position: (300.0, 200.0),
                properties: HashMap::new(),
            },
        ],
        connections: vec![
            NodeConnection {
                from_node: "input_left".to_string(),
                from_output: "triggered".to_string(),
                to_node: "move_left".to_string(),
                to_input: "execute".to_string(),
            },
            NodeConnection {
                from_node: "input_right".to_string(),
                from_output: "triggered".to_string(),
                to_node: "move_right".to_string(),
                to_input: "execute".to_string(),
            },
        ],
        variables: HashMap::new(),
    }
}

/// Create a ship shooter script
pub fn create_ship_shooter_script() -> VisualScript {
    VisualScript {
        name: "Ship Shooter".to_string(),
        nodes: vec![
            ScriptNode {
                id: "fire_input".to_string(),
                node_type: NodeType::OnInput(InputType::KeyPressed("Space".to_string())),
                position: (100.0, 100.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "spawn_bullet".to_string(),
                node_type: NodeType::SpawnObject("bullet".to_string()),
                position: (400.0, 100.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "play_shoot_sound".to_string(),
                node_type: NodeType::PlaySound("laser_shoot.wav".to_string()),
                position: (400.0, 200.0),
                properties: HashMap::new(),
            },
        ],
        connections: vec![
            NodeConnection {
                from_node: "fire_input".to_string(),
                from_output: "triggered".to_string(),
                to_node: "spawn_bullet".to_string(),
                to_input: "execute".to_string(),
            },
            NodeConnection {
                from_node: "spawn_bullet".to_string(),
                from_output: "finished".to_string(),
                to_node: "play_shoot_sound".to_string(),
                to_input: "execute".to_string(),
            },
        ],
        variables: HashMap::new(),
    }
}

/// Create an enemy AI script
pub fn create_enemy_ai_script() -> VisualScript {
    VisualScript {
        name: "Enemy AI".to_string(),
        nodes: vec![
            ScriptNode {
                id: "update_tick".to_string(),
                node_type: NodeType::OnUpdate,
                position: (100.0, 100.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "move_towards_player".to_string(),
                node_type: NodeType::MoveTowards {
                    target: "Player".to_string(),
                    speed: 50.0,
                },
                position: (400.0, 100.0),
                properties: HashMap::new(),
            },
        ],
        connections: vec![
            NodeConnection {
                from_node: "update_tick".to_string(),
                from_output: "tick".to_string(),
                to_node: "move_towards_player".to_string(),
                to_input: "execute".to_string(),
            },
        ],
        variables: HashMap::new(),
    }
}

/// Create an enemy shooter script
pub fn create_enemy_shooter_script() -> VisualScript {
    VisualScript {
        name: "Enemy Shooter".to_string(),
        nodes: vec![
            ScriptNode {
                id: "shoot_timer".to_string(),
                node_type: NodeType::OnTimer(2.0), // Shoot every 2 seconds
                position: (100.0, 100.0),
                properties: HashMap::new(),
            },
            ScriptNode {
                id: "spawn_enemy_bullet".to_string(),
                node_type: NodeType::SpawnObject("enemy_bullet".to_string()),
                position: (400.0, 100.0),
                properties: HashMap::new(),
            },
        ],
        connections: vec![
            NodeConnection {
                from_node: "shoot_timer".to_string(),
                from_output: "triggered".to_string(),
                to_node: "spawn_enemy_bullet".to_string(),
                to_input: "execute".to_string(),
            },
        ],
        variables: HashMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_player_movement_script() {
        let script = create_player_movement_script();
        assert_eq!(script.name, "Player Movement");
        assert_eq!(script.nodes.len(), 6);
        assert_eq!(script.connections.len(), 3);
    }

    #[test]
    fn test_create_coin_collection_script() {
        let script = create_coin_collection_script();
        assert_eq!(script.name, "Coin Collection");
        assert_eq!(script.nodes.len(), 4);
        assert_eq!(script.connections.len(), 3);
    }

    #[test]
    fn test_script_serialization() {
        let script = create_player_movement_script();
        let json = serde_json::to_string(&script).unwrap();
        let deserialized: VisualScript = serde_json::from_str(&json).unwrap();
        assert_eq!(script.name, deserialized.name);
    }
}