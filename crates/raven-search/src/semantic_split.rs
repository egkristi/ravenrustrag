//! Semantic splitter: groups sentences by embedding similarity.
//!
//! Splits text into sentences, embeds each sentence, then groups consecutive
//! sentences whose cosine similarity exceeds a threshold into chunks.

use raven_core::{Chunk, Document, Result};
use raven_embed::Embedder;
use std::sync::Arc;

/// Splits documents by grouping semantically similar consecutive sentences.
///
/// Algorithm:
/// 1. Split text into sentences (reuses `SentenceSplitter` logic)
/// 2. Embed each sentence
/// 3. Compare consecutive sentence embeddings via cosine similarity
/// 4. Split when similarity drops below threshold
pub struct SemanticSplitter {
    embedder: Arc<dyn Embedder>,
    /// Cosine similarity threshold: split when consecutive sentences
    /// drop below this value (default 0.5)
    threshold: f32,
    /// Maximum chunk size in characters (hard limit)
    max_chunk_chars: usize,
}

impl SemanticSplitter {
    pub fn new(embedder: Arc<dyn Embedder>) -> Self {
        Self {
            embedder,
            threshold: 0.5,
            max_chunk_chars: 2000,
        }
    }

    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }

    pub fn with_max_chunk_chars(mut self, max: usize) -> Self {
        self.max_chunk_chars = max;
        self
    }

    /// Split documents into semantically coherent chunks.
    pub async fn split(&self, documents: Vec<Document>) -> Result<Vec<Chunk>> {
        let mut all_chunks = Vec::new();

        for doc in documents {
            let sentences = split_sentences(&doc.text);
            if sentences.is_empty() {
                continue;
            }

            if sentences.len() == 1 {
                let mut chunk = Chunk::new(&doc.id, &doc.text);
                doc.metadata.clone_into(&mut chunk.metadata);
                chunk
                    .metadata
                    .insert("chunk_index".to_string(), "0".to_string());
                chunk
                    .metadata
                    .insert("source_id".to_string(), doc.id.clone());
                all_chunks.push(chunk);
                continue;
            }

            // Embed all sentences
            let sentence_strings: Vec<String> = sentences.iter().map(ToString::to_string).collect();
            let embeddings = self.embedder.embed(&sentence_strings).await?;

            // Group by consecutive similarity
            let groups = self.group_by_similarity(&sentences, &embeddings);

            for (chunk_index, group) in groups.iter().enumerate() {
                let text = group.join(" ");
                let mut chunk = Chunk::new(&doc.id, &text);
                doc.metadata.clone_into(&mut chunk.metadata);
                chunk
                    .metadata
                    .insert("chunk_index".to_string(), chunk_index.to_string());
                chunk
                    .metadata
                    .insert("source_id".to_string(), doc.id.clone());
                all_chunks.push(chunk);
            }
        }

        Ok(all_chunks)
    }

    fn group_by_similarity(
        &self,
        sentences: &[String],
        embeddings: &[Vec<f32>],
    ) -> Vec<Vec<String>> {
        let mut groups: Vec<Vec<String>> = Vec::new();
        let mut current_group: Vec<String> = vec![sentences[0].clone()];
        let mut current_len = sentences[0].len();

        for i in 1..sentences.len() {
            let sim = raven_core::cosine_similarity(&embeddings[i - 1], &embeddings[i]);
            let would_exceed = current_len + sentences[i].len() + 1 > self.max_chunk_chars;

            if sim < self.threshold || would_exceed {
                groups.push(std::mem::take(&mut current_group));
                current_len = 0;
            }

            current_group.push(sentences[i].clone());
            current_len += sentences[i].len() + 1;
        }

        if !current_group.is_empty() {
            groups.push(current_group);
        }

        groups
    }
}

/// Simple sentence splitter (extracted from raven-split logic)
fn split_sentences(text: &str) -> Vec<String> {
    let mut sentences = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let ch = chars[i];
        current.push(ch);

        if (ch == '.' || ch == '!' || ch == '?') && current.len() > 1 {
            let next_is_boundary = if i + 1 >= len {
                true
            } else if chars[i + 1].is_whitespace() {
                let mut j = i + 1;
                while j < len && chars[j].is_whitespace() {
                    j += 1;
                }
                j >= len || chars[j].is_uppercase() || chars[j] == '"' || chars[j] == '\''
            } else {
                false
            };

            if next_is_boundary {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() {
                    sentences.push(trimmed);
                    current = String::new();
                }
            }
        }

        i += 1;
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        sentences.push(trimmed);
    }

    sentences
}

#[cfg(test)]
mod tests {
    use super::*;
    use raven_embed::DummyEmbedder;

    #[tokio::test]
    async fn test_semantic_splitter_single_sentence() {
        let embedder = Arc::new(DummyEmbedder::new(8));
        let splitter = SemanticSplitter::new(embedder);

        let docs = vec![Document::new("Just one sentence here.")];
        let chunks = splitter.split(docs).await.unwrap();
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].text, "Just one sentence here.");
    }

    #[tokio::test]
    async fn test_semantic_splitter_multiple() {
        let embedder = Arc::new(DummyEmbedder::new(8));
        // With DummyEmbedder, each sentence gets a unique embedding based on content hash,
        // so consecutive sentences will have varying similarity.
        let splitter = SemanticSplitter::new(embedder).with_threshold(0.99);

        let docs = vec![Document::new(
            "Rust is a systems programming language. Python is an interpreted language. \
             The weather today is sunny and warm. Cats are independent animals.",
        )];
        let chunks = splitter.split(docs).await.unwrap();
        // With very high threshold, most sentence pairs will split
        assert!(chunks.len() > 1);

        for (i, chunk) in chunks.iter().enumerate() {
            assert_eq!(chunk.metadata.get("chunk_index").unwrap(), &i.to_string());
        }
    }

    #[tokio::test]
    async fn test_semantic_splitter_max_chunk() {
        let embedder = Arc::new(DummyEmbedder::new(8));
        let splitter = SemanticSplitter::new(embedder)
            .with_threshold(0.0) // Never split by similarity
            .with_max_chunk_chars(50); // But force split by size

        let docs = vec![Document::new(
            "First sentence is here. Second sentence is here. Third sentence is here. Fourth sentence.",
        )];
        let chunks = splitter.split(docs).await.unwrap();
        assert!(chunks.len() > 1);
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((raven_core::cosine_similarity(&a, &b) - 1.0).abs() < 0.001);

        let c = vec![0.0, 1.0, 0.0];
        assert!((raven_core::cosine_similarity(&a, &c)).abs() < 0.001);
    }

    #[test]
    fn test_split_sentences() {
        let sentences = split_sentences("First sentence. Second sentence. Third.");
        assert_eq!(sentences.len(), 3);
    }
}
