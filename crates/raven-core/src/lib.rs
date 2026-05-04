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
            let known_keys = ["embedder", "store", "splitter", "server", "context"];
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
    }
}

/// Compute SHA-256 content hash for incremental indexing and deduplication.
pub fn fingerprint(text: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    format!("{:x}", hasher.finalize())
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
}
