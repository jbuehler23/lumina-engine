use axum::{
    extract::{Path, State, WebSocketUpgrade},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, services::ServeDir};

use lumina_web_editor::{AppState, api, handle_websocket};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    log::info!("Starting Lumina Web Editor");

    let app_state = AppState::new();

    // Build our application with routes
    let app = Router::new()
        // Serve the web editor frontend
        .route("/", get(serve_editor))
        
        // API routes for project management
        .route("/api/projects", get(api::list_projects).post(api::create_project))
        .route("/api/projects/:project_id", get(api::get_project).put(api::update_project).delete(api::delete_project))
        .route("/api/projects/:project_id/assets", post(api::upload_asset))
        .route("/api/projects/:project_id/build", post(api::build_project))
        .route("/api/projects/:project_id/share", post(api::share_project))
        
        // WebSocket for real-time collaboration and live preview
        .route("/ws/:project_id", get(websocket_handler))
        
        // Serve static files (frontend assets, uploaded game assets)
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/assets", ServeDir::new("assets"))
        
        // Add CORS middleware for development
        .layer(CorsLayer::permissive())
        
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    log::info!("Lumina Web Editor listening on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Serve the main editor HTML page
async fn serve_editor() -> impl IntoResponse {
    Html(include_str!("../static/index.html"))
}

/// WebSocket handler for real-time communication
async fn websocket_handler(
    ws: WebSocketUpgrade,
    Path(project_id): Path<String>,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(move |socket| handle_websocket(socket, project_id, state))
}