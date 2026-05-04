//! MCP (Model Context Protocol) server for RavenRustRAG.
//!
//! Provides a stdio JSON-RPC server for AI assistants to search and index documents.

use raven_core::Document;
use raven_search::DocumentIndex;
use raven_split::TextSplitter;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::info;

const MCP_PROTOCOL_VERSION: &str = "2024-11-05";

// JSON-RPC standard error codes
const JSONRPC_PARSE_ERROR: i32 = -32700;
const JSONRPC_INVALID_PARAMS: i32 = -32602;
const JSONRPC_METHOD_NOT_FOUND: i32 = -32601;
const JSONRPC_INTERNAL_ERROR: i32 = -32603;

// MCP-specific error codes
const MCP_TOOL_NOT_FOUND: i32 = -32002;

/// Strip control characters from input text (U+0000-U+001F except \n and \t)
fn sanitize_input(s: &str) -> String {
    s.chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .collect()
}

// --- JSON-RPC types ---

#[derive(Deserialize)]
pub struct JsonRpcRequest {
    #[allow(dead_code)]
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

#[derive(Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

impl JsonRpcResponse {
    fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    fn error(id: Value, code: i32, message: String) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError { code, message }),
        }
    }
}

// --- MCP tool definitions ---

fn tool_definitions() -> Value {
    serde_json::json!({
        "tools": [
            {
                "name": "search",
                "description": "Search the document index for relevant content",
                "inputSchema": {
                    "type": "object",
                    "required": ["query"],
                    "properties": {
                        "query": { "type": "string", "description": "Search query text" },
                        "top_k": { "type": "integer", "description": "Number of results (default: 5)", "default": 5 }
                    }
                }
            },
            {
                "name": "get_prompt",
                "description": "Search and format an LLM-ready prompt with citations",
                "inputSchema": {
                    "type": "object",
                    "required": ["query"],
                    "properties": {
                        "query": { "type": "string", "description": "Query to answer" },
                        "top_k": { "type": "integer", "description": "Number of context chunks", "default": 3 }
                    }
                }
            },
            {
                "name": "collection_info",
                "description": "Get index statistics (document count)",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "index_documents",
                "description": "Add documents to the index",
                "inputSchema": {
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
                                    "metadata": { "type": "object" }
                                }
                            }
                        }
                    }
                }
            }
        ]
    })
}

// --- MCP state ---

pub struct McpServer {
    index: Arc<DocumentIndex>,
    splitter: TextSplitter,
}

impl McpServer {
    pub fn new(index: Arc<DocumentIndex>, splitter: TextSplitter) -> Self {
        Self { index, splitter }
    }

    /// Handle a single JSON-RPC request. Returns None for notifications.
    pub async fn handle_request(&self, req: JsonRpcRequest) -> Option<JsonRpcResponse> {
        let id = req.id.clone().unwrap_or(Value::Null);

        match req.method.as_str() {
            "initialize" => Some(JsonRpcResponse::success(
                id,
                serde_json::json!({
                    "protocolVersion": MCP_PROTOCOL_VERSION,
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "ravenrustrag",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                }),
            )),
            "notifications/initialized" => None, // No response for notifications
            "tools/list" => Some(JsonRpcResponse::success(id, tool_definitions())),
            "tools/call" => Some(self.handle_tool_call(id, req.params).await),
            _ => Some(JsonRpcResponse::error(
                id,
                JSONRPC_METHOD_NOT_FOUND,
                format!("Method not found: {}", req.method),
            )),
        }
    }

    async fn handle_tool_call(&self, id: Value, params: Value) -> JsonRpcResponse {
        let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let arguments = params
            .get("arguments")
            .cloned()
            .unwrap_or(Value::Object(serde_json::Map::default()));

        match tool_name {
            "search" => self.tool_search(id, arguments).await,
            "get_prompt" => self.tool_get_prompt(id, arguments).await,
            "collection_info" => self.tool_collection_info(id).await,
            "index_documents" => self.tool_index_documents(id, arguments).await,
            _ => {
                JsonRpcResponse::error(id, MCP_TOOL_NOT_FOUND, format!("Unknown tool: {tool_name}"))
            }
        }
    }

    async fn tool_search(&self, id: Value, args: Value) -> JsonRpcResponse {
        let raw_query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
        let query = sanitize_input(raw_query);
        let query = query.as_str();
        let top_k = args
            .get("top_k")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(5) as usize;

        // Validate query
        if query.is_empty() {
            return JsonRpcResponse::error(
                id,
                JSONRPC_INVALID_PARAMS,
                "Query must not be empty".to_string(),
            );
        }
        if query.len() > 10_000 {
            return JsonRpcResponse::error(
                id,
                JSONRPC_INVALID_PARAMS,
                "Query too long (max 10000 characters)".to_string(),
            );
        }
        if top_k == 0 || top_k > 100 {
            return JsonRpcResponse::error(
                id,
                JSONRPC_INVALID_PARAMS,
                "top_k must be between 1 and 100".to_string(),
            );
        }

        match self.index.query(query, top_k).await {
            Ok(results) => {
                let items: Vec<Value> = results
                    .iter()
                    .map(|r| {
                        serde_json::json!({
                            "text": r.chunk.text,
                            "score": r.score,
                            "source": r.chunk.metadata.get("source").unwrap_or(&r.chunk.doc_id),
                        })
                    })
                    .collect();
                JsonRpcResponse::success(
                    id,
                    serde_json::json!({
                        "content": [{ "type": "text", "text": serde_json::to_string_pretty(&items).unwrap_or_default() }]
                    }),
                )
            }
            Err(e) => JsonRpcResponse::error(id, JSONRPC_INTERNAL_ERROR, e.to_string()),
        }
    }

    async fn tool_get_prompt(&self, id: Value, args: Value) -> JsonRpcResponse {
        let raw_query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
        let query = sanitize_input(raw_query);
        let query = query.as_str();
        let top_k = args
            .get("top_k")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(3) as usize;

        // Validate query
        if query.is_empty() {
            return JsonRpcResponse::error(
                id,
                JSONRPC_INVALID_PARAMS,
                "Query must not be empty".to_string(),
            );
        }
        if query.len() > 10_000 {
            return JsonRpcResponse::error(
                id,
                JSONRPC_INVALID_PARAMS,
                "Query too long (max 10000 characters)".to_string(),
            );
        }
        if top_k == 0 || top_k > 100 {
            return JsonRpcResponse::error(
                id,
                JSONRPC_INVALID_PARAMS,
                "top_k must be between 1 and 100".to_string(),
            );
        }

        match self.index.query_for_prompt(query, top_k).await {
            Ok(prompt) => JsonRpcResponse::success(
                id,
                serde_json::json!({
                    "content": [{ "type": "text", "text": prompt }]
                }),
            ),
            Err(e) => JsonRpcResponse::error(id, JSONRPC_INTERNAL_ERROR, e.to_string()),
        }
    }

    async fn tool_collection_info(&self, id: Value) -> JsonRpcResponse {
        match self.index.count().await {
            Ok(count) => JsonRpcResponse::success(
                id,
                serde_json::json!({
                    "content": [{ "type": "text", "text": format!("Index contains {} chunks", count) }]
                }),
            ),
            Err(e) => JsonRpcResponse::error(id, JSONRPC_INTERNAL_ERROR, e.to_string()),
        }
    }

    async fn tool_index_documents(&self, id: Value, args: Value) -> JsonRpcResponse {
        let docs_val = args
            .get("documents")
            .cloned()
            .unwrap_or(Value::Array(vec![]));
        let doc_arr = docs_val.as_array().cloned().unwrap_or_default();

        // Limit batch size (#7)
        if doc_arr.len() > 100 {
            return JsonRpcResponse::error(
                id,
                JSONRPC_INVALID_PARAMS,
                "Too many documents (max 100 per call)".to_string(),
            );
        }

        let docs: Vec<Document> = doc_arr
            .into_iter()
            .filter_map(|v| {
                let text = v.get("text")?.as_str()?.to_string();
                let mut doc = Document::new(text);
                if let Some(meta) = v.get("metadata").and_then(|m| m.as_object()) {
                    for (k, v) in meta {
                        if let Some(s) = v.as_str() {
                            doc = doc.with_metadata(k.clone(), s.to_string());
                        }
                    }
                }
                Some(doc)
            })
            .collect();

        let count = docs.len();
        match self.index.add_documents(docs, &self.splitter).await {
            Ok(()) => JsonRpcResponse::success(
                id,
                serde_json::json!({
                    "content": [{ "type": "text", "text": format!("Indexed {} documents", count) }]
                }),
            ),
            Err(e) => JsonRpcResponse::error(id, JSONRPC_INTERNAL_ERROR, e.to_string()),
        }
    }

    /// Run the MCP server on stdio
    pub async fn run_stdio(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("RavenRustRAG MCP server started (stdio)");
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            let line = line.trim().to_string();
            if line.is_empty() {
                continue;
            }

            let req: JsonRpcRequest = match serde_json::from_str(&line) {
                Ok(r) => r,
                Err(e) => {
                    let err_resp = JsonRpcResponse::error(
                        Value::Null,
                        JSONRPC_PARSE_ERROR,
                        format!("Parse error: {e}"),
                    );
                    let out = serde_json::to_string(&err_resp).unwrap_or_default();
                    stdout.write_all(out.as_bytes()).await?;
                    stdout.write_all(b"\n").await?;
                    stdout.flush().await?;
                    continue;
                }
            };

            if let Some(resp) = self.handle_request(req).await {
                let out = serde_json::to_string(&resp).unwrap_or_default();
                stdout.write_all(out.as_bytes()).await?;
                stdout.write_all(b"\n").await?;
                stdout.flush().await?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use raven_embed::DummyEmbedder;
    use raven_search::DocumentIndex;
    use raven_store::MemoryStore;

    fn test_server() -> McpServer {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(DummyEmbedder::new(3));
        let index = Arc::new(DocumentIndex::new(store, embedder));
        let splitter = TextSplitter::new(200, 20);
        McpServer::new(index, splitter)
    }

    fn make_request(method: &str, params: Value) -> JsonRpcRequest {
        JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(Value::from(1)),
            method: method.to_string(),
            params,
        }
    }

    #[tokio::test]
    async fn test_initialize() {
        let server = test_server();
        let req = make_request("initialize", Value::Object(serde_json::Map::default()));
        let resp = server.handle_request(req).await.unwrap();
        assert!(resp.error.is_none());
        let result = resp.result.unwrap();
        assert_eq!(result["protocolVersion"], MCP_PROTOCOL_VERSION);
        assert_eq!(result["serverInfo"]["name"], "ravenrustrag");
    }

    #[tokio::test]
    async fn test_tools_list() {
        let server = test_server();
        let req = make_request("tools/list", Value::Object(serde_json::Map::default()));
        let resp = server.handle_request(req).await.unwrap();
        assert!(resp.error.is_none());
        let tools = resp.result.unwrap();
        assert!(tools["tools"].is_array());
        assert_eq!(tools["tools"].as_array().unwrap().len(), 4);
    }

    #[tokio::test]
    async fn test_unknown_method() {
        let server = test_server();
        let req = make_request("nonexistent", Value::Object(serde_json::Map::default()));
        let resp = server.handle_request(req).await.unwrap();
        assert!(resp.error.is_some());
        assert_eq!(resp.error.unwrap().code, JSONRPC_METHOD_NOT_FOUND);
    }

    #[tokio::test]
    async fn test_notification_no_response() {
        let server = test_server();
        let req = make_request(
            "notifications/initialized",
            Value::Object(serde_json::Map::default()),
        );
        let resp = server.handle_request(req).await;
        assert!(resp.is_none());
    }

    #[tokio::test]
    async fn test_search_empty_query() {
        let server = test_server();
        let req = make_request(
            "tools/call",
            serde_json::json!({"name": "search", "arguments": {"query": ""}}),
        );
        let resp = server.handle_request(req).await.unwrap();
        assert!(resp.error.is_some());
        assert_eq!(resp.error.unwrap().code, JSONRPC_INVALID_PARAMS);
    }

    #[tokio::test]
    async fn test_search_query_too_long() {
        let server = test_server();
        let long_query = "a".repeat(10_001);
        let req = make_request(
            "tools/call",
            serde_json::json!({"name": "search", "arguments": {"query": long_query}}),
        );
        let resp = server.handle_request(req).await.unwrap();
        assert!(resp.error.is_some());
        assert_eq!(resp.error.unwrap().code, JSONRPC_INVALID_PARAMS);
    }

    #[tokio::test]
    async fn test_search_invalid_top_k() {
        let server = test_server();
        let req = make_request(
            "tools/call",
            serde_json::json!({"name": "search", "arguments": {"query": "test", "top_k": 0}}),
        );
        let resp = server.handle_request(req).await.unwrap();
        assert!(resp.error.is_some());
        assert_eq!(resp.error.unwrap().code, JSONRPC_INVALID_PARAMS);
    }

    #[tokio::test]
    async fn test_search_success() {
        let server = test_server();
        let req = make_request(
            "tools/call",
            serde_json::json!({"name": "search", "arguments": {"query": "test", "top_k": 5}}),
        );
        let resp = server.handle_request(req).await.unwrap();
        assert!(resp.error.is_none());
        let result = resp.result.unwrap();
        assert!(result["content"].is_array());
    }

    #[tokio::test]
    async fn test_collection_info() {
        let server = test_server();
        let req = make_request(
            "tools/call",
            serde_json::json!({"name": "collection_info", "arguments": {}}),
        );
        let resp = server.handle_request(req).await.unwrap();
        assert!(resp.error.is_none());
        let text = resp.result.unwrap()["content"][0]["text"]
            .as_str()
            .unwrap()
            .to_string();
        assert!(text.contains("0 chunks"));
    }

    #[tokio::test]
    async fn test_index_documents() {
        let server = test_server();
        let req = make_request(
            "tools/call",
            serde_json::json!({
                "name": "index_documents",
                "arguments": {
                    "documents": [
                        {"text": "Rust is fast"},
                        {"text": "Python is slow"}
                    ]
                }
            }),
        );
        let resp = server.handle_request(req).await.unwrap();
        assert!(resp.error.is_none());
        let text = resp.result.unwrap()["content"][0]["text"]
            .as_str()
            .unwrap()
            .to_string();
        assert!(text.contains("Indexed 2 documents"));
    }

    #[tokio::test]
    async fn test_index_too_many_documents() {
        let server = test_server();
        let docs: Vec<Value> = (0..101)
            .map(|i| serde_json::json!({"text": format!("doc {i}")}))
            .collect();
        let req = make_request(
            "tools/call",
            serde_json::json!({
                "name": "index_documents",
                "arguments": { "documents": docs }
            }),
        );
        let resp = server.handle_request(req).await.unwrap();
        assert!(resp.error.is_some());
        assert_eq!(resp.error.unwrap().code, JSONRPC_INVALID_PARAMS);
    }

    #[tokio::test]
    async fn test_unknown_tool() {
        let server = test_server();
        let req = make_request(
            "tools/call",
            serde_json::json!({"name": "nonexistent_tool", "arguments": {}}),
        );
        let resp = server.handle_request(req).await.unwrap();
        assert!(resp.error.is_some());
        assert_eq!(resp.error.unwrap().code, MCP_TOOL_NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_prompt() {
        let server = test_server();
        let req = make_request(
            "tools/call",
            serde_json::json!({"name": "get_prompt", "arguments": {"query": "What is RAG?"}}),
        );
        let resp = server.handle_request(req).await.unwrap();
        assert!(resp.error.is_none());
        let text = resp.result.unwrap()["content"][0]["text"]
            .as_str()
            .unwrap()
            .to_string();
        assert!(text.contains("What is RAG?"));
    }
}
