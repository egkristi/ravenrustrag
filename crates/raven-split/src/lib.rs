use raven_core::{Chunk, Document};

/// Text splitting strategy
pub trait Splitter: Send + Sync {
    fn split(&self, documents: Vec<Document>) -> Vec<Chunk>;
}

/// Simple character-based splitter with overlap
pub struct TextSplitter {
    chunk_size: usize,
    chunk_overlap: usize,
}

impl TextSplitter {
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> Self {
        assert!(chunk_overlap < chunk_size, "overlap must be less than chunk_size");
        Self { chunk_size, chunk_overlap }
    }

    pub fn with_chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = size;
        self
    }

    pub fn with_overlap(mut self, overlap: usize) -> Self {
        self.chunk_overlap = overlap;
        self
    }
}

impl Default for TextSplitter {
    fn default() -> Self {
        Self::new(512, 50)
    }
}

impl Splitter for TextSplitter {
    fn split(&self, documents: Vec<Document>) -> Vec<Chunk> {
        let mut chunks = Vec::new();

        for doc in documents {
            let text = &doc.text;
            if text.len() <= self.chunk_size {
                let mut chunk = Chunk::new(&doc.id, &doc.text);
                chunk.metadata = doc.metadata.clone();
                chunks.push(chunk);
                continue;
            }

            let step = self.chunk_size - self.chunk_overlap;
            let mut start = 0;

            while start < text.len() {
                let end = (start + self.chunk_size).min(text.len());
                let chunk_text = &text[start..end];
                
                let mut chunk = Chunk::new(&doc.id, chunk_text);
                chunk.metadata = doc.metadata.clone();
                chunks.push(chunk);

                if end == text.len() {
                    break;
                }
                start += step;
            }
        }

        chunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_splitter() {
        let splitter = TextSplitter::new(10, 2);
        let docs = vec![
            Document::new("This is a longer text that needs to be split into multiple chunks.")
        ];

        let chunks = splitter.split(docs);
        assert!(chunks.len() > 1);
        
        for chunk in &chunks {
            assert!(chunk.text.len() <= 10);
        }
    }

    #[test]
    fn test_short_document() {
        let splitter = TextSplitter::new(100, 10);
        let docs = vec![Document::new("Short")];

        let chunks = splitter.split(docs);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].text, "Short");
    }
}