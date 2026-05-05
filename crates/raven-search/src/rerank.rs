//! Cross-encoder reranking for improved search precision.
//!
//! Fetches more candidates than needed, then reranks using a cross-encoder model
//! that scores (query, document) pairs for relevance.

use raven_core::{Result, SearchResult};

/// Reranker trait: scores (query, document) pairs for relevance.
#[async_trait::async_trait]
pub trait Reranker: Send + Sync {
    /// Score a batch of (query, document) pairs.
    /// Returns scores in the same order as the input documents.
    async fn score(&self, query: &str, documents: &[&str]) -> Result<Vec<f32>>;
}

/// Rerank search results using a cross-encoder model.
///
/// Fetches `fetch_multiplier * top_k` candidates, then reranks and returns `top_k`.
pub async fn rerank(
    reranker: &dyn Reranker,
    query: &str,
    candidates: &[SearchResult],
    top_k: usize,
) -> Result<Vec<SearchResult>> {
    if candidates.is_empty() {
        return Ok(Vec::new());
    }

    let texts: Vec<&str> = candidates.iter().map(|r| r.chunk.text.as_str()).collect();
    let scores = reranker.score(query, &texts).await?;

    let mut scored: Vec<(f32, &SearchResult)> = scores.into_iter().zip(candidates.iter()).collect();

    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(top_k);

    Ok(scored
        .into_iter()
        .map(|(score, result)| SearchResult {
            chunk: result.chunk.clone(),
            score,
            distance: 1.0 - score,
        })
        .collect())
}

// =============================================================================
// Dummy reranker (for testing)
// =============================================================================

/// A simple reranker that uses keyword overlap scoring (for testing/fallback).
pub struct KeywordReranker;

#[async_trait::async_trait]
impl Reranker for KeywordReranker {
    async fn score(&self, query: &str, documents: &[&str]) -> Result<Vec<f32>> {
        let query_lower = query.to_lowercase();
        let query_terms: std::collections::HashSet<&str> = query_lower.split_whitespace().collect();

        Ok(documents
            .iter()
            .map(|doc| {
                let doc_lower = doc.to_lowercase();
                let doc_terms: std::collections::HashSet<&str> =
                    doc_lower.split_whitespace().collect();
                let overlap = query_terms
                    .iter()
                    .filter(|t| doc_terms.contains(*t))
                    .count();
                overlap as f32 / query_terms.len().max(1) as f32
            })
            .collect())
    }
}

// =============================================================================
// ONNX cross-encoder reranker (behind `onnx` feature)
// =============================================================================

#[cfg(feature = "onnx")]
mod onnx_reranker {
    use super::*;
    use ndarray::Array2;
    use ort::session::Session;
    use ort::value::TensorRef;
    use std::path::Path;
    use std::sync::Mutex;

    /// Cross-encoder reranker using ONNX Runtime for local inference.
    ///
    /// Loads a cross-encoder model (e.g. ms-marco-MiniLM-L-6-v2) that scores
    /// (query, passage) pairs for relevance. Behind the `onnx` feature flag.
    ///
    /// Requires the `onnx` feature flag and Rust 1.88+.
    pub struct OnnxReranker {
        session: Mutex<Session>,
        tokenizer: tokenizers::Tokenizer,
    }

    impl OnnxReranker {
        /// Create a new ONNX cross-encoder reranker.
        ///
        /// - `model_path`: Path to the cross-encoder `.onnx` model file (fp32, fp16, or int8 quantized)
        /// - `tokenizer_path`: Path to `tokenizer.json` (HuggingFace format)
        pub fn new(model_path: impl AsRef<Path>, tokenizer_path: impl AsRef<Path>) -> Result<Self> {
            Self::with_threads(model_path, tokenizer_path, 4)
        }

        /// Create an ONNX reranker with custom thread count.
        ///
        /// Supports fp32, fp16, and int8 quantized cross-encoder models.
        /// ORT automatically handles quantized operators.
        ///
        /// - `model_path`: Path to the cross-encoder `.onnx` model
        /// - `tokenizer_path`: Path to `tokenizer.json`
        /// - `num_threads`: Number of intra-op threads (0 = auto)
        pub fn with_threads(
            model_path: impl AsRef<Path>,
            tokenizer_path: impl AsRef<Path>,
            num_threads: usize,
        ) -> Result<Self> {
            let builder = Session::builder().map_err(|e| {
                raven_core::RavenError::Embed(format!("ONNX reranker session builder error: {e}"))
            })?;

            let builder = if num_threads > 0 {
                builder.with_intra_threads(num_threads).map_err(|e| {
                    raven_core::RavenError::Embed(format!("ONNX reranker thread config error: {e}"))
                })?
            } else {
                builder
            };

            let session = builder.commit_from_file(model_path.as_ref()).map_err(|e| {
                raven_core::RavenError::Embed(format!("ONNX reranker model load error: {e}"))
            })?;

            let tokenizer =
                tokenizers::Tokenizer::from_file(tokenizer_path.as_ref()).map_err(|e| {
                    raven_core::RavenError::Embed(format!("Reranker tokenizer load error: {e}"))
                })?;

            Ok(Self {
                session: Mutex::new(session),
                tokenizer,
            })
        }
    }

    #[async_trait::async_trait]
    impl Reranker for OnnxReranker {
        async fn score(&self, query: &str, documents: &[&str]) -> Result<Vec<f32>> {
            if documents.is_empty() {
                return Ok(Vec::new());
            }

            // Encode (query, document) pairs for cross-encoder
            let pairs: Vec<tokenizers::EncodeInput> = documents
                .iter()
                .map(|doc| tokenizers::EncodeInput::Dual(query.into(), (*doc).into()))
                .collect();

            let encodings = self
                .tokenizer
                .encode_batch(pairs, true)
                .map_err(|e| raven_core::RavenError::Embed(format!("Tokenization error: {e}")))?;

            let max_len = encodings
                .iter()
                .map(|e| e.get_ids().len())
                .max()
                .unwrap_or(0);
            let batch_size = encodings.len();

            let mut input_ids = Array2::<i64>::zeros((batch_size, max_len));
            let mut attention_mask = Array2::<i64>::zeros((batch_size, max_len));

            for (i, encoding) in encodings.iter().enumerate() {
                for (j, &id) in encoding.get_ids().iter().enumerate() {
                    input_ids[[i, j]] = i64::from(id);
                }
                for (j, &mask) in encoding.get_attention_mask().iter().enumerate() {
                    attention_mask[[i, j]] = i64::from(mask);
                }
            }

            let ids_tensor = TensorRef::from_array_view(&input_ids).map_err(|e| {
                raven_core::RavenError::Embed(format!("ONNX reranker input_ids tensor: {e}"))
            })?;
            let mask_tensor = TensorRef::from_array_view(&attention_mask).map_err(|e| {
                raven_core::RavenError::Embed(format!("ONNX reranker mask tensor: {e}"))
            })?;

            let mut session = self.session.lock().map_err(|e| {
                raven_core::RavenError::Embed(format!("ONNX reranker session lock poisoned: {e}"))
            })?;

            let outputs = session
                .run(ort::inputs![ids_tensor, mask_tensor])
                .map_err(|e| {
                    raven_core::RavenError::Embed(format!("ONNX reranker inference error: {e}"))
                })?;

            let (logits_shape, logits_data) =
                outputs[0].try_extract_tensor::<f32>().map_err(|e| {
                    raven_core::RavenError::Embed(format!("ONNX reranker output error: {e}"))
                })?;

            // Extract relevance scores (logit for the positive class)
            let num_cols = if logits_shape.len() >= 2 {
                logits_shape[1] as usize
            } else {
                1
            };

            let scores: Vec<f32> = (0..batch_size)
                .map(|i| {
                    // Cross-encoders typically output a single logit or [neg, pos] logits
                    if num_cols >= 2 {
                        sigmoid(logits_data[i * num_cols + 1])
                    } else {
                        sigmoid(logits_data[i * num_cols])
                    }
                })
                .collect();

            Ok(scores)
        }
    }

    fn sigmoid(x: f32) -> f32 {
        1.0 / (1.0 + (-x).exp())
    }
}

#[cfg(feature = "onnx")]
pub use onnx_reranker::OnnxReranker;

#[cfg(test)]
mod tests {
    use super::*;
    use raven_core::Chunk;

    #[tokio::test]
    async fn test_keyword_reranker() {
        let reranker = KeywordReranker;

        let scores = reranker
            .score(
                "rust programming",
                &[
                    "rust is a systems programming language",
                    "python is good for scripting",
                    "rust and memory safety",
                ],
            )
            .await
            .unwrap();

        assert_eq!(scores.len(), 3);
        // First doc has both "rust" and "programming"
        assert!(scores[0] > scores[1]);
    }

    #[tokio::test]
    async fn test_rerank_function() {
        let reranker = KeywordReranker;

        let candidates = vec![
            SearchResult {
                chunk: Chunk::new("d1", "python is great"),
                score: 0.9,
                distance: 0.1,
            },
            SearchResult {
                chunk: Chunk::new("d2", "rust programming language"),
                score: 0.8,
                distance: 0.2,
            },
            SearchResult {
                chunk: Chunk::new("d3", "rust is fast and safe"),
                score: 0.7,
                distance: 0.3,
            },
        ];

        let reranked = rerank(&reranker, "rust programming", &candidates, 2)
            .await
            .unwrap();

        assert_eq!(reranked.len(), 2);
        // After reranking, "rust programming language" should be first
        assert!(reranked[0].chunk.text.contains("rust"));
    }

    #[tokio::test]
    async fn test_rerank_empty() {
        let reranker = KeywordReranker;
        let results = rerank(&reranker, "test", &[], 5).await.unwrap();
        assert!(results.is_empty());
    }
}
