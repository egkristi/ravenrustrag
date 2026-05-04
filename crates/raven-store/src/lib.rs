//! Vector store backends for RavenRustRAG.
//!
//! Provides the `VectorStore` trait and implementations: SQLite (persistent) and Memory (testing).

use async_trait::async_trait;
use raven_core::{Chunk, RavenError, Result, SearchResult};
use rusqlite::{params, Connection};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Metadata filter for search queries
#[derive(Debug, Clone, Default)]
pub struct MetadataFilter {
    /// Key-value pairs that must all match (AND logic)
    pub conditions: HashMap<String, String>,
}

impl MetadataFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.conditions.insert(key.into(), value.into());
        self
    }

    pub fn is_empty(&self) -> bool {
        self.conditions.is_empty()
    }

    /// Check if a chunk's metadata matches all filter conditions
    pub fn matches(&self, metadata: &HashMap<String, String>) -> bool {
        self.conditions
            .iter()
            .all(|(k, v)| metadata.get(k).is_some_and(|mv| mv == v))
    }
}

/// Vector storage backend trait
#[async_trait]
pub trait VectorStore: Send + Sync {
    /// Add chunks to the store
    async fn add(&self, chunks: &[Chunk]) -> Result<()>;

    /// Search for similar chunks
    async fn search(&self, query: &[f32], top_k: usize) -> Result<Vec<SearchResult>>;

    /// Search with metadata filtering
    async fn search_filtered(
        &self,
        query: &[f32],
        top_k: usize,
        filter: &MetadataFilter,
    ) -> Result<Vec<SearchResult>> {
        // Default: search then filter (backends can override for efficiency)
        let results = self.search(query, top_k * 3).await?;
        let filtered: Vec<SearchResult> = results
            .into_iter()
            .filter(|r| filter.matches(&r.chunk.metadata))
            .take(top_k)
            .collect();
        Ok(filtered)
    }

    /// Delete all chunks for a document
    async fn delete(&self, doc_id: &str) -> Result<()>;

    /// Get total chunk count
    async fn count(&self) -> Result<usize>;

    /// Clear all data
    async fn clear(&self) -> Result<()>;

    /// Get all chunks (for export)
    async fn all(&self) -> Result<Vec<Chunk>>;

    /// Check if a fingerprint exists and matches
    async fn get_fingerprint(&self, path: &str) -> Result<Option<String>>;

    /// Set fingerprint for a path
    async fn set_fingerprint(&self, path: &str, hash: &str) -> Result<()>;

    /// Delete fingerprint for a path
    async fn delete_fingerprint(&self, path: &str) -> Result<()>;

    /// Save BM25 term data for a chunk (for persistent BM25 index)
    async fn save_bm25_terms(
        &self,
        _chunk_id: &str,
        _terms: &HashMap<String, f32>,
        _doc_length: f32,
    ) -> Result<()> {
        Ok(()) // Default no-op for stores that don't support BM25 persistence
    }

    /// Load all BM25 term data (for rebuilding BM25 index on startup)
    async fn load_bm25_data(&self) -> Result<Vec<Bm25TermData>> {
        Ok(vec![]) // Default empty for stores that don't support BM25 persistence
    }

    /// Delete BM25 terms for a document
    async fn delete_bm25_terms(&self, _doc_id: &str) -> Result<()> {
        Ok(())
    }

    /// Clear all BM25 term data
    async fn clear_bm25(&self) -> Result<()> {
        Ok(())
    }
}

/// Data structure for BM25 term storage
#[derive(Debug, Clone)]
pub struct Bm25TermData {
    pub chunk_id: String,
    pub doc_id: String,
    pub text: String,
    pub terms: HashMap<String, f32>,
    pub doc_length: f32,
}

/// SQLite-backed vector store with flat (brute-force) search
pub struct SqliteStore {
    conn: Arc<Mutex<Connection>>,
    dimension: usize,
}

/// Current schema version
const SCHEMA_VERSION: i64 = 1;

impl SqliteStore {
    #[allow(clippy::unused_async)]
    pub async fn new(path: impl AsRef<Path>, dimension: usize) -> Result<Self> {
        let conn = Connection::open(path)
            .map_err(|e| RavenError::Store(format!("Failed to open SQLite: {e}")))?;

        // Enable WAL mode for concurrent read performance
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             PRAGMA cache_size=-64000;
             PRAGMA busy_timeout=5000;
             PRAGMA mmap_size=268435456;",
        )
        .map_err(|e| RavenError::Store(format!("Failed to set PRAGMA: {e}")))?;

        // Run schema migrations
        Self::migrate(&conn)?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            dimension,
        })
    }

    /// Run schema migrations from current version to latest
    fn migrate(conn: &Connection) -> Result<()> {
        // Create schema_version table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| RavenError::Store(format!("Failed to create schema_version table: {e}")))?;

        let current_version: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM schema_version",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        if current_version < 1 {
            Self::migrate_to_v1(conn)?;
        }

        // Future migrations:
        // if current_version < 2 { Self::migrate_to_v2(conn)?; }

        Ok(())
    }

    /// Migration to version 1: initial schema
    fn migrate_to_v1(conn: &Connection) -> Result<()> {
        tracing::info!("Running migration to schema version 1");

        conn.execute_batch(
            "BEGIN;

            CREATE TABLE IF NOT EXISTS chunks (
                id TEXT PRIMARY KEY,
                doc_id TEXT NOT NULL,
                text TEXT NOT NULL,
                metadata TEXT NOT NULL DEFAULT '{}',
                embedding BLOB NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_chunks_doc_id ON chunks(doc_id);

            CREATE TABLE IF NOT EXISTS fingerprints (
                path TEXT PRIMARY KEY,
                content_hash TEXT NOT NULL,
                modified INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS bm25_terms (
                chunk_id TEXT PRIMARY KEY,
                doc_id TEXT NOT NULL,
                text TEXT NOT NULL,
                terms TEXT NOT NULL,
                doc_length REAL NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_bm25_doc_id ON bm25_terms(doc_id);

            COMMIT;",
        )
        .map_err(|e| RavenError::Store(format!("Migration v1 failed: {e}")))?;

        // Record the migration
        conn.execute(
            "INSERT INTO schema_version (version, updated_at) VALUES (?1, datetime('now'))",
            params![SCHEMA_VERSION],
        )
        .map_err(|e| RavenError::Store(format!("Failed to record schema version: {e}")))?;

        tracing::info!("Migration to schema version 1 complete");
        Ok(())
    }

    /// Get the current schema version of the database
    pub async fn schema_version(&self) -> Result<i64> {
        let conn = self.conn.lock().await;
        let version: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM schema_version",
                [],
                |row| row.get(0),
            )
            .map_err(|e| RavenError::Store(format!("Failed to read schema version: {e}")))?;
        Ok(version)
    }

    #[inline]
    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        raven_core::cosine_similarity(a, b)
    }
}

#[async_trait]
impl VectorStore for SqliteStore {
    async fn add(&self, chunks: &[Chunk]) -> Result<()> {
        let conn = self.conn.lock().await;
        let tx = conn
            .unchecked_transaction()
            .map_err(|e| RavenError::Store(format!("Transaction failed: {e}")))?;

        for chunk in chunks {
            let embedding = chunk
                .embedding
                .as_ref()
                .ok_or_else(|| RavenError::Store("Chunk missing embedding".to_string()))?;

            if embedding.len() != self.dimension {
                return Err(RavenError::Store(format!(
                    "Embedding dimension mismatch: expected {}, got {}",
                    self.dimension,
                    embedding.len()
                )));
            }

            let embedding_bytes = embedding
                .iter()
                .flat_map(|f| f.to_le_bytes())
                .collect::<Vec<_>>();
            let metadata = serde_json::to_string(&chunk.metadata).map_err(RavenError::Serde)?;

            tx.execute(
                "INSERT OR REPLACE INTO chunks (id, doc_id, text, metadata, embedding) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![&chunk.id, &chunk.doc_id, &chunk.text, metadata, embedding_bytes],
            )
            .map_err(|e| RavenError::Store(format!("Insert failed: {e}")))?;
        }

        tx.commit()
            .map_err(|e| RavenError::Store(format!("Commit failed: {e}")))?;

        Ok(())
    }

    async fn search(&self, query: &[f32], top_k: usize) -> Result<Vec<SearchResult>> {
        let conn = self.conn.lock().await;

        let mut stmt = conn
            .prepare("SELECT id, doc_id, text, metadata, embedding FROM chunks")
            .map_err(|e| RavenError::Store(format!("Query prepare failed: {e}")))?;

        let chunk_iter = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let doc_id: String = row.get(1)?;
                let text: String = row.get(2)?;
                let metadata_str: String = row.get(3)?;
                let embedding_bytes: Vec<u8> = row.get(4)?;

                let metadata = serde_json::from_str(&metadata_str).unwrap_or_default();

                let embedding = embedding_bytes
                    .chunks_exact(4)
                    .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
                    .collect::<Vec<f32>>();

                Ok(Chunk {
                    id,
                    doc_id,
                    text,
                    metadata,
                    embedding: Some(embedding),
                })
            })
            .map_err(|e| RavenError::Store(format!("Query failed: {e}")))?;

        let mut scored: Vec<(f32, Chunk)> = Vec::new();

        for chunk_result in chunk_iter {
            let chunk = chunk_result.map_err(|e| RavenError::Store(format!("Row error: {e}")))?;
            if let Some(embedding) = chunk.embedding.as_ref() {
                let score = Self::cosine_similarity(query, embedding);
                scored.push((score, chunk));
            }
        }

        // Sort by score descending
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(top_k);

        let results = scored
            .into_iter()
            .map(|(score, chunk)| SearchResult {
                distance: 1.0 - score,
                score,
                chunk,
            })
            .collect();

        Ok(results)
    }

    async fn search_filtered(
        &self,
        query: &[f32],
        top_k: usize,
        filter: &MetadataFilter,
    ) -> Result<Vec<SearchResult>> {
        if filter.is_empty() {
            return self.search(query, top_k).await;
        }

        let conn = self.conn.lock().await;

        let mut stmt = conn
            .prepare("SELECT id, doc_id, text, metadata, embedding FROM chunks")
            .map_err(|e| RavenError::Store(format!("Query prepare failed: {e}")))?;

        let chunk_iter = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let doc_id: String = row.get(1)?;
                let text: String = row.get(2)?;
                let metadata_str: String = row.get(3)?;
                let embedding_bytes: Vec<u8> = row.get(4)?;

                let metadata: HashMap<String, String> =
                    serde_json::from_str(&metadata_str).unwrap_or_default();

                let embedding = embedding_bytes
                    .chunks_exact(4)
                    .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
                    .collect::<Vec<f32>>();

                Ok(Chunk {
                    id,
                    doc_id,
                    text,
                    metadata,
                    embedding: Some(embedding),
                })
            })
            .map_err(|e| RavenError::Store(format!("Query failed: {e}")))?;

        let mut scored: Vec<(f32, Chunk)> = Vec::new();

        for chunk_result in chunk_iter {
            let chunk = chunk_result.map_err(|e| RavenError::Store(format!("Row error: {e}")))?;
            // Apply metadata filter before scoring
            if !filter.matches(&chunk.metadata) {
                continue;
            }
            if let Some(embedding) = chunk.embedding.as_ref() {
                let score = Self::cosine_similarity(query, embedding);
                scored.push((score, chunk));
            }
        }

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(top_k);

        let results = scored
            .into_iter()
            .map(|(score, chunk)| SearchResult {
                distance: 1.0 - score,
                score,
                chunk,
            })
            .collect();

        Ok(results)
    }

    async fn delete(&self, doc_id: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM chunks WHERE doc_id = ?1", [doc_id])
            .map_err(|e| RavenError::Store(format!("Delete failed: {e}")))?;
        Ok(())
    }

    async fn count(&self) -> Result<usize> {
        let conn = self.conn.lock().await;
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM chunks", [], |row| row.get(0))
            .map_err(|e| RavenError::Store(format!("Count failed: {e}")))?;
        Ok(count as usize)
    }

    async fn clear(&self) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM chunks", [])
            .map_err(|e| RavenError::Store(format!("Clear failed: {e}")))?;
        Ok(())
    }

    async fn all(&self) -> Result<Vec<Chunk>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn
            .prepare("SELECT id, doc_id, text, metadata, embedding FROM chunks")
            .map_err(|e| RavenError::Store(format!("Query prepare failed: {e}")))?;

        let chunks = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let doc_id: String = row.get(1)?;
                let text: String = row.get(2)?;
                let metadata_str: String = row.get(3)?;
                let embedding_bytes: Vec<u8> = row.get(4)?;

                let metadata = serde_json::from_str(&metadata_str).unwrap_or_default();
                let embedding = embedding_bytes
                    .chunks_exact(4)
                    .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
                    .collect::<Vec<f32>>();

                Ok(Chunk {
                    id,
                    doc_id,
                    text,
                    metadata,
                    embedding: Some(embedding),
                })
            })
            .map_err(|e| RavenError::Store(format!("Query failed: {e}")))?;

        let mut result = Vec::new();
        for chunk in chunks {
            result.push(chunk.map_err(|e| RavenError::Store(format!("Row error: {e}")))?);
        }
        Ok(result)
    }

    async fn get_fingerprint(&self, path: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().await;
        let result = conn.query_row(
            "SELECT content_hash FROM fingerprints WHERE path = ?1",
            [path],
            |row| row.get::<_, String>(0),
        );
        match result {
            Ok(hash) => Ok(Some(hash)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(RavenError::Store(format!("Fingerprint query failed: {e}"))),
        }
    }

    async fn set_fingerprint(&self, path: &str, hash: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT OR REPLACE INTO fingerprints (path, content_hash, modified) VALUES (?1, ?2, ?3)",
            rusqlite::params![path, hash, chrono::Utc::now().timestamp()],
        )
        .map_err(|e| RavenError::Store(format!("Fingerprint set failed: {e}")))?;
        Ok(())
    }

    async fn delete_fingerprint(&self, path: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM fingerprints WHERE path = ?1", [path])
            .map_err(|e| RavenError::Store(format!("Fingerprint delete failed: {e}")))?;
        Ok(())
    }

    async fn save_bm25_terms(
        &self,
        chunk_id: &str,
        terms: &HashMap<String, f32>,
        doc_length: f32,
    ) -> Result<()> {
        let conn = self.conn.lock().await;
        let terms_json = serde_json::to_string(terms).map_err(RavenError::Serde)?;

        // We need the doc_id and text from the chunks table
        let (doc_id, text): (String, String) = conn
            .query_row(
                "SELECT doc_id, text FROM chunks WHERE id = ?1",
                [chunk_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .map_err(|e| RavenError::Store(format!("BM25 chunk lookup failed: {e}")))?;

        conn.execute(
            "INSERT OR REPLACE INTO bm25_terms (chunk_id, doc_id, text, terms, doc_length) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![chunk_id, doc_id, text, terms_json, doc_length],
        )
        .map_err(|e| RavenError::Store(format!("BM25 save failed: {e}")))?;
        Ok(())
    }

    async fn load_bm25_data(&self) -> Result<Vec<Bm25TermData>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn
            .prepare("SELECT chunk_id, doc_id, text, terms, doc_length FROM bm25_terms")
            .map_err(|e| RavenError::Store(format!("BM25 load prepare failed: {e}")))?;

        let rows = stmt
            .query_map([], |row| {
                let chunk_id: String = row.get(0)?;
                let doc_id: String = row.get(1)?;
                let text: String = row.get(2)?;
                let terms_json: String = row.get(3)?;
                let doc_length: f32 = row.get(4)?;

                let terms: HashMap<String, f32> =
                    serde_json::from_str(&terms_json).unwrap_or_default();

                Ok(Bm25TermData {
                    chunk_id,
                    doc_id,
                    text,
                    terms,
                    doc_length,
                })
            })
            .map_err(|e| RavenError::Store(format!("BM25 load failed: {e}")))?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|e| RavenError::Store(format!("BM25 row error: {e}")))?);
        }
        Ok(result)
    }

    async fn delete_bm25_terms(&self, doc_id: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM bm25_terms WHERE doc_id = ?1", [doc_id])
            .map_err(|e| RavenError::Store(format!("BM25 delete failed: {e}")))?;
        Ok(())
    }

    async fn clear_bm25(&self) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM bm25_terms", [])
            .map_err(|e| RavenError::Store(format!("BM25 clear failed: {e}")))?;
        Ok(())
    }
}

/// In-memory store for testing
pub struct MemoryStore {
    chunks: Arc<Mutex<Vec<Chunk>>>,
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            chunks: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl VectorStore for MemoryStore {
    async fn add(&self, chunks: &[Chunk]) -> Result<()> {
        let mut store = self.chunks.lock().await;
        store.extend(chunks.iter().cloned());
        Ok(())
    }

    async fn search(&self, query: &[f32], top_k: usize) -> Result<Vec<SearchResult>> {
        let store = self.chunks.lock().await;

        let mut scored: Vec<(f32, Chunk)> = Vec::new();
        for chunk in store.iter() {
            if let Some(embedding) = &chunk.embedding {
                let score = SqliteStore::cosine_similarity(query, embedding);
                scored.push((score, chunk.clone()));
            }
        }

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(top_k);

        Ok(scored
            .into_iter()
            .map(|(score, chunk)| SearchResult {
                distance: 1.0 - score,
                score,
                chunk,
            })
            .collect())
    }

    async fn delete(&self, doc_id: &str) -> Result<()> {
        let mut store = self.chunks.lock().await;
        store.retain(|c| c.doc_id != doc_id);
        Ok(())
    }

    async fn count(&self) -> Result<usize> {
        Ok(self.chunks.lock().await.len())
    }

    async fn clear(&self) -> Result<()> {
        self.chunks.lock().await.clear();
        Ok(())
    }

    async fn all(&self) -> Result<Vec<Chunk>> {
        Ok(self.chunks.lock().await.clone())
    }

    async fn get_fingerprint(&self, _path: &str) -> Result<Option<String>> {
        Ok(None)
    }

    async fn set_fingerprint(&self, _path: &str, _hash: &str) -> Result<()> {
        Ok(())
    }

    async fn delete_fingerprint(&self, _path: &str) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_store() {
        let store = MemoryStore::new();

        let chunks = vec![
            Chunk::new("doc1", "hello world").with_embedding(vec![1.0, 0.0, 0.0]),
            Chunk::new("doc1", "goodbye world").with_embedding(vec![0.0, 1.0, 0.0]),
        ];

        store.add(&chunks).await.unwrap();
        assert_eq!(store.count().await.unwrap(), 2);

        let results = store.search(&[1.0, 0.0, 0.0], 1).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].chunk.text, "hello world");
        assert!(results[0].score > 0.99);

        store.delete("doc1").await.unwrap();
        assert_eq!(store.count().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_sqlite_store() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let store = SqliteStore::new(&db_path, 3).await.unwrap();

        let chunks = vec![
            Chunk::new("doc1", "hello world").with_embedding(vec![1.0, 0.0, 0.0]),
            Chunk::new("doc1", "goodbye world").with_embedding(vec![0.0, 1.0, 0.0]),
        ];

        store.add(&chunks).await.unwrap();
        assert_eq!(store.count().await.unwrap(), 2);

        let results = store.search(&[1.0, 0.0, 0.0], 1).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].chunk.text, "hello world");

        store.clear().await.unwrap();
        assert_eq!(store.count().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_metadata_filter() {
        let store = MemoryStore::new();

        let mut c1 = Chunk::new("doc1", "hello").with_embedding(vec![1.0, 0.0, 0.0]);
        c1.metadata.insert("lang".to_string(), "en".to_string());

        let mut c2 = Chunk::new("doc2", "bonjour").with_embedding(vec![0.9, 0.1, 0.0]);
        c2.metadata.insert("lang".to_string(), "fr".to_string());

        let mut c3 = Chunk::new("doc3", "hola").with_embedding(vec![0.8, 0.2, 0.0]);
        c3.metadata.insert("lang".to_string(), "en".to_string());

        store.add(&[c1, c2, c3]).await.unwrap();

        // Filter for English only
        let filter = MetadataFilter::new().with("lang", "en");
        let results = store
            .search_filtered(&[1.0, 0.0, 0.0], 10, &filter)
            .await
            .unwrap();
        assert_eq!(results.len(), 2);
        for r in &results {
            assert_eq!(r.chunk.metadata.get("lang").unwrap(), "en");
        }

        // Filter for French only
        let filter = MetadataFilter::new().with("lang", "fr");
        let results = store
            .search_filtered(&[1.0, 0.0, 0.0], 10, &filter)
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].chunk.text, "bonjour");

        // Empty filter returns all
        let filter = MetadataFilter::new();
        let results = store
            .search_filtered(&[1.0, 0.0, 0.0], 10, &filter)
            .await
            .unwrap();
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_sqlite_metadata_filter() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("filter_test.db");

        let store = SqliteStore::new(&db_path, 3).await.unwrap();

        let mut c1 = Chunk::new("doc1", "hello").with_embedding(vec![1.0, 0.0, 0.0]);
        c1.metadata
            .insert("type".to_string(), "greeting".to_string());

        let mut c2 = Chunk::new("doc2", "goodbye").with_embedding(vec![0.0, 1.0, 0.0]);
        c2.metadata
            .insert("type".to_string(), "farewell".to_string());

        store.add(&[c1, c2]).await.unwrap();

        let filter = MetadataFilter::new().with("type", "greeting");
        let results = store
            .search_filtered(&[1.0, 0.0, 0.0], 10, &filter)
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].chunk.text, "hello");
    }

    #[tokio::test]
    async fn test_memory_store_multiple_docs() {
        let store = MemoryStore::new();

        let chunks = vec![
            Chunk::new("doc1", "first").with_embedding(vec![1.0, 0.0]),
            Chunk::new("doc2", "second").with_embedding(vec![0.0, 1.0]),
            Chunk::new("doc3", "third").with_embedding(vec![0.7, 0.7]),
        ];

        store.add(&chunks).await.unwrap();
        assert_eq!(store.count().await.unwrap(), 3);

        // Delete only doc1
        store.delete("doc1").await.unwrap();
        assert_eq!(store.count().await.unwrap(), 2);

        // doc2 and doc3 should still be searchable
        let results = store.search(&[0.0, 1.0], 10).await.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].chunk.text, "second");
    }

    #[tokio::test]
    async fn test_memory_store_clear() {
        let store = MemoryStore::new();
        store
            .add(&[Chunk::new("d", "text").with_embedding(vec![1.0])])
            .await
            .unwrap();
        assert_eq!(store.count().await.unwrap(), 1);
        store.clear().await.unwrap();
        assert_eq!(store.count().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_memory_store_empty_search() {
        let store = MemoryStore::new();
        let results = store.search(&[1.0, 0.0], 5).await.unwrap();
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_sqlite_store_persistence() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("persist.db");

        // Add data
        {
            let store = SqliteStore::new(&db_path, 3).await.unwrap();
            store
                .add(&[Chunk::new("doc1", "persistent data").with_embedding(vec![1.0, 0.0, 0.0])])
                .await
                .unwrap();
        }

        // Reopen and verify
        {
            let store = SqliteStore::new(&db_path, 3).await.unwrap();
            assert_eq!(store.count().await.unwrap(), 1);
            let results = store.search(&[1.0, 0.0, 0.0], 1).await.unwrap();
            assert_eq!(results[0].chunk.text, "persistent data");
        }
    }

    #[tokio::test]
    async fn test_sqlite_store_delete() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("del.db");

        let store = SqliteStore::new(&db_path, 3).await.unwrap();
        store
            .add(&[
                Chunk::new("doc1", "a").with_embedding(vec![1.0, 0.0, 0.0]),
                Chunk::new("doc2", "b").with_embedding(vec![0.0, 1.0, 0.0]),
            ])
            .await
            .unwrap();

        store.delete("doc1").await.unwrap();
        assert_eq!(store.count().await.unwrap(), 1);

        let results = store.search(&[1.0, 0.0, 0.0], 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].chunk.doc_id, "doc2");
    }

    #[tokio::test]
    async fn test_sqlite_fingerprint() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("fp.db");

        let store = SqliteStore::new(&db_path, 3).await.unwrap();

        // No fingerprint initially
        let fp = store.get_fingerprint("file1.txt").await.unwrap();
        assert!(fp.is_none());

        // Store and retrieve
        store.set_fingerprint("file1.txt", "abc123").await.unwrap();
        let fp = store.get_fingerprint("file1.txt").await.unwrap();
        assert_eq!(fp, Some("abc123".to_string()));

        // Update
        store.set_fingerprint("file1.txt", "def456").await.unwrap();
        let fp = store.get_fingerprint("file1.txt").await.unwrap();
        assert_eq!(fp, Some("def456".to_string()));
    }

    #[tokio::test]
    async fn test_metadata_filter_multiple_keys() {
        let store = MemoryStore::new();

        let mut c1 = Chunk::new("d1", "match both").with_embedding(vec![1.0, 0.0]);
        c1.metadata.insert("lang".to_string(), "en".to_string());
        c1.metadata
            .insert("type".to_string(), "article".to_string());

        let mut c2 = Chunk::new("d2", "match one").with_embedding(vec![0.9, 0.1]);
        c2.metadata.insert("lang".to_string(), "en".to_string());
        c2.metadata.insert("type".to_string(), "note".to_string());

        store.add(&[c1, c2]).await.unwrap();

        let filter = MetadataFilter::new()
            .with("lang", "en")
            .with("type", "article");
        let results = store
            .search_filtered(&[1.0, 0.0], 10, &filter)
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].chunk.text, "match both");
    }

    #[tokio::test]
    async fn test_search_top_k_limit() {
        let store = MemoryStore::new();
        let chunks: Vec<Chunk> = (0..20)
            .map(|i| {
                Chunk::new(format!("doc{i}"), format!("text {i}"))
                    .with_embedding(vec![1.0 - i as f32 * 0.01, i as f32 * 0.01])
            })
            .collect();
        store.add(&chunks).await.unwrap();

        let results = store.search(&[1.0, 0.0], 5).await.unwrap();
        assert_eq!(results.len(), 5);
        // Should be sorted by score descending
        for i in 0..results.len() - 1 {
            assert!(results[i].score >= results[i + 1].score);
        }
    }
}

// =============================================================================
// HNSW-accelerated search (optional feature)
// =============================================================================

#[cfg(feature = "hnsw")]
pub mod hnsw {
    //! HNSW (Hierarchical Navigable Small World) acceleration for vector search.
    //!
    //! Provides O(log n) approximate nearest neighbor search instead of O(n) brute-force.
    //! Use `HnswIndex` as an acceleration layer on top of any `VectorStore`.

    use super::*;
    use instant_distance::{Builder, HnswMap, Search};

    /// A point in the HNSW index referencing a stored chunk.
    #[derive(Clone)]
    struct HnswPoint {
        embedding: Vec<f32>,
    }

    impl instant_distance::Point for HnswPoint {
        fn distance(&self, other: &Self) -> f32 {
            // instant-distance expects distance (lower = closer), not similarity
            1.0 - raven_core::cosine_similarity(&self.embedding, &other.embedding)
        }
    }

    /// HNSW index for O(log n) approximate nearest neighbor search.
    ///
    /// Build from chunks, then search. Rebuilding is required after adding new chunks.
    pub struct HnswIndex {
        map: Option<HnswMap<HnswPoint, String>>,
        chunks: Vec<Chunk>,
    }

    impl HnswIndex {
        pub fn new() -> Self {
            Self {
                map: None,
                chunks: Vec::new(),
            }
        }

        /// Build the HNSW index from chunks that have embeddings.
        pub fn build(&mut self, chunks: Vec<Chunk>) {
            let points: Vec<HnswPoint> = chunks
                .iter()
                .filter_map(|c| {
                    c.embedding.as_ref().map(|e| HnswPoint {
                        embedding: e.clone(),
                    })
                })
                .collect();

            let values: Vec<String> = chunks
                .iter()
                .filter(|c| c.embedding.is_some())
                .map(|c| c.id.clone())
                .collect();

            if points.is_empty() {
                self.map = None;
                self.chunks = Vec::new();
                return;
            }

            let map = Builder::default().build(points, values);
            self.map = Some(map);
            self.chunks = chunks;
        }

        /// Search for approximate nearest neighbors.
        pub fn search(&self, query: &[f32], top_k: usize) -> Vec<SearchResult> {
            let Some(ref map) = self.map else {
                return Vec::new();
            };

            let query_point = HnswPoint {
                embedding: query.to_vec(),
            };
            let mut search = Search::default();
            let results = map.search(&query_point, &mut search);

            let chunk_map: HashMap<&str, &Chunk> =
                self.chunks.iter().map(|c| (c.id.as_str(), c)).collect();

            results
                .take(top_k)
                .filter_map(|item| {
                    let chunk_id = item.value;
                    let distance = item.distance;
                    chunk_map.get(chunk_id.as_str()).map(|chunk| SearchResult {
                        chunk: (*chunk).clone(),
                        score: 1.0 - distance,
                        distance,
                    })
                })
                .collect()
        }

        /// Number of indexed points.
        pub fn len(&self) -> usize {
            self.chunks.iter().filter(|c| c.embedding.is_some()).count()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    impl Default for HnswIndex {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_hnsw_index_basic() {
            let mut index = HnswIndex::new();

            let chunks = vec![
                Chunk::new("doc1", "hello").with_embedding(vec![1.0, 0.0, 0.0]),
                Chunk::new("doc2", "world").with_embedding(vec![0.0, 1.0, 0.0]),
                Chunk::new("doc3", "test").with_embedding(vec![0.0, 0.0, 1.0]),
            ];

            index.build(chunks);
            assert_eq!(index.len(), 3);

            let results = index.search(&[1.0, 0.0, 0.0], 1);
            assert_eq!(results.len(), 1);
            assert_eq!(results[0].chunk.text, "hello");
            assert!(results[0].score > 0.99);
        }

        #[test]
        fn test_hnsw_index_empty() {
            let index = HnswIndex::new();
            let results = index.search(&[1.0, 0.0, 0.0], 5);
            assert!(results.is_empty());
        }

        #[test]
        fn test_hnsw_top_k() {
            let mut index = HnswIndex::new();

            let chunks: Vec<Chunk> = (0..100)
                .map(|i| {
                    let angle = i as f32 * 0.01;
                    Chunk::new(format!("doc{i}"), format!("text {i}")).with_embedding(vec![
                        angle.cos(),
                        angle.sin(),
                        0.0,
                    ])
                })
                .collect();

            index.build(chunks);
            assert_eq!(index.len(), 100);

            let results = index.search(&[1.0, 0.0, 0.0], 5);
            assert_eq!(results.len(), 5);
            // Results should be sorted by score (descending)
            for i in 0..results.len() - 1 {
                assert!(results[i].score >= results[i + 1].score);
            }
        }
    }
}

#[cfg(feature = "hnsw")]
pub use hnsw::HnswIndex;
