use anyhow::Result;
use clap::{Parser, Subcommand};
use raven_core::ServerConfig;
use raven_embed::Embedder;
use raven_load::Loader;
use raven_mcp::McpServer;
use raven_search::DocumentIndex;
use raven_server::AppState;
use raven_split::TextSplitter;
use raven_store::{SqliteStore, VectorStore};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, warn};

#[derive(Parser)]
#[command(name = "raven")]
#[command(about = "RavenRustRAG — Fearlessly fast retrieval.")]
#[command(version = "0.1.0-alpha.1")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to config file
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Index documents from a directory
    Index {
        /// Path to documents
        path: PathBuf,

        /// Database path
        #[arg(short, long, default_value = "./raven.db")]
        db: PathBuf,

        /// Embedding backend (ollama or openai)
        #[arg(short, long, default_value = "ollama")]
        backend: String,

        /// Ollama URL
        #[arg(short, long, default_value = "http://localhost:11434")]
        url: String,

        /// Embedding model
        #[arg(short, long, default_value = "nomic-embed-text")]
        model: String,

        /// Chunk size
        #[arg(long, default_value_t = 512)]
        chunk_size: usize,

        /// Chunk overlap
        #[arg(long, default_value_t = 50)]
        chunk_overlap: usize,

        /// File extensions to include (comma-separated)
        #[arg(long, default_value = "txt,md")]
        extensions: String,
    },

    /// Query the index
    Query {
        /// Search query
        query: String,

        /// Database path
        #[arg(short, long, default_value = "./raven.db")]
        db: PathBuf,

        /// Embedding backend (ollama or openai)
        #[arg(short, long, default_value = "ollama")]
        backend: String,

        /// Ollama URL
        #[arg(short, long, default_value = "http://localhost:11434")]
        url: String,

        /// Embedding model
        #[arg(short, long, default_value = "nomic-embed-text")]
        model: String,

        /// Number of results
        #[arg(short = 'k', long, default_value_t = 5)]
        top_k: usize,

        /// Use hybrid search (BM25 + vector with RRF)
        #[arg(long)]
        hybrid: bool,

        /// Alpha blend for hybrid search (1.0 = pure vector, 0.0 = pure BM25)
        #[arg(long, default_value_t = 0.5)]
        alpha: f32,
    },

    /// Get a formatted LLM prompt with citations
    Prompt {
        /// Query text
        query: String,

        /// Database path
        #[arg(short, long, default_value = "./raven.db")]
        db: PathBuf,

        /// Embedding backend (ollama or openai)
        #[arg(short, long, default_value = "ollama")]
        backend: String,

        /// Ollama URL
        #[arg(short, long, default_value = "http://localhost:11434")]
        url: String,

        /// Embedding model
        #[arg(short, long, default_value = "nomic-embed-text")]
        model: String,

        /// Number of context chunks
        #[arg(short = 'k', long, default_value_t = 3)]
        top_k: usize,
    },

    /// Show index statistics
    Info {
        /// Database path
        #[arg(short, long, default_value = "./raven.db")]
        db: PathBuf,
    },

    /// Start API server
    Serve {
        /// Host
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// Port
        #[arg(short, long, default_value_t = 8484)]
        port: u16,

        /// Database path
        #[arg(short, long, default_value = "./raven.db")]
        db: PathBuf,

        /// Embedding backend (ollama or openai)
        #[arg(short, long, default_value = "ollama")]
        backend: String,

        /// Ollama URL
        #[arg(short, long, default_value = "http://localhost:11434")]
        url: String,

        /// Embedding model
        #[arg(short, long, default_value = "nomic-embed-text")]
        model: String,

        /// API key for authentication (optional)
        #[arg(long)]
        api_key: Option<String>,
    },

    /// Clear the index
    Clear {
        /// Database path
        #[arg(short, long, default_value = "./raven.db")]
        db: PathBuf,
    },

    /// Export index to JSONL
    Export {
        /// Output file
        #[arg(short, long, default_value = "raven-export.jsonl")]
        output: PathBuf,

        /// Database path
        #[arg(short, long, default_value = "./raven.db")]
        db: PathBuf,
    },

    /// Import documents from JSONL
    Import {
        /// Input file
        file: PathBuf,

        /// Database path
        #[arg(short, long, default_value = "./raven.db")]
        db: PathBuf,

        /// Embedding backend (ollama or openai)
        #[arg(short, long, default_value = "ollama")]
        backend: String,

        /// Ollama URL
        #[arg(short, long, default_value = "http://localhost:11434")]
        url: String,

        /// Embedding model
        #[arg(short, long, default_value = "nomic-embed-text")]
        model: String,
    },

    /// Start MCP server (stdio, for AI assistants)
    Mcp {
        /// Database path
        #[arg(short, long, default_value = "./raven.db")]
        db: PathBuf,

        /// Embedding backend (ollama or openai)
        #[arg(short, long, default_value = "ollama")]
        backend: String,

        /// Ollama URL
        #[arg(short, long, default_value = "http://localhost:11434")]
        url: String,

        /// Embedding model
        #[arg(short, long, default_value = "nomic-embed-text")]
        model: String,
    },

    /// Run diagnostics
    Doctor {
        /// Ollama URL to check
        #[arg(short, long, default_value = "http://localhost:11434")]
        url: String,

        /// Database path to check
        #[arg(short, long, default_value = "./raven.db")]
        db: PathBuf,
    },

    /// Watch a directory and auto-index on changes
    Watch {
        /// Path to watch
        path: PathBuf,

        /// Database path
        #[arg(short, long, default_value = "./raven.db")]
        db: PathBuf,

        /// Embedding backend (ollama or openai)
        #[arg(short, long, default_value = "ollama")]
        backend: String,

        /// Ollama URL
        #[arg(short, long, default_value = "http://localhost:11434")]
        url: String,

        /// Embedding model
        #[arg(short, long, default_value = "nomic-embed-text")]
        model: String,

        /// File extensions to watch (comma-separated)
        #[arg(long, default_value = "txt,md")]
        extensions: String,

        /// Debounce interval in milliseconds
        #[arg(long, default_value_t = 500)]
        debounce: u64,
    },

    /// Run performance benchmarks
    Benchmark {
        /// Number of documents to generate
        #[arg(short, long, default_value_t = 100)]
        num_docs: usize,

        /// Number of query iterations
        #[arg(short, long, default_value_t = 50)]
        iterations: usize,
    },
}

fn make_embedder(backend: &str, url: &str, model: &str) -> Arc<dyn Embedder> {
    raven_embed::create_cached_embedder(backend, model, Some(url), None, 1000)
}

async fn make_store(db: &PathBuf) -> Result<Arc<SqliteStore>> {
    Ok(Arc::new(SqliteStore::new(db, 768).await?))
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .init();

    match cli.command {
        Commands::Index {
            path,
            db,
            backend,
            url,
            model,
            chunk_size,
            chunk_overlap,
            extensions,
        } => {
            info!("Indexing documents from {:?}", path);

            let exts: Vec<&str> = extensions.split(',').map(str::trim).collect();
            let ext_refs: Vec<&str> = exts.clone();

            // Collect file paths for incremental indexing
            let entries: Vec<_> = walkdir::WalkDir::new(&path)
                .follow_links(false)
                .into_iter()
                .filter_map(std::result::Result::ok)
                .filter(|e| e.file_type().is_file())
                .filter(|e| {
                    e.path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .is_some_and(|ext| {
                            ext_refs.iter().any(|e2| e2.trim_start_matches('.') == ext)
                        })
                })
                .collect();

            info!("Found {} files", entries.len());

            if entries.is_empty() {
                warn!("No documents found in {:?}", path);
                return Ok(());
            }

            let embedder = make_embedder(&backend, &url, &model);
            let store = make_store(&db).await?;
            let index = DocumentIndex::new(store.clone(), embedder);
            let splitter = TextSplitter::new(chunk_size, chunk_overlap);

            let pb = indicatif::ProgressBar::new(entries.len() as u64);
            pb.set_style(
                indicatif::ProgressStyle::default_bar()
                    .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
                    .unwrap(),
            );

            let mut indexed = 0usize;
            let mut skipped = 0usize;

            for entry in &entries {
                let file_path = entry.path();
                let path_str = file_path.to_string_lossy().to_string();

                // Read file content and check fingerprint
                let content = match std::fs::read_to_string(file_path) {
                    Ok(c) => c,
                    Err(e) => {
                        warn!("Failed to read {}: {}", path_str, e);
                        pb.inc(1);
                        continue;
                    }
                };

                let hash = raven_core::fingerprint(&content);

                // Check if already indexed with same content
                if let Ok(Some(existing)) = store.get_fingerprint(&path_str).await {
                    if existing == hash {
                        skipped += 1;
                        pb.inc(1);
                        continue;
                    }
                    // Content changed — delete old chunks for this file
                    store.delete(&path_str).await.ok();
                }

                // Load and index the document
                match Loader::from_file(file_path) {
                    Ok(doc) => {
                        let doc = doc.with_metadata("source_path", &path_str);
                        index.add_documents(vec![doc], &splitter).await?;
                        store.set_fingerprint(&path_str, &hash).await?;
                        indexed += 1;
                    }
                    Err(e) => {
                        warn!("Failed to load {}: {}", path_str, e);
                    }
                }

                pb.inc(1);
            }

            let total_chunks = index.count().await?;
            pb.finish_with_message(format!(
                "Done! {indexed} new, {skipped} skipped, {total_chunks} total chunks"
            ));
        }

        Commands::Query {
            query,
            db,
            backend,
            url,
            model,
            top_k,
            hybrid,
            alpha,
        } => {
            let embedder = make_embedder(&backend, &url, &model);
            let store = make_store(&db).await?;
            let index = DocumentIndex::new(store.clone(), embedder);

            let results = if hybrid {
                // For hybrid search, populate BM25 index from stored chunks
                let all_chunks = store.all().await?;
                index.add_chunks(&all_chunks).await?;
                info!("BM25 index loaded with {} chunks", all_chunks.len());
                index.query_hybrid(&query, top_k, alpha).await?
            } else {
                index.query(&query, top_k).await?
            };

            let mode = if hybrid { "hybrid" } else { "vector" };
            println!("\n🐦‍⬛ Results for: \"{query}\" ({mode})\n");

            if results.is_empty() {
                println!("No results found.");
            } else {
                for (i, result) in results.iter().enumerate() {
                    let source = result
                        .chunk
                        .metadata
                        .get("source")
                        .cloned()
                        .unwrap_or_else(|| "unknown".to_string());
                    println!("[{}] (score: {:.4})", i + 1, result.score);
                    println!("    Source: {source}");
                    let preview: String = result.chunk.text.chars().take(200).collect();
                    println!("    {preview}\n");
                }
            }
        }

        Commands::Prompt {
            query,
            db,
            backend,
            url,
            model,
            top_k,
        } => {
            let embedder = make_embedder(&backend, &url, &model);
            let store = make_store(&db).await?;
            let index = DocumentIndex::new(store, embedder);

            let prompt = index.query_for_prompt(&query, top_k).await?;
            println!("{prompt}");
        }

        Commands::Info { db } => {
            let store = make_store(&db).await?;
            let count = store.count().await?;

            println!("🐦‍⬛ RavenRustRAG Index Info\n");
            println!("  Database: {}", db.display());
            println!("  Chunks:   {count}");
        }

        Commands::Serve {
            host,
            port,
            db,
            backend,
            url,
            model,
            api_key,
        } => {
            let embedder = make_embedder(&backend, &url, &model);
            let store = make_store(&db).await?;
            let index = DocumentIndex::new(store.clone(), embedder);

            // Populate BM25 index from stored chunks for hybrid search support
            let all_chunks = store.all().await?;
            if !all_chunks.is_empty() {
                index.add_chunks(&all_chunks).await?;
                info!("BM25 index loaded with {} chunks", all_chunks.len());
            }

            let api_key = api_key.or_else(|| std::env::var("RAVEN_API_KEY").ok());

            let config = ServerConfig {
                host: host.clone(),
                port,
                api_key,
                ..ServerConfig::default()
            };

            println!("🐦‍⬛ RavenRustRAG server starting on http://{host}:{port}");
            println!("   Database: {}", db.display());
            println!("   Model: {model} ({url})");
            println!("   Endpoints: /health /stats /metrics /query /prompt /index /openapi.json");
            println!("   Press Ctrl+C to stop.\n");

            let state = Arc::new(AppState::new(index, config, TextSplitter::new(512, 50)));

            raven_server::serve(state)
                .await
                .map_err(|e| anyhow::anyhow!("{e}"))?;
        }

        Commands::Clear { db } => {
            let store = make_store(&db).await?;
            store.clear().await?;
            println!("✓ Cleared index at {}", db.display());
        }

        Commands::Export { output, db } => {
            let store = make_store(&db).await?;
            let count = store.count().await?;
            println!("Exporting {} chunks from {}...", count, db.display());

            let chunks = store.all().await?;
            // Convert chunks to documents for JSONL export
            let docs: Vec<raven_core::Document> = chunks
                .into_iter()
                .map(|c| {
                    let mut doc = raven_core::Document::new(&c.text).with_id(&c.doc_id);
                    for (k, v) in &c.metadata {
                        doc = doc.with_metadata(k, v);
                    }
                    doc = doc.with_metadata("chunk_id", &c.id);
                    doc
                })
                .collect();
            let written = raven_load::export_jsonl(&docs, &output)?;
            println!("✓ Exported {} documents to {}", written, output.display());
        }

        Commands::Import {
            file,
            db,
            backend,
            url,
            model,
        } => {
            let docs = raven_load::import_jsonl(&file)?;
            println!("Loaded {} documents from {}", docs.len(), file.display());

            if docs.is_empty() {
                println!("No documents to import.");
                return Ok(());
            }

            let embedder = make_embedder(&backend, &url, &model);
            let store = make_store(&db).await?;
            let index = DocumentIndex::new(store, embedder);
            let splitter = TextSplitter::new(512, 50);

            index.add_documents(docs, &splitter).await?;
            println!("✓ Imported to {}", db.display());
        }

        Commands::Mcp {
            db,
            backend,
            url,
            model,
        } => {
            let embedder = make_embedder(&backend, &url, &model);
            let store = make_store(&db).await?;
            let index = Arc::new(DocumentIndex::new(store, embedder));
            let splitter = TextSplitter::new(512, 50);

            let server = McpServer::new(index, splitter);
            server
                .run_stdio()
                .await
                .map_err(|e| anyhow::anyhow!("{e}"))?;
        }

        Commands::Doctor { url, db } => {
            println!("🐦‍⬛ RavenRustRAG Doctor\n");

            // Check database
            print!("  Database ({})... ", db.display());
            if db.exists() {
                match SqliteStore::new(&db, 768).await {
                    Ok(store) => {
                        let count = store.count().await.unwrap_or(0);
                        println!("✓ OK ({count} chunks)");
                    }
                    Err(e) => println!("✗ Error: {e}"),
                }
            } else {
                println!("⚠ Not found (will be created on first index)");
            }

            // Check Ollama
            print!("  Ollama ({url})... ");
            let client = reqwest::Client::new();
            match client
                .get(format!("{url}/api/tags"))
                .timeout(std::time::Duration::from_secs(3))
                .send()
                .await
            {
                Ok(resp) if resp.status().is_success() => println!("✓ OK"),
                Ok(resp) => println!("⚠ Status: {}", resp.status()),
                Err(e) => println!("✗ Not reachable: {e}"),
            }

            println!("\n  Version: {}", env!("CARGO_PKG_VERSION"));
        }

        Commands::Watch {
            path,
            db,
            backend,
            url,
            model,
            extensions,
            debounce,
        } => {
            let embedder = make_embedder(&backend, &url, &model);
            let store = make_store(&db).await?;
            let index = Arc::new(DocumentIndex::new(store.clone(), embedder));
            let splitter: Arc<dyn raven_split::Splitter> = Arc::new(TextSplitter::new(512, 50));

            let exts: Vec<&str> = extensions.split(',').map(str::trim).collect();

            println!(
                "🐦‍⬛ Watching {path:?} for changes (extensions: {extensions}, debounce: {debounce}ms)"
            );
            println!("   Press Ctrl+C to stop.\n");

            raven_search::watch_directory(index, store, splitter, &path, &exts, debounce)
                .await
                .map_err(|e| anyhow::anyhow!("{e}"))?;
        }

        Commands::Benchmark {
            num_docs,
            iterations,
        } => {
            use raven_embed::DummyEmbedder;
            use raven_store::MemoryStore;

            println!("Running benchmarks (num_docs={num_docs}, iterations={iterations})...\n");

            let embedder = Arc::new(DummyEmbedder::new(128));
            let splitter = TextSplitter::new(200, 20);

            // Generate test documents
            let docs: Vec<raven_core::Document> = (0..num_docs)
                .map(|i| {
                    raven_core::Document::new(format!(
                        "Document {i} about Rust programming, memory safety, and performance. \
                         Systems programming with zero-cost abstractions and fearless concurrency."
                    ))
                })
                .collect();

            // Benchmark indexing
            let store = Arc::new(MemoryStore::new());
            let index = DocumentIndex::new(store, embedder);

            let start = std::time::Instant::now();
            index.add_documents(docs, &splitter).await?;
            let index_time = start.elapsed();
            let chunks = index.count().await?;

            println!("  Index:  {num_docs} docs -> {chunks} chunks in {index_time:?}");
            println!(
                "          {:.1} docs/sec",
                num_docs as f64 / index_time.as_secs_f64()
            );

            // Benchmark vector query
            let mut query_times = Vec::with_capacity(iterations);
            for _ in 0..iterations {
                let start = std::time::Instant::now();
                let _ = index.query("Rust programming performance", 5).await?;
                query_times.push(start.elapsed());
            }
            let avg_query = query_times.iter().sum::<std::time::Duration>() / iterations as u32;
            let min_query = query_times.iter().min().copied().unwrap_or_default();
            let max_query = query_times.iter().max().copied().unwrap_or_default();

            println!("\n  Query (vector, {iterations} iterations):");
            println!("          avg: {avg_query:?}  min: {min_query:?}  max: {max_query:?}");

            // Benchmark hybrid query
            let mut hybrid_times = Vec::with_capacity(iterations);
            for _ in 0..iterations {
                let start = std::time::Instant::now();
                let _ = index
                    .query_hybrid("Rust programming performance", 5, 0.5)
                    .await?;
                hybrid_times.push(start.elapsed());
            }
            let avg_hybrid = hybrid_times.iter().sum::<std::time::Duration>() / iterations as u32;
            let min_hybrid = hybrid_times.iter().min().copied().unwrap_or_default();
            let max_hybrid = hybrid_times.iter().max().copied().unwrap_or_default();

            println!("\n  Query (hybrid, {iterations} iterations):");
            println!("          avg: {avg_hybrid:?}  min: {min_hybrid:?}  max: {max_hybrid:?}");

            // Benchmark BM25
            let mut bm25_times = Vec::with_capacity(iterations);
            let bm25_idx = {
                let mut b = raven_search::Bm25Index::new();
                let all_chunks: Vec<raven_core::Chunk> = (0..chunks)
                    .map(|i| {
                        raven_core::Chunk::new(
                            format!("doc_{i}"),
                            format!("Rust programming document number {i}"),
                        )
                    })
                    .collect();
                b.add(&all_chunks);
                b
            };
            for _ in 0..iterations {
                let start = std::time::Instant::now();
                let _ = bm25_idx.search("Rust programming", 5);
                bm25_times.push(start.elapsed());
            }
            let avg_bm25 = bm25_times.iter().sum::<std::time::Duration>() / iterations as u32;

            println!("\n  BM25 ({chunks} chunks, {iterations} iterations):");
            println!("          avg: {avg_bm25:?}");
            println!("\nBenchmark complete.");
        }
    }

    Ok(())
}
