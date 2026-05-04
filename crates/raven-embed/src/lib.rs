use async_trait::async_trait;
use raven_core::{RavenError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

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
    cache: Arc<Mutex<HashMap<String, Vec<f32>>>>,
    max_size: usize,
    hits: Arc<Mutex<u64>>,
    misses: Arc<Mutex<u64>>,
}

impl EmbeddingCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            max_size,
            hits: Arc::new(Mutex::new(0)),
            misses: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn get(&self, text: &str) -> Option<Vec<f32>> {
        let cache = self.cache.lock().await;
        let result = cache.get(text).cloned();
        if result.is_some() {
            *self.hits.lock().await += 1;
        } else {
            *self.misses.lock().await += 1;
        }
        result
    }

    pub async fn set(&self, text: String, embedding: Vec<f32>) {
        let mut cache = self.cache.lock().await;
        if cache.len() >= self.max_size && !cache.contains_key(&text) {
            let key_to_remove = cache.keys().next().cloned();
            if let Some(key) = key_to_remove {
                cache.remove(&key);
            }
        }
        cache.insert(text, embedding);
    }

    pub async fn stats(&self) -> (u64, u64, usize) {
        let hits = *self.hits.lock().await;
        let misses = *self.misses.lock().await;
        let size = self.cache.lock().await.len();
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
    match backend {
        "openai" => {
            let base_url = url.unwrap_or("https://api.openai.com/v1");
            let mut embedder = OpenAIBackend::new(base_url, model);
            if let Some(key) = api_key {
                embedder = embedder.with_api_key(key);
            }
            Arc::new(embedder)
        }
        _ => {
            // Default: Ollama
            let base_url = url.unwrap_or("http://localhost:11434");
            Arc::new(OllamaBackend::new(base_url, model))
        }
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
    match backend {
        "openai" => {
            let base_url = url.unwrap_or("https://api.openai.com/v1");
            let mut embedder = OpenAIBackend::new(base_url, model);
            if let Some(key) = api_key {
                embedder = embedder.with_api_key(key);
            }
            Arc::new(CachedEmbedder::new(embedder, cache_size))
        }
        _ => {
            let base_url = url.unwrap_or("http://localhost:11434");
            let embedder = OllamaBackend::new(base_url, model);
            Arc::new(CachedEmbedder::new(embedder, cache_size))
        }
    }
}

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
}
