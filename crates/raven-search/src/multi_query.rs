//! Multi-query expansion: rewrite a single query into multiple variants.
//!
//! Generates alternative phrasings to improve recall.

use raven_core::{Result, SearchResult};

/// Expand a single query into multiple search variants.
///
/// Strategies:
/// - Original query (always included)
/// - Question form ("What is X?")
/// - Definition form ("X definition")
/// - Synonym expansion (keyword extraction + rephrasing)
pub fn expand_query(query: &str) -> Vec<String> {
    let mut variants = Vec::with_capacity(4);
    let trimmed = query.trim();

    if trimmed.is_empty() {
        return variants;
    }

    // Always include the original
    variants.push(trimmed.to_string());

    // If it's not already a question, create a question form
    if !trimmed.ends_with('?') {
        let q = if trimmed.to_lowercase().starts_with("what")
            || trimmed.to_lowercase().starts_with("how")
            || trimmed.to_lowercase().starts_with("why")
            || trimmed.to_lowercase().starts_with("when")
            || trimmed.to_lowercase().starts_with("where")
            || trimmed.to_lowercase().starts_with("who")
        {
            format!("{trimmed}?")
        } else {
            format!("What is {trimmed}?")
        };
        variants.push(q);
    }

    // Keyword extraction: take significant words (>3 chars, not stopwords)
    let keywords = extract_keywords(trimmed);
    if keywords.len() >= 2 {
        variants.push(keywords.join(" "));
    }

    // Definition/explanation form
    if !trimmed.to_lowercase().contains("definition")
        && !trimmed.to_lowercase().contains("explain")
    {
        let first_noun_phrase = keywords.first().map_or(trimmed, String::as_str);
        variants.push(format!("{first_noun_phrase} explained"));
    }

    variants.dedup();
    variants
}

/// Merge results from multiple query expansions using score-based deduplication.
pub fn merge_expanded_results(
    results_per_query: &[Vec<SearchResult>],
    top_k: usize,
) -> Vec<SearchResult> {
    let mut seen = std::collections::HashMap::<String, SearchResult>::new();

    for results in results_per_query {
        for (rank, result) in results.iter().enumerate() {
            let entry = seen.entry(result.chunk.id.clone());
            entry
                .and_modify(|existing| {
                    if result.score > existing.score {
                        existing.score = result.score;
                    }
                })
                .or_insert_with(|| {
                    let mut r = result.clone();
                    // Boost results that appear in multiple expansions
                    r.score += 0.01 * (results_per_query.len() as f32 - 1.0) / (rank as f32 + 1.0);
                    r
                });
        }
    }

    let mut merged: Vec<SearchResult> = seen.into_values().collect();
    merged.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    merged.truncate(top_k);
    merged
}

const STOPWORDS: &[&str] = &[
    "the", "a", "an", "is", "are", "was", "were", "be", "been", "being", "have", "has", "had",
    "do", "does", "did", "will", "would", "could", "should", "may", "might", "shall", "can",
    "of", "in", "to", "for", "with", "on", "at", "from", "by", "about", "as", "into", "through",
    "and", "or", "but", "not", "no", "if", "then", "than", "that", "this", "it", "its",
];

fn extract_keywords(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|w| {
            w.to_lowercase()
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_string()
        })
        .filter(|w| w.len() > 2 && !STOPWORDS.contains(&w.as_str()))
        .collect()
}

/// Query the index with multi-query expansion.
/// Runs the original query plus expansions, then merges results.
pub async fn query_expanded(
    index: &crate::DocumentIndex,
    query: &str,
    top_k: usize,
) -> Result<Vec<SearchResult>> {
    let variants = expand_query(query);
    let mut all_results = Vec::with_capacity(variants.len());

    for variant in &variants {
        let results = index.query(variant, top_k).await?;
        all_results.push(results);
    }

    Ok(merge_expanded_results(&all_results, top_k))
}

#[cfg(test)]
mod tests {
    use super::*;
    use raven_core::Document;
    use raven_embed::DummyEmbedder;
    use raven_store::MemoryStore;
    use std::sync::Arc;

    #[test]
    fn test_expand_query_basic() {
        let variants = expand_query("Rust programming");
        assert!(variants.len() >= 2);
        assert_eq!(variants[0], "Rust programming");
        assert!(variants.iter().any(|v| v.contains('?')));
    }

    #[test]
    fn test_expand_query_already_question() {
        let variants = expand_query("What is Rust?");
        assert_eq!(variants[0], "What is Rust?");
        // Should not double-question
        assert!(!variants.iter().any(|v| v.ends_with("??")));
    }

    #[test]
    fn test_expand_query_empty() {
        let variants = expand_query("");
        assert!(variants.is_empty());
    }

    #[test]
    fn test_extract_keywords() {
        let kw = extract_keywords("The quick brown fox jumps over the lazy dog");
        assert!(kw.contains(&"quick".to_string()));
        assert!(kw.contains(&"brown".to_string()));
        assert!(!kw.contains(&"the".to_string()));
    }

    #[test]
    fn test_merge_results_dedup() {
        use raven_core::Chunk;
        let c1 = Chunk::new("d1", "doc 1");
        let r1 = SearchResult {
            chunk: c1.clone(),
            score: 0.9,
            distance: 0.1,
        };
        let r1_dup = SearchResult {
            chunk: c1,
            score: 0.8,
            distance: 0.2,
        };

        let merged = merge_expanded_results(&[vec![r1], vec![r1_dup]], 10);
        assert_eq!(merged.len(), 1);
        assert!(merged[0].score >= 0.9);
    }

    #[tokio::test]
    async fn test_query_expanded() {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(DummyEmbedder::new(3));
        let index = crate::DocumentIndex::new(store, embedder);

        let splitter = raven_split::TextSplitter::new(200, 10);
        let docs = vec![
            Document::new("Rust is a systems programming language"),
            Document::new("Python is great for data science"),
        ];
        index.add_documents(docs, &splitter).await.unwrap();

        let results = query_expanded(&index, "Rust programming", 5)
            .await
            .unwrap();
        assert!(!results.is_empty());
    }
}
