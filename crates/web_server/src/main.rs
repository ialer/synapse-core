use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{Json, Path, Query, State};
use axum::http::{Method, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use data_core::DataType;
use synapse_service::SynapseApp;

// ============================================================
// App State
// ============================================================

struct AppState {
    app: Mutex<SynapseApp>,
}

type SharedState = Arc<AppState>;

// ============================================================
// Request / Response types
// ============================================================

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct TokenResponse {
    token: String,
}

#[derive(Serialize)]
struct StatsResponse {
    data_count: usize,
    index_count: usize,
    message_count: usize,
}

#[derive(Deserialize)]
struct StoreDataRequest {
    token: String,
    data_type: String,
    content: String,
    tags: Vec<String>,
}

#[derive(Serialize)]
struct IdResponse {
    id: String,
}

#[derive(Deserialize)]
struct UpdateDataRequest {
    token: String,
    content: String,
    tags: Vec<String>,
}

#[derive(Serialize)]
struct SuccessResponse {
    success: bool,
}

#[derive(Deserialize)]
struct TokenQuery {
    token: String,
}

#[derive(Deserialize)]
struct ListDataQuery {
    token: String,
    #[serde(default = "default_offset")]
    offset: usize,
    #[serde(default = "default_limit_val")]
    limit: usize,
}

fn default_offset() -> usize {
    0
}

fn default_limit_val() -> usize {
    20
}

#[derive(Serialize)]
struct ListDataResponse {
    items: Vec<DataItemResponse>,
    total: usize,
    offset: usize,
    limit: usize,
}

#[derive(Serialize)]
struct DataItemResponse {
    id: String,
    data_type: String,
    tags: Vec<String>,
    created_at: String,
}

#[derive(Serialize)]
struct DataDetailResponse {
    id: String,
    data_type: String,
    content: String,
    tags: Vec<String>,
    created_at: String,
    updated_at: String,
    version: u64,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    #[serde(default = "default_limit")]
    limit: usize,
    #[serde(default)]
    tag: Option<String>,
}

fn default_limit() -> usize {
    10
}

#[derive(Serialize)]
struct SearchItemResponse {
    id: String,
    content: String,
    metadata: HashMap<String, String>,
}

#[derive(Deserialize)]
struct SendMessageRequest {
    token: String,
    recipient_id: String,
    title: String,
    content: String,
}

#[derive(Deserialize)]
struct MessagesQuery {
    #[serde(default = "default_msg_limit")]
    limit: usize,
}

fn default_msg_limit() -> usize {
    50
}

#[derive(Serialize)]
struct MessageResponse {
    id: String,
    sender_id: String,
    recipient_id: String,
    title: String,
    content: String,
    sent_at: String,
    is_read: bool,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// ============================================================
// Error helper
// ============================================================

fn error_response(status: StatusCode, msg: &str) -> (StatusCode, Json<ErrorResponse>) {
    (
        status,
        Json(ErrorResponse {
            error: msg.to_string(),
        }),
    )
}

// ============================================================
// Handlers
// ============================================================

// GET /api/health
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    })
}

// POST /api/login
async fn login(
    State(state): State<SharedState>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    let app = state.app.lock().await;
    match app.login(&req.username, &req.password).await {
        Ok(token) => (StatusCode::OK, Json(TokenResponse { token })).into_response(),
        Err(e) => error_response(StatusCode::UNAUTHORIZED, &e.to_string()).into_response(),
    }
}

// POST /api/register
async fn register(
    State(state): State<SharedState>,
    Json(req): Json<RegisterRequest>,
) -> impl IntoResponse {
    let mut app = state.app.lock().await;
    match app.register(&req.username, &req.password).await {
        Ok(token) => (StatusCode::CREATED, Json(TokenResponse { token })).into_response(),
        Err(e) => error_response(StatusCode::BAD_REQUEST, &e.to_string()).into_response(),
    }
}

// GET /api/stats
async fn stats(State(state): State<SharedState>) -> Json<StatsResponse> {
    let app = state.app.lock().await;
    let s = app.stats();
    Json(StatsResponse {
        data_count: s.data_count,
        index_count: s.index_count,
        message_count: s.message_count,
    })
}

// POST /api/data
async fn store_data(
    State(state): State<SharedState>,
    Json(req): Json<StoreDataRequest>,
) -> impl IntoResponse {
    let data_type = match DataType::from_str(&req.data_type) {
        Some(dt) => dt,
        None => {
            return error_response(
                StatusCode::BAD_REQUEST,
                &format!("Invalid data type: {}", req.data_type),
            )
            .into_response()
        }
    };

    let mut app = state.app.lock().await;
    match app
        .secure_store(&req.token, data_type, req.content.into_bytes(), req.tags)
        .await
    {
        Ok(entity) => (
            StatusCode::CREATED,
            Json(IdResponse {
                id: entity.id.to_string(),
            }),
        )
            .into_response(),
        Err(e) => error_response(StatusCode::BAD_REQUEST, &e.to_string()).into_response(),
    }
}

// GET /api/data/:id?token=xxx
async fn get_data(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Query(q): Query<TokenQuery>,
) -> impl IntoResponse {
    let app = state.app.lock().await;
    match app.secure_get_decrypted(&q.token, &id).await {
        Ok((entity, decrypted)) => Json(DataDetailResponse {
            id: entity.id.to_string(),
            data_type: entity.data_type.to_string(),
            content: String::from_utf8_lossy(&decrypted).to_string(),
            tags: entity.tags,
            created_at: entity.created_at.to_rfc3339(),
            updated_at: entity.updated_at.to_rfc3339(),
            version: entity.version,
        })
        .into_response(),
        Err(e) => error_response(StatusCode::BAD_REQUEST, &e.to_string()).into_response(),
    }
}

// GET /api/data/list?token=xxx
async fn list_data(
    State(state): State<SharedState>,
    Query(q): Query<ListDataQuery>,
) -> impl IntoResponse {
    let app = state.app.lock().await;
    // Verify token first
    if app.verify_token(&q.token).await.is_err() {
        return error_response(StatusCode::UNAUTHORIZED, "Invalid token").into_response();
    }

    let all_items: Vec<DataItemResponse> = app
        .list_all_data()
        .into_iter()
        .map(|info| DataItemResponse {
            id: info.id,
            data_type: info.data_type,
            tags: info.tags,
            created_at: info.created_at,
        })
        .collect();

    let total = all_items.len();
    let items: Vec<DataItemResponse> = all_items
        .into_iter()
        .skip(q.offset)
        .take(q.limit)
        .collect();

    Json(ListDataResponse {
        items,
        total,
        offset: q.offset,
        limit: q.limit,
    })
    .into_response()
}

// PUT /api/data/:id
async fn update_data(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateDataRequest>,
) -> impl IntoResponse {
    let mut app = state.app.lock().await;
    match app
        .secure_update(&req.token, &id, req.content.into_bytes(), req.tags)
        .await
    {
        Ok(_) => Json(SuccessResponse { success: true }).into_response(),
        Err(e) => error_response(StatusCode::BAD_REQUEST, &e.to_string()).into_response(),
    }
}

// DELETE /api/data/:id?token=xxx
async fn delete_data(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Query(q): Query<TokenQuery>,
) -> impl IntoResponse {
    let mut app = state.app.lock().await;
    match app.secure_delete(&q.token, &id).await {
        Ok(_) => Json(SuccessResponse { success: true }).into_response(),
        Err(e) => error_response(StatusCode::BAD_REQUEST, &e.to_string()).into_response(),
    }
}

// GET /api/search?q=xxx&limit=10&tag=xxx
async fn search(
    State(state): State<SharedState>,
    Query(q): Query<SearchQuery>,
) -> Json<Vec<SearchItemResponse>> {
    let app = state.app.lock().await;

    // If tag filter is provided, search by tag via proper accessor
    if let Some(ref tag) = q.tag {
        let items: Vec<SearchItemResponse> = app
            .search_by_tag(tag, q.limit)
            .into_iter()
            .map(|info| SearchItemResponse {
                id: info.id,
                content: String::new(),
                metadata: HashMap::from([
                    ("type".to_string(), info.data_type),
                    ("tags".to_string(), info.tags.join(",")),
                ]),
            })
            .collect();
        return Json(items);
    }

    // Default: full-text search via indexer
    let results = app.search(&q.q, q.limit);
    let items: Vec<SearchItemResponse> = results
        .into_iter()
        .map(|e| SearchItemResponse {
            id: e.id,
            content: e.content,
            metadata: e.metadata,
        })
        .collect();
    Json(items)
}

// POST /api/messages
async fn send_message(
    State(state): State<SharedState>,
    Json(req): Json<SendMessageRequest>,
) -> impl IntoResponse {
    let mut app = state.app.lock().await;
    match app.send_message(&req.token, &req.recipient_id, &req.title, &req.content) {
        Ok(_) => Json(SuccessResponse { success: true }).into_response(),
        Err(e) => error_response(StatusCode::BAD_REQUEST, &e.to_string()).into_response(),
    }
}

// GET /api/messages/:user_id?limit=50
async fn get_messages(
    State(state): State<SharedState>,
    Path(user_id): Path<String>,
    Query(q): Query<MessagesQuery>,
) -> Json<Vec<MessageResponse>> {
    let app = state.app.lock().await;
    let messages = app.get_messages(&user_id, q.limit);
    let items: Vec<MessageResponse> = messages
        .into_iter()
        .map(|m| {
            let is_read = m.is_read();
            MessageResponse {
                id: m.id,
                sender_id: m.sender_id,
                recipient_id: m.recipient_id,
                title: m.title,
                content: m.content,
                sent_at: m.sent_at.to_rfc3339(),
                is_read,
            }
        })
        .collect();
    Json(items)
}

// ============================================================
// Main
// ============================================================

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("╔══════════════════════════════════════════╗");
    println!("║   SynapseCore Web Server v0.1.0         ║");
    println!("║   REST API for browser access           ║");
    println!("╚══════════════════════════════════════════╝");

    // Create SynapseApp with local storage
    let mut app = SynapseApp::new_local("./data").await?;
    app.init().await?;
    println!("[web] Application initialized, loading from ./data");

    // Shared state
    let state = SharedState::new(AppState {
        app: Mutex::new(app),
    });

    // CORS layer - allow all origins for development
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any);

    // Build API routes
    let api_routes = Router::new()
        .route("/api/health", get(health))
        .route("/api/login", post(login))
        .route("/api/register", post(register))
        .route("/api/stats", get(stats))
        .route("/api/data", post(store_data))
        .route("/api/data/list", get(list_data))
        .route("/api/data/:id", get(get_data).put(update_data).delete(delete_data))
        .route("/api/search", get(search))
        .route("/api/messages", post(send_message))
        .route("/api/messages/:user_id", get(get_messages));

    // Try to serve the Vue frontend from dist/ directory
    let dist_path = std::path::Path::new("dist");
    let app_router = if dist_path.exists() && dist_path.is_dir() {
        println!("[web] Serving Vue frontend from ./dist");
        Router::new()
            .fallback_service(ServeDir::new("dist").append_index_html_on_directories(true))
            .merge(api_routes)
    } else {
        println!("[web] No dist/ directory found, API-only mode");
        Router::new().merge(api_routes)
    };

    let app = app_router.layer(cors).with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("[web] Listening on http://{}", addr);
    println!("[web] API base: http://{}/api/", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
