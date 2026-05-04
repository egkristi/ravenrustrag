//! Axum HTTP API server for RavenRustRAG.
//!
//! Provides REST endpoints for querying, indexing, and managing the document index.

use axum::{
    extract::{Json, Path as AxumPath, State},
    http::{HeaderMap, HeaderValue, Method, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{delete, get, post},
    Router,
};
use raven_core::{Document, SearchResult, ServerConfig};
use raven_search::DocumentIndex;
use raven_split::TextSplitter;
use raven_store::MetadataFilter;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use subtle::ConstantTimeEq;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{error, info};

/// Shared application state
pub struct AppState {
    pub index: DocumentIndex,
    pub config: ServerConfig,
    pub splitter: TextSplitter,
    pub metrics: Metrics,
    rate_limiter: Mutex<RateLimiter>,
}

impl AppState {
    pub fn new(index: DocumentIndex, config: ServerConfig, splitter: TextSplitter) -> Self {
        let rate = config.rate_limit_per_second;
        Self {
            index,
            config,
            splitter,
            metrics: Metrics::default(),
            rate_limiter: Mutex::new(RateLimiter::new(rate)),
        }
    }
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

// --- Rate limiter (token bucket) ---

struct RateLimiter {
    tokens: f64,
    max_tokens: f64,
    refill_rate: f64, // tokens per second
    last_refill: Instant,
}

impl RateLimiter {
    fn new(rate_per_second: u32) -> Self {
        let rate = f64::from(rate_per_second);
        Self {
            tokens: rate,
            max_tokens: rate,
            refill_rate: rate,
            last_refill: Instant::now(),
        }
    }

    fn try_acquire(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.max_tokens);
        self.last_refill = now;

        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}

async fn rate_limit_middleware(
    State(state): State<Arc<AppState>>,
    req: axum::extract::Request,
    next: Next,
) -> impl IntoResponse {
    let mut limiter = state.rate_limiter.lock().await;
    if limiter.try_acquire() {
        drop(limiter);
        next.run(req).await.into_response()
    } else {
        drop(limiter);
        (
            StatusCode::TOO_MANY_REQUESTS,
            Json(serde_json::json!({"error": "Rate limit exceeded"})),
        )
            .into_response()
    }
}

// --- Request/Response types ---

/// Strip control characters from input text (U+0000-U+001F except \n and \t)
fn sanitize_input(s: &str) -> String {
    s.chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .collect()
}

#[derive(Deserialize)]
pub struct QueryRequest {
    pub query: String,
    #[serde(default = "default_top_k")]
    pub top_k: usize,
    #[serde(default)]
    pub hybrid: bool,
    #[serde(default = "default_alpha")]
    pub alpha: f32,
    /// Metadata filter: key-value pairs that must match (AND logic)
    #[serde(default)]
    pub filter: Option<std::collections::HashMap<String, String>>,
}

fn default_top_k() -> usize {
    5
}

fn default_alpha() -> f32 {
    0.5
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
    #[serde(default)]
    pub hybrid: bool,
    #[serde(default = "default_alpha")]
    pub alpha: f32,
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

#[derive(Serialize)]
pub struct ReadyResponse {
    pub status: String,
    pub checks: ReadyChecks,
}

#[derive(Serialize)]
pub struct ReadyChecks {
    pub database: bool,
}

// --- Handlers ---

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

async fn readiness(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let db_ok = state.index.count().await.is_ok();
    let status = if db_ok { "ready" } else { "not_ready" };
    let code = if db_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    (
        code,
        Json(ReadyResponse {
            status: status.to_string(),
            checks: ReadyChecks { database: db_ok },
        }),
    )
}

async fn stats(State(state): State<Arc<AppState>>, headers: HeaderMap) -> impl IntoResponse {
    // Require auth unless public_stats is enabled (#6)
    if !state.config.public_stats && !check_auth(&headers, &state.config) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Unauthorized"})),
        )
            .into_response();
    }

    match state.index.count().await {
        Ok(count) => Json(StatsResponse {
            documents: count,
            status: "ok".to_string(),
        })
        .into_response(),
        Err(e) => {
            error!("Stats failed: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            )
                .into_response()
        }
    }
}

async fn query_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(mut req): Json<QueryRequest>,
) -> impl IntoResponse {
    state.metrics.requests_total.fetch_add(1, Ordering::Relaxed);
    state.metrics.queries_total.fetch_add(1, Ordering::Relaxed);

    // Sanitize input
    req.query = sanitize_input(&req.query);

    if !check_auth(&headers, &state.config) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Unauthorized"})),
        )
            .into_response();
    }

    // Validate query length (#3)
    if req.query.len() > state.config.max_query_length {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": format!("Query too long (max {} characters)", state.config.max_query_length)
            })),
        )
            .into_response();
    }

    if req.query.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Query must not be empty"})),
        )
            .into_response();
    }

    let top_k = req.top_k.clamp(1, 1000);

    // Build metadata filter if provided
    let has_filter = req.filter.as_ref().is_some_and(|f| !f.is_empty());

    let result = if req.hybrid {
        state.index.query_hybrid(&req.query, top_k, req.alpha).await
    } else if has_filter {
        let mut mf = MetadataFilter::new();
        for (k, v) in req.filter.unwrap_or_default() {
            mf = mf.with(k, v);
        }
        state.index.query_filtered(&req.query, top_k, &mf).await
    } else {
        state.index.query(&req.query, top_k).await
    };

    match result {
        Ok(results) => {
            let count = results.len();
            let items: Vec<ResultItem> = results.into_iter().map(Into::into).collect();
            Json(QueryResponse {
                results: items,
                count,
            })
            .into_response()
        }
        Err(e) => {
            error!("Query failed: {e}");
            state.metrics.errors_total.fetch_add(1, Ordering::Relaxed);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            )
                .into_response()
        }
    }
}

async fn prompt_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(mut req): Json<PromptRequest>,
) -> impl IntoResponse {
    state.metrics.requests_total.fetch_add(1, Ordering::Relaxed);
    state.metrics.queries_total.fetch_add(1, Ordering::Relaxed);

    // Sanitize input
    req.query = sanitize_input(&req.query);

    if !check_auth(&headers, &state.config) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Unauthorized"})),
        )
            .into_response();
    }

    // Validate query length (#3)
    if req.query.len() > state.config.max_query_length {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": format!("Query too long (max {} characters)", state.config.max_query_length)
            })),
        )
            .into_response();
    }

    if req.query.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Query must not be empty"})),
        )
            .into_response();
    }

    let top_k = req.top_k.clamp(1, 1000);

    let result = if req.hybrid {
        state.index.query_hybrid(&req.query, top_k, req.alpha).await
    } else {
        state.index.query(&req.query, top_k).await
    };

    match result {
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
                use std::fmt::Write;
                let mut p = format!("Query: {}\n\nContext:\n", req.query);
                for (i, r) in results.iter().enumerate() {
                    let src = r.chunk.metadata.get("source").unwrap_or(&r.chunk.doc_id);
                    let _ = write!(p, "\n[{}] Source: {}\n{}\n", i + 1, src, r.chunk.text);
                }
                p.push_str("\n---\nAnswer the query using the provided context.");
                p
            };

            Json(PromptResponse { prompt, sources }).into_response()
        }
        Err(e) => {
            error!("Prompt failed: {e}");
            state.metrics.errors_total.fetch_add(1, Ordering::Relaxed);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            )
                .into_response()
        }
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
        Ok(()) => Json(IndexResponse {
            indexed: count,
            message: format!("Indexed {count} documents"),
        })
        .into_response(),
        Err(e) => {
            error!("Index failed: {e}");
            state.metrics.errors_total.fetch_add(1, Ordering::Relaxed);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            )
                .into_response()
        }
    }
}

async fn delete_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    AxumPath(doc_id): AxumPath<String>,
) -> impl IntoResponse {
    state.metrics.requests_total.fetch_add(1, Ordering::Relaxed);

    if !check_auth(&headers, &state.config) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Unauthorized"})),
        )
            .into_response();
    }

    if doc_id.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Document ID must not be empty"})),
        )
            .into_response();
    }

    match state.index.delete(&doc_id).await {
        Ok(()) => Json(serde_json::json!({
            "deleted": doc_id,
            "message": format!("Deleted document {doc_id}")
        }))
        .into_response(),
        Err(e) => {
            error!("Delete failed: {e}");
            state.metrics.errors_total.fetch_add(1, Ordering::Relaxed);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            )
                .into_response()
        }
    }
}

#[allow(clippy::too_many_lines)]
async fn openapi() -> impl IntoResponse {
    let schema = serde_json::json!({
        "openapi": "3.0.3",
        "info": {
            "title": "RavenRustRAG API",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "Local-first RAG engine. Retrieval-Augmented Generation with vector + BM25 hybrid search."
        },
        "servers": [
            { "url": "http://localhost:8484", "description": "Local development" }
        ],
        "components": {
            "securitySchemes": {
                "bearerAuth": {
                    "type": "http",
                    "scheme": "bearer",
                    "description": "API key via Authorization: Bearer <token>. Only required when RAVEN_API_KEY is set."
                }
            },
            "schemas": {
                "Error": {
                    "type": "object",
                    "properties": {
                        "error": { "type": "string" }
                    }
                },
                "SearchResult": {
                    "type": "object",
                    "properties": {
                        "text": { "type": "string" },
                        "score": { "type": "number" },
                        "distance": { "type": "number" },
                        "doc_id": { "type": "string" },
                        "metadata": { "type": "object", "additionalProperties": { "type": "string" } }
                    }
                }
            }
        },
        "paths": {
            "/health": {
                "get": {
                    "summary": "Liveness probe",
                    "description": "Always returns 200. Use as Kubernetes liveness probe.",
                    "responses": {
                        "200": {
                            "description": "Server is alive",
                            "content": { "application/json": { "schema": {
                                "type": "object",
                                "properties": {
                                    "status": { "type": "string" },
                                    "version": { "type": "string" }
                                }
                            }}}
                        }
                    }
                }
            },
            "/ready": {
                "get": {
                    "summary": "Readiness probe",
                    "description": "Checks database connectivity. Use as Kubernetes readiness probe.",
                    "responses": {
                        "200": {
                            "description": "Service is ready",
                            "content": { "application/json": { "schema": {
                                "type": "object",
                                "properties": {
                                    "status": { "type": "string" },
                                    "checks": { "type": "object", "properties": { "database": { "type": "boolean" } } }
                                }
                            }}}
                        },
                        "503": {
                            "description": "Service not ready",
                            "content": { "application/json": { "schema": {
                                "type": "object",
                                "properties": {
                                    "status": { "type": "string" },
                                    "checks": { "type": "object", "properties": { "database": { "type": "boolean" } } }
                                }
                            }}}
                        }
                    }
                }
            },
            "/stats": {
                "get": {
                    "summary": "Index statistics",
                    "security": [{ "bearerAuth": [] }],
                    "responses": {
                        "200": {
                            "description": "Stats",
                            "content": { "application/json": { "schema": {
                                "type": "object",
                                "properties": {
                                    "documents": { "type": "integer" },
                                    "status": { "type": "string" }
                                }
                            }}}
                        },
                        "401": { "description": "Unauthorized", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Error" } } } }
                    }
                }
            },
            "/query": {
                "post": {
                    "summary": "Search documents",
                    "security": [{ "bearerAuth": [] }],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["query"],
                                    "properties": {
                                        "query": { "type": "string" },
                                        "top_k": { "type": "integer", "default": 5 },
                                        "hybrid": { "type": "boolean", "default": false, "description": "Use hybrid BM25+vector search with RRF" },
                                        "alpha": { "type": "number", "default": 0.5, "description": "Blend factor: 1.0=pure vector, 0.0=pure BM25" }
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Search results",
                            "content": { "application/json": { "schema": {
                                "type": "object",
                                "properties": {
                                    "results": { "type": "array", "items": { "$ref": "#/components/schemas/SearchResult" } },
                                    "count": { "type": "integer" }
                                }
                            }}}
                        },
                        "400": { "description": "Invalid request", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Error" } } } },
                        "401": { "description": "Unauthorized", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Error" } } } },
                        "429": { "description": "Rate limit exceeded", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Error" } } } }
                    }
                }
            },
            "/prompt": {
                "post": {
                    "summary": "LLM-ready prompt with context",
                    "security": [{ "bearerAuth": [] }],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["query"],
                                    "properties": {
                                        "query": { "type": "string" },
                                        "top_k": { "type": "integer", "default": 5 },
                                        "template": { "type": "string", "description": "Custom prompt template with {context}, {query}, {sources} placeholders" },
                                        "hybrid": { "type": "boolean", "default": false },
                                        "alpha": { "type": "number", "default": 0.5 }
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Formatted prompt",
                            "content": { "application/json": { "schema": {
                                "type": "object",
                                "properties": {
                                    "prompt": { "type": "string" },
                                    "sources": { "type": "array", "items": { "type": "string" } }
                                }
                            }}}
                        },
                        "400": { "description": "Invalid request", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Error" } } } },
                        "401": { "description": "Unauthorized", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Error" } } } }
                    }
                }
            },
            "/index": {
                "post": {
                    "summary": "Add documents to the index",
                    "security": [{ "bearerAuth": [] }],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["documents"],
                                    "properties": {
                                        "documents": {
                                            "type": "array",
                                            "items": {
                                                "type": "object",
                                                "required": ["text"],
                                                "properties": {
                                                    "text": { "type": "string" },
                                                    "metadata": { "type": "object", "additionalProperties": { "type": "string" } },
                                                    "id": { "type": "string" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Indexed count",
                            "content": { "application/json": { "schema": {
                                "type": "object",
                                "properties": {
                                    "indexed": { "type": "integer" },
                                    "message": { "type": "string" }
                                }
                            }}}
                        },
                        "401": { "description": "Unauthorized", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Error" } } } }
                    }
                }
            },
            "/documents/{doc_id}": {
                "delete": {
                    "summary": "Delete a document and its chunks",
                    "security": [{ "bearerAuth": [] }],
                    "parameters": [{
                        "name": "doc_id",
                        "in": "path",
                        "required": true,
                        "schema": { "type": "string" }
                    }],
                    "responses": {
                        "200": {
                            "description": "Document deleted",
                            "content": { "application/json": { "schema": {
                                "type": "object",
                                "properties": {
                                    "deleted": { "type": "string" },
                                    "message": { "type": "string" }
                                }
                            }}}
                        },
                        "401": { "description": "Unauthorized", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Error" } } } }
                    }
                }
            },
            "/collections": {
                "get": {
                    "summary": "List collections",
                    "responses": {
                        "200": {
                            "description": "Collection list",
                            "content": { "application/json": { "schema": {
                                "type": "object",
                                "properties": {
                                    "collections": { "type": "array", "items": {
                                        "type": "object",
                                        "properties": {
                                            "name": { "type": "string" },
                                            "chunks": { "type": "integer" },
                                            "embedder": { "type": "string" }
                                        }
                                    }}
                                }
                            }}}
                        }
                    }
                }
            },
            "/metrics": {
                "get": {
                    "summary": "Server metrics",
                    "security": [{ "bearerAuth": [] }],
                    "responses": {
                        "200": {
                            "description": "Metrics",
                            "content": { "application/json": { "schema": {
                                "type": "object",
                                "properties": {
                                    "requests_total": { "type": "integer" },
                                    "queries_total": { "type": "integer" },
                                    "index_requests_total": { "type": "integer" },
                                    "errors_total": { "type": "integer" },
                                    "uptime_seconds": { "type": "integer" },
                                    "chunks_total": { "type": "integer" },
                                    "version": { "type": "string" }
                                }
                            }}}
                        },
                        "401": { "description": "Unauthorized", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Error" } } } }
                    }
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
        .and_then(|v| v.strip_prefix("Bearer "))
        .is_some_and(|token| token.as_bytes().ct_eq(expected_key.as_bytes()).into())
}

// --- Server builder ---

fn build_cors_layer(config: &ServerConfig) -> CorsLayer {
    if config.cors_origins.is_empty() {
        // Default: only allow localhost origins (#1)
        CorsLayer::new()
            .allow_origin([
                "http://localhost"
                    .parse::<HeaderValue>()
                    .expect("valid header"),
                "http://127.0.0.1"
                    .parse::<HeaderValue>()
                    .expect("valid header"),
                "http://localhost:8484"
                    .parse::<HeaderValue>()
                    .expect("valid header"),
                "http://127.0.0.1:8484"
                    .parse::<HeaderValue>()
                    .expect("valid header"),
            ])
            .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
            .allow_headers([
                "content-type".parse().expect("valid header"),
                "authorization".parse().expect("valid header"),
            ])
    } else {
        let origins: Vec<HeaderValue> = config
            .cors_origins
            .iter()
            .filter_map(|o| o.parse().ok())
            .collect();
        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
            .allow_headers([
                "content-type".parse().expect("valid header"),
                "authorization".parse().expect("valid header"),
            ])
    }
}

pub fn build_router(state: Arc<AppState>) -> Router {
    let cors = build_cors_layer(&state.config);

    // Request timeout (#5)
    let timeout = Duration::from_secs(state.config.request_timeout_secs);

    Router::new()
        .route("/health", get(health))
        .route("/ready", get(readiness))
        .route("/stats", get(stats))
        .route("/collections", get(collections_handler))
        .route("/metrics", get(metrics_handler))
        .route("/query", post(query_handler))
        .route("/prompt", post(prompt_handler))
        .route("/index", post(index_handler))
        .route("/documents/:doc_id", delete(delete_handler))
        .route("/openapi.json", get(openapi))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            rate_limit_middleware,
        ))
        .layer(cors)
        .layer(axum::extract::DefaultBodyLimit::max(10 * 1024 * 1024)) // 10MB
        .layer(
            ServiceBuilder::new()
                .layer(axum::error_handling::HandleErrorLayer::new(
                    |_err: tower::BoxError| async move { StatusCode::REQUEST_TIMEOUT },
                ))
                .layer(tower::timeout::TimeoutLayer::new(timeout)),
        )
        .with_state(state)
}

async fn collections_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let count = state.index.count().await.unwrap_or(0);
    Json(serde_json::json!({
        "collections": [{
            "name": "default",
            "chunks": count,
            "embedder": "configured"
        }]
    }))
}

async fn metrics_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // Require auth unless public_stats is enabled (#6)
    if !state.config.public_stats && !check_auth(&headers, &state.config) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Unauthorized"})),
        )
            .into_response();
    }

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
    .into_response()
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
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
    info!("Received shutdown signal, shutting down...");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use http_body_util::BodyExt;
    use hyper::Request;
    use raven_embed::DummyEmbedder;
    use raven_search::DocumentIndex;
    use raven_store::MemoryStore;
    use tower::ServiceExt;

    fn test_state(api_key: Option<&str>) -> Arc<AppState> {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(DummyEmbedder::new(3));
        let index = DocumentIndex::new(store, embedder);
        let config = ServerConfig {
            api_key: api_key.map(String::from),
            ..ServerConfig::default()
        };
        let splitter = TextSplitter::new(200, 20);
        Arc::new(AppState::new(index, config, splitter))
    }

    async fn response_json(response: axum::response::Response) -> serde_json::Value {
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn test_health() {
        let state = test_state(None);
        let app = build_router(state);

        let req = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let json = response_json(resp).await;
        assert_eq!(json["status"], "ok");
        assert!(json["version"].is_string());
    }

    #[tokio::test]
    async fn test_ready() {
        let state = test_state(None);
        let app = build_router(state);

        let req = Request::builder()
            .uri("/ready")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let json = response_json(resp).await;
        assert_eq!(json["status"], "ready");
        assert_eq!(json["checks"]["database"], true);
    }

    #[tokio::test]
    async fn test_stats_no_auth_required() {
        let state = test_state(None);
        let app = build_router(state);

        let req = Request::builder()
            .uri("/stats")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let json = response_json(resp).await;
        assert_eq!(json["documents"], 0);
    }

    #[tokio::test]
    async fn test_stats_requires_auth() {
        let state = test_state(Some("secret-key"));
        let app = build_router(state);

        // Without auth
        let req = Request::builder()
            .uri("/stats")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_stats_with_valid_auth() {
        let state = test_state(Some("secret-key"));
        let app = build_router(state);

        let req = Request::builder()
            .uri("/stats")
            .header("authorization", "Bearer secret-key")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_stats_with_wrong_auth() {
        let state = test_state(Some("secret-key"));
        let app = build_router(state);

        let req = Request::builder()
            .uri("/stats")
            .header("authorization", "Bearer wrong-key")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_query_empty_rejected() {
        let state = test_state(None);
        let app = build_router(state);

        let req = Request::builder()
            .method("POST")
            .uri("/query")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"query": ""}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_query_too_long_rejected() {
        let state = test_state(None);
        let app = build_router(state);

        let long_query = "a".repeat(10_001);
        let body = serde_json::json!({"query": long_query}).to_string();

        let req = Request::builder()
            .method("POST")
            .uri("/query")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_query_success() {
        let state = test_state(None);
        let app = build_router(state);

        let req = Request::builder()
            .method("POST")
            .uri("/query")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"query": "test", "top_k": 5}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let json = response_json(resp).await;
        assert_eq!(json["count"], 0);
        assert!(json["results"].is_array());
    }

    #[tokio::test]
    async fn test_index_and_query() {
        let state = test_state(None);

        // Index a document
        let app = build_router(state.clone());
        let body = serde_json::json!({
            "documents": [{"text": "Rust is a systems programming language."}]
        });
        let req = Request::builder()
            .method("POST")
            .uri("/index")
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let json = response_json(resp).await;
        assert_eq!(json["indexed"], 1);

        // Query for it
        let app = build_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/query")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"query": "Rust programming"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let json = response_json(resp).await;
        assert!(json["count"].as_u64().unwrap() > 0);
    }

    #[tokio::test]
    async fn test_prompt_endpoint() {
        let state = test_state(None);
        let app = build_router(state);

        let req = Request::builder()
            .method("POST")
            .uri("/prompt")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"query": "What is RAG?"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let json = response_json(resp).await;
        assert!(json["prompt"].is_string());
        assert!(json["sources"].is_array());
    }

    #[tokio::test]
    async fn test_index_empty_documents() {
        let state = test_state(None);
        let app = build_router(state);

        let req = Request::builder()
            .method("POST")
            .uri("/index")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"documents": []}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let json = response_json(resp).await;
        assert_eq!(json["indexed"], 0);
    }

    #[tokio::test]
    async fn test_openapi_schema() {
        let state = test_state(None);
        let app = build_router(state);

        let req = Request::builder()
            .uri("/openapi.json")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let json = response_json(resp).await;
        assert_eq!(json["openapi"], "3.0.3");
        assert!(json["paths"]["/query"].is_object());
        assert!(json["paths"]["/documents/{doc_id}"].is_object());
        assert!(json["components"]["securitySchemes"]["bearerAuth"].is_object());
    }

    #[tokio::test]
    async fn test_collections_endpoint() {
        let state = test_state(None);
        let app = build_router(state);

        let req = Request::builder()
            .uri("/collections")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let json = response_json(resp).await;
        assert!(json["collections"].is_array());
    }

    #[tokio::test]
    async fn test_metrics_endpoint() {
        let state = test_state(None);
        let app = build_router(state);

        let req = Request::builder()
            .uri("/metrics")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let json = response_json(resp).await;
        assert!(json["uptime_seconds"].is_number());
        assert!(json["version"].is_string());
    }

    #[tokio::test]
    async fn test_delete_endpoint() {
        let state = test_state(None);

        // Index a document first
        let app = build_router(state.clone());
        let body = serde_json::json!({
            "documents": [{"text": "Test document", "id": "test-doc-1"}]
        });
        let req = Request::builder()
            .method("POST")
            .uri("/index")
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Delete it
        let app = build_router(state.clone());
        let req = Request::builder()
            .method("DELETE")
            .uri("/documents/test-doc-1")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let json = response_json(resp).await;
        assert_eq!(json["deleted"], "test-doc-1");
    }

    #[tokio::test]
    async fn test_delete_requires_auth() {
        let state = test_state(Some("secret"));
        let app = build_router(state);

        let req = Request::builder()
            .method("DELETE")
            .uri("/documents/some-doc")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_hybrid_query() {
        let state = test_state(None);

        // Index documents
        let app = build_router(state.clone());
        let body = serde_json::json!({
            "documents": [
                {"text": "Rust is a systems programming language"},
                {"text": "Python is used for machine learning"}
            ]
        });
        let req = Request::builder()
            .method("POST")
            .uri("/index")
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Hybrid query
        let app = build_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/query")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{"query": "Rust programming", "hybrid": true, "alpha": 0.5}"#,
            ))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let json = response_json(resp).await;
        assert!(json["count"].as_u64().unwrap() > 0);
    }
}

// =============================================================================
// OpenTelemetry integration (optional, behind "otel" feature)
// =============================================================================

#[cfg(feature = "otel")]
pub mod telemetry {
    //! OpenTelemetry tracing export.
    //!
    //! When enabled, exports spans to an OTLP endpoint (default: `http://localhost:4317`).
    //! Set `OTEL_EXPORTER_OTLP_ENDPOINT` to override.

    use opentelemetry::trace::TracerProvider;
    use opentelemetry_sdk::runtime::Tokio;
    use tracing_opentelemetry::OpenTelemetryLayer;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::EnvFilter;

    /// Initialize tracing with OpenTelemetry OTLP exporter.
    ///
    /// Call this once at application startup, before creating the server.
    /// Returns a guard that shuts down the tracer on drop.
    pub fn init_telemetry(service_name: &str) -> OtelGuard {
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .build()
            .expect("Failed to create OTLP exporter");

        let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
            .with_batch_exporter(exporter, Tokio)
            .build();

        let tracer = provider.tracer(service_name.to_string());

        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(tracing_subscriber::fmt::layer())
            .with(OpenTelemetryLayer::new(tracer))
            .init();

        OtelGuard { provider }
    }

    /// Guard that shuts down the OpenTelemetry tracer provider on drop.
    pub struct OtelGuard {
        provider: opentelemetry_sdk::trace::SdkTracerProvider,
    }

    impl Drop for OtelGuard {
        fn drop(&mut self) {
            if let Err(e) = self.provider.shutdown() {
                eprintln!("Error shutting down OTel provider: {e:?}");
            }
        }
    }
}
