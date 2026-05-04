use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use raven_core::{Document, SearchResult, ServerConfig};
use raven_search::DocumentIndex;
use raven_split::TextSplitter;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

/// Shared application state
pub struct AppState {
    pub index: DocumentIndex,
    pub config: ServerConfig,
    pub splitter: TextSplitter,
    pub metrics: Metrics,
}

/// Server metrics
pub struct Metrics {
    pub requests_total: AtomicU64,
    pub queries_total: AtomicU64,
    pub index_total: AtomicU64,
    pub errors_total: AtomicU64,
    pub started_at: Instant,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            requests_total: AtomicU64::new(0),
            queries_total: AtomicU64::new(0),
            index_total: AtomicU64::new(0),
            errors_total: AtomicU64::new(0),
            started_at: Instant::now(),
        }
    }
}

// --- Request/Response types ---

#[derive(Deserialize)]
pub struct QueryRequest {
    pub query: String,
    #[serde(default = "default_top_k")]
    pub top_k: usize,
}

fn default_top_k() -> usize {
    5
}

#[derive(Serialize)]
pub struct QueryResponse {
    pub results: Vec<ResultItem>,
    pub count: usize,
}

#[derive(Serialize)]
pub struct ResultItem {
    pub text: String,
    pub score: f32,
    pub distance: f32,
    pub metadata: std::collections::HashMap<String, String>,
    pub doc_id: String,
}

impl From<SearchResult> for ResultItem {
    fn from(r: SearchResult) -> Self {
        Self {
            text: r.chunk.text,
            score: r.score,
            distance: r.distance,
            metadata: r.chunk.metadata,
            doc_id: r.chunk.doc_id,
        }
    }
}

#[derive(Deserialize)]
pub struct PromptRequest {
    pub query: String,
    #[serde(default = "default_top_k")]
    pub top_k: usize,
    pub template: Option<String>,
}

#[derive(Serialize)]
pub struct PromptResponse {
    pub prompt: String,
    pub sources: Vec<String>,
}

#[derive(Deserialize)]
pub struct IndexRequest {
    pub documents: Vec<IndexDocument>,
}

#[derive(Deserialize)]
pub struct IndexDocument {
    pub text: String,
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
    pub id: Option<String>,
}

#[derive(Serialize)]
pub struct IndexResponse {
    pub indexed: usize,
    pub message: String,
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub documents: usize,
    pub status: String,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

// --- Handlers ---

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

async fn stats(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.index.count().await {
        Ok(count) => Json(StatsResponse {
            documents: count,
            status: "ok".to_string(),
        })
        .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn query_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<QueryRequest>,
) -> impl IntoResponse {
    state.metrics.requests_total.fetch_add(1, Ordering::Relaxed);
    state.metrics.queries_total.fetch_add(1, Ordering::Relaxed);

    if !check_auth(&headers, &state.config) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Unauthorized"})),
        )
            .into_response();
    }

    let top_k = req.top_k.clamp(1, 1000);

    match state.index.query(&req.query, top_k).await {
        Ok(results) => {
            let count = results.len();
            let items: Vec<ResultItem> = results.into_iter().map(Into::into).collect();
            Json(QueryResponse {
                results: items,
                count,
            })
            .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn prompt_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<PromptRequest>,
) -> impl IntoResponse {
    state.metrics.requests_total.fetch_add(1, Ordering::Relaxed);
    state.metrics.queries_total.fetch_add(1, Ordering::Relaxed);

    if !check_auth(&headers, &state.config) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Unauthorized"})),
        )
            .into_response();
    }

    let top_k = req.top_k.clamp(1, 1000);

    match state.index.query(&req.query, top_k).await {
        Ok(results) => {
            let sources: Vec<String> = results
                .iter()
                .map(|r| {
                    r.chunk
                        .metadata
                        .get("source")
                        .cloned()
                        .unwrap_or_else(|| r.chunk.doc_id.clone())
                })
                .collect();

            let prompt = if let Some(template) = req.template {
                let context = results
                    .iter()
                    .enumerate()
                    .map(|(i, r)| format!("[{}] {}", i + 1, r.chunk.text))
                    .collect::<Vec<_>>()
                    .join("\n\n");
                template
                    .replace("{context}", &context)
                    .replace("{query}", &req.query)
                    .replace("{sources}", &sources.join(", "))
            } else {
                let mut p = format!("Query: {}\n\nContext:\n", req.query);
                for (i, r) in results.iter().enumerate() {
                    let src = r.chunk.metadata.get("source").unwrap_or(&r.chunk.doc_id);
                    p.push_str(&format!(
                        "\n[{}] Source: {}\n{}\n",
                        i + 1,
                        src,
                        r.chunk.text
                    ));
                }
                p.push_str("\n---\nAnswer the query using the provided context.");
                p
            };

            Json(PromptResponse { prompt, sources }).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn index_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<IndexRequest>,
) -> impl IntoResponse {
    state.metrics.requests_total.fetch_add(1, Ordering::Relaxed);
    state.metrics.index_total.fetch_add(1, Ordering::Relaxed);

    if !check_auth(&headers, &state.config) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Unauthorized"})),
        )
            .into_response();
    }

    if req.documents.is_empty() {
        return Json(IndexResponse {
            indexed: 0,
            message: "No documents provided".to_string(),
        })
        .into_response();
    }

    let docs: Vec<Document> = req
        .documents
        .into_iter()
        .map(|d| {
            let mut doc = Document::new(d.text);
            for (k, v) in d.metadata {
                doc = doc.with_metadata(k, v);
            }
            if let Some(id) = d.id {
                doc = doc.with_id(id);
            }
            doc
        })
        .collect();

    let count = docs.len();

    match state.index.add_documents(docs, &state.splitter).await {
        Ok(_) => Json(IndexResponse {
            indexed: count,
            message: format!("Indexed {} documents", count),
        })
        .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn openapi() -> impl IntoResponse {
    let schema = serde_json::json!({
        "openapi": "3.0.3",
        "info": {
            "title": "RavenRustRAG API",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "Fearlessly fast local-first RAG engine"
        },
        "paths": {
            "/health": {
                "get": {
                    "summary": "Health check",
                    "responses": { "200": { "description": "OK" } }
                }
            },
            "/stats": {
                "get": {
                    "summary": "Index statistics",
                    "responses": { "200": { "description": "Stats" } }
                }
            },
            "/query": {
                "post": {
                    "summary": "Search documents",
                    "requestBody": {
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["query"],
                                    "properties": {
                                        "query": { "type": "string" },
                                        "top_k": { "type": "integer", "default": 5 }
                                    }
                                }
                            }
                        }
                    },
                    "responses": { "200": { "description": "Results" } }
                }
            },
            "/prompt": {
                "post": {
                    "summary": "LLM-ready prompt",
                    "responses": { "200": { "description": "Formatted prompt" } }
                }
            },
            "/index": {
                "post": {
                    "summary": "Add documents",
                    "responses": { "200": { "description": "Indexed count" } }
                }
            }
        }
    });
    Json(schema)
}

// --- Auth helper ---

fn check_auth(headers: &HeaderMap, config: &ServerConfig) -> bool {
    let Some(expected_key) = &config.api_key else {
        return true; // No auth required
    };

    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .map(|v| {
            v.strip_prefix("Bearer ")
                .map(|token| token == expected_key)
                .unwrap_or(false)
        })
        .unwrap_or(false)
}

// --- Server builder ---

pub fn build_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health))
        .route("/stats", get(stats))
        .route("/metrics", get(metrics_handler))
        .route("/query", post(query_handler))
        .route("/prompt", post(prompt_handler))
        .route("/index", post(index_handler))
        .route("/openapi.json", get(openapi))
        .layer(cors)
        .layer(axum::extract::DefaultBodyLimit::max(10 * 1024 * 1024)) // 10MB
        .with_state(state)
}

async fn metrics_handler(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let uptime = state.metrics.started_at.elapsed().as_secs();
    let chunks = state.index.count().await.unwrap_or(0);

    Json(serde_json::json!({
        "requests_total": state.metrics.requests_total.load(Ordering::Relaxed),
        "queries_total": state.metrics.queries_total.load(Ordering::Relaxed),
        "index_requests_total": state.metrics.index_total.load(Ordering::Relaxed),
        "errors_total": state.metrics.errors_total.load(Ordering::Relaxed),
        "uptime_seconds": uptime,
        "chunks_total": chunks,
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

pub async fn serve(state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = format!("{}:{}", state.config.host, state.config.port);
    let router = build_router(state);

    info!("RavenRustRAG server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    info!("Server shut down gracefully.");
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    info!("Received shutdown signal, shutting down...");
}
