# MCP Server

RavenRustRAG includes an MCP (Model Context Protocol) server for integration with AI assistants like Claude, Copilot, Cursor, and Aider.

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

---

## Integrations

### 1. Claude Desktop

Add to your Claude Desktop config file:

- **macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows:** `%APPDATA%\Claude\claude_desktop_config.json`
- **Linux:** `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "ravenrag": {
      "command": "ravenrag",
      "args": ["mcp", "--db", "/path/to/raven.db"]
    }
  }
}
```

To restrict tools (read-only):

```json
{
  "mcpServers": {
    "ravenrag": {
      "command": "ravenrag",
      "args": ["mcp", "--db", "/path/to/raven.db", "--filter", "search,get_prompt"]
    }
  }
}
```

### 2. Claude Code

Claude Code discovers MCP servers from the same config as Claude Desktop, or you can add one directly via CLI:

```bash
claude mcp add ravenrag -- ravenrag mcp --db /path/to/raven.db
```

To verify it's registered:

```bash
claude mcp list
```

### 3. Cursor

Add to your Cursor MCP settings:

- **macOS:** `~/.cursor/mcp.json`
- **Windows:** `%USERPROFILE%\.cursor\mcp.json`
- **Linux:** `~/.cursor/mcp.json`

```json
{
  "mcpServers": {
    "ravenrag": {
      "command": "ravenrag",
      "args": ["mcp", "--db", "/path/to/raven.db"]
    }
  }
}
```

Or per-project in `.cursor/mcp.json` at the workspace root:

```json
{
  "mcpServers": {
    "ravenrag": {
      "command": "ravenrag",
      "args": ["mcp", "--db", "./raven.db"]
    }
  }
}
```

### 4. Aider

Aider supports MCP servers via its `--mcp` flag:

```bash
aider --mcp "ravenrag mcp --db ./raven.db"
```

Or add to your `.aider.conf.yml`:

```yaml
mcp-servers:
  - command: ravenrag
    args: ["mcp", "--db", "./raven.db"]
```

### 5. VS Code (GitHub Copilot)

Add to `.vscode/mcp.json` in your workspace root:

```json
{
  "servers": {
    "ravenrag": {
      "command": "ravenrag",
      "args": ["mcp", "--db", "./raven.db"]
    }
  }
}
```

Or configure globally in VS Code settings (`settings.json`):

```json
{
  "github.copilot.chat.mcp.servers": {
    "ravenrag": {
      "command": "ravenrag",
      "args": ["mcp", "--db", "/path/to/raven.db"]
    }
  }
}
```

After adding, Copilot Chat will automatically discover the tools. You can invoke them with `@ravenrag` or the agent will use them when relevant.

### Docker (any client)

If you installed via Docker, use this command format:

```json
{
  "mcpServers": {
    "ravenrag": {
      "command": "docker",
      "args": [
        "run", "--rm", "-i",
        "-v", "raven-data:/data",
        "ghcr.io/egkristi/ravenrustrag:latest",
        "mcp", "--db", "/data/raven.db"
      ]
    }
  }
}
```

---

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
