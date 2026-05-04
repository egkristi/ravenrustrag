use async_trait::async_trait;
use raven_core::{Chunk, RavenError, Result};
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

/// Ollama embedding backend
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
            dimension: 768, // nomic-embed-text default
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
            .send()
            .await
            .map_err(|e| RavenError::Embed(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "unknown error".to_string());
            return Err(RavenError::Embed(format!(
                "Ollama returned {}: {}",
                status, text
            )));
        }

        let body: OllamaEmbedResponse = response
            .json()
            .await
            .map_err(|e| RavenError::Embed(format!("JSON parse error: {}", e)))?;

        Ok(body.embeddings)
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}

/// Simple in-memory LRU cache for embeddings
pub struct EmbeddingCache {
    cache: Arc<Mutex<HashMap<String, Vec<f32>>>>,
    max_size: usize,
}

impl EmbeddingCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            max_size,
        }
    }

    pub async fn get(&self, text: &str) -> Option<Vec<f32>> {
        let cache = self.cache.lock().await;
        cache.get(text).cloned()
    }

    pub async fn set(&self, text: String, embedding: Vec<f32>) {
        let mut cache = self.cache.lock().await;
        if cache.len() >= self.max_size && !cache.contains_key(&text) {
            // Simple eviction: remove first key
            let key_to_remove = cache.keys().next().cloned();
            if let Some(key) = key_to_remove {
                cache.remove(&key);
            }
        }
        cache.insert(text, embedding);
    }
}

/// Cached embedder wrapper
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
}

#[async_trait]
impl<E: Embedder> Embedder for CachedEmbedder<E> {
    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let mut results = vec![Vec::new(); texts.len()];
        let mut missing = Vec::new();
        let mut missing_indices = Vec::new();

        // Check cache
        for (i, text) in texts.iter().enumerate() {
            if let Some(emb) = self.cache.get(text).await {
                results[i] = emb;
            } else {
                missing.push(text.clone());
                missing_indices.push(i);
            }
        }

        // Embed missing
        if !missing.is_empty() {
            let embeddings = self.inner.embed(&missing).await?;
            for (idx, (i, emb)) in missing_indices.iter().zip(embeddings.into_iter()).enumerate() {
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

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyEmbedder;

    #[async_trait]
    impl Embedder for DummyEmbedder {
        async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
            Ok(texts.iter().map(|_| vec![1.0, 2.0, 3.0]).collect())
        }

        fn dimension(&self) -> usize {
            3
        }

        fn model_name(&self) -> &str {
            "dummy"
        }
    }

    #[tokio::test]
    async fn test_cached_embedder() {
        let embedder = DummyEmbedder;
        let cached = CachedEmbedder::new(embedder, 100);

        let texts = vec!["hello".to_string(), "world".to_string()];
        let result1 = cached.embed(&texts).await.unwrap();
        assert_eq!(result1.len(), 2);

        // Second call should hit cache
        let result2 = cached.embed(&texts).await.unwrap();
        assert_eq!(result1, result2);
    }
}