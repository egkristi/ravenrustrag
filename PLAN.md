# RavenRustRAG — Implementation Plan

> **Status:** v0.1.0-alpha — Fase 1 komplett, Fase 2 i gang  
> **Motto:** *Make it work, make it right, make it fast — in that order.*  
> **Mål:** Funksjonelt overlegen Python-versjonen (RavenRAG v0.7.0) med ordenstall bedre ytelse.

---

## Referanse: RavenRAG v0.7.0 (Python)

Komplett featureliste i Python-versjonen per 2026-05-04 (~4 200 linjer, 24 moduler):

| Kategori | Python-features | Rust-status |
|---|---|---|
| **Core** | Document, QueryResult (citation), DocumentIndex, async (aadd/aquery) | ✅ Fase 1 |
| **Embedding** | sentence-transformers, Ollama, OpenAI, vLLM, custom protocol | 🟡 Kun Ollama |
| **Storage** | ChromaDB, FAISS, SQLite-vec, VectorStoreBackend protocol | ✅ SQLite + Memory |
| **Splitting** | TextSplitter, TokenSplitter, SemanticSplitter | 🟡 Kun Text |
| **Loaders** | .txt .md .pdf .docx .pptx .xlsx .csv .rtf .html + plugin system | 🟡 Kun .txt/.md |
| **Search** | Vector, BM25 hybrid (RRF), cross-encoder reranking, streaming | 🟡 Kun vector |
| **Graph** | KnowledgeGraph, GraphRetriever, entity extraction, RRF fusion | ❌ |
| **Server** | HTTP (stdlib), auth, CORS, /metrics, /openapi.json, 7 endpoints | ❌ Stub |
| **MCP** | stdio JSON-RPC, 3 tools (search, get_prompt, collection_info) | ❌ Stub |
| **CLI** | 11 commands (index, query, prompt, serve, watch, info, export, import, doctor, mcp, benchmark) | 🟡 5 commands |
| **Pipeline** | Pipeline class, run/query/stream, error strategies | ❌ |
| **Config** | TOML + pyproject.toml + env vars, auto-discovery | ✅ Basis |
| **Cache** | Thread-safe LRU embedding cache | ✅ |
| **Eval** | MRR, NDCG, Recall@k | ❌ |
| **Watch** | File watcher with debounce + delete tracking | ❌ |
| **Export** | JSONL backup/restore | ❌ |
| **Fingerprint** | SHA-256 incremental indexing | ✅ |
| **Observability** | @timed decorator, /metrics, raven benchmark | ❌ |
| **Multi-collection** | MultiCollectionRouter, cross-index query | ❌ |
| **Parent-child** | query_parent() — search chunks, return parents | ❌ |
| **Context** | ContextFormatter, templates, citations in prompts | ✅ Basis |
| **Docker** | Multi-stage, model pre-download, non-root, healthcheck | ❌ |
| **CI** | GitHub Actions, lint, test (75% coverage), container build | ❌ |

### Kjente svakheter i Python-versjonen

Disse skal **ikke** reproduseres i Rust:

1. **Ingen tråd-sikkerhet** — samtidige forespørsler kan korrumpere tilstand
2. **Sync-first** — async er `asyncio.to_thread` wrappers, ikke ekte async
3. **ChromaDB-lekkasje** — `query_parent()` bryter VectorStoreBackend-abstraksjonen
4. **Minimal TOML-parser** — regex-basert, håndterer ikke arrays/escaped quotes
5. **Ingen rate limiting** — server DoS-sårbar
6. **Ingen request timeout** — treg query blokkerer tråd for alltid
7. **BM25 ikke persistert** — gjenoppbygges i minne ved endring
8. **Flat vektor-søk i SQLite-backend** — O(n), ingen indeks
9. **Stor oppstartstid** — 2-5s pga Python import + model loading
10. **Høyt minnebruk** — 200-500MB+ baseline

### Rust-fordeler som gjør oss overlegne

| Dimensjon | Python | Rust |
|---|---|---|
| **Oppstart** | 2–5s | <50ms |
| **Query-latens** | 50–200ms | 1–10ms (uten embedding) |
| **Minne** | 200–500MB+ | 20–50MB |
| **Deploy** | virtualenv + deps | Én statisk binær |
| **Concurrency** | GIL-bundet | Lock-free reads, Tokio async |
| **Sikkerhet** | Runtime exceptions | Compile-time guarantees |
| **Tråd-sikkerhet** | Ingen | Send + Sync, Arc<RwLock> |

---

## 1. Arkitektur

```
┌──────────────────────────────────────────────────────────────────┐
│                          RavenRustRAG                             │
├──────────────────────────────────────────────────────────────────┤
│  CLI │ Axum HTTP Server │ MCP Server (stdio) │ Library (crate)   │
├──────────────────────────────────────────────────────────────────┤
│  Pipeline: load → split → embed → store → search → rerank → fmt │
├──────────┬──────────┬───────────┬──────────┬─────────────────────┤
│ Loaders  │ Splitters│ Embedders │  Stores  │ Search & Retrieval  │
│  .txt    │  Text    │  Ollama   │  SQLite  │  Vector (flat/HNSW) │
│  .md     │  Token   │  OpenAI   │  Memory  │  BM25 keyword       │
│  .pdf    │ Semantic │  ONNX     │  Custom  │  Hybrid (RRF)       │
│  .docx   │          │  Custom   │          │  Cross-encoder      │
│  .html   │          │           │          │  Graph traversal    │
│  .csv    │          │           │          │  Parent-child       │
│  .json   │          │           │          │  Multi-collection   │
│  plugin  │          │           │          │  Streaming          │
└──────────┴──────────┴───────────┴──────────┴─────────────────────┘
```

## 2. Crate-struktur

```
ravenrustrag/
├── Cargo.toml                  # Workspace root
├── crates/
│   ├── raven-core/             # Document, Chunk, SearchResult, Config, errors, fingerprint
│   ├── raven-embed/            # Embedder trait + Ollama, OpenAI, ONNX backends + cache
│   ├── raven-store/            # VectorStore trait + SQLite, Memory backends
│   ├── raven-split/            # Splitter trait + Text, Token, Semantic splitters
│   ├── raven-load/             # Loader trait + file loaders + plugin registry
│   ├── raven-search/           # DocumentIndex, Pipeline, HybridSearcher, Reranker, Graph
│   ├── raven-server/           # Axum HTTP API (auth, CORS, /metrics, /openapi.json)
│   ├── raven-cli/              # CLI binary: 11+ commands
│   └── raven-mcp/              # MCP server (stdio JSON-RPC)
├── raven.toml                  # Default config
├── Dockerfile                  # Multi-stage, static binary, scratch/alpine
└── .github/workflows/          # CI: test, lint, release, container
```

---

## 3. Fase 1: Foundation ✅ KOMPLETT

### 3.1 raven-core ✅
- [x] `Document` — med metadata, id (SHA-256 fallback)
- [x] `Chunk` — doc_id, text, metadata, embedding
- [x] `SearchResult` — chunk, score, distance, citation
- [x] `RavenError` — thiserror-basert enum
- [x] `Config` — TOML + env var støtte
- [x] Fingerprint (SHA-256 content hash)

### 3.2 raven-embed ✅
- [x] `Embedder` trait (async)
- [x] `OllamaBackend` — HTTP client til Ollama /api/embed
- [x] `EmbeddingCache` — LRU in-memory cache
- [x] `CachedEmbedder` — transparent cache wrapper

### 3.3 raven-store ✅
- [x] `VectorStore` trait (async)
- [x] `SqliteStore` — rusqlite + cosine similarity
- [x] `MemoryStore` — for testing
- [x] Metadata-filtering
- [x] Fingerprint-tabell for inkrementell indeksering

### 3.4 raven-split ✅
- [x] `Splitter` trait
- [x] `TextSplitter` — character-basert med configurable overlap

### 3.5 raven-load ✅
- [x] `Loader` — from_file, from_directory
- [x] Extension-filtering
- [x] Recursive directory walking

### 3.6 raven-search ✅
- [x] `DocumentIndex` — pipeline-orkestrator
- [x] Builder pattern
- [x] `add_documents()` — split → embed → store
- [x] `query()` — embed → search
- [x] `query_for_prompt()` — LLM-klar kontekst med sitater

### 3.7 raven-cli ✅
- [x] `raven index <path>` — indekser med progress bar
- [x] `raven query "tekst"` — søk med scoring
- [x] `raven info` — statistikk
- [x] `raven clear` — tøm indeks
- [x] `raven serve` — placeholder

---

## 4. Fase 2: Feature Parity med Python

**Mål:** Match alle features i RavenRAG v0.7.0, men med bedre design.

### 4.1 HTTP API Server (raven-server)
- [ ] Axum-basert server med Tokio
- [ ] `GET /health` — helsesjekk
- [ ] `GET /stats` — indeksstatistikk
- [ ] `GET /collections` — liste collections
- [ ] `GET /metrics` — timing og cache stats
- [ ] `GET /openapi.json` — OpenAPI 3.0 schema
- [ ] `POST /query` — søk (top_k, where, rerank, hybrid, alpha)
- [ ] `POST /prompt` — LLM-ferdig prompt
- [ ] `POST /index` — legg til dokumenter
- [ ] Bearer token auth (via header + config/env)
- [ ] CORS-konfigurasjon (tower-http)
- [ ] Request size limit (10MB)
- [ ] Request timeout (configurable)
- [ ] Rate limiting (tower middleware) — **bedre enn Python**
- [ ] Graceful shutdown

### 4.2 MCP Server (raven-mcp)
- [ ] JSON-RPC over stdio (MCP 2024-11-05)
- [ ] Tool: `search` — query med top_k
- [ ] Tool: `get_prompt` — søk + formater LLM prompt
- [ ] Tool: `collection_info` — indeksstatistikk
- [ ] Tool: `index_documents` — legg til dokumenter **ny vs Python**
- [ ] Proper error codes og schema validation

### 4.3 Flere embedding-backends
- [ ] `OpenAIBackend` — OpenAI-kompatibel API (OpenAI, LM Studio, LocalAI, vLLM)
- [ ] ONNX Runtime local embeddings — **bedre enn Python** (native, ingen Python-runtime)
- [ ] Backend auto-detection basert på URL-scheme (`ollama://`, `openai://`, `onnx://`)

### 4.4 Splitter-utvidelser
- [ ] `TokenSplitter` — tokenizer-bevisst splitting (tiktoken-rs)
- [ ] `SemanticSplitter` — sentence-boundary + embedding cosine similarity
- [ ] Metadata preservation (chunk_index, source_id) gjennom hele pipeline

### 4.5 Fil-loadere
- [ ] Markdown med frontmatter-parsing (YAML metadata → doc metadata)
- [ ] PDF loader (pdf-extract eller lopdf)
- [ ] HTML loader (scraper crate — strip tags, beholde struktur)
- [ ] CSV loader (csv crate)
- [ ] JSON/JSONL loader
- [ ] DOCX loader (docx-rs)
- [ ] Plugin-system: `register_loader` for egne filtyper
- [ ] Auto-detect filtype og velg loader

### 4.6 Hybrid Search
- [ ] BM25-indeks (tantivy eller egenbygd) — **bedre enn Python** (persistert, ikke in-memory)
- [ ] `HybridSearcher` — vector + BM25 med Reciprocal Rank Fusion
- [ ] Configurable alpha (0.0 = ren BM25, 1.0 = ren vektor)
- [ ] Metadata-filtering på begge signaler

### 4.7 Cross-encoder Reranking
- [ ] ONNX-basert cross-encoder (lokal, ingen Python) — **bedre enn Python**
- [ ] Rerank trait med pluggbare backends
- [ ] Fetch 4x → rerank → return top_k

### 4.8 Watch Mode
- [ ] `notify` crate for filsystem-events
- [ ] Debounce med konfigurerbar ventetid
- [ ] Sletting-støtte (fjern dokumenter når filer slettes)
- [ ] Extension-filtering
- [ ] CLI: `raven watch ./docs --extensions ".md,.txt"`

### 4.9 Export/Import
- [ ] JSONL eksport (`raven export -o backup.jsonl`)
- [ ] JSONL import (`raven import backup.jsonl`)
- [ ] Skip invalid/empty rows ved import
- [ ] Streaming I/O for store filer — **bedre enn Python** (ikke last alt i minne)

### 4.10 Context Formatting
- [ ] `ContextFormatter` med templates ({context}, {query}, {sources})
- [ ] Citation-insetting i formattert output
- [ ] Konfiguerbare templates via raven.toml

### 4.11 CLI-utvidelser
- [ ] `raven serve` — start HTTP server
- [ ] `raven prompt "tekst"` — formattert LLM-prompt
- [ ] `raven watch <path>` — auto-reindex
- [ ] `raven export` / `raven import` — JSONL backup/restore
- [ ] `raven doctor` — diagnostikk (sjekk Ollama, db, config)
- [ ] `raven mcp` — start MCP server
- [ ] `raven benchmark` — ytelsesmåling (Criterion-basert) — **bedre enn Python**
- [ ] `--hybrid`, `--rerank`, `--verbose` flagg på query

### 4.12 Konfigurasjon
- [ ] `raven.toml` auto-discovery (gå opp fra cwd)
- [ ] Env var overrides (RAVEN_DB, RAVEN_MODEL, RAVEN_API_KEY, etc.)
- [ ] Ukjent-nøkkel varsling (typo-beskyttelse)
- [ ] Full config validering ved oppstart

### 4.13 Docker & CI
- [ ] Multi-stage Dockerfile (builder → scratch/alpine)
- [ ] Statisk binær (`musl` target) — **bedre enn Python** (~15MB vs ~1.5GB image)
- [ ] GitHub Actions: test, lint (clippy), format (rustfmt), release
- [ ] Container build og push til GHCR
- [ ] Cross-compile for linux/amd64 og linux/arm64

---

## 5. Fase 3: Rust-Overlegenhet

Features som gjør Rust-versjonen **strengt bedre** enn Python:

### 5.1 Avansert Retrieval
- [ ] Parent-child retrieval (`query_parent()` — via VectorStore trait, ingen abstraksjonsbrudd)
- [ ] Multi-collection routing (`MultiCollectionRouter`)
- [ ] Streaming results (`query_stream()` — async Stream trait)
- [ ] Multi-query expansion (omskriv spørring til flere varianter)

### 5.2 Knowledge Graph
- [ ] Entity extraction (NER via ONNX eller regex-heuristikk)
- [ ] In-memory graph med JSON persistence
- [ ] Graph traversal (BFS med max_hops)
- [ ] `GraphRetriever` — RRF-fusjon mellom graf og vektor
- [ ] `raven graph build` / `raven graph query` CLI-kommandoer

### 5.3 Eval & Benchmarking
- [ ] `evaluate()` — MRR, NDCG, Recall@k mot ground truth
- [ ] Criterion-baserte micro-benchmarks
- [ ] `raven benchmark` med detaljert rapport (index speed, query latens, minne)
- [ ] CI-drevet ytelsesregresjon

### 5.4 Observability
- [ ] Tracing med `tracing` crate (strukturert logging)
- [ ] Timing-spans for alle pipeline-steg
- [ ] `/metrics` endpoint (Prometheus-kompatibel) — **bedre enn Python**
- [ ] OpenTelemetry-eksport (valgfri feature)

### 5.5 HNSW Vector Search
- [ ] Erstatt flat brute-force med HNSW (instant-distance eller usearch)
- [ ] O(log n) søk i stedet for O(n)
- [ ] Skalerbart til millioner av dokumenter — **mye bedre enn Python**

### 5.6 Ytelsesfordeler
- [ ] SIMD-akselerert cosine similarity (via ndarray eller manuell)
- [ ] Lock-free concurrent reads (Arc<RwLock> eller dashmap)
- [ ] Zero-copy deserialisering der mulig
- [ ] Memory-mapped SQLite for stor skala
- [ ] Batch embedding med parallelisme

---

## 6. Fase 4: Polish & Release

### 6.1 Dokumentasjon
- [ ] rustdoc for alle public items
- [ ] mdBook brukerguide
- [ ] Migreringsguide fra Python RavenRAG
- [ ] Ytelsessammenligninger vs Python-versjonen
- [ ] Troubleshooting-seksjon

### 6.2 Publisering
- [ ] crates.io publish
- [ ] `cargo install ravenrustrag`
- [ ] GitHub Releases med pre-built binaries (linux, macos, windows)
- [ ] Homebrew formula
- [ ] AUR package

### 6.3 Kvalitet
- [ ] 80%+ testdekning
- [ ] Property-based testing (proptest) for splitters og search
- [ ] Fuzzing for parsere og input-håndtering
- [ ] Concurrent stress-tester
- [ ] 10k+ dokument skaleringstest

---

## 7. Kjente begrensninger (nåværende)

1. **Mangler C-linker på host** — `cargo build` krever `build-essential` / Xcode CLI tools (pga rusqlite bundled)
2. **Flat vektor-søk** — O(n) brute-force. Tilstrekkelig for <10k dokumenter. HNSW i Fase 3.
3. **Kun Ollama-embedder** — OpenAI/ONNX kommer i Fase 2.
4. **Server/MCP er stubs** — Implementeres i Fase 2.

## 8. Bygginstruksjoner

```bash
# Clone
git clone https://github.com/egkristi/ravenrustrag.git
cd ravenrustrag

# Forutsetninger
# macOS: xcode-select --install
# Ubuntu: sudo apt install build-essential pkg-config
# Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
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
**Neste milepæl:** Fase 2 — feature parity med Python v0.7.0