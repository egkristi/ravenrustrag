use axum::body::Body;
use http_body_util::BodyExt;
use hyper::Request;
use raven_core::ServerConfig;
use raven_embed::DummyEmbedder;
use raven_search::DocumentIndex;
use raven_server::{build_router, AppState};
use raven_split::TextSplitter;
use raven_store::MemoryStore;
use std::sync::Arc;
use tower::ServiceExt;

fn test_state(api_key: Option<&str>) -> Arc<AppState> {
    let store = Arc::new(MemoryStore::new());
    let embedder = Arc::new(DummyEmbedder::new(128));
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
    let collected = body.collect().await.expect("collect body");
    let bytes = collected.to_bytes();
    serde_json::from_slice(&bytes).expect("parse json")
}

// ---------------------------------------------------------------------------
// Index → Query round-trip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_index_then_query() {
    let state = test_state(None);
    let app = build_router(state);

    // Index two documents
    let body = serde_json::json!({
        "documents": [
            {"text": "Rust is a systems programming language.", "metadata": {"source": "intro.md"}},
            {"text": "RAG combines search with language models.", "metadata": {"source": "rag.md"}}
        ]
    });
    let req = Request::builder()
        .method("POST")
        .uri("/index")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body).expect("serialize")))
        .expect("build request");
    let resp = app.clone().oneshot(req).await.expect("send");
    assert_eq!(resp.status(), 200);

    // Query
    let query_body = serde_json::json!({
        "query": "What is Rust?",
        "top_k": 2
    });
    let req = Request::builder()
        .method("POST")
        .uri("/query")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_vec(&query_body).expect("serialize"),
        ))
        .expect("build request");
    let resp = app.oneshot(req).await.expect("send");
    assert_eq!(resp.status(), 200);

    let json = response_json(resp).await;
    assert!(json["results"].is_array());
}

// ---------------------------------------------------------------------------
// Auth enforcement
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_auth_required_for_query() {
    let state = test_state(Some("secret-key"));
    let app = build_router(state);

    // Without auth header → 401
    let body = serde_json::json!({"query": "test", "top_k": 3});
    let req = Request::builder()
        .method("POST")
        .uri("/query")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body).expect("serialize")))
        .expect("build request");
    let resp = app.clone().oneshot(req).await.expect("send");
    assert_eq!(resp.status(), 401);

    // With correct auth → 200
    let req = Request::builder()
        .method("POST")
        .uri("/query")
        .header("content-type", "application/json")
        .header("authorization", "Bearer secret-key")
        .body(Body::from(serde_json::to_vec(&body).expect("serialize")))
        .expect("build request");
    let resp = app.clone().oneshot(req).await.expect("send");
    assert_eq!(resp.status(), 200);

    // With wrong auth → 401
    let req = Request::builder()
        .method("POST")
        .uri("/query")
        .header("content-type", "application/json")
        .header("authorization", "Bearer wrong-key")
        .body(Body::from(serde_json::to_vec(&body).expect("serialize")))
        .expect("build request");
    let resp = app.oneshot(req).await.expect("send");
    assert_eq!(resp.status(), 401);
}

// ---------------------------------------------------------------------------
// Health and stats endpoints (no auth)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_health_no_auth_needed() {
    let state = test_state(Some("my-key"));
    let app = build_router(state);

    let req = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .expect("build request");
    let resp = app.oneshot(req).await.expect("send");
    assert_eq!(resp.status(), 200);
}

// ---------------------------------------------------------------------------
// OpenAPI schema
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_openapi_valid_json() {
    let state = test_state(None);
    let app = build_router(state);

    let req = Request::builder()
        .uri("/openapi.json")
        .body(Body::empty())
        .expect("build request");
    let resp = app.oneshot(req).await.expect("send");
    assert_eq!(resp.status(), 200);

    let json = response_json(resp).await;
    assert_eq!(json["openapi"], "3.0.3");
    assert!(json["paths"].is_object());
    assert!(json["info"]["title"].is_string());
}

// ---------------------------------------------------------------------------
// Prompt endpoint
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_prompt_endpoint() {
    let state = test_state(None);
    let app = build_router(state);

    // Index a document first
    let body = serde_json::json!({
        "documents": [
            {"text": "Embeddings map text into vector space for similarity search.", "metadata": {}}
        ]
    });
    let req = Request::builder()
        .method("POST")
        .uri("/index")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body).expect("serialize")))
        .expect("build request");
    let resp = app.clone().oneshot(req).await.expect("send");
    assert_eq!(resp.status(), 200);

    // Prompt
    let prompt_body = serde_json::json!({"query": "What are embeddings?", "top_k": 1});
    let req = Request::builder()
        .method("POST")
        .uri("/prompt")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_vec(&prompt_body).expect("serialize"),
        ))
        .expect("build request");
    let resp = app.oneshot(req).await.expect("send");
    assert_eq!(resp.status(), 200);

    let json = response_json(resp).await;
    assert!(json["prompt"].is_string());
}

// ---------------------------------------------------------------------------
// Metrics endpoint
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_metrics_returns_stats() {
    let state = test_state(None);
    let app = build_router(state);

    let req = Request::builder()
        .uri("/metrics")
        .body(Body::empty())
        .expect("build request");
    let resp = app.oneshot(req).await.expect("send");
    assert_eq!(resp.status(), 200);

    let json = response_json(resp).await;
    assert!(json["requests_total"].is_number());
    assert!(json["uptime_seconds"].is_number());
}

// ---------------------------------------------------------------------------
// Delete endpoint
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_delete_nonexistent_returns_ok() {
    let state = test_state(None);
    let app = build_router(state);

    let req = Request::builder()
        .method("DELETE")
        .uri("/documents/nonexistent-id")
        .body(Body::empty())
        .expect("build request");
    let resp = app.oneshot(req).await.expect("send");
    // Should succeed (idempotent delete)
    assert_eq!(resp.status(), 200);
}

// ---------------------------------------------------------------------------
// Collections endpoint
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_collections_endpoint() {
    let state = test_state(None);
    let app = build_router(state);

    let req = Request::builder()
        .uri("/collections")
        .body(Body::empty())
        .expect("build request");
    let resp = app.oneshot(req).await.expect("send");
    assert_eq!(resp.status(), 200);

    let json = response_json(resp).await;
    assert!(json["collections"].is_array());
}

// ---------------------------------------------------------------------------
// Query validation
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_query_empty_string_rejected() {
    let state = test_state(None);
    let app = build_router(state);

    let body = serde_json::json!({"query": "", "top_k": 3});
    let req = Request::builder()
        .method("POST")
        .uri("/query")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body).expect("serialize")))
        .expect("build request");
    let resp = app.oneshot(req).await.expect("send");
    assert_eq!(resp.status(), 400);
}

// ---------------------------------------------------------------------------
// Index with invalid JSON
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_index_invalid_json() {
    let state = test_state(None);
    let app = build_router(state);

    let req = Request::builder()
        .method("POST")
        .uri("/index")
        .header("content-type", "application/json")
        .body(Body::from("not json"))
        .expect("build request");
    let resp = app.oneshot(req).await.expect("send");
    assert!(
        resp.status() == 400 || resp.status() == 422,
        "expected 400 or 422, got {}",
        resp.status()
    );
}
