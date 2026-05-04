# Knowledge Graph

RavenRustRAG can build a knowledge graph from indexed documents to enable entity-relationship traversal alongside vector search.

## Overview

The knowledge graph extracts entities (proper nouns, technical terms) from chunks and creates edges between entities that co-occur. This enables:

- Finding related documents through entity connections
- Multi-hop traversal from query entities to distant but relevant content
- Graph-vector fusion for enriched search results

## Building the Graph

```bash
# Build from indexed documents
raven graph build --db ./raven.db --output ./raven-graph.json
```

The graph is serialized as JSON and can be rebuilt at any time from the indexed chunks.

## Querying the Graph

```bash
raven graph query "Rust async runtime" --max-hops 2 --top-k 5
```

This:
1. Extracts entities from the query
2. Finds matching nodes in the graph
3. Traverses up to `max-hops` edges
4. Returns chunks associated with discovered entities

## Entity Extraction

Entities are extracted using pattern matching:
- **Proper nouns**: Capitalized words not at sentence start
- **Technical terms**: CamelCase identifiers, acronyms, hyphenated compounds
- **Multi-word entities**: Consecutive capitalized words (e.g., "Knowledge Graph")

## Graph Structure

```json
{
  "nodes": {
    "Rust": {
      "entity": "Rust",
      "chunk_ids": ["chunk_1", "chunk_5"],
      "frequency": 12
    }
  },
  "edges": [
    {
      "from": "Rust",
      "to": "Tokio",
      "weight": 3
    }
  ]
}
```

## Graph-Vector Fusion

When using the graph retriever programmatically, results can be fused with vector search results for comprehensive retrieval that combines semantic similarity with structural relationships.
