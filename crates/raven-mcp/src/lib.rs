use raven_core::Document;
use raven_search::DocumentIndex;
use raven_split::TextSplitter;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::info;

const MCP_PROTOCOL_VERSION: &str = "2024-11-05";

// --- JSON-RPC types ---

#[derive(Deserialize)]
struct JsonRpcRequest {
    #[allow(dead_code)]
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
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

    async fn handle_request(&self, req: JsonRpcRequest) -> Option<JsonRpcResponse> {
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
                -32601,
                format!("Method not found: {}", req.method),
            )),
        }
    }

    async fn handle_tool_call(&self, id: Value, params: Value) -> JsonRpcResponse {
        let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let arguments = params
            .get("arguments")
            .cloned()
            .unwrap_or(Value::Object(Default::default()));

        match tool_name {
            "search" => self.tool_search(id, arguments).await,
            "get_prompt" => self.tool_get_prompt(id, arguments).await,
            "collection_info" => self.tool_collection_info(id).await,
            "index_documents" => self.tool_index_documents(id, arguments).await,
            _ => JsonRpcResponse::error(id, -32602, format!("Unknown tool: {}", tool_name)),
        }
    }

    async fn tool_search(&self, id: Value, args: Value) -> JsonRpcResponse {
        let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
        let top_k = args.get("top_k").and_then(|v| v.as_u64()).unwrap_or(5) as usize;

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
            Err(e) => JsonRpcResponse::error(id, -32000, e.to_string()),
        }
    }

    async fn tool_get_prompt(&self, id: Value, args: Value) -> JsonRpcResponse {
        let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
        let top_k = args.get("top_k").and_then(|v| v.as_u64()).unwrap_or(3) as usize;

        match self.index.query_for_prompt(query, top_k).await {
            Ok(prompt) => JsonRpcResponse::success(
                id,
                serde_json::json!({
                    "content": [{ "type": "text", "text": prompt }]
                }),
            ),
            Err(e) => JsonRpcResponse::error(id, -32000, e.to_string()),
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
            Err(e) => JsonRpcResponse::error(id, -32000, e.to_string()),
        }
    }

    async fn tool_index_documents(&self, id: Value, args: Value) -> JsonRpcResponse {
        let docs_val = args
            .get("documents")
            .cloned()
            .unwrap_or(Value::Array(vec![]));
        let doc_arr = docs_val.as_array().cloned().unwrap_or_default();

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
            Ok(_) => JsonRpcResponse::success(
                id,
                serde_json::json!({
                    "content": [{ "type": "text", "text": format!("Indexed {} documents", count) }]
                }),
            ),
            Err(e) => JsonRpcResponse::error(id, -32000, e.to_string()),
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
                    let err_resp =
                        JsonRpcResponse::error(Value::Null, -32700, format!("Parse error: {}", e));
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
