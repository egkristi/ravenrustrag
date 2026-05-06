# MCP Server

RavenRustRAG includes an MCP (Model Context Protocol) server for integration with AI assistants like Claude Desktop.

## Starting

```bash
ravenrag mcp --db ./raven.db
```

The MCP server communicates over stdio using JSON-RPC 2.0.

### Filtering Tools

Restrict which tools are exposed using `--filter`:

```bash
# Only expose search and prompt tools (read-only access)
ravenrag mcp --filter search,get_prompt
```

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
- `tools/list` — returns available tools (respects `--filter`)
- `tools/call` — executes a tool
- `resources/list` — lists browseable resources
- `resources/read` — reads a resource by URI
- `prompts/list` — lists available prompt templates
- `prompts/get` — renders a prompt template with arguments
- Proper error responses with JSON-RPC error codes

**MCP Protocol Version:** `2024-11-05`

## Resources

Resources expose index metadata as browseable URIs.

| URI | Description |
|-----|-------------|
| `raven://index/stats` | Index statistics (chunk count, model name) |

## Prompts

Prompt templates that AI assistants can invoke:

### `rag_answer`

Generate an answer using retrieved context.

Arguments:
- `query` (string, required): The question to answer
- `top_k` (string, optional): Number of context chunks (default: "3")

### `summarize_index`

Summarize the contents of the document index.

No arguments required.

## Input Validation

Input validation:
- Queries are limited to 10,000 characters
- `top_k` must be between 1 and 100
- Batch indexing is limited to 100 documents per call
