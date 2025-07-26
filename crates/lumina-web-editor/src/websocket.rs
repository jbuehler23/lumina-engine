use axum::extract::ws::{Message, WebSocket};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;

/// Messages sent between client and server via WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    /// Client requests to join a project session
    JoinProject { project_id: String },
    
    /// Server confirms project join
    ProjectJoined { project_id: String },
    
    /// Real-time project updates (scene changes, script edits, etc.)
    ProjectUpdate { 
        project_id: String,
        update: ProjectUpdateType,
    },
    
    /// Live preview commands
    PreviewCommand {
        project_id: String,
        command: PreviewCommandType,
    },
    
    /// Error messages
    Error { message: String },
    
    /// Heartbeat to keep connection alive
    Ping,
    Pong,
}

/// Different types of project updates
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "update_type")]
pub enum ProjectUpdateType {
    /// GameObject was added/modified/removed
    GameObjectChanged {
        scene_name: String,
        object_id: String,
        change_type: ChangeType,
    },
    
    /// Scene was modified
    SceneChanged {
        scene_name: String,
        change_type: ChangeType,
    },
    
    /// Visual script was modified
    ScriptChanged {
        script_id: String,
        change_type: ChangeType,
    },
    
    /// Asset was added/removed
    AssetChanged {
        asset_id: String,
        change_type: ChangeType,
    },
    
    /// Project settings changed
    SettingsChanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Added,
    Modified,
    Removed,
}

/// Commands for live preview control
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command_type")]
pub enum PreviewCommandType {
    /// Start/restart the game preview
    Start { scene_name: String },
    
    /// Pause the game preview
    Pause,
    
    /// Resume the game preview
    Resume,
    
    /// Stop the game preview
    Stop,
    
    /// Step one frame (when paused)
    Step,
    
    /// Inject input event for testing
    InjectInput { 
        input_type: String,
        value: serde_json::Value,
    },
}

/// Handle WebSocket connection for a specific project
pub async fn handle_websocket(
    socket: WebSocket,
    project_id: String,
    state: AppState,
) {
    log::info!("WebSocket connection established for project: {}", project_id);
    
    let (mut sender, mut receiver) = socket.split();
    
    // Verify project exists
    let project_uuid = match Uuid::parse_str(&project_id) {
        Ok(id) => id,
        Err(_) => {
            let error_msg = WebSocketMessage::Error {
                message: "Invalid project ID".to_string(),
            };
            if let Ok(msg) = serde_json::to_string(&error_msg) {
                let _ = sender.send(Message::Text(msg)).await;
            }
            return;
        }
    };
    
    {
        let project_manager = state.project_manager.read().await;
        if project_manager.get_project(&project_uuid).is_none() {
            let error_msg = WebSocketMessage::Error {
                message: "Project not found".to_string(),
            };
            if let Ok(msg) = serde_json::to_string(&error_msg) {
                let _ = sender.send(Message::Text(msg)).await;
            }
            return;
        }
    }
    
    // Send confirmation that client joined the project
    let join_msg = WebSocketMessage::ProjectJoined {
        project_id: project_id.clone(),
    };
    if let Ok(msg) = serde_json::to_string(&join_msg) {
        if sender.send(Message::Text(msg)).await.is_err() {
            return;
        }
    }
    
    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                match serde_json::from_str::<WebSocketMessage>(&text) {
                    Ok(ws_msg) => {
                        handle_websocket_message(ws_msg, &project_id, &state, &mut sender).await;
                    }
                    Err(e) => {
                        log::warn!("Failed to parse WebSocket message: {}", e);
                        let error_msg = WebSocketMessage::Error {
                            message: format!("Invalid message format: {}", e),
                        };
                        if let Ok(msg) = serde_json::to_string(&error_msg) {
                            let _ = sender.send(Message::Text(msg)).await;
                        }
                    }
                }
            }
            Ok(Message::Close(_)) => {
                log::info!("WebSocket connection closed for project: {}", project_id);
                break;
            }
            Ok(Message::Ping(data)) => {
                if sender.send(Message::Pong(data)).await.is_err() {
                    break;
                }
            }
            Ok(Message::Pong(_)) => {
                // Handle pong if needed
            }
            Ok(Message::Binary(_)) => {
                log::warn!("Received unexpected binary message");
            }
            Err(e) => {
                log::error!("WebSocket error: {}", e);
                break;
            }
        }
    }
    
    log::info!("WebSocket connection ended for project: {}", project_id);
}

/// Handle individual WebSocket messages
async fn handle_websocket_message(
    message: WebSocketMessage,
    project_id: &str,
    state: &AppState,
    sender: &mut futures::stream::SplitSink<WebSocket, Message>,
) {
    match message {
        WebSocketMessage::Ping => {
            let pong = WebSocketMessage::Pong;
            if let Ok(msg) = serde_json::to_string(&pong) {
                let _ = sender.send(Message::Text(msg)).await;
            }
        }
        
        WebSocketMessage::PreviewCommand { command, .. } => {
            handle_preview_command(command, project_id, state, sender).await;
        }
        
        WebSocketMessage::ProjectUpdate { update, .. } => {
            // TODO: Apply the update to the project
            // This would modify the project state and potentially
            // broadcast the change to other connected clients
            log::info!("Received project update: {:?}", update);
        }
        
        _ => {
            log::warn!("Unhandled WebSocket message type");
        }
    }
}

/// Handle preview commands (start, stop, pause, etc.)
async fn handle_preview_command(
    command: PreviewCommandType,
    project_id: &str,
    state: &AppState,
    sender: &mut futures::stream::SplitSink<WebSocket, Message>,
) {
    match command {
        PreviewCommandType::Start { scene_name } => {
            log::info!("Starting preview for project {} scene {}", project_id, scene_name);
            
            // TODO: Convert project to runnable game
            // 1. Get project from manager
            // 2. Convert scene to ECS world
            // 3. Start game loop
            // 4. Send preview updates back to client
            
            // For now, just acknowledge the command
            let response = WebSocketMessage::PreviewCommand {
                project_id: project_id.to_string(),
                command: PreviewCommandType::Start {
                    scene_name: scene_name.clone(),
                },
            };
            
            if let Ok(msg) = serde_json::to_string(&response) {
                let _ = sender.send(Message::Text(msg)).await;
            }
        }
        
        PreviewCommandType::Stop => {
            log::info!("Stopping preview for project {}", project_id);
            // TODO: Stop the game loop
        }
        
        PreviewCommandType::Pause => {
            log::info!("Pausing preview for project {}", project_id);
            // TODO: Pause the game loop
        }
        
        PreviewCommandType::Resume => {
            log::info!("Resuming preview for project {}", project_id);
            // TODO: Resume the game loop
        }
        
        PreviewCommandType::Step => {
            log::info!("Stepping one frame for project {}", project_id);
            // TODO: Execute one frame of the game loop
        }
        
        PreviewCommandType::InjectInput { input_type, value } => {
            log::info!("Injecting input {} = {:?} for project {}", input_type, value, project_id);
            // TODO: Inject input into the running game
        }
    }
}