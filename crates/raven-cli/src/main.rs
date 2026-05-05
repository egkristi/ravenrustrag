use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use colored::Colorize;
use raven_core::ServerConfig;
use raven_embed::{Embedder, GeneratorConfig};
use raven_load::Loader;
use raven_mcp::McpServer;
use raven_search::DocumentIndex;
use raven_search::{GraphRetriever, KnowledgeGraph};
use raven_server::AppState;
use raven_split::TextSplitter;
use raven_store::{SqliteStore, VectorStore};
use std::path::{Path, PathBuf};
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

    /// Output as JSON (for scripting)
    #[arg(long, global = true)]
    json: bool,

    /// Log format: text or json (for containers/K8s)
    #[arg(long, global = true, default_value = "text")]
    log_format: String,
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
        #[arg(long, default_value = "txt,md,csv,json,html,pdf,docx")]
        extensions: String,

        /// Show what would be indexed without actually indexing
        #[arg(long)]
        dry_run: bool,
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

    /// Ask a question — full RAG pipeline with LLM-generated answer
    Ask {
        /// Question to answer
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

        /// LLM model for generation
        #[arg(long, default_value = "llama3")]
        llm_model: String,

        /// Number of context chunks
        #[arg(short = 'k', long, default_value_t = 3)]
        top_k: usize,

        /// Temperature for generation
        #[arg(long, default_value_t = 0.7)]
        temperature: f32,
    },

    /// Show index statistics
    Info {
        /// Database path
        #[arg(short, long, default_value = "./raven.db")]
        db: PathBuf,
    },

    /// Start API server
    Serve {
        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1", env = "RAVEN_HOST")]
        host: String,

        /// Port
        #[arg(short, long, default_value_t = 8484, env = "RAVEN_PORT")]
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

    /// Show index health and stats at a glance
    Status {
        /// Database path
        #[arg(short, long, default_value = "./raven.db")]
        db: PathBuf,

        /// Ollama URL to check
        #[arg(short, long, default_value = "http://localhost:11434")]
        url: String,
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
        #[arg(long, default_value = "txt,md,csv,json,html,pdf,docx")]
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

    /// Build or query the knowledge graph
    Graph {
        #[command(subcommand)]
        action: GraphAction,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Initialize a new raven.toml configuration file
    Init {
        /// Output path (default: ./raven.toml)
        #[arg(short, long, default_value = "raven.toml")]
        output: PathBuf,

        /// Overwrite existing config file
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
enum GraphAction {
    /// Build knowledge graph from indexed documents
    Build {
        /// Database path
        #[arg(short, long, default_value = "./raven.db")]
        db: PathBuf,

        /// Graph output file
        #[arg(short, long, default_value = "./raven-graph.json")]
        output: PathBuf,
    },

    /// Query the knowledge graph
    Query {
        /// Search query
        query: String,

        /// Graph file
        #[arg(short, long, default_value = "./raven-graph.json")]
        graph: PathBuf,

        /// Max hops for graph traversal
        #[arg(long, default_value_t = 2)]
        max_hops: usize,

        /// Number of results
        #[arg(short = 'k', long, default_value_t = 5)]
        top_k: usize,
    },
}

fn make_embedder(backend: &str, url: &str, model: &str) -> Arc<dyn Embedder> {
    raven_embed::create_cached_embedder(backend, model, Some(url), None, 1000)
}

async fn make_store(db: &PathBuf, dimension: usize) -> Result<Arc<SqliteStore>> {
    Ok(Arc::new(SqliteStore::new(db, dimension).await?))
}

/// Resolve effective values: CLI defaults are overridden by config
fn resolve_params<'a>(
    backend: &'a str,
    url: &'a str,
    model: &'a str,
    db: &'a Path,
    cfg: &'a raven_core::Config,
) -> (&'a str, &'a str, &'a str, PathBuf) {
    let eff_backend = if backend == "ollama" {
        cfg.embedder.backend.as_str()
    } else {
        backend
    };
    let eff_url = if url == "http://localhost:11434" {
        cfg.embedder
            .url
            .as_deref()
            .unwrap_or("http://localhost:11434")
    } else {
        url
    };
    let eff_model = if model == "nomic-embed-text" {
        cfg.embedder.model.as_str()
    } else {
        model
    };
    let eff_db = if db == Path::new("./raven.db") {
        PathBuf::from(&cfg.store.path)
    } else {
        db.to_path_buf()
    };
    (eff_backend, eff_url, eff_model, eff_db)
}

#[tokio::main]
#[allow(clippy::too_many_lines)]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    let log_format = std::env::var("RAVEN_LOG_FORMAT").unwrap_or_else(|_| cli.log_format.clone());

    if log_format == "json" {
        tracing_subscriber::fmt()
            .json()
            .with_max_level(log_level)
            .with_target(false)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(log_level)
            .with_target(false)
            .init();
    }

    // Load config: explicit --config path, or auto-discover raven.toml, or defaults
    let cfg = raven_core::Config::load(cli.config.as_deref()).unwrap_or_else(|e| {
        warn!("Failed to load config: {e}, using defaults");
        raven_core::Config::default()
    });

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
            dry_run,
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

            // Dry-run mode: just report what would be indexed
            if dry_run {
                let (_, _, _, eff_db) = resolve_params(&backend, &url, &model, &db, &cfg);
                let store = make_store(&eff_db, 0).await.ok();

                let mut would_index = 0usize;
                let mut would_skip = 0usize;

                for entry in &entries {
                    let file_path = entry.path();
                    let path_str = file_path.to_string_lossy().to_string();
                    let Ok(content) = std::fs::read_to_string(file_path) else {
                        continue;
                    };
                    let hash = raven_core::fingerprint(&content);

                    let skip = if let Some(ref s) = store {
                        matches!(s.get_fingerprint(&path_str).await, Ok(Some(h)) if h == hash)
                    } else {
                        false
                    };

                    if skip {
                        would_skip += 1;
                        if !cli.json {
                            println!("  skip  {path_str}");
                        }
                    } else {
                        would_index += 1;
                        if !cli.json {
                            println!("  index {path_str}");
                        }
                    }
                }

                if cli.json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "dry_run": true,
                            "would_index": would_index,
                            "would_skip": would_skip,
                            "total_files": entries.len(),
                        }))?
                    );
                } else {
                    println!(
                        "\nDry run: {would_index} to index, {would_skip} unchanged, {} total",
                        entries.len()
                    );
                }
                return Ok(());
            }

            let (eff_backend, eff_url, eff_model, eff_db) =
                resolve_params(&backend, &url, &model, &db, &cfg);
            let embedder = make_embedder(eff_backend, eff_url, eff_model);
            let store = make_store(&eff_db, embedder.dimension()).await?;
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
            let (eff_backend, eff_url, eff_model, eff_db) =
                resolve_params(&backend, &url, &model, &db, &cfg);
            let embedder = make_embedder(eff_backend, eff_url, eff_model);
            let store = make_store(&eff_db, embedder.dimension()).await?;
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

            if cli.json {
                let items: Vec<serde_json::Value> = results
                    .iter()
                    .map(|r| {
                        serde_json::json!({
                            "text": r.chunk.text,
                            "score": r.score,
                            "distance": r.distance,
                            "doc_id": r.chunk.doc_id,
                            "metadata": r.chunk.metadata,
                        })
                    })
                    .collect();
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "query": query,
                        "mode": mode,
                        "count": results.len(),
                        "results": items,
                    }))?
                );
            } else {
                println!(
                    "\n{} \"{}\" ({})\n",
                    "Results for:".bold(),
                    query.cyan(),
                    mode.dimmed()
                );

                if results.is_empty() {
                    println!("{}", "No results found.".yellow());
                } else {
                    for (i, result) in results.iter().enumerate() {
                        let source = result
                            .chunk
                            .metadata
                            .get("source")
                            .cloned()
                            .unwrap_or_else(|| "unknown".to_string());
                        let score_color = if result.score > 0.8 {
                            format!("{:.4}", result.score).green()
                        } else if result.score > 0.5 {
                            format!("{:.4}", result.score).yellow()
                        } else {
                            format!("{:.4}", result.score).red()
                        };
                        println!("{} (score: {})", format!("[{}]", i + 1).bold(), score_color);
                        println!("    {}: {}", "Source".dimmed(), source);
                        let preview: String = result.chunk.text.chars().take(200).collect();
                        println!("    {preview}\n");
                    }
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
            let (eff_backend, eff_url, eff_model, eff_db) =
                resolve_params(&backend, &url, &model, &db, &cfg);
            let embedder = make_embedder(eff_backend, eff_url, eff_model);
            let store = make_store(&eff_db, embedder.dimension()).await?;
            let index = DocumentIndex::new(store, embedder);

            let prompt = index.query_for_prompt(&query, top_k).await?;
            println!("{prompt}");
        }

        Commands::Ask {
            query,
            db,
            backend,
            url,
            model,
            llm_model,
            top_k,
            temperature,
        } => {
            let (eff_backend, eff_url, eff_model, eff_db) =
                resolve_params(&backend, &url, &model, &db, &cfg);
            let embedder = make_embedder(eff_backend, eff_url, eff_model);
            let store = make_store(&eff_db, embedder.dimension()).await?;
            let index = DocumentIndex::new(store, embedder);

            // Get context prompt from RAG pipeline
            let prompt = index.query_for_prompt(&query, top_k).await?;

            // Generate answer using LLM
            let config = GeneratorConfig {
                model: llm_model,
                temperature,
                max_tokens: Some(2048),
                system_prompt: Some(
                    "You are a helpful assistant. Answer the question based on the provided context. \
                     If the context doesn't contain relevant information, say so."
                        .to_string(),
                ),
            };
            let generator = raven_embed::create_generator(&backend, Some(&url), config);

            if cli.json {
                let answer = generator
                    .generate(&prompt)
                    .await
                    .map_err(|e| anyhow::anyhow!("{e}"))?;
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "query": query,
                        "answer": answer,
                        "model": generator.model_name(),
                    }))?
                );
            } else {
                // Stream tokens to stdout
                use std::io::Write;
                println!();
                let answer = generator
                    .generate_stream(&prompt, &|token| {
                        print!("{token}");
                        std::io::stdout().flush().ok();
                    })
                    .await
                    .map_err(|e| anyhow::anyhow!("{e}"))?;

                if !answer.ends_with('\n') {
                    println!();
                }
            }
        }

        Commands::Info { db } => {
            let store = make_store(&db, 0).await?;
            let count = store.count().await?;

            if cli.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "database": db.display().to_string(),
                        "chunks": count,
                        "version": env!("CARGO_PKG_VERSION"),
                    }))?
                );
            } else {
                println!("RavenRustRAG Index Info\n");
                println!("  Database: {}", db.display());
                println!("  Chunks:   {count}");
            }
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
            let (eff_backend, eff_url, eff_model, eff_db) =
                resolve_params(&backend, &url, &model, &db, &cfg);
            let embedder = make_embedder(eff_backend, eff_url, eff_model);
            let store = make_store(&eff_db, embedder.dimension()).await?;
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
            println!(
                "   Endpoints: /health /ready /stats /metrics /query /prompt /index /openapi.json"
            );
            println!("   Press Ctrl+C to stop.\n");

            let state = Arc::new(AppState::new(index, config, TextSplitter::new(512, 50)));

            raven_server::serve(state)
                .await
                .map_err(|e| anyhow::anyhow!("{e}"))?;
        }

        Commands::Clear { db } => {
            let store = make_store(&db, 0).await?;
            store.clear().await?;
            println!("✓ Cleared index at {}", db.display());
        }

        Commands::Status { db, url } => {
            let db_exists = db.exists();
            let store = if db_exists {
                make_store(&db, 0).await.ok()
            } else {
                None
            };

            let chunk_count = if let Some(ref s) = store {
                s.count().await.unwrap_or(0)
            } else {
                0
            };

            let schema_ver = if let Some(ref s) = store {
                s.schema_version().await.unwrap_or(0)
            } else {
                0
            };

            // Check Ollama connectivity
            let client = reqwest::Client::new();
            let ollama_ok = client
                .get(format!("{url}/api/tags"))
                .timeout(std::time::Duration::from_secs(2))
                .send()
                .await
                .is_ok_and(|r| r.status().is_success());

            // Check for config file
            let config_file = raven_core::Config::discover();

            if cli.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "database": {
                            "path": db.display().to_string(),
                            "exists": db_exists,
                            "chunks": chunk_count,
                            "schema_version": schema_ver,
                        },
                        "ollama": {
                            "url": url,
                            "reachable": ollama_ok,
                        },
                        "config": config_file.as_ref().map(|p| p.display().to_string()),
                        "version": env!("CARGO_PKG_VERSION"),
                    }))?
                );
            } else {
                println!("\n{}\n", "RavenRustRAG Status".bold());

                // Database
                let db_status = if db_exists {
                    format!(
                        "{} ({chunk_count} chunks, schema v{schema_ver})",
                        "ok".green()
                    )
                } else {
                    "not found".red().to_string()
                };
                println!("  {} {}  {}", "Database:".dimmed(), db.display(), db_status);

                // Ollama
                let ollama_status = if ollama_ok {
                    "reachable".green().to_string()
                } else {
                    "unreachable".red().to_string()
                };
                println!("  {} {}  {}", "Ollama:".dimmed(), url, ollama_status);

                // Config
                if let Some(ref path) = config_file {
                    println!("  {} {}", "Config:".dimmed(), path.display());
                } else {
                    println!(
                        "  {} {}",
                        "Config:".dimmed(),
                        "none (using defaults)".yellow()
                    );
                }

                // Version
                println!("  {} {}", "Version:".dimmed(), env!("CARGO_PKG_VERSION"));
                println!();
            }
        }

        Commands::Export { output, db } => {
            let store = make_store(&db, 0).await?;
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

            let (eff_backend, eff_url, eff_model, eff_db) =
                resolve_params(&backend, &url, &model, &db, &cfg);
            let embedder = make_embedder(eff_backend, eff_url, eff_model);
            let store = make_store(&eff_db, embedder.dimension()).await?;
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
            let (eff_backend, eff_url, eff_model, eff_db) =
                resolve_params(&backend, &url, &model, &db, &cfg);
            let embedder = make_embedder(eff_backend, eff_url, eff_model);
            let store = make_store(&eff_db, embedder.dimension()).await?;
            let index = Arc::new(DocumentIndex::new(store, embedder));
            let splitter = TextSplitter::new(512, 50);

            let server = McpServer::new(index, splitter);
            server
                .run_stdio()
                .await
                .map_err(|e| anyhow::anyhow!("{e}"))?;
        }

        Commands::Doctor { url, db } => {
            let db_status = if db.exists() {
                match SqliteStore::new(&db, 0).await {
                    Ok(store) => {
                        let count = store.count().await.unwrap_or(0);
                        let version = store.schema_version().await.unwrap_or(0);
                        format!("ok ({count} chunks, schema v{version})")
                    }
                    Err(e) => format!("error: {e}"),
                }
            } else {
                "not found".to_string()
            };

            let client = reqwest::Client::new();
            let ollama_status = match client
                .get(format!("{url}/api/tags"))
                .timeout(std::time::Duration::from_secs(3))
                .send()
                .await
            {
                Ok(resp) if resp.status().is_success() => "ok".to_string(),
                Ok(resp) => format!("status: {}", resp.status()),
                Err(e) => format!("unreachable: {e}"),
            };

            if cli.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "version": env!("CARGO_PKG_VERSION"),
                        "database": { "path": db.display().to_string(), "status": db_status },
                        "ollama": { "url": url, "status": ollama_status },
                    }))?
                );
            } else {
                println!("RavenRustRAG Doctor\n");
                print!("  Database ({})... ", db.display());
                println!("{db_status}");
                print!("  Ollama ({url})... ");
                println!("{ollama_status}");
                println!("\n  Version: {}", env!("CARGO_PKG_VERSION"));
            }
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
            let (eff_backend, eff_url, eff_model, eff_db) =
                resolve_params(&backend, &url, &model, &db, &cfg);
            let embedder = make_embedder(eff_backend, eff_url, eff_model);
            let store = make_store(&eff_db, embedder.dimension()).await?;
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

            let docs: Vec<raven_core::Document> = (0..num_docs)
                .map(|i| {
                    raven_core::Document::new(format!(
                        "Document {i} about Rust programming, memory safety, and performance. \
                         Systems programming with zero-cost abstractions and fearless concurrency."
                    ))
                })
                .collect();

            let store = Arc::new(MemoryStore::new());
            let index = DocumentIndex::new(store, embedder);

            let start = std::time::Instant::now();
            index.add_documents(docs, &splitter).await?;
            let index_time = start.elapsed();
            let chunks = index.count().await?;

            if !cli.json {
                println!("  Index:  {num_docs} docs -> {chunks} chunks in {index_time:?}");
                println!(
                    "          {:.1} docs/sec",
                    num_docs as f64 / index_time.as_secs_f64()
                );
            }

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

            if !cli.json {
                println!("\n  Query (vector, {iterations} iterations):");
                println!("          avg: {avg_query:?}  min: {min_query:?}  max: {max_query:?}");
            }

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

            if !cli.json {
                println!("\n  Query (hybrid, {iterations} iterations):");
                println!("          avg: {avg_hybrid:?}  min: {min_hybrid:?}  max: {max_hybrid:?}");
            }

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

            if cli.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "num_docs": num_docs,
                        "chunks": chunks,
                        "iterations": iterations,
                        "index_ms": index_time.as_millis(),
                        "docs_per_sec": num_docs as f64 / index_time.as_secs_f64(),
                        "query_vector_avg_us": avg_query.as_micros(),
                        "query_vector_min_us": min_query.as_micros(),
                        "query_vector_max_us": max_query.as_micros(),
                        "query_hybrid_avg_us": avg_hybrid.as_micros(),
                        "query_hybrid_min_us": min_hybrid.as_micros(),
                        "query_hybrid_max_us": max_hybrid.as_micros(),
                        "bm25_avg_us": avg_bm25.as_micros(),
                    }))?
                );
            } else {
                println!("\n  BM25 ({chunks} chunks, {iterations} iterations):");
                println!("          avg: {avg_bm25:?}");
                println!("\nBenchmark complete.");
            }
        }

        Commands::Graph { action } => match action {
            GraphAction::Build { db, output } => {
                let store = make_store(&db, 0).await?;
                let chunks = store.all().await?;

                if chunks.is_empty() {
                    println!("No documents indexed. Run 'raven index' first.");
                    return Ok(());
                }

                let mut retriever = GraphRetriever::new(KnowledgeGraph::new());
                retriever.build_from_chunks(&chunks);

                let graph = retriever.graph();
                graph.save(&output)?;

                if cli.json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "entities": graph.entity_count(),
                            "edges": graph.edge_count(),
                            "output": output.display().to_string(),
                        }))?
                    );
                } else {
                    println!(
                        "Knowledge graph built: {} entities, {} edges",
                        graph.entity_count(),
                        graph.edge_count()
                    );
                    println!("Saved to: {}", output.display());
                }
            }
            GraphAction::Query {
                query,
                graph: graph_path,
                max_hops,
                top_k,
            } => {
                if !graph_path.exists() {
                    println!(
                        "Graph file not found: {}. Run 'raven graph build' first.",
                        graph_path.display()
                    );
                    return Ok(());
                }

                let kg = KnowledgeGraph::load(&graph_path)?;
                let retriever = GraphRetriever::new(kg).with_max_hops(max_hops);
                let results = retriever.retrieve(&query, top_k);

                if cli.json {
                    let items: Vec<serde_json::Value> = results
                        .iter()
                        .map(|(chunk_id, score)| {
                            serde_json::json!({
                                "chunk_id": chunk_id,
                                "score": score,
                            })
                        })
                        .collect();
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "query": query,
                            "results": items,
                        }))?
                    );
                } else {
                    println!("\nGraph query: \"{query}\" (max_hops={max_hops})\n");
                    if results.is_empty() {
                        println!("No graph results found.");
                    } else {
                        for (i, (chunk_id, score)) in results.iter().enumerate() {
                            println!("[{}] chunk={} score={:.4}", i + 1, chunk_id, score);
                        }
                    }
                }
            }
        },

        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            clap_complete::generate(shell, &mut cmd, "raven", &mut std::io::stdout());
        }

        Commands::Init { output, force } => {
            if output.exists() && !force {
                anyhow::bail!(
                    "Config file {} already exists. Use --force to overwrite.",
                    output.display()
                );
            }

            let config = r#"# RavenRustRAG configuration
# See https://egkristi.github.io/ravenrustrag/configuration/ for details

[embedder]
backend = "ollama"           # "ollama" or "openai"
model = "nomic-embed-text"
url = "http://localhost:11434"
# api_key = ""               # Required for OpenAI backend

[store]
path = "./raven.db"

[splitter]
chunk_size = 512
chunk_overlap = 50

[pipeline]
embed_batch_size = 64
store_batch_size = 100

[server]
host = "127.0.0.1"
port = 8484
# api_key = "your-secret-key"
# cors_origins = ["http://localhost:3000"]
request_timeout_secs = 60
rate_limit_per_second = 100
"#;

            std::fs::write(&output, config)?;
            println!("Created {}", output.display());
        }
    }

    Ok(())
}
