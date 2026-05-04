# RavenRustRAG — Implementation Plan

> **Status:** v0.1.0 foundation lagt  
> **Target:** v0.1.0-alpha bygbar < 1 uke  
> **Motto:** *Make it work, make it right, make it fast — in that order.*

## Hva som er nytt i RavenRAG v0.7.0 (Python)

Basert på analyse av oppdatert repo (2026-05-04):

| Feature | Status i Python | Plan for Rust |
|---|---|---|
| Knowledge graph retrieval | ✅ Entity extraction + graph traversal | Fase 3 |
| Docker + CI container build | ✅ Dockerfile, GitHub Actions | Fase 2 |
| OpenAPI schema in server | ✅ Innebygget spec | Fase 2 |
| QueryResult.citation | ✅ Metadata-drevet | ✅ I core |
| Eval metrics (MRR, NDCG) | ✅ Innebygget | Fase 3 |
| Context formatting/templates | ✅ Templat-støtte | Fase 2 |
| MCP server | ✅ stdio transport | Fase 2 |
| Watch mode | ✅ Auto-reindex | Fase 2 |
| Export/import JSONL | ✅ Backup/restore | Fase 2 |
| Multi-collection routing | ✅ MultiCollectionRouter | Fase 3 |
| Parent-child retrieval | ✅ Search chunks, return parents | Fase 3 |
| Semantic splitting | ✅ Embedder-drevet | Fase 2 |
| CSV/RTF/PPTX/XLSX loaders | ✅ Plugin-system | Fase 2 |
| Async pipeline API | ✅ aadd(), aquery() | ✅ I core (async/await) |
| FAISS/SQLite alt stores | ✅ Pluggable | ✅ I store |
| Cross-encoder reranking | ✅ Optional | Fase 2 |
| BM25 hybrid search | ✅ rank_bm25 + RRF | Fase 2 |

## 1. Arkitektur

```
┌─────────────────────────────────────────────────────────────┐
│                        RavenRustRAG                          │
├─────────────────────────────────────────────────────────────┤
│  CLI │ API Server │ MCP Server │ Library                    │
├─────────────────────────────────────────────────────────────┤
│  Pipeline: load → split → embed → store → search → format   │
├─────────┬──────────┬──────────┬──────────┬──────────────────┤
│ Loaders │ Splitters│ Embedders│  Stores  │ Search/Rerank    │
│  .txt   │  Text    │  Ollama  │  SQLite  │  Vector (flat)   │
│  .md    │  Token   │  OpenAI  │  Memory  │  BM25 (stretch)  │
│  .pdf*  │Semantic* │  Local*  │  Custom  │  Hybrid*         │
│  .csv*  │          │          │          │  Rerank*         │
│  .html* │          │          │          │  Graph*          │
└─────────┴──────────┴──────────┴──────────┴──────────────────┘
  * = stretch goal for v0.1.0
```

## 2. Crate-struktur

```
ravenrustrag/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── raven-core/         # Document, Chunk, SearchResult, Config, errors
│   ├── raven-embed/        # Embedder trait, OllamaBackend, caching
│   ├── raven-store/        # VectorStore trait, SqliteStore, MemoryStore
│   ├── raven-split/        # Splitter trait, TextSplitter (stretch: Semantic)
│   ├── raven-load/         # Loader trait, TextLoader, DirectoryLoader
│   ├── raven-search/       # DocumentIndex, Pipeline, Builder
│   ├── raven-server/       # Axum HTTP API (stretch for v0.1.0)
│   ├── raven-cli/          # CLI binary: index, query, info, clear, serve
│   └── raven-mcp/          # MCP server (stretch)
├── raven.toml              # Eksempel config
├── Dockerfile              # Multi-stage build (stretch)
└── .github/workflows/      # CI (stretch)
```

## 3. Fase 1: Foundation (Denne uken)

### 3.1 raven-core ✅
- [x] `Document` — med metadata, id (SHA-256 fallback)
- [x] `Chunk` — doc_id, text, metadata, embedding
- [x] `SearchResult` — chunk, score, distance, citation-property
- [x] `RavenError` — thiserror-basert enum
- [x] `Config` — TOML + env var støtte
- [x] Fingerprint (SHA-256)

### 3.2 raven-embed ✅
- [x] `Embedder` trait (async)
- [x] `OllamaBackend` — HTTP client til Ollama /api/embed
- [x] `EmbeddingCache` — LRU in-memory cache
- [x] `CachedEmbedder` — wrapper

### 3.3 raven-store ✅
- [x] `VectorStore` trait (async)
- [x] `SqliteStore` — rusqlite + flat brute-force cosine search
- [x] `MemoryStore` — for testing
- [x] Metadata-filtering støtte
- [x] Fingerprint-tabell for inkrementell indeksering

### 3.4 raven-split ✅
- [x] `Splitter` trait
- [x] `TextSplitter` — character-basert med overlap

### 3.5 raven-load ✅
- [x] `Loader` — from_file, from_directory
- [x] Extension-filtering
- [x] Recursive directory walking

### 3.6 raven-search ✅
- [x] `DocumentIndex` — hjertet
- [x] Builder pattern
- [x] `add_documents()` — split + embed + store pipeline
- [x] `query()` — embed + search
- [x] `query_for_prompt()` — formatert LLM prompt med sitater

### 3.7 raven-cli ✅ (skeleton)
- [x] `raven index <path>` — indekser dokumenter
- [x] `raven query "tekst"` — søk
- [x] `raven info` — statistikk
- [x] `raven clear` — tøm indeks
- [x] `raven serve` — placeholder for server

## 4. Fase 2: Features (Neste uke)

- [ ] HTTP API server (Axum) — /health, /query, /index, /stats, /openapi.json
- [ ] CLI: `raven serve --host --port`
- [ ] Semantic splitting (sentence boundaries + embedder)
- [ ] PDF/CSV/HTML loaders
- [ ] Watch mode (notify-rs)
- [ ] Export/import JSONL
- [ ] Context formatting med templates
- [ ] Docker multi-stage build
- [ ] GitHub Actions CI
- [ ] Cross-encoder reranking
- [ ] BM25 hybrid search
- [ ] MCP server
- [ ] Config file (raven.toml)

## 5. Fase 3: Advanced (Senere)

- [ ] Knowledge graph (entity extraction, graph traversal)
- [ ] Multi-collection routing
- [ ] Parent-child retrieval
- [ ] Eval metrics (MRR, NDCG, Recall@k)
- [ ] HNSW vektor-søk (i stedet for flat)
- [ ] ONNX Runtime local embeddings
- [ ] Benchmarks (Criterion)
- [ ] rustdoc + mdBook guide
- [ ] crates.io publish

## 6. Kjente begrensninger

1. **Mangler C-linker på host** — `cargo build` feiler på `cc not found`. Krever `build-essential` på systemet.
2. **Flat vektor-søk** — O(n) brute-force. Tilstrekkelig for < 10k dokumenter.
3. **Kun Ollama-embedder** — OpenAI/ONNX kommer i Fase 2.

## 7. Bygginstruksjoner (når linker er tilgjengelig)

```bash
# Clone
git clone https://github.com/egkristi/ravenrustrag.git
cd ravenrustrag

# Build debug
cargo build

# Build release (optimalisert)
cargo build --release

# Test
cargo test

# Kjør CLI
./target/release/raven index ./docs --db ./raven.db
./target/release/raven query "What is RAG?"

# Med Ollama
raven index ./docs --url http://localhost:11434 --model nomic-embed-text
```

---

**Sist oppdatert:** 2026-05-04  
**Neste gjennomgang:** Når C-linker er tilgjengelig og koden kompilerer