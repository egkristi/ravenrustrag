//! Core types and configuration for RavenRustRAG.
//!
//! Provides `Document`, `Chunk`, `SearchResult`, `Config`, `RavenError`, and fingerprinting.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tracing::info;
use uuid::Uuid;

/// A raw document before splitting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub text: String,
    pub metadata: HashMap<String, String>,
}

impl Document {
    /// Create a new document with auto-generated UUID and given text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            text: text.into(),
            metadata: HashMap::new(),
        }
    }

    /// Add a metadata key-value pair (builder pattern).
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Override the auto-generated ID.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }
}

/// A chunk of a document, ready for embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub id: String,
    pub doc_id: String,
    pub text: String,
    pub metadata: HashMap<String, String>,
    pub embedding: Option<Vec<f32>>,
}

impl Chunk {
    pub fn new(doc_id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            doc_id: doc_id.into(),
            text: text.into(),
            metadata: HashMap::new(),
            embedding: None,
        }
    }

    pub fn with_embedding(mut self, embedding: Vec<f32>) -> Self {
        self.embedding = Some(embedding);
        self
    }
}

/// A search result with score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub chunk: Chunk,
    pub score: f32,
    pub distance: f32,
}

/// Fingerprint for incremental indexing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fingerprint {
    pub path: String,
    pub content_hash: String,
    pub modified: chrono::DateTime<chrono::Utc>,
}

/// Main error type
#[derive(Error, Debug)]
pub enum RavenError {
    #[error("store error: {0}")]
    Store(String),

    #[error("embedder error: {0}")]
    Embed(String),

    #[error("splitter error: {0}")]
    Split(String),

    #[error("loader error: {0}")]
    Load(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("config error: {0}")]
    Config(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, RavenError>;

/// Configuration for RavenRustRAG
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub embedder: EmbedderConfig,
    pub store: StoreConfig,
    pub splitter: SplitterConfig,
    pub server: ServerConfig,
    #[serde(default)]
    pub context: ContextConfig,
    #[serde(default)]
    pub pipeline: PipelineConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedderConfig {
    pub backend: String,
    pub model: String,
    pub url: Option<String>,
}

impl Default for EmbedderConfig {
    fn default() -> Self {
        Self {
            backend: "ollama".to_string(),
            model: "nomic-embed-text".to_string(),
            url: Some("http://localhost:11434".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreConfig {
    pub backend: String,
    pub path: String,
}

impl Default for StoreConfig {
    fn default() -> Self {
        Self {
            backend: "sqlite".to_string(),
            path: "./raven.db".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitterConfig {
    pub kind: String,
    pub chunk_size: usize,
    pub chunk_overlap: usize,
}

impl Default for SplitterConfig {
    fn default() -> Self {
        Self {
            kind: "text".to_string(),
            chunk_size: 512,
            chunk_overlap: 50,
        }
    }
}

/// Pipeline tuning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    /// Batch size for embedding operations
    #[serde(default = "default_embed_batch_size")]
    pub embed_batch_size: usize,
    /// Batch size for store operations
    #[serde(default = "default_store_batch_size")]
    pub store_batch_size: usize,
}

fn default_embed_batch_size() -> usize {
    64
}

fn default_store_batch_size() -> usize {
    100
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            embed_batch_size: default_embed_batch_size(),
            store_batch_size: default_store_batch_size(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub api_key: Option<String>,
    #[serde(default)]
    pub cors_origins: Vec<String>,
    #[serde(default = "default_request_timeout")]
    pub request_timeout_secs: u64,
    #[serde(default = "default_rate_limit")]
    pub rate_limit_per_second: u32,
    #[serde(default = "default_max_query_length")]
    pub max_query_length: usize,
    #[serde(default)]
    pub public_stats: bool,
}

fn default_request_timeout() -> u64 {
    60
}

fn default_rate_limit() -> u32 {
    100
}

fn default_max_query_length() -> usize {
    10_000
}

/// Context formatting configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContextConfig {
    pub template: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8484,
            api_key: None,
            cors_origins: Vec::new(),
            request_timeout_secs: default_request_timeout(),
            rate_limit_per_second: default_rate_limit(),
            max_query_length: default_max_query_length(),
            public_stats: false,
        }
    }
}

impl Config {
    /// Load config: try explicit path, then auto-discover raven.toml, then defaults.
    /// Always applies env var overrides on top.
    pub fn load(path: Option<&Path>) -> Result<Self> {
        let mut config = if let Some(p) = path {
            Self::from_file(p)?
        } else if let Some(found) = Self::discover() {
            info!("Using config: {}", found.display());
            Self::from_file(&found)?
        } else {
            Self::default()
        };
        config.apply_env_overrides();
        Ok(config)
    }

    /// Read config from a TOML file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)
            .map_err(|e| RavenError::Config(format!("TOML parse error: {e}")))?;

        // Warn about unknown top-level keys
        if let Ok(raw) = content.parse::<toml::Table>() {
            let known_keys = [
                "embedder", "store", "splitter", "server", "context", "pipeline",
            ];
            for key in raw.keys() {
                if !known_keys.contains(&key.as_str()) {
                    tracing::warn!("Unknown config key: '{key}' (possible typo?)");
                }
            }
        }

        Ok(config)
    }

    /// Walk up from cwd looking for raven.toml
    pub fn discover() -> Option<PathBuf> {
        let mut dir = std::env::current_dir().ok()?;
        loop {
            let candidate = dir.join("raven.toml");
            if candidate.is_file() {
                return Some(candidate);
            }
            if !dir.pop() {
                return None;
            }
        }
    }

    /// Override config fields from environment variables
    pub fn apply_env_overrides(&mut self) {
        if let Ok(val) = std::env::var("RAVEN_DB") {
            self.store.path = val;
        }
        if let Ok(val) = std::env::var("RAVEN_MODEL") {
            self.embedder.model = val;
        }
        if let Ok(val) = std::env::var("RAVEN_API_KEY") {
            self.server.api_key = Some(val);
        }
        if let Ok(val) = std::env::var("RAVEN_EMBED_URL") {
            self.embedder.url = Some(val);
        }
        if let Ok(val) = std::env::var("RAVEN_EMBED_BACKEND") {
            self.embedder.backend = val;
        }
        if let Ok(val) = std::env::var("RAVEN_HOST") {
            self.server.host = val;
        }
        if let Ok(val) = std::env::var("RAVEN_PORT") {
            if let Ok(port) = val.parse() {
                self.server.port = port;
            }
        }
        if let Ok(val) = std::env::var("RAVEN_CHUNK_SIZE") {
            if let Ok(size) = val.parse() {
                self.splitter.chunk_size = size;
            }
        }
        if let Ok(val) = std::env::var("RAVEN_CHUNK_OVERLAP") {
            if let Ok(overlap) = val.parse() {
                self.splitter.chunk_overlap = overlap;
            }
        }
        if let Ok(val) = std::env::var("RAVEN_CORS_ORIGINS") {
            self.server.cors_origins = val.split(',').map(|s| s.trim().to_string()).collect();
        }
        if let Ok(val) = std::env::var("RAVEN_REQUEST_TIMEOUT") {
            if let Ok(timeout) = val.parse() {
                self.server.request_timeout_secs = timeout;
            }
        }
        if let Ok(val) = std::env::var("RAVEN_RATE_LIMIT") {
            if let Ok(rate) = val.parse() {
                self.server.rate_limit_per_second = rate;
            }
        }
        if let Ok(val) = std::env::var("RAVEN_MAX_QUERY_LENGTH") {
            if let Ok(len) = val.parse() {
                self.server.max_query_length = len;
            }
        }
        if let Ok(val) = std::env::var("RAVEN_PUBLIC_STATS") {
            self.server.public_stats = val == "true" || val == "1";
        }
        if let Ok(val) = std::env::var("RAVEN_EMBED_BATCH_SIZE") {
            if let Ok(size) = val.parse() {
                self.pipeline.embed_batch_size = size;
            }
        }
        if let Ok(val) = std::env::var("RAVEN_STORE_BATCH_SIZE") {
            if let Ok(size) = val.parse() {
                self.pipeline.store_batch_size = size;
            }
        }
    }
}

/// Compute SHA-256 content hash for incremental indexing and deduplication.
pub fn fingerprint(text: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// SIMD-friendly cosine similarity between two f32 slices.
///
/// Written in a loop structure that LLVM auto-vectorizes to SIMD instructions
/// (SSE/AVX on x86, NEON on ARM). Uses chunks_exact for safe vectorization hint.
#[inline]
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    debug_assert_eq!(a.len(), b.len(), "vectors must have equal length");
    let n = a.len().min(b.len());
    if n == 0 {
        return 0.0;
    }

    let mut dot = 0.0_f32;
    let mut norm_a = 0.0_f32;
    let mut norm_b = 0.0_f32;

    // chunks_exact(4) enables LLVM auto-vectorization
    let a_chunks = a[..n].chunks_exact(4);
    let b_chunks = b[..n].chunks_exact(4);
    let a_rem = a_chunks.remainder();
    let b_rem = b_chunks.remainder();

    for (ac, bc) in a_chunks.zip(b_chunks) {
        dot += ac[0] * bc[0] + ac[1] * bc[1] + ac[2] * bc[2] + ac[3] * bc[3];
        norm_a += ac[0] * ac[0] + ac[1] * ac[1] + ac[2] * ac[2] + ac[3] * ac[3];
        norm_b += bc[0] * bc[0] + bc[1] * bc[1] + bc[2] * bc[2] + bc[3] * bc[3];
    }

    // Handle remainder
    for (ai, bi) in a_rem.iter().zip(b_rem.iter()) {
        dot += ai * bi;
        norm_a += ai * ai;
        norm_b += bi * bi;
    }

    let denom = norm_a.sqrt() * norm_b.sqrt();
    if denom == 0.0 {
        0.0
    } else {
        dot / denom
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new("Hello world").with_metadata("source", "test.txt");

        assert_eq!(doc.text, "Hello world");
        assert_eq!(doc.metadata.get("source"), Some(&"test.txt".to_string()));
        assert!(!doc.id.is_empty());
    }

    #[test]
    fn test_fingerprint() {
        let fp1 = fingerprint("hello");
        let fp2 = fingerprint("hello");
        let fp3 = fingerprint("world");

        assert_eq!(fp1, fp2);
        assert_ne!(fp1, fp3);
        assert_eq!(fp1.len(), 64); // SHA-256 hex
    }

    #[test]
    fn test_config_defaults() {
        let config = Config::default();
        assert_eq!(config.embedder.backend, "ollama");
        assert_eq!(config.store.path, "./raven.db");
        assert_eq!(config.server.port, 8484);
        assert_eq!(config.splitter.chunk_size, 512);
    }

    #[test]
    fn test_config_from_toml() {
        let toml_str = r#"
[embedder]
backend = "openai"
model = "text-embedding-3-small"
url = "https://api.openai.com"

[store]
backend = "sqlite"
path = "/tmp/test.db"

[splitter]
kind = "text"
chunk_size = 256
chunk_overlap = 25

[server]
host = "0.0.0.0"
port = 9090
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.embedder.backend, "openai");
        assert_eq!(config.store.path, "/tmp/test.db");
        assert_eq!(config.server.port, 9090);
        assert_eq!(config.splitter.chunk_size, 256);
    }

    #[test]
    fn test_config_pipeline_defaults() {
        let config = Config::default();
        assert_eq!(config.pipeline.embed_batch_size, 64);
        assert_eq!(config.pipeline.store_batch_size, 100);
    }

    #[test]
    fn test_config_pipeline_from_toml() {
        let toml_str = r#"
[embedder]
backend = "ollama"
model = "nomic-embed-text"

[store]
backend = "sqlite"
path = "./test.db"

[splitter]
kind = "text"
chunk_size = 512
chunk_overlap = 50

[server]
host = "127.0.0.1"
port = 8484

[pipeline]
embed_batch_size = 32
store_batch_size = 50
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.pipeline.embed_batch_size, 32);
        assert_eq!(config.pipeline.store_batch_size, 50);
    }

    #[test]
    fn test_config_bad_toml() {
        let result: std::result::Result<Config, _> = toml::from_str("invalid [[ toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_raven_error_display() {
        let err = RavenError::Store("test error".to_string());
        assert_eq!(format!("{err}"), "store error: test error");

        let err = RavenError::NotFound("missing".to_string());
        assert_eq!(format!("{err}"), "not found: missing");
    }

    #[test]
    fn test_chunk_creation() {
        let chunk = Chunk::new("doc-1", "hello world");
        assert_eq!(chunk.doc_id, "doc-1");
        assert_eq!(chunk.text, "hello world");
        assert!(chunk.embedding.is_none());
        assert!(!chunk.id.is_empty());

        let chunk = chunk.with_embedding(vec![1.0, 2.0, 3.0]);
        assert_eq!(chunk.embedding.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn test_document_with_id() {
        let doc = Document::new("text").with_id("custom-id");
        assert_eq!(doc.id, "custom-id");
    }

    #[test]
    fn test_cosine_similarity_identical() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert!((cosine_similarity(&a, &a) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0, 0.0];
        assert!(cosine_similarity(&a, &b).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![-1.0, -2.0, -3.0];
        assert!((cosine_similarity(&a, &b) + 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_zero() {
        let a = vec![0.0, 0.0, 0.0];
        let b = vec![1.0, 2.0, 3.0];
        assert_eq!(cosine_similarity(&a, &b), 0.0);
    }

    #[test]
    fn test_cosine_similarity_empty() {
        assert_eq!(cosine_similarity(&[], &[]), 0.0);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Cosine similarity of any vector with itself is 1.0 (if non-zero)
        #[test]
        fn cosine_self_is_one(v in proptest::collection::vec(-10.0f32..10.0f32, 1..128)) {
            let has_nonzero = v.iter().any(|x| *x != 0.0);
            if has_nonzero {
                let sim = cosine_similarity(&v, &v);
                prop_assert!((sim - 1.0).abs() < 1e-5, "Expected ~1.0, got {sim}");
            }
        }

        /// Cosine similarity is always in [-1.0, 1.0]
        #[test]
        fn cosine_in_range(
            a in proptest::collection::vec(-100.0f32..100.0f32, 1..128usize),
            b_raw in proptest::collection::vec(-100.0f32..100.0f32, 1..128usize),
        ) {
            // Ensure same length
            let len = a.len().min(b_raw.len());
            let a = &a[..len];
            let b = &b_raw[..len];
            let sim = cosine_similarity(a, b);
            prop_assert!((-1.0 - 1e-5..=1.0 + 1e-5).contains(&sim),
                "Cosine similarity {} out of range [-1, 1]", sim);
        }

        /// Cosine similarity is symmetric: cos(a, b) == cos(b, a)
        #[test]
        fn cosine_is_symmetric(
            a in proptest::collection::vec(-10.0f32..10.0f32, 1..64usize),
            b_raw in proptest::collection::vec(-10.0f32..10.0f32, 1..64usize),
        ) {
            let len = a.len().min(b_raw.len());
            let a = &a[..len];
            let b = &b_raw[..len];
            let ab = cosine_similarity(a, b);
            let ba = cosine_similarity(b, a);
            prop_assert!((ab - ba).abs() < 1e-6, "cos(a,b)={ab} != cos(b,a)={ba}");
        }

        /// Fingerprint is deterministic
        #[test]
        fn fingerprint_deterministic(content in ".*") {
            let h1 = fingerprint(&content);
            let h2 = fingerprint(&content);
            prop_assert_eq!(h1, h2);
        }

        /// Fingerprint is always 64 hex chars (SHA-256)
        #[test]
        fn fingerprint_length(content in ".*") {
            let h = fingerprint(&content);
            prop_assert_eq!(h.len(), 64);
            prop_assert!(h.chars().all(|c| c.is_ascii_hexdigit()));
        }

        /// Different content produces different fingerprints (probabilistic)
        #[test]
        fn fingerprint_different(a in ".{1,100}", b in ".{1,100}") {
            if a != b {
                prop_assert_ne!(fingerprint(&a), fingerprint(&b));
            }
        }

        /// Config serialize/deserialize roundtrip
        #[test]
        fn config_roundtrip(
            port in 1u16..65535u16,
            chunk_size in 10usize..10000usize,
            chunk_overlap in 1usize..100usize,
        ) {
            prop_assume!(chunk_overlap < chunk_size);
            let config = Config {
                server: ServerConfig {
                    port,
                    ..ServerConfig::default()
                },
                splitter: SplitterConfig {
                    chunk_size,
                    chunk_overlap,
                    ..SplitterConfig::default()
                },
                ..Config::default()
            };
            let toml_str = toml::to_string(&config).unwrap();
            let decoded: Config = toml::from_str(&toml_str).unwrap();
            prop_assert_eq!(config.server.port, decoded.server.port);
            prop_assert_eq!(config.splitter.chunk_size, decoded.splitter.chunk_size);
            prop_assert_eq!(config.splitter.chunk_overlap, decoded.splitter.chunk_overlap);
        }
    }
}
