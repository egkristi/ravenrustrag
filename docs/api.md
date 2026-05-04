# HTTP API

RavenRustRAG includes an HTTP API server built with Axum.

## Starting the Server

```bash
raven serve --port 8484
```

With authentication:

```bash
RAVEN_API_KEY=my-secret-key raven serve
```

## Authentication

When `RAVEN_API_KEY` is set, protected endpoints require a Bearer token:

```
Authorization: Bearer my-secret-key
```

Endpoints without auth requirement: `/health`, `/ready`, `/stats`, `/openapi.json`, `/metrics`

## Endpoints

### `GET /health`

Health check. Always returns 200.

```json
{"status": "ok"}
```

### `GET /ready`

Readiness check. Returns 200 when the server is ready to accept queries.

### `GET /stats`

Index statistics.

```json
{
  "total_chunks": 1523,
  "total_documents": 42,
  "database_size_bytes": 8388608
}
```

### `GET /metrics`

Prometheus-format metrics (request counts, latencies).

### `GET /openapi.json`

OpenAPI 3.0 schema for the API.

### `POST /query`

Search documents. **Requires auth** (when configured).

Request:
```json
{
  "query": "how does authentication work",
  "top_k": 5,
  "hybrid": false,
  "alpha": 0.5,
  "filter": {
    "source": "docs/auth.md"
  }
}
```

Response:
```json
{
  "results": [
    {
      "text": "Authentication is handled via Bearer tokens...",
      "score": 0.892,
      "metadata": {
        "source": "docs/auth.md",
        "doc_id": "abc123"
      }
    }
  ],
  "query_time_ms": 12
}
```

### `POST /prompt`

Search and format as an LLM prompt. **Requires auth**.

Request:
```json
{
  "query": "explain the build process",
  "top_k": 3
}
```

Response:
```json
{
  "prompt": "Use the following context to answer the question.\n\nContext:\n[1] (score: 0.89) ...\n\nQuestion: explain the build process",
  "sources": ["docs/build.md"],
  "query_time_ms": 15
}
```

### `POST /index`

Add documents to the index. **Requires auth**.

Request:
```json
{
  "documents": [
    {
      "content": "Document text content here",
      "metadata": {
        "source": "manual-entry",
        "title": "My Doc"
      }
    }
  ]
}
```

Response:
```json
{
  "indexed": 1,
  "chunks": 3
}
```

### `DELETE /documents`

Delete documents by source path. **Requires auth**.

Request:
```json
{
  "source": "docs/old-file.md"
}
```

### `GET /collections`

List available collections (when multi-collection is enabled).

## CORS

The server includes permissive CORS headers by default, allowing requests from any origin. This is suitable for development; in production, configure a reverse proxy with stricter policies.

## Rate Limiting

The server applies basic rate limiting per IP address to prevent abuse. Default: 100 requests per minute.
