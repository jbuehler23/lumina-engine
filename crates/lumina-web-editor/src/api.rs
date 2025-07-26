use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, project::{GameTemplate, Project}};

/// Request to create a new project
#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub template: GameTemplate,
    pub description: Option<String>,
}

/// Response when creating or getting a project
#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub project: Project,
}

/// List all projects
pub async fn list_projects(
    State(state): State<AppState>,
) -> Result<Json<Vec<Project>>, StatusCode> {
    let project_manager = state.project_manager.read().await;
    let projects: Vec<Project> = project_manager.list_projects().into_iter().cloned().collect();
    Ok(Json(projects))
}

/// Create a new project
pub async fn create_project(
    State(state): State<AppState>,
    Json(request): Json<CreateProjectRequest>,
) -> Result<Json<ProjectResponse>, StatusCode> {
    let mut project_manager = state.project_manager.write().await;
    
    let mut project = project_manager.create_project(request.name, request.template);
    
    if let Some(description) = request.description {
        project.description = description;
        // Update the project in the manager
        if let Err(_) = project_manager.update_project(&project.id, project.clone()) {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    log::info!("Created new project: {} ({})", project.name, project.id);
    
    Ok(Json(ProjectResponse { project }))
}

/// Get a specific project
pub async fn get_project(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<ProjectResponse>, StatusCode> {
    let project_id = Uuid::parse_str(&project_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let project_manager = state.project_manager.read().await;
    
    match project_manager.get_project(&project_id) {
        Some(project) => Ok(Json(ProjectResponse { project: project.clone() })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Update a project
pub async fn update_project(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Json(project): Json<Project>,
) -> Result<Json<ProjectResponse>, StatusCode> {
    let project_id = Uuid::parse_str(&project_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let mut project_manager = state.project_manager.write().await;
    
    match project_manager.update_project(&project_id, project.clone()) {
        Ok(_) => {
            log::info!("Updated project: {} ({})", project.name, project.id);
            Ok(Json(ProjectResponse { project }))
        },
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// Delete a project
pub async fn delete_project(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let project_id = Uuid::parse_str(&project_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let mut project_manager = state.project_manager.write().await;
    
    match project_manager.delete_project(&project_id) {
        Ok(_) => {
            log::info!("Deleted project: {}", project_id);
            Ok(StatusCode::NO_CONTENT)
        },
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// Upload an asset to a project
#[derive(Debug, Deserialize)]
pub struct UploadAssetRequest {
    pub name: String,
    pub asset_type: String,
    pub data: String, // Base64 encoded asset data
}

pub async fn upload_asset(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Json(request): Json<UploadAssetRequest>,
) -> Result<StatusCode, StatusCode> {
    let project_id = Uuid::parse_str(&project_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // TODO: Implement asset upload
    // 1. Decode base64 data
    // 2. Save file to assets directory
    // 3. Add asset to project
    // 4. Return asset info
    
    log::info!("Asset upload requested for project {}: {}", project_id, request.name);
    
    // For now, just return success
    Ok(StatusCode::OK)
}

/// Build project to WebAssembly
#[derive(Debug, Serialize)]
pub struct BuildResponse {
    pub success: bool,
    pub build_id: String,
    pub wasm_url: Option<String>,
    pub errors: Vec<String>,
}

pub async fn build_project(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<BuildResponse>, StatusCode> {
    let project_id = Uuid::parse_str(&project_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let project_manager = state.project_manager.read().await;
    
    if let Some(project) = project_manager.get_project(&project_id) {
        log::info!("Building project: {} ({})", project.name, project.id);
        
        // TODO: Implement actual WASM compilation
        // 1. Convert project to Rust code
        // 2. Compile to WebAssembly
        // 3. Generate HTML/JS wrapper
        // 4. Return build artifacts
        
        // For now, return a mock successful build
        let build_id = Uuid::new_v4().to_string();
        Ok(Json(BuildResponse {
            success: true,
            build_id,
            wasm_url: Some(format!("/builds/{}/game.wasm", project_id)),
            errors: Vec::new(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Share project request
#[derive(Debug, Deserialize)]
pub struct ShareProjectRequest {
    pub project: Project,
    pub assets: serde_json::Value,
}

/// Share project response
#[derive(Debug, Serialize)]
pub struct ShareProjectResponse {
    pub share_id: String,
    pub share_url: String,
}

/// Create a shareable link for a project
pub async fn share_project(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Json(request): Json<ShareProjectRequest>,
) -> Result<Json<ShareProjectResponse>, StatusCode> {
    let project_id = Uuid::parse_str(&project_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Generate a unique share ID
    let share_id = Uuid::new_v4().to_string();
    
    log::info!("Creating share link for project: {} ({})", request.project.name, project_id);
    
    // TODO: Store shared project data
    // For now, just return the share ID
    
    let share_url = format!("/play/{}", share_id);
    
    Ok(Json(ShareProjectResponse {
        share_id,
        share_url,
    }))
}