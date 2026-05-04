use anyhow::Result;
use clap::{Parser, Subcommand};
use raven_core::Config;
use raven_embed::{OllamaBackend, CachedEmbedder};
use raven_load::Loader;
use raven_search::DocumentIndex;
use raven_split::TextSplitter;
use raven_store::SqliteStore;
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
    },

    /// Query the index
    Query {
        /// Search query
        query: String,

        /// Database path
        #[arg(short, long, default_value = "./raven.db")]
        db: PathBuf,

        /// Ollama URL
        #[arg(short, long, default_value = "http://localhost:11434")]
        url: String,

        /// Embedding model
        #[arg(short, long, default_value = "nomic-embed-text")]
        model: String,

        /// Number of results
        #[arg(short, long, default_value_t = 5)]
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
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,

        /// Port
        #[arg(short, long, default_value_t = 8484)]
        port: u16,

        /// Database path
        #[arg(short, long, default_value = "./raven.db")]
        db: PathBuf,

        /// Ollama URL
        #[arg(short, long, default_value = "http://localhost:11434")]
        url: String,

        /// Embedding model
        #[arg(short, long, default_value = "nomic-embed-text")]
        model: String,
    },

    /// Clear the index
    Clear {
        /// Database path
        #[arg(short, long, default_value = "./raven.db")]
        db: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(if cli.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        });
    subscriber.init();

    match cli.command {
        Commands::Index {
            path,
            db,
            url,
            model,
            chunk_size,
            chunk_overlap,
        } => {
            info!("Indexing documents from {:?}", path);

            // Load documents
            let docs = Loader::from_directory(&path, Some(&["txt", "md"]))?;
            info!("Loaded {} documents", docs.len());

            if docs.is_empty() {
                warn!("No documents found in {:?}", path);
                return Ok(());
            }

            // Setup embedder and store
            let embedder = Arc::new(OllamaBackend::new(&url, &model
            ).with_dimension(768));
            let cached_embedder = Arc::new(CachedEmbedder::new(embedder, 1000));
            
            let store = Arc::new(SqliteStore::new(&db, 768).await?);
            let index = DocumentIndex::new(store, cached_embedder);

            // Split and index
            let splitter = TextSplitter::new(chunk_size, chunk_overlap);
            
            let pb = indicatif::ProgressBar::new(docs.len() as u64);
            pb.set_style(
                indicatif::ProgressStyle::default_bar()
                    .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
                    .unwrap()
            );

            // Process in batches for progress
            let batch_size = 10;
            for batch in docs.chunks(batch_size) {
                let batch_docs: Vec<_> = batch.to_vec();
                let batch_len = batch_docs.len();
                index.add_documents(batch_docs, &splitter).await?;
                pb.inc(batch_len as u64);
                pb.set_message(format!("Indexed {} docs", pb.position()));
            }

            pb.finish_with_message(format!("Indexed {} documents", pb.position()));
            info!("Index complete: {} chunks", index.count().await?);
        }

        Commands::Query {
            query,
            db,
            url,
            model,
            top_k,
        } => {
            info!("Querying: {}", query);

            let embedder = Arc::new(OllamaBackend::new(&url, &model
            ).with_dimension(768));
            let store = Arc::new(SqliteStore::new(&db, 768).await?);
            let index = DocumentIndex::new(store, embedder);

            let results = index.query(&query, top_k).await?;

            println!("\n🐦‍⬛ Query results for: \"{}\"\n", query);
            
            if results.is_empty() {
                println!("No results found.");
            } else {
                for (i, result) in results.iter().enumerate() {
                    let source = result.chunk.metadata.get("source")
                        .unwrap_or(&"unknown".to_string());
                    println!("[{}] (score: {:.4})", i + 1, result.score);
                    println!("    Source: {}", source);
                    println!("    {}\n", &result.chunk.text.chars().take(200).collect::<String>());
                }
            }
        }

        Commands::Info { db } => {
            let store = Arc::new(SqliteStore::new(&db, 768).await?);
            let count = store.count().await?;
            
            println!("🐦‍⬛ RavenRustRAG Index Info\n");
            println!("  Database: {}", db.display());
            println!("  Documents: {}", count);
        }

        Commands::Serve {
            host,
            port,
            db,
            url,
            model,
        } => {
            info!("Starting server on {}:{}", host, port);
            println!("🐦‍⬛ RavenRustRAG server starting on http://{}:{}", host, port);
            println!("   Database: {}", db.display());
            println!("   Model: {} ({})", model, url);
            println!("   Press Ctrl+C to stop.\n");
            
            // TODO: Implement server in raven-server crate
            println!("Server mode not yet implemented. Use `raven query` instead.");
        }

        Commands::Clear { db } => {
            let store = Arc::new(SqliteStore::new(&db, 768).await?);
            store.clear().await?;
            info!("Cleared index: {}", db.display());
            println!("✓ Cleared index at {}", db.display());
        }
    }

    Ok(())
}