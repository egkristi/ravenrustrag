//! Vector store backends for RavenRustRAG.
//!
//! Provides the `VectorStore` trait and implementations: SQLite (persistent) and Memory (testing).

use async_trait::async_trait;
use raven_core::{Chunk, RavenError, Result, SearchResult};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Vector storage backend trait
#[async_trait]
pub trait VectorStore: Send + Sync {
    /// Add chunks to the store
    async fn add(&self, chunks: &[Chunk]) -> Result<()>;

    /// Search for similar chunks
    async fn search(&self, query: &[f32], top_k: usize) -> Result<Vec<SearchResult>>;

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
}

/// SQLite-backed vector store with flat (brute-force) search
pub struct SqliteStore {
    conn: Arc<Mutex<Connection>>,
    dimension: usize,
}

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
             PRAGMA busy_timeout=5000;",
        )
        .map_err(|e| RavenError::Store(format!("Failed to set PRAGMA: {e}")))?;

        // Create tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS chunks (
                id TEXT PRIMARY KEY,
                doc_id TEXT NOT NULL,
                text TEXT NOT NULL,
                metadata TEXT NOT NULL DEFAULT '{}',
                embedding BLOB NOT NULL
            )",
            [],
        )
        .map_err(|e| RavenError::Store(format!("Failed to create table: {e}")))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_chunks_doc_id ON chunks(doc_id)",
            [],
        )
        .map_err(|e| RavenError::Store(format!("Failed to create index: {e}")))?;

        // Fingerprint table for incremental indexing
        conn.execute(
            "CREATE TABLE IF NOT EXISTS fingerprints (
                path TEXT PRIMARY KEY,
                content_hash TEXT NOT NULL,
                modified INTEGER NOT NULL
            )",
            [],
        )
        .map_err(|e| RavenError::Store(format!("Failed to create fingerprints table: {e}")))?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            dimension,
        })
    }

    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        dot / (norm_a * norm_b)
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
}
