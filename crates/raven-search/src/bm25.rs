use raven_core::{Chunk, SearchResult};
use raven_store::Bm25TermData;
use std::collections::HashMap;

/// BM25 index for keyword-based search.
/// Parameters follow the Okapi BM25 formula with standard defaults.
pub struct Bm25Index {
    /// Term frequency per document: doc_index -> term -> count
    term_freqs: Vec<HashMap<String, f32>>,
    /// Document lengths (in tokens)
    doc_lengths: Vec<f32>,
    /// Average document length
    avg_dl: f32,
    /// Document frequency: term -> number of documents containing term
    doc_freq: HashMap<String, usize>,
    /// Total number of documents
    num_docs: usize,
    /// Stored chunks (parallel to term_freqs)
    chunks: Vec<Chunk>,
    /// BM25 k1 parameter (term saturation)
    k1: f32,
    /// BM25 b parameter (length normalization)
    b: f32,
}

impl Bm25Index {
    pub fn new() -> Self {
        Self {
            term_freqs: Vec::new(),
            doc_lengths: Vec::new(),
            avg_dl: 0.0,
            doc_freq: HashMap::new(),
            num_docs: 0,
            chunks: Vec::new(),
            k1: 1.5,
            b: 0.75,
        }
    }

    pub fn with_params(mut self, k1: f32, b: f32) -> Self {
        self.k1 = k1;
        self.b = b;
        self
    }

    /// Add chunks to the BM25 index
    pub fn add(&mut self, chunks: &[Chunk]) {
        for chunk in chunks {
            let tokens = tokenize(&chunk.text);
            let doc_len = tokens.len() as f32;

            let mut tf: HashMap<String, f32> = HashMap::new();
            let mut seen_terms: std::collections::HashSet<String> =
                std::collections::HashSet::new();

            for token in &tokens {
                *tf.entry(token.clone()).or_insert(0.0) += 1.0;
                seen_terms.insert(token.clone());
            }

            // Update document frequency
            for term in &seen_terms {
                *self.doc_freq.entry(term.clone()).or_insert(0) += 1;
            }

            self.term_freqs.push(tf);
            self.doc_lengths.push(doc_len);
            self.chunks.push(chunk.clone());
            self.num_docs += 1;
        }

        // Recompute average document length
        if self.num_docs > 0 {
            self.avg_dl = self.doc_lengths.iter().sum::<f32>() / self.num_docs as f32;
        }
    }

    /// Search using BM25 scoring
    pub fn search(&self, query: &str, top_k: usize) -> Vec<SearchResult> {
        if self.num_docs == 0 {
            return vec![];
        }

        let query_tokens = tokenize(query);
        let mut scores: Vec<(f32, usize)> = Vec::with_capacity(self.num_docs);

        for (doc_idx, tf) in self.term_freqs.iter().enumerate() {
            let dl = self.doc_lengths[doc_idx];
            let mut score = 0.0f32;

            for token in &query_tokens {
                let freq = tf.get(token).copied().unwrap_or(0.0);
                if freq == 0.0 {
                    continue;
                }

                let df = *self.doc_freq.get(token).unwrap_or(&0) as f32;
                let idf = ((self.num_docs as f32 - df + 0.5) / (df + 0.5) + 1.0).ln();

                let numerator = freq * (self.k1 + 1.0);
                let denominator = freq + self.k1 * (1.0 - self.b + self.b * dl / self.avg_dl);

                score += idf * numerator / denominator;
            }

            if score > 0.0 {
                scores.push((score, doc_idx));
            }
        }

        scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scores.truncate(top_k);

        scores
            .into_iter()
            .map(|(score, idx)| SearchResult {
                chunk: self.chunks[idx].clone(),
                score,
                distance: 0.0,
            })
            .collect()
    }

    /// Clear the index
    pub fn clear(&mut self) {
        self.term_freqs.clear();
        self.doc_lengths.clear();
        self.doc_freq.clear();
        self.chunks.clear();
        self.num_docs = 0;
        self.avg_dl = 0.0;
    }

    /// Remove all chunks belonging to a given doc_id from the index.
    /// Updates term frequencies, document frequencies, and average document length
    /// without requiring a full rebuild.
    pub fn remove_by_doc_id(&mut self, doc_id: &str) -> usize {
        // Find indices of chunks belonging to this doc_id
        let indices: Vec<usize> = self
            .chunks
            .iter()
            .enumerate()
            .filter(|(_, c)| c.doc_id == doc_id)
            .map(|(i, _)| i)
            .collect();

        if indices.is_empty() {
            return 0;
        }

        let removed = indices.len();

        // Decrement document frequency for terms in removed documents
        for &idx in &indices {
            let seen_terms: std::collections::HashSet<&String> =
                self.term_freqs[idx].keys().collect();
            for term in seen_terms {
                if let Some(df) = self.doc_freq.get_mut(term) {
                    *df = df.saturating_sub(1);
                    if *df == 0 {
                        self.doc_freq.remove(term);
                    }
                }
            }
        }

        // Remove in reverse order to preserve indices
        for &idx in indices.iter().rev() {
            self.term_freqs.swap_remove(idx);
            self.doc_lengths.swap_remove(idx);
            self.chunks.swap_remove(idx);
        }

        self.num_docs = self.chunks.len();

        // Recompute average document length
        if self.num_docs > 0 {
            self.avg_dl = self.doc_lengths.iter().sum::<f32>() / self.num_docs as f32;
        } else {
            self.avg_dl = 0.0;
        }

        removed
    }

    pub fn count(&self) -> usize {
        self.num_docs
    }

    /// Get term frequencies and doc length for the last N added chunks
    /// (used for persistence — returns data for chunks at given indices)
    pub fn get_term_data(&self, start_idx: usize) -> Vec<(String, HashMap<String, f32>, f32)> {
        let mut result = Vec::new();
        for i in start_idx..self.num_docs {
            let chunk_id = self.chunks[i].id.clone();
            let tf = self.term_freqs[i].clone();
            let dl = self.doc_lengths[i];
            result.push((chunk_id, tf, dl));
        }
        result
    }

    /// Rebuild BM25 index from stored term data (avoids re-tokenizing)
    pub fn load_from_stored(&mut self, data: &[Bm25TermData]) {
        self.clear();
        for entry in data {
            let chunk = Chunk::new(&entry.doc_id, &entry.text);

            // Update document frequency from stored terms
            let mut seen_terms: std::collections::HashSet<String> =
                std::collections::HashSet::new();
            for term in entry.terms.keys() {
                seen_terms.insert(term.clone());
            }
            for term in &seen_terms {
                *self.doc_freq.entry(term.clone()).or_insert(0) += 1;
            }

            self.term_freqs.push(entry.terms.clone());
            self.doc_lengths.push(entry.doc_length);
            self.chunks.push(chunk);
            self.num_docs += 1;
        }

        if self.num_docs > 0 {
            self.avg_dl = self.doc_lengths.iter().sum::<f32>() / self.num_docs as f32;
        }
    }
}

impl Default for Bm25Index {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple whitespace + lowercase tokenizer with basic punctuation removal
fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|s| s.len() > 1) // skip single chars
        .map(std::string::ToString::to_string)
        .collect()
}

// =============================================================================
// Hybrid search with Reciprocal Rank Fusion
// =============================================================================

/// Merge vector search and BM25 results using Reciprocal Rank Fusion.
/// `alpha` controls the blend: 1.0 = pure vector, 0.0 = pure BM25.
pub fn reciprocal_rank_fusion(
    vector_results: &[SearchResult],
    bm25_results: &[SearchResult],
    alpha: f32,
    top_k: usize,
) -> Vec<SearchResult> {
    let k = 60.0f32; // RRF constant

    let mut scores: HashMap<String, (f32, Option<SearchResult>)> = HashMap::new();

    // Score vector results
    for (rank, result) in vector_results.iter().enumerate() {
        let rrf_score = alpha / (k + rank as f32 + 1.0);
        let entry = scores.entry(result.chunk.id.clone()).or_insert((0.0, None));
        entry.0 += rrf_score;
        if entry.1.is_none() {
            entry.1 = Some(result.clone());
        }
    }

    // Score BM25 results
    for (rank, result) in bm25_results.iter().enumerate() {
        let rrf_score = (1.0 - alpha) / (k + rank as f32 + 1.0);
        let entry = scores.entry(result.chunk.id.clone()).or_insert((0.0, None));
        entry.0 += rrf_score;
        if entry.1.is_none() {
            entry.1 = Some(result.clone());
        }
    }

    let mut fused: Vec<(f32, SearchResult)> = scores
        .into_values()
        .filter_map(|(score, result)| result.map(|r| (score, r)))
        .collect();

    fused.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    fused.truncate(top_k);

    fused
        .into_iter()
        .map(|(score, mut result)| {
            result.score = score;
            result
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("Hello world! This is a test.");
        assert!(tokens.contains(&"hello".to_string()));
        assert!(tokens.contains(&"world".to_string()));
        assert!(tokens.contains(&"test".to_string()));
    }

    #[test]
    fn test_bm25_basic() {
        let mut index = Bm25Index::new();

        let chunks = vec![
            Chunk::new("doc1", "Rust programming language is fast and safe"),
            Chunk::new("doc2", "Python is a popular programming language"),
            Chunk::new("doc3", "The weather today is sunny and warm"),
        ];

        index.add(&chunks);
        assert_eq!(index.count(), 3);

        let results = index.search("Rust programming", 3);
        assert!(!results.is_empty());
        // Rust doc should rank first (contains both query terms)
        assert_eq!(results[0].chunk.doc_id, "doc1");
    }

    #[test]
    fn test_bm25_remove_by_doc_id() {
        let mut index = Bm25Index::new();

        let chunks = vec![
            Chunk::new("doc1", "Rust programming language is fast and safe"),
            Chunk::new("doc2", "Python is a popular programming language"),
            Chunk::new("doc3", "The weather today is sunny and warm"),
        ];

        index.add(&chunks);
        assert_eq!(index.count(), 3);

        // Remove doc1
        let removed = index.remove_by_doc_id("doc1");
        assert_eq!(removed, 1);
        assert_eq!(index.count(), 2);

        // Search should no longer return doc1
        let results = index.search("Rust programming", 3);
        for r in &results {
            assert_ne!(r.chunk.doc_id, "doc1");
        }

        // doc2 should still be findable
        let results = index.search("Python programming", 3);
        assert!(!results.is_empty());
        assert_eq!(results[0].chunk.doc_id, "doc2");
    }

    #[test]
    fn test_bm25_remove_nonexistent() {
        let mut index = Bm25Index::new();
        let chunks = vec![Chunk::new("doc1", "Hello world")];
        index.add(&chunks);

        let removed = index.remove_by_doc_id("nonexistent");
        assert_eq!(removed, 0);
        assert_eq!(index.count(), 1);
    }

    #[test]
    fn test_bm25_empty() {
        let index = Bm25Index::new();
        let results = index.search("anything", 5);
        assert!(results.is_empty());
    }

    #[test]
    fn test_rrf_fusion() {
        let c1 = Chunk::new("d1", "doc 1");
        let c2 = Chunk::new("d2", "doc 2");
        let c3 = Chunk::new("d3", "doc 3");

        let vector = vec![
            SearchResult {
                chunk: c1.clone(),
                score: 0.9,
                distance: 0.1,
            },
            SearchResult {
                chunk: c2.clone(),
                score: 0.8,
                distance: 0.2,
            },
        ];

        let bm25 = vec![
            SearchResult {
                chunk: c2.clone(),
                score: 5.0,
                distance: 0.0,
            },
            SearchResult {
                chunk: c3.clone(),
                score: 3.0,
                distance: 0.0,
            },
        ];

        let fused = reciprocal_rank_fusion(&vector, &bm25, 0.5, 3);
        assert_eq!(fused.len(), 3);
        // c2 appears in both, should rank highest
        assert_eq!(fused[0].chunk.doc_id, "d2");
    }

    #[test]
    fn test_bm25_load_from_stored() {
        let mut index = Bm25Index::new();

        let chunks = vec![
            Chunk::new("doc1", "Rust programming language is fast"),
            Chunk::new("doc2", "Python is great for scripting"),
        ];
        index.add(&chunks);

        // Get term data
        let data = index.get_term_data(0);
        assert_eq!(data.len(), 2);

        // Rebuild from stored data
        let stored: Vec<Bm25TermData> = data
            .into_iter()
            .enumerate()
            .map(|(i, (id, terms, dl))| Bm25TermData {
                chunk_id: id,
                doc_id: chunks[i].doc_id.clone(),
                text: chunks[i].text.clone(),
                terms,
                doc_length: dl,
            })
            .collect();

        let mut rebuilt = Bm25Index::new();
        rebuilt.load_from_stored(&stored);

        assert_eq!(rebuilt.count(), 2);

        // Search should give same top result
        let results = rebuilt.search("Rust programming", 3);
        assert!(!results.is_empty());
        assert_eq!(results[0].chunk.doc_id, "doc1");
    }

    #[test]
    fn test_rrf_pure_vector() {
        let c1 = Chunk::new("d1", "doc 1");
        let vector = vec![SearchResult {
            chunk: c1,
            score: 0.9,
            distance: 0.1,
        }];

        let fused = reciprocal_rank_fusion(&vector, &[], 1.0, 5);
        assert_eq!(fused.len(), 1);
    }

    #[test]
    fn test_rrf_pure_bm25() {
        let c1 = Chunk::new("d1", "doc 1");
        let bm25 = vec![SearchResult {
            chunk: c1,
            score: 5.0,
            distance: 0.0,
        }];

        let fused = reciprocal_rank_fusion(&[], &bm25, 0.0, 5);
        assert_eq!(fused.len(), 1);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;
    use raven_core::Chunk;

    proptest! {
        /// BM25 scores are always non-negative
        #[test]
        fn bm25_scores_non_negative(
            docs in proptest::collection::vec("[a-z]{1,20}( [a-z]{1,20}){0,10}", 1..20),
            query in "[a-z]{1,15}( [a-z]{1,15}){0,3}",
        ) {
            let mut idx = Bm25Index::new();
            let chunks: Vec<Chunk> = docs.iter().enumerate()
                .map(|(i, text)| Chunk::new(format!("doc_{i}"), text.clone()))
                .collect();
            idx.add(&chunks);

            let results = idx.search(&query, 10);
            for r in &results {
                prop_assert!(r.score >= 0.0, "Negative BM25 score: {}", r.score);
            }
        }

        /// BM25 search returns at most top_k results
        #[test]
        fn bm25_respects_top_k(
            num_docs in 1usize..30,
            top_k in 1usize..20,
        ) {
            let mut idx = Bm25Index::new();
            let chunks: Vec<Chunk> = (0..num_docs)
                .map(|i| Chunk::new(format!("doc_{i}"), format!("word{i} hello world test")))
                .collect();
            idx.add(&chunks);

            let results = idx.search("hello world", top_k);
            prop_assert!(results.len() <= top_k);
        }

        /// RRF fusion: result count is at most top_k
        #[test]
        fn rrf_respects_top_k(
            n_vector in 0usize..10,
            n_bm25 in 0usize..10,
            top_k in 1usize..15,
            alpha in 0.0f32..1.0f32,
        ) {
            let vector: Vec<SearchResult> = (0..n_vector)
                .map(|i| SearchResult {
                    chunk: Chunk::new(format!("v{i}"), format!("vector doc {i}")),
                    score: 1.0 - (i as f32 * 0.1),
                    distance: i as f32 * 0.1,
                })
                .collect();
            let bm25: Vec<SearchResult> = (0..n_bm25)
                .map(|i| SearchResult {
                    chunk: Chunk::new(format!("b{i}"), format!("bm25 doc {i}")),
                    score: 5.0 - (i as f32 * 0.5),
                    distance: 0.0,
                })
                .collect();

            let fused = reciprocal_rank_fusion(&vector, &bm25, alpha, top_k);
            prop_assert!(fused.len() <= top_k,
                "RRF returned {} results, expected <= {}", fused.len(), top_k);
        }

        /// RRF fusion: scores are monotonically non-increasing
        #[test]
        fn rrf_scores_monotonic(
            n in 2usize..15,
        ) {
            let vector: Vec<SearchResult> = (0..n)
                .map(|i| SearchResult {
                    chunk: Chunk::new(format!("v{i}"), format!("doc {i}")),
                    score: 1.0 - (i as f32 * 0.05),
                    distance: i as f32 * 0.05,
                })
                .collect();

            let fused = reciprocal_rank_fusion(&vector, &[], 1.0, n);
            for window in fused.windows(2) {
                prop_assert!(window[0].score >= window[1].score,
                    "Scores not monotonic: {} < {}", window[0].score, window[1].score);
            }
        }
    }
}
