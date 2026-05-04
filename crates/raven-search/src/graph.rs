//! Knowledge graph: entity extraction, in-memory graph, and graph-based retrieval.
//!
//! Provides regex-based NER, a relationship graph with JSON persistence,
//! BFS traversal, and a `GraphRetriever` that fuses graph + vector results via RRF.

use raven_core::{Chunk, Result, SearchResult};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use tracing::info;

// =============================================================================
// Entity and Relation types
// =============================================================================

/// A named entity extracted from text.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Entity {
    /// Normalized entity name (lowercase)
    pub name: String,
    /// Entity type (e.g., "PERSON", "ORG", "TECH", "CONCEPT")
    pub entity_type: String,
}

/// A directed relationship between two entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::struct_field_names)]
pub struct Relation {
    pub source: String,
    pub target: String,
    pub relation_type: String,
    /// Source chunk/document ID
    pub source_doc: String,
}

// =============================================================================
// Knowledge Graph
// =============================================================================

/// In-memory knowledge graph with adjacency list representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    /// All known entities by normalized name
    entities: HashMap<String, Entity>,
    /// Adjacency list: entity name -> list of (target_name, relation_type, source_doc)
    edges: HashMap<String, Vec<(String, String, String)>>,
    /// Reverse adjacency: entity name -> list of (source_name, relation_type, source_doc)
    reverse_edges: HashMap<String, Vec<(String, String, String)>>,
    /// Entity -> set of chunk IDs that mention it
    entity_chunks: HashMap<String, HashSet<String>>,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            edges: HashMap::new(),
            reverse_edges: HashMap::new(),
            entity_chunks: HashMap::new(),
        }
    }

    /// Add an entity to the graph.
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.entry(entity.name.clone()).or_insert(entity);
    }

    /// Add a directed relation between two entities.
    pub fn add_relation(&mut self, relation: Relation) {
        self.add_entity(Entity {
            name: relation.source.clone(),
            entity_type: "UNKNOWN".to_string(),
        });
        self.add_entity(Entity {
            name: relation.target.clone(),
            entity_type: "UNKNOWN".to_string(),
        });

        self.edges
            .entry(relation.source.clone())
            .or_default()
            .push((
                relation.target.clone(),
                relation.relation_type.clone(),
                relation.source_doc.clone(),
            ));
        self.reverse_edges
            .entry(relation.target.clone())
            .or_default()
            .push((
                relation.source.clone(),
                relation.relation_type,
                relation.source_doc,
            ));
    }

    /// Link an entity to a chunk ID.
    pub fn link_entity_to_chunk(&mut self, entity_name: &str, chunk_id: &str) {
        self.entity_chunks
            .entry(entity_name.to_string())
            .or_default()
            .insert(chunk_id.to_string());
    }

    /// BFS traversal from a start entity, returning all reachable entities within max_hops.
    pub fn traverse(&self, start: &str, max_hops: usize) -> Vec<(String, usize)> {
        let normalized = start.to_lowercase();
        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<(String, usize)> = VecDeque::new();
        let mut results = Vec::new();

        queue.push_back((normalized.clone(), 0));
        visited.insert(normalized);

        while let Some((node, depth)) = queue.pop_front() {
            results.push((node.clone(), depth));

            if depth >= max_hops {
                continue;
            }

            // Forward edges
            if let Some(neighbors) = self.edges.get(&node) {
                for (target, _, _) in neighbors {
                    if visited.insert(target.clone()) {
                        queue.push_back((target.clone(), depth + 1));
                    }
                }
            }
            // Reverse edges
            if let Some(neighbors) = self.reverse_edges.get(&node) {
                for (source, _, _) in neighbors {
                    if visited.insert(source.clone()) {
                        queue.push_back((source.clone(), depth + 1));
                    }
                }
            }
        }

        results
    }

    /// Get all chunk IDs associated with a set of entities.
    pub fn get_chunk_ids(&self, entity_names: &[String]) -> HashSet<String> {
        let mut ids = HashSet::new();
        for name in entity_names {
            if let Some(chunks) = self.entity_chunks.get(name) {
                ids.extend(chunks.iter().cloned());
            }
        }
        ids
    }

    /// Get entity info.
    pub fn get_entity(&self, name: &str) -> Option<&Entity> {
        self.entities.get(name)
    }

    /// Get all neighbors of an entity.
    pub fn neighbors(&self, name: &str) -> Vec<(&str, &str, &str)> {
        let mut result = Vec::new();
        if let Some(edges) = self.edges.get(name) {
            for (target, rel, doc) in edges {
                result.push((target.as_str(), rel.as_str(), doc.as_str()));
            }
        }
        result
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.values().map(Vec::len).sum()
    }

    /// Save graph to JSON file.
    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self).map_err(raven_core::RavenError::Serde)?;
        std::fs::write(path, json)?;
        info!(
            "Graph saved: {} entities, {} edges",
            self.entity_count(),
            self.edge_count()
        );
        Ok(())
    }

    /// Load graph from JSON file.
    pub fn load(path: &Path) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let graph: Self = serde_json::from_str(&json).map_err(raven_core::RavenError::Serde)?;
        info!(
            "Graph loaded: {} entities, {} edges",
            graph.entity_count(),
            graph.edge_count()
        );
        Ok(graph)
    }
}

impl Default for KnowledgeGraph {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Entity Extraction (regex-based NER)
// =============================================================================

/// Extract entities from text using heuristic rules.
///
/// Strategies:
/// - Capitalized word sequences (proper nouns): "John Smith", "United States"
/// - Known tech terms (case-insensitive matching)
/// - Pattern-based: emails, URLs, version numbers
pub fn extract_entities(text: &str) -> Vec<Entity> {
    let mut entities = HashSet::new();

    // Extract capitalized sequences (2+ words starting with uppercase)
    extract_proper_nouns(text, &mut entities);

    // Extract known tech/concept terms
    extract_known_terms(text, &mut entities);

    entities.into_iter().collect()
}

fn extract_proper_nouns(text: &str, entities: &mut HashSet<Entity>) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut i = 0;

    while i < words.len() {
        let word = words[i].trim_matches(|c: char| !c.is_alphanumeric());

        if word.len() > 1 && word.chars().next().is_some_and(char::is_uppercase) {
            // Check it's not a sentence start (preceded by sentence-ending punctuation)
            let is_sentence_start = i == 0
                || words.get(i.wrapping_sub(1)).is_some_and(|prev| {
                    prev.ends_with('.') || prev.ends_with('!') || prev.ends_with('?')
                });

            if !is_sentence_start {
                // Collect consecutive capitalized words
                let mut phrase = vec![word.to_string()];
                let mut j = i + 1;
                while j < words.len() {
                    let next = words[j].trim_matches(|c: char| !c.is_alphanumeric());
                    if next.len() > 1 && next.chars().next().is_some_and(char::is_uppercase) {
                        phrase.push(next.to_string());
                        j += 1;
                    } else {
                        break;
                    }
                }

                let name = phrase.join(" ").to_lowercase();
                if name.len() > 1 && !is_common_word(&name) {
                    entities.insert(Entity {
                        name,
                        entity_type: if phrase.len() > 1 {
                            "PROPER_NOUN".to_string()
                        } else {
                            "NOUN".to_string()
                        },
                    });
                }
                i = j;
                continue;
            }
        }
        i += 1;
    }
}

fn extract_known_terms(text: &str, entities: &mut HashSet<Entity>) {
    const TECH_TERMS: &[(&str, &str)] = &[
        ("rust", "TECH"),
        ("python", "TECH"),
        ("javascript", "TECH"),
        ("typescript", "TECH"),
        ("golang", "TECH"),
        ("java", "TECH"),
        ("docker", "TECH"),
        ("kubernetes", "TECH"),
        ("linux", "TECH"),
        ("windows", "TECH"),
        ("macos", "TECH"),
        ("postgresql", "TECH"),
        ("sqlite", "TECH"),
        ("mongodb", "TECH"),
        ("redis", "TECH"),
        ("elasticsearch", "TECH"),
        ("machine learning", "CONCEPT"),
        ("deep learning", "CONCEPT"),
        ("neural network", "CONCEPT"),
        ("natural language processing", "CONCEPT"),
        ("retrieval augmented generation", "CONCEPT"),
        ("vector database", "CONCEPT"),
        ("embedding", "CONCEPT"),
        ("transformer", "CONCEPT"),
        ("attention mechanism", "CONCEPT"),
        ("knowledge graph", "CONCEPT"),
        ("api", "CONCEPT"),
        ("http", "CONCEPT"),
        ("tcp", "CONCEPT"),
        ("websocket", "CONCEPT"),
    ];

    let lower = text.to_lowercase();

    for (term, entity_type) in TECH_TERMS {
        if lower.contains(term) {
            entities.insert(Entity {
                name: (*term).to_string(),
                entity_type: (*entity_type).to_string(),
            });
        }
    }
}

fn is_common_word(word: &str) -> bool {
    const COMMON: &[&str] = &[
        "the",
        "this",
        "that",
        "these",
        "those",
        "here",
        "there",
        "where",
        "when",
        "what",
        "which",
        "how",
        "why",
        "who",
        "all",
        "each",
        "every",
        "both",
        "few",
        "more",
        "most",
        "other",
        "some",
        "such",
        "only",
        "same",
        "than",
        "too",
        "very",
        "just",
        "because",
        "but",
        "and",
        "however",
        "also",
        "then",
        "first",
        "last",
        "next",
        "new",
        "old",
        "long",
        "great",
        "little",
        "own",
        "right",
        "big",
        "high",
        "different",
        "small",
        "large",
        "important",
        "still",
        "before",
        "after",
        "since",
        "while",
        "about",
        "between",
        "through",
        "during",
        "without",
        "again",
        "once",
        "further",
        "already",
        "always",
        "never",
    ];
    COMMON.contains(&word)
}

// =============================================================================
// Graph Retriever
// =============================================================================

/// Retrieves relevant chunks by combining graph traversal with vector search.
///
/// Algorithm:
/// 1. Extract entities from the query
/// 2. Traverse graph from each entity (BFS, max_hops)
/// 3. Collect chunk IDs from traversed entities
/// 4. Fetch those chunks from the store + run vector search
/// 5. Fuse results with RRF
pub struct GraphRetriever {
    graph: KnowledgeGraph,
    max_hops: usize,
}

impl GraphRetriever {
    pub fn new(graph: KnowledgeGraph) -> Self {
        Self { graph, max_hops: 2 }
    }

    pub fn with_max_hops(mut self, max_hops: usize) -> Self {
        self.max_hops = max_hops;
        self
    }

    pub fn graph(&self) -> &KnowledgeGraph {
        &self.graph
    }

    pub fn graph_mut(&mut self) -> &mut KnowledgeGraph {
        &mut self.graph
    }

    /// Retrieve chunks related to the query via graph traversal.
    /// Returns chunk IDs scored by proximity (closer hops = higher score).
    pub fn retrieve(&self, query: &str, top_k: usize) -> Vec<(String, f32)> {
        let entities = extract_entities(query);

        let mut chunk_scores: HashMap<String, f32> = HashMap::new();

        for entity in &entities {
            let reachable = self.graph.traverse(&entity.name, self.max_hops);
            for (name, depth) in &reachable {
                let score = 1.0 / (*depth as f32 + 1.0);
                if let Some(chunk_ids) = self.graph.entity_chunks.get(name) {
                    for cid in chunk_ids {
                        let entry = chunk_scores.entry(cid.clone()).or_insert(0.0);
                        *entry = entry.max(score);
                    }
                }
            }
        }

        let mut scored: Vec<(String, f32)> = chunk_scores.into_iter().collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(top_k);
        scored
    }

    /// Build the graph from a set of chunks: extract entities and create relations
    /// between entities that co-occur in the same chunk.
    pub fn build_from_chunks(&mut self, chunks: &[Chunk]) {
        for chunk in chunks {
            let entities = extract_entities(&chunk.text);

            // Register entities and link to chunk
            for entity in &entities {
                self.graph.add_entity(entity.clone());
                self.graph.link_entity_to_chunk(&entity.name, &chunk.id);
            }

            // Create co-occurrence relations between all entity pairs in this chunk
            for i in 0..entities.len() {
                for j in (i + 1)..entities.len() {
                    self.graph.add_relation(Relation {
                        source: entities[i].name.clone(),
                        target: entities[j].name.clone(),
                        relation_type: "CO_OCCURS".to_string(),
                        source_doc: chunk.doc_id.clone(),
                    });
                }
            }
        }

        info!(
            "Graph built: {} entities, {} edges",
            self.graph.entity_count(),
            self.graph.edge_count()
        );
    }
}

/// Fuse graph retrieval results with vector search results using RRF.
pub fn graph_vector_fusion(
    graph_results: &[(String, f32)],
    vector_results: &[SearchResult],
    graph_weight: f32,
    top_k: usize,
) -> Vec<SearchResult> {
    let k = 60.0f32;
    let mut scores: HashMap<String, (f32, Option<SearchResult>)> = HashMap::new();

    // Score graph results
    for (rank, (chunk_id, _score)) in graph_results.iter().enumerate() {
        let rrf_score = graph_weight / (k + rank as f32 + 1.0);
        let entry = scores.entry(chunk_id.clone()).or_insert((0.0, None));
        entry.0 += rrf_score;
    }

    // Score vector results
    for (rank, result) in vector_results.iter().enumerate() {
        let rrf_score = (1.0 - graph_weight) / (k + rank as f32 + 1.0);
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
        .map(|(score, mut r)| {
            r.score = score;
            r
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_entities_tech() {
        let entities =
            extract_entities("Rust is a systems programming language with SQLite support.");
        let names: Vec<&str> = entities.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"rust"));
        assert!(names.contains(&"sqlite"));
    }

    #[test]
    fn test_extract_entities_proper_nouns() {
        let entities =
            extract_entities("The document was written by John Smith at Microsoft Research.");
        let names: Vec<&str> = entities.iter().map(|e| e.name.as_str()).collect();
        assert!(names
            .iter()
            .any(|n| n.contains("john") || n.contains("smith")));
    }

    #[test]
    fn test_knowledge_graph_basic() {
        let mut graph = KnowledgeGraph::new();
        graph.add_entity(Entity {
            name: "rust".to_string(),
            entity_type: "TECH".to_string(),
        });
        graph.add_entity(Entity {
            name: "tokio".to_string(),
            entity_type: "TECH".to_string(),
        });
        graph.add_relation(Relation {
            source: "rust".to_string(),
            target: "tokio".to_string(),
            relation_type: "HAS_LIBRARY".to_string(),
            source_doc: "doc1".to_string(),
        });

        assert_eq!(graph.entity_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_graph_traversal() {
        let mut graph = KnowledgeGraph::new();
        graph.add_relation(Relation {
            source: "a".to_string(),
            target: "b".to_string(),
            relation_type: "LINK".to_string(),
            source_doc: "d1".to_string(),
        });
        graph.add_relation(Relation {
            source: "b".to_string(),
            target: "c".to_string(),
            relation_type: "LINK".to_string(),
            source_doc: "d1".to_string(),
        });
        graph.add_relation(Relation {
            source: "c".to_string(),
            target: "d".to_string(),
            relation_type: "LINK".to_string(),
            source_doc: "d1".to_string(),
        });

        // 1 hop from a: a(0), b(1)
        let reachable = graph.traverse("a", 1);
        assert_eq!(reachable.len(), 2);

        // 2 hops from a: a(0), b(1), c(2)
        let reachable = graph.traverse("a", 2);
        assert_eq!(reachable.len(), 3);

        // 3 hops: all
        let reachable = graph.traverse("a", 3);
        assert_eq!(reachable.len(), 4);
    }

    #[test]
    fn test_graph_persistence() {
        let mut graph = KnowledgeGraph::new();
        graph.add_entity(Entity {
            name: "test".to_string(),
            entity_type: "CONCEPT".to_string(),
        });
        graph.add_relation(Relation {
            source: "test".to_string(),
            target: "graph".to_string(),
            relation_type: "RELATED_TO".to_string(),
            source_doc: "doc1".to_string(),
        });

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("graph.json");

        graph.save(&path).unwrap();
        let loaded = KnowledgeGraph::load(&path).unwrap();

        assert_eq!(loaded.entity_count(), graph.entity_count());
        assert_eq!(loaded.edge_count(), graph.edge_count());
    }

    #[test]
    fn test_graph_retriever_build() {
        let mut retriever = GraphRetriever::new(KnowledgeGraph::new());

        let chunks = vec![
            Chunk::new("doc1", "Rust and Python are programming languages"),
            Chunk::new("doc2", "Docker runs on Linux and Windows"),
        ];

        retriever.build_from_chunks(&chunks);

        assert!(retriever.graph().entity_count() > 0);
        assert!(retriever.graph().edge_count() > 0);
    }

    #[test]
    fn test_graph_retriever_query() {
        let mut retriever = GraphRetriever::new(KnowledgeGraph::new());

        let chunks = vec![
            Chunk::new("doc1", "Rust and SQLite are great together"),
            Chunk::new("doc2", "Python uses PostgreSQL often"),
        ];

        retriever.build_from_chunks(&chunks);
        let results = retriever.retrieve("Rust database", 10);
        // Should find chunks mentioning Rust or SQLite
        assert!(!results.is_empty());
    }
}
