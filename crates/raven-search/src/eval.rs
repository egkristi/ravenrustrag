use raven_core::SearchResult;

/// Mean Reciprocal Rank: 1/rank of first relevant result.
/// Returns 0.0 if no relevant result is found.
pub fn mrr(results: &[SearchResult], relevant_ids: &[String]) -> f64 {
    for (i, result) in results.iter().enumerate() {
        if relevant_ids.contains(&result.chunk.doc_id) {
            return 1.0 / (i as f64 + 1.0);
        }
    }
    0.0
}

/// Recall@k: fraction of relevant documents found in top-k results.
pub fn recall_at_k(results: &[SearchResult], relevant_ids: &[String], k: usize) -> f64 {
    if relevant_ids.is_empty() {
        return 0.0;
    }
    let top_k = &results[..results.len().min(k)];
    let found = top_k
        .iter()
        .filter(|r| relevant_ids.contains(&r.chunk.doc_id))
        .count();
    found as f64 / relevant_ids.len() as f64
}

/// Precision@k: fraction of top-k results that are relevant.
pub fn precision_at_k(results: &[SearchResult], relevant_ids: &[String], k: usize) -> f64 {
    let top_k = &results[..results.len().min(k)];
    if top_k.is_empty() {
        return 0.0;
    }
    let found = top_k
        .iter()
        .filter(|r| relevant_ids.contains(&r.chunk.doc_id))
        .count();
    found as f64 / top_k.len() as f64
}

/// Normalized Discounted Cumulative Gain (NDCG@k).
/// Uses binary relevance (1 if relevant, 0 otherwise).
pub fn ndcg_at_k(results: &[SearchResult], relevant_ids: &[String], k: usize) -> f64 {
    let top_k = &results[..results.len().min(k)];

    // DCG: sum of 1/log2(i+2) for relevant results at position i
    let dcg: f64 = top_k
        .iter()
        .enumerate()
        .map(|(i, r)| {
            if relevant_ids.contains(&r.chunk.doc_id) {
                1.0 / (i as f64 + 2.0).log2()
            } else {
                0.0
            }
        })
        .sum();

    // Ideal DCG: all relevant docs ranked first
    let ideal_count = relevant_ids.len().min(k);
    let idcg: f64 = (0..ideal_count)
        .map(|i| 1.0 / (i as f64 + 2.0).log2())
        .sum();

    if idcg == 0.0 {
        0.0
    } else {
        dcg / idcg
    }
}

/// Evaluation result for a single query.
#[derive(Debug, Clone)]
pub struct EvalResult {
    pub query: String,
    pub mrr: f64,
    pub recall_at_k: f64,
    pub precision_at_k: f64,
    pub ndcg_at_k: f64,
    pub k: usize,
    pub num_results: usize,
    pub num_relevant: usize,
}

/// Evaluate a batch of queries against ground truth.
/// `queries` is a list of (query_text, relevant_doc_ids, search_results).
pub fn evaluate_batch(
    queries: &[(String, Vec<String>, Vec<SearchResult>)],
    k: usize,
) -> Vec<EvalResult> {
    queries
        .iter()
        .map(|(query, relevant, results)| EvalResult {
            query: query.clone(),
            mrr: mrr(results, relevant),
            recall_at_k: recall_at_k(results, relevant, k),
            precision_at_k: precision_at_k(results, relevant, k),
            ndcg_at_k: ndcg_at_k(results, relevant, k),
            k,
            num_results: results.len(),
            num_relevant: relevant.len(),
        })
        .collect()
}

/// Average eval results into a summary.
pub fn eval_summary(results: &[EvalResult]) -> EvalResult {
    if results.is_empty() {
        return EvalResult {
            query: "average".to_string(),
            mrr: 0.0,
            recall_at_k: 0.0,
            precision_at_k: 0.0,
            ndcg_at_k: 0.0,
            k: 0,
            num_results: 0,
            num_relevant: 0,
        };
    }

    let n = results.len() as f64;
    EvalResult {
        query: format!("average ({} queries)", results.len()),
        mrr: results.iter().map(|r| r.mrr).sum::<f64>() / n,
        recall_at_k: results.iter().map(|r| r.recall_at_k).sum::<f64>() / n,
        precision_at_k: results.iter().map(|r| r.precision_at_k).sum::<f64>() / n,
        ndcg_at_k: results.iter().map(|r| r.ndcg_at_k).sum::<f64>() / n,
        k: results[0].k,
        num_results: results.iter().map(|r| r.num_results).sum(),
        num_relevant: results.iter().map(|r| r.num_relevant).sum(),
    }
}

impl std::fmt::Display for EvalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Query: {} | MRR: {:.4} | Recall@{}: {:.4} | Precision@{}: {:.4} | NDCG@{}: {:.4}",
            self.query,
            self.mrr,
            self.k,
            self.recall_at_k,
            self.k,
            self.precision_at_k,
            self.k,
            self.ndcg_at_k
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use raven_core::Chunk;

    fn make_result(doc_id: &str) -> SearchResult {
        SearchResult {
            chunk: Chunk::new(doc_id, "text"),
            score: 1.0,
            distance: 0.0,
        }
    }

    #[test]
    fn test_mrr_first() {
        let results = vec![make_result("a"), make_result("b"), make_result("c")];
        let relevant = vec!["a".to_string()];
        assert!((mrr(&results, &relevant) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_mrr_second() {
        let results = vec![make_result("x"), make_result("a"), make_result("c")];
        let relevant = vec!["a".to_string()];
        assert!((mrr(&results, &relevant) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_mrr_none() {
        let results = vec![make_result("x"), make_result("y")];
        let relevant = vec!["a".to_string()];
        assert!((mrr(&results, &relevant)).abs() < 1e-6);
    }

    #[test]
    fn test_recall_at_k() {
        let results = vec![make_result("a"), make_result("b"), make_result("c")];
        let relevant = vec!["a".to_string(), "c".to_string()];
        assert!((recall_at_k(&results, &relevant, 3) - 1.0).abs() < 1e-6);
        assert!((recall_at_k(&results, &relevant, 1) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_precision_at_k() {
        let results = vec![make_result("a"), make_result("x"), make_result("c")];
        let relevant = vec!["a".to_string(), "c".to_string()];
        // At k=3: 2/3
        assert!((precision_at_k(&results, &relevant, 3) - 2.0 / 3.0).abs() < 1e-6);
        // At k=1: 1/1
        assert!((precision_at_k(&results, &relevant, 1) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_ndcg_perfect() {
        let results = vec![make_result("a"), make_result("b")];
        let relevant = vec!["a".to_string(), "b".to_string()];
        // Perfect ranking = NDCG of 1.0
        assert!((ndcg_at_k(&results, &relevant, 2) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_ndcg_imperfect() {
        let results = vec![make_result("x"), make_result("a")];
        let relevant = vec!["a".to_string()];
        // Relevant doc at position 2: DCG = 1/log2(3), IDCG = 1/log2(2)
        let expected = (1.0 / 3.0f64.log2()) / (1.0 / 2.0f64.log2());
        assert!((ndcg_at_k(&results, &relevant, 2) - expected).abs() < 1e-6);
    }

    #[test]
    fn test_eval_summary() {
        let r1 = EvalResult {
            query: "q1".to_string(),
            mrr: 1.0,
            recall_at_k: 1.0,
            precision_at_k: 0.5,
            ndcg_at_k: 1.0,
            k: 5,
            num_results: 5,
            num_relevant: 2,
        };
        let r2 = EvalResult {
            query: "q2".to_string(),
            mrr: 0.5,
            recall_at_k: 0.5,
            precision_at_k: 0.5,
            ndcg_at_k: 0.5,
            k: 5,
            num_results: 5,
            num_relevant: 2,
        };

        let summary = eval_summary(&[r1, r2]);
        assert!((summary.mrr - 0.75).abs() < 1e-6);
        assert!((summary.recall_at_k - 0.75).abs() < 1e-6);
        assert!((summary.ndcg_at_k - 0.75).abs() < 1e-6);
    }
}
