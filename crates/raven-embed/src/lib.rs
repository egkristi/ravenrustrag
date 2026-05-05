//! Embedding backends for RavenRustRAG.
//!
//! Provides the `Embedder` trait and implementations: Ollama, OpenAI, Cached, and Dummy.

use async_trait::async_trait;
use raven_core::{RavenError, Result};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Embedding backend trait
#[async_trait]
pub trait Embedder: Send + Sync {
    /// Embed multiple texts in a single batch
    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>>;

    /// Get the embedding dimension
    fn dimension(&self) -> usize;

    /// Get the model name
    fn model_name(&self) -> &str;
}

// =============================================================================
// Ollama backend
// =============================================================================

pub struct OllamaBackend {
    client: reqwest::Client,
    base_url: String,
    model: String,
    dimension: usize,
}

impl OllamaBackend {
    pub fn new(base_url: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.into(),
            model: model.into(),
            dimension: 768,
        }
    }

    pub fn with_dimension(mut self, dim: usize) -> Self {
        self.dimension = dim;
        self
    }
}

#[derive(Serialize)]
struct OllamaEmbedRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Deserialize)]
struct OllamaEmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

#[async_trait]
impl Embedder for OllamaBackend {
    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let request = OllamaEmbedRequest {
            model: self.model.clone(),
            input: texts.to_vec(),
        };

        let url = format!("{}/api/embed", self.base_url);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| RavenError::Embed(format!("HTTP error: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "unknown error".to_string());
            return Err(RavenError::Embed(format!(
                "Ollama returned {status}: {text}"
            )));
        }

        let body: OllamaEmbedResponse = response
            .json()
            .await
            .map_err(|e| RavenError::Embed(format!("JSON parse error: {e}")))?;

        Ok(body.embeddings)
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}

// =============================================================================
// OpenAI-compatible backend (OpenAI, LM Studio, LocalAI, vLLM, etc.)
// =============================================================================

pub struct OpenAIBackend {
    client: reqwest::Client,
    base_url: String,
    model: String,
    api_key: Option<String>,
    dimension: usize,
}

impl OpenAIBackend {
    pub fn new(base_url: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.into(),
            model: model.into(),
            api_key: None,
            dimension: 1536,
        }
    }

    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    pub fn with_dimension(mut self, dim: usize) -> Self {
        self.dimension = dim;
        self
    }
}

#[derive(Serialize)]
struct OpenAIEmbedRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Deserialize)]
struct OpenAIEmbedResponse {
    data: Vec<OpenAIEmbedData>,
}

#[derive(Deserialize)]
struct OpenAIEmbedData {
    embedding: Vec<f32>,
}

#[async_trait]
impl Embedder for OpenAIBackend {
    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let request = OpenAIEmbedRequest {
            model: self.model.clone(),
            input: texts.to_vec(),
        };

        let url = format!("{}/embeddings", self.base_url);
        let mut req_builder = self
            .client
            .post(&url)
            .json(&request)
            .timeout(std::time::Duration::from_secs(30));

        if let Some(ref key) = self.api_key {
            req_builder = req_builder.bearer_auth(key);
        }

        let response = req_builder
            .send()
            .await
            .map_err(|e| RavenError::Embed(format!("HTTP error: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "unknown error".to_string());
            return Err(RavenError::Embed(format!(
                "OpenAI API returned {status}: {text}"
            )));
        }

        let body: OpenAIEmbedResponse = response
            .json()
            .await
            .map_err(|e| RavenError::Embed(format!("JSON parse error: {e}")))?;

        let embeddings = body.data.into_iter().map(|d| d.embedding).collect();
        Ok(embeddings)
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}

// =============================================================================
// Embedding cache
// =============================================================================

pub struct EmbeddingCache {
    cache: moka::sync::Cache<String, Vec<f32>>,
    hits: AtomicU64,
    misses: AtomicU64,
    size: AtomicU64,
}

impl EmbeddingCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: moka::sync::Cache::new(max_size as u64),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
            size: AtomicU64::new(0),
        }
    }

    #[allow(clippy::unused_async)]
    pub async fn get(&self, text: &str) -> Option<Vec<f32>> {
        let result = self.cache.get(text);
        if result.is_some() {
            self.hits.fetch_add(1, Ordering::Relaxed);
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
        }
        result
    }

    #[allow(clippy::unused_async)]
    pub async fn set(&self, text: String, embedding: Vec<f32>) {
        if self.cache.get(&text).is_none() {
            self.size.fetch_add(1, Ordering::Relaxed);
        }
        self.cache.insert(text, embedding);
    }

    #[allow(clippy::unused_async)]
    pub async fn stats(&self) -> (u64, u64, usize) {
        let hits = self.hits.load(Ordering::Relaxed);
        let misses = self.misses.load(Ordering::Relaxed);
        let size = self.size.load(Ordering::Relaxed) as usize;
        (hits, misses, size)
    }
}

// =============================================================================
// Cached embedder wrapper
// =============================================================================

pub struct CachedEmbedder<E: Embedder> {
    inner: E,
    cache: EmbeddingCache,
}

impl<E: Embedder> CachedEmbedder<E> {
    pub fn new(inner: E, cache_size: usize) -> Self {
        Self {
            inner,
            cache: EmbeddingCache::new(cache_size),
        }
    }

    pub async fn cache_stats(&self) -> (u64, u64, usize) {
        self.cache.stats().await
    }
}

#[async_trait]
impl<E: Embedder> Embedder for CachedEmbedder<E> {
    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let mut results = vec![Vec::new(); texts.len()];
        let mut missing = Vec::new();
        let mut missing_indices = Vec::new();

        for (i, text) in texts.iter().enumerate() {
            if let Some(emb) = self.cache.get(text).await {
                results[i] = emb;
            } else {
                missing.push(text.clone());
                missing_indices.push(i);
            }
        }

        if !missing.is_empty() {
            let embeddings = self.inner.embed(&missing).await?;
            for (idx, (i, emb)) in missing_indices.iter().zip(embeddings).enumerate() {
                self.cache.set(missing[idx].clone(), emb.clone()).await;
                results[*i] = emb;
            }
        }

        Ok(results)
    }

    fn dimension(&self) -> usize {
        self.inner.dimension()
    }

    fn model_name(&self) -> &str {
        self.inner.model_name()
    }
}

// =============================================================================
// Dummy embedder for testing
// =============================================================================

pub struct DummyEmbedder {
    dim: usize,
}

impl DummyEmbedder {
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
}

impl Default for DummyEmbedder {
    fn default() -> Self {
        Self { dim: 3 }
    }
}

#[async_trait]
impl Embedder for DummyEmbedder {
    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        Ok(texts
            .iter()
            .map(|t| {
                // Deterministic but unique: use first bytes of text hash
                let hash = raven_core::fingerprint(t);
                let bytes = hash.as_bytes();
                (0..self.dim)
                    .map(|i| {
                        let b = f32::from(bytes[i % bytes.len()]);
                        (b - 96.0) / 26.0 // Normalize to roughly [-1, 1]
                    })
                    .collect()
            })
            .collect())
    }

    fn dimension(&self) -> usize {
        self.dim
    }

    fn model_name(&self) -> &'static str {
        "dummy"
    }
}

// =============================================================================
// Backend auto-detection
// =============================================================================

/// Create an embedder from a config, auto-detecting the backend.
///
/// Supported URL schemes:
/// - `ollama://model` or `ollama://host:port/model`
/// - `openai://model` or `openai://host:port/model`
/// - Plain URL with backend hint: backend="ollama" or backend="openai"
pub fn create_embedder(
    backend: &str,
    model: &str,
    url: Option<&str>,
    api_key: Option<&str>,
) -> Arc<dyn Embedder> {
    if backend == "openai" {
        let base_url = url.unwrap_or("https://api.openai.com/v1");
        let mut embedder = OpenAIBackend::new(base_url, model);
        if let Some(key) = api_key {
            embedder = embedder.with_api_key(key);
        }
        Arc::new(embedder)
    } else {
        // Default: Ollama
        let base_url = url.unwrap_or("http://localhost:11434");
        Arc::new(OllamaBackend::new(base_url, model))
    }
}

/// Create a cached embedder from config
pub fn create_cached_embedder(
    backend: &str,
    model: &str,
    url: Option<&str>,
    api_key: Option<&str>,
    cache_size: usize,
) -> Arc<dyn Embedder> {
    if backend == "openai" {
        let base_url = url.unwrap_or("https://api.openai.com/v1");
        let mut embedder = OpenAIBackend::new(base_url, model);
        if let Some(key) = api_key {
            embedder = embedder.with_api_key(key);
        }
        Arc::new(CachedEmbedder::new(embedder, cache_size))
    } else {
        let base_url = url.unwrap_or("http://localhost:11434");
        let embedder = OllamaBackend::new(base_url, model);
        Arc::new(CachedEmbedder::new(embedder, cache_size))
    }
}

// =============================================================================
// LLM Generation (text completion)
// =============================================================================

/// LLM text generation backend trait
#[async_trait]
pub trait Generator: Send + Sync {
    /// Generate a completion for the given prompt
    async fn generate(&self, prompt: &str) -> Result<String>;

    /// Generate with streaming — calls callback for each token, returns full text
    async fn generate_stream(
        &self,
        prompt: &str,
        callback: &(dyn Fn(String) + Send + Sync),
    ) -> Result<String>;

    /// Get the model name
    fn model_name(&self) -> &str;
}

/// Configuration for LLM generation
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
    pub system_prompt: Option<String>,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            model: "llama3".to_string(),
            temperature: 0.7,
            max_tokens: Some(2048),
            system_prompt: None,
        }
    }
}

/// Ollama LLM generation backend
pub struct OllamaGenerator {
    client: reqwest::Client,
    base_url: String,
    config: GeneratorConfig,
}

impl OllamaGenerator {
    pub fn new(base_url: impl Into<String>, config: GeneratorConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.into(),
            config,
        }
    }
}

#[derive(Serialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaOptions {
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<u32>,
}

#[derive(Deserialize)]
struct OllamaGenerateResponse {
    response: String,
}

#[derive(Deserialize)]
struct OllamaStreamChunk {
    response: String,
    #[allow(dead_code)]
    done: bool,
}

#[async_trait]
impl Generator for OllamaGenerator {
    async fn generate(&self, prompt: &str) -> Result<String> {
        let request = OllamaGenerateRequest {
            model: self.config.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
            system: self.config.system_prompt.clone(),
            options: OllamaOptions {
                temperature: self.config.temperature,
                num_predict: self.config.max_tokens,
            },
        };

        let url = format!("{}/api/generate", self.base_url);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .timeout(std::time::Duration::from_secs(120))
            .send()
            .await
            .map_err(|e| RavenError::Embed(format!("Ollama generate error: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "unknown error".to_string());
            return Err(RavenError::Embed(format!(
                "Ollama generate returned {status}: {text}"
            )));
        }

        let body: OllamaGenerateResponse = response
            .json()
            .await
            .map_err(|e| RavenError::Embed(format!("JSON parse error: {e}")))?;

        Ok(body.response)
    }

    async fn generate_stream(
        &self,
        prompt: &str,
        callback: &(dyn Fn(String) + Send + Sync),
    ) -> Result<String> {
        let request = OllamaGenerateRequest {
            model: self.config.model.clone(),
            prompt: prompt.to_string(),
            stream: true,
            system: self.config.system_prompt.clone(),
            options: OllamaOptions {
                temperature: self.config.temperature,
                num_predict: self.config.max_tokens,
            },
        };

        let url = format!("{}/api/generate", self.base_url);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .timeout(std::time::Duration::from_secs(300))
            .send()
            .await
            .map_err(|e| RavenError::Embed(format!("Ollama stream error: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "unknown error".to_string());
            return Err(RavenError::Embed(format!(
                "Ollama stream returned {status}: {text}"
            )));
        }

        let mut full_response = String::new();
        let bytes = response
            .bytes()
            .await
            .map_err(|e| RavenError::Embed(format!("Failed to read stream: {e}")))?;

        // Ollama streaming format: newline-delimited JSON
        let text = String::from_utf8_lossy(&bytes).into_owned();
        for line in text.lines() {
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(chunk) = serde_json::from_str::<OllamaStreamChunk>(line) {
                full_response.push_str(&chunk.response);
                callback(chunk.response);
            }
        }

        Ok(full_response)
    }

    fn model_name(&self) -> &str {
        &self.config.model
    }
}

/// Create a generator from configuration
pub fn create_generator(
    _backend: &str,
    url: Option<&str>,
    config: GeneratorConfig,
) -> Arc<dyn Generator> {
    let base_url = url.unwrap_or("http://localhost:11434");
    // Currently only Ollama is supported; OpenAI-compatible can be added later
    Arc::new(OllamaGenerator::new(base_url, config))
}

// =============================================================================
// ONNX Runtime local embedder (optional feature)
// =============================================================================

#[cfg(feature = "onnx")]
pub mod onnx {
    //! Local ONNX Runtime embedding backend (stub).
    //!
    //! Full ONNX integration requires the `ort` and `ndarray` crates, which depend
    //! on `reqwest 0.13`. The workspace currently uses `reqwest 0.11`.
    //!
    //! When the workspace migrates to reqwest 0.12+, add to Cargo.toml:
    //! ```toml
    //! ort = { version = "2.0", default-features = false, features = ["std", "download-binaries"] }
    //! ndarray = "0.16"
    //! ```
    //!
    //! The full OnnxEmbedder implementation is available in git history (commit 7c48ab2).
    //! It supports:
    //! - Loading sentence-transformer ONNX models
    //! - Batch inference with configurable dimensions
    //! - Simple tokenization (production should use the `tokenizers` crate)

    use super::*;

    /// Placeholder ONNX embedder that returns an error indicating ONNX deps are not available.
    pub struct OnnxEmbedder {
        dimension: usize,
    }

    impl OnnxEmbedder {
        pub fn new(
            _model_path: impl Into<std::path::PathBuf>,
            dimension: usize,
        ) -> std::result::Result<Self, String> {
            Ok(Self { dimension })
        }
    }

    #[async_trait::async_trait]
    impl Embedder for OnnxEmbedder {
        async fn embed(&self, _texts: &[String]) -> raven_core::Result<Vec<Vec<f32>>> {
            Err(raven_core::RavenError::Embed(
                "ONNX Runtime not available: requires ort + ndarray crates (reqwest 0.13 conflict). \
                 See raven-embed/Cargo.toml for migration instructions.".to_string()
            ))
        }

        fn dimension(&self) -> usize {
            self.dimension
        }

        fn model_name(&self) -> &str {
            "onnx-stub"
        }
    }
}

#[cfg(feature = "onnx")]
pub use onnx::OnnxEmbedder;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cached_embedder() {
        let embedder = DummyEmbedder::default();
        let cached = CachedEmbedder::new(embedder, 100);

        let texts = vec!["hello".to_string(), "world".to_string()];
        let result1 = cached.embed(&texts).await.unwrap();
        assert_eq!(result1.len(), 2);

        // Second call should hit cache
        let result2 = cached.embed(&texts).await.unwrap();
        assert_eq!(result1, result2);

        let (hits, misses, size) = cached.cache_stats().await;
        assert_eq!(hits, 2);
        assert_eq!(misses, 2);
        assert_eq!(size, 2);
    }

    #[tokio::test]
    async fn test_dummy_embedder_deterministic() {
        let embedder = DummyEmbedder::new(10);
        let texts = vec!["hello".to_string()];
        let r1 = embedder.embed(&texts).await.unwrap();
        let r2 = embedder.embed(&texts).await.unwrap();
        assert_eq!(r1, r2);
        assert_eq!(r1[0].len(), 10);
    }

    #[tokio::test]
    async fn test_dummy_embedder_different_inputs() {
        let embedder = DummyEmbedder::new(8);
        let r1 = embedder.embed(&["hello".to_string()]).await.unwrap();
        let r2 = embedder.embed(&["world".to_string()]).await.unwrap();
        // Different inputs should produce different embeddings
        assert_ne!(r1[0], r2[0]);
    }

    #[tokio::test]
    async fn test_dummy_embedder_batch() {
        let embedder = DummyEmbedder::new(4);
        let texts: Vec<String> = (0..10).map(|i| format!("text {i}")).collect();
        let results = embedder.embed(&texts).await.unwrap();
        assert_eq!(results.len(), 10);
        for emb in &results {
            assert_eq!(emb.len(), 4);
        }
    }

    #[tokio::test]
    async fn test_cached_embedder_empty_input() {
        let embedder = DummyEmbedder::default();
        let cached = CachedEmbedder::new(embedder, 100);
        let result = cached.embed(&[]).await.unwrap();
        assert_eq!(result.len(), 0);
    }

    #[tokio::test]
    async fn test_embedding_cache_direct() {
        let cache = EmbeddingCache::new(10);
        assert_eq!(cache.get("hello").await, None);
        cache.set("hello".to_string(), vec![1.0, 2.0, 3.0]).await;
        assert_eq!(cache.get("hello").await, Some(vec![1.0, 2.0, 3.0]));
        let (hits, misses, size) = cache.stats().await;
        assert_eq!(hits, 1);
        assert_eq!(misses, 1);
        assert_eq!(size, 1);
    }

    #[test]
    fn test_generator_config_default() {
        let config = GeneratorConfig::default();
        assert_eq!(config.model, "llama3");
        assert!((config.temperature - 0.7).abs() < f32::EPSILON);
        assert_eq!(config.max_tokens, Some(2048));
        assert!(config.system_prompt.is_none());
    }

    #[test]
    fn test_create_generator_default() {
        let config = GeneratorConfig::default();
        let gen = create_generator("ollama", None, config);
        assert_eq!(gen.model_name(), "llama3");
    }

    #[test]
    fn test_create_generator_custom_url() {
        let config = GeneratorConfig {
            model: "mistral".to_string(),
            ..Default::default()
        };
        let gen = create_generator("ollama", Some("http://custom:11434"), config);
        assert_eq!(gen.model_name(), "mistral");
    }

    #[test]
    fn test_create_embedder_factory() {
        let embedder = create_embedder("ollama", "nomic-embed-text", None, None);
        assert_eq!(embedder.model_name(), "nomic-embed-text");
    }

    #[test]
    fn test_create_cached_embedder_factory() {
        let cached = create_cached_embedder("ollama", "nomic-embed-text", None, None, 50);
        assert_eq!(cached.model_name(), "nomic-embed-text");
    }

    #[tokio::test]
    async fn test_cached_embedder_mixed_hits_misses() {
        let embedder = DummyEmbedder::new(4);
        let cached = CachedEmbedder::new(embedder, 100);

        // First batch: populate cache with "hello" and "world"
        let batch1 = vec!["hello".to_string(), "world".to_string()];
        let r1 = cached.embed(&batch1).await.unwrap();
        assert_eq!(r1.len(), 2);

        // Second batch: "hello" is cached, "new" is not
        let batch2 = vec!["hello".to_string(), "new".to_string()];
        let r2 = cached.embed(&batch2).await.unwrap();
        assert_eq!(r2.len(), 2);
        // "hello" should return same result from cache
        assert_eq!(r1[0], r2[0]);

        let (hits, misses, size) = cached.cache_stats().await;
        assert_eq!(hits, 1); // "hello" on second call
        assert_eq!(misses, 3); // "hello", "world" on first call + "new" on second
        assert_eq!(size, 3);
    }

    #[tokio::test]
    async fn test_cache_eviction() {
        let cache = EmbeddingCache::new(2);
        cache.set("a".to_string(), vec![1.0]).await;
        cache.set("b".to_string(), vec![2.0]).await;
        assert!(cache.get("a").await.is_some());
        assert!(cache.get("b").await.is_some());
        // Add a third — one should be evicted (moka uses LRU-like)
        cache.set("c".to_string(), vec![3.0]).await;
        assert!(cache.get("c").await.is_some());
    }

    #[test]
    fn test_ollama_backend_builder() {
        let backend =
            OllamaBackend::new("http://localhost:11434", "nomic-embed-text").with_dimension(384);
        assert_eq!(backend.dimension(), 384);
        assert_eq!(backend.model_name(), "nomic-embed-text");
    }

    #[test]
    fn test_openai_backend_builder() {
        let backend = OpenAIBackend::new("https://api.openai.com/v1", "text-embedding-ada-002")
            .with_api_key("sk-test-key")
            .with_dimension(1536);
        assert_eq!(backend.dimension(), 1536);
        assert_eq!(backend.model_name(), "text-embedding-ada-002");
    }

    #[test]
    fn test_dummy_embedder_default() {
        let embedder = DummyEmbedder::default();
        assert_eq!(embedder.dimension(), 3);
        assert_eq!(embedder.model_name(), "dummy");
    }

    #[test]
    fn test_create_embedder_openai() {
        let embedder = create_embedder("openai", "text-embedding-3-small", None, None);
        assert_eq!(embedder.model_name(), "text-embedding-3-small");
    }

    #[test]
    fn test_create_cached_embedder_openai() {
        let cached = create_cached_embedder(
            "openai",
            "text-embedding-3-small",
            Some("http://localhost:1234/v1"),
            Some("sk-key"),
            100,
        );
        assert_eq!(cached.model_name(), "text-embedding-3-small");
    }

    #[tokio::test]
    async fn test_dummy_embedder_empty_input() {
        let embedder = DummyEmbedder::new(5);
        let result = embedder.embed(&[]).await.unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_generator_config_custom() {
        let config = GeneratorConfig {
            model: "mistral".to_string(),
            temperature: 0.1,
            max_tokens: Some(512),
            system_prompt: Some("You are helpful.".to_string()),
        };
        assert_eq!(config.model, "mistral");
        assert!((config.temperature - 0.1).abs() < f32::EPSILON);
        assert_eq!(config.max_tokens, Some(512));
        assert_eq!(config.system_prompt, Some("You are helpful.".to_string()));
    }
}
