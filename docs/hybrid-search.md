# Hybrid Search

RavenRustRAG combines vector similarity search with BM25 keyword matching using Reciprocal Rank Fusion (RRF).

## How It Works

1. **Vector search**: Embeds the query and finds chunks with the highest cosine similarity
2. **BM25 search**: Tokenizes the query and scores chunks using TF-IDF term frequency
3. **RRF fusion**: Merges both ranked lists using reciprocal rank fusion with configurable alpha blending

## Usage

### CLI

```bash
ravenrag query "rust error handling" --hybrid --alpha 0.7
```

- `--hybrid` enables BM25+vector fusion
- `--alpha` controls the blend (1.0 = pure vector, 0.0 = pure BM25, default 0.5)

### API

```json
POST /query
{
  "query": "rust error handling",
  "top_k": 5,
  "hybrid": true,
  "alpha": 0.7
}
```

## When to Use Hybrid Search

| Scenario | Recommendation |
|----------|---------------|
| Short keyword queries | Hybrid (BM25 helps with exact matches) |
| Natural language questions | Pure vector often sufficient |
| Code/technical terms | Hybrid (exact term matching helps) |
| Typos/paraphrases | Pure vector (semantic similarity handles variations) |

## BM25 Implementation

The BM25 index is stored alongside vector embeddings in SQLite. Terms are tokenized, lowercased, and stored per chunk. The index is built automatically during document indexing and persisted for fast retrieval.

Parameters (hardcoded defaults):
- `k1 = 1.2` (term frequency saturation)
- `b = 0.75` (document length normalization)

## Reciprocal Rank Fusion

RRF combines rankings without requiring score normalization:

```
RRF_score(d) = alpha * 1/(k + rank_vector(d)) + (1-alpha) * 1/(k + rank_bm25(d))
```

Where `k = 60` (standard RRF constant).
