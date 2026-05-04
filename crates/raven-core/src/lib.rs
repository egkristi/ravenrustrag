use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// A raw document before splitting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub text: String,
    pub metadata: HashMap<String, String>,
}

impl Document {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            text: text.into(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub embedder: EmbedderConfig,
    pub store: StoreConfig,
    pub splitter: SplitterConfig,
    pub server: ServerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            embedder: EmbedderConfig::default(),
            store: StoreConfig::default(),
            splitter: SplitterConfig::default(),
            server: ServerConfig::default(),
        }
    }
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
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8484,
            api_key: None,
        }
    }
}

/// Compute content fingerprint
pub fn fingerprint(text: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new("Hello world")
            .with_metadata("source", "test.txt");
        
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
}