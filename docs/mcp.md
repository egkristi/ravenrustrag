# MCP Server

RavenRustRAG includes an MCP (Model Context Protocol) server for integration with AI assistants like Claude Desktop.

## Starting

```bash
raven mcp --db ./raven.db
```

The MCP server communicates over stdio using JSON-RPC 2.0.

## Configuration for Claude Desktop

Add to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "ravenrag": {
      "command": "/path/to/raven",
      "args": ["mcp", "--db", "/path/to/raven.db"]
    }
  }
}
```

## Configuration for VS Code (Copilot)

Add to `.vscode/mcp.json`:

```json
{
  "servers": {
    "ravenrag": {
      "command": "raven",
      "args": ["mcp", "--db", "./raven.db"]
    }
  }
}
```

## Available Tools

### `search`

Query the index with semantic search.

Parameters:
- `query` (string, required): Search query text
- `top_k` (integer, optional): Number of results (default: 5)

### `get_prompt`

Search and format results as an LLM-ready prompt with citations.

Parameters:
- `query` (string, required): Query text
- `top_k` (integer, optional): Number of context chunks (default: 3)

### `collection_info`

Get index statistics (document count, chunk count, database size).

No parameters required.

### `index_documents`

Add documents to the index.

Parameters:
- `documents` (array, required): Array of `{ content, metadata }` objects

## Protocol Details

The MCP server implements:
- `initialize` / `initialized` handshake
- `tools/list` — returns available tools
- `tools/call` — executes a tool
- Proper error responses with JSON-RPC error codes

Input validation:
- Queries are limited to 10,000 characters
- `top_k` must be between 1 and 100
- Batch indexing is limited to 100 documents per call
