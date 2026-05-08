# MCP Registry Listing — RavenRustRAG

This directory contains the configuration for listing RavenRustRAG in the [MCP Registry](https://github.com/modelcontextprotocol/registry).

## Files

| File | Purpose |
|------|---------|
| `server.json` | MCP Registry server manifest |

## Registry Details

- **Server name:** `io.github.egkristi/ravenrustrag`
- **Package type:** Docker/OCI image
- **Registry:** GitHub Container Registry (`ghcr.io`)
- **Image:** `ghcr.io/egkristi/ravenrustrag:latest`

## Tools Exposed

RavenRustRAG exposes the following MCP tools:

- **`search`** — Search the document index for relevant content using vector similarity
- **`get_prompt`** — Search and format an LLM-ready prompt with citations and context
- **`collection_info`** — Get index statistics including document count and embedding model info
- **`index_documents`** — Add documents to the index for later retrieval

## Resources

- **`raven://index/stats`** — Returns a JSON summary of the index

## Prompts

- **`rag_answer`** — RAG-style question answering with citations
- **`summarize_index`** — High-level summary of the indexed content

## Publishing Checklist

1. Ensure Docker image is built and pushed to `ghcr.io/egkristi/ravenrustrag:latest`
2. ✅ Verify `server.json` has correct `$schema` and `name`
3. ✅ Verify Dockerfile has `LABEL io.modelcontextprotocol.server.name="io.github.egkristi/ravenrustrag"`
4. ✅ Submit PR to [modelcontextprotocol/registry](https://github.com/modelcontextprotocol/registry) adding `server.json` to the registry

## Ownership Verification

The MCP Registry verifies ownership by checking the `io.modelcontextprotocol.server.name` Docker label, which must match the server name in `server.json`.

## References

- [MCP Registry Documentation](https://modelcontextprotocol.io/registry)
- [GitHub MCP Registry Blog Post](https://github.blog/ai-and-ml/github-copilot/meet-the-github-mcp-registry-the-fastest-way-to-discover-mcp-servers/)
