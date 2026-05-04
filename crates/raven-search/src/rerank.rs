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
