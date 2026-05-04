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
        assert!(
            chunk_overlap < chunk_size,
            "overlap must be less than chunk_size"
        );
        Self {
            chunk_size,
            chunk_overlap,
        }
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

// =============================================================================
// Token-based splitter
// =============================================================================

/// Token-aware splitter that splits on word boundaries.
/// Uses an approximate token count (~4 chars per token for English/GPT models).
pub struct TokenSplitter {
    max_tokens: usize,
    token_overlap: usize,
    chars_per_token: f32,
}

impl TokenSplitter {
    pub fn new(max_tokens: usize, token_overlap: usize) -> Self {
        assert!(
            token_overlap < max_tokens,
            "overlap must be less than max_tokens"
        );
        Self {
            max_tokens,
            token_overlap,
            chars_per_token: 4.0,
        }
    }

    pub fn with_chars_per_token(mut self, cpt: f32) -> Self {
        self.chars_per_token = cpt;
        self
    }

    fn estimate_tokens(&self, text: &str) -> usize {
        (text.len() as f32 / self.chars_per_token).ceil() as usize
    }

    /// Split text into words, then group words into chunks that fit max_tokens
    fn split_text(&self, text: &str) -> Vec<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.is_empty() {
            return vec![];
        }

        let mut chunks = Vec::new();
        let mut current = String::new();
        let mut current_tokens = 0usize;

        let overlap_tokens = self.token_overlap;

        for word in &words {
            let word_tokens = self.estimate_tokens(word);

            if current_tokens + word_tokens > self.max_tokens && !current.is_empty() {
                chunks.push(current.trim().to_string());

                // Build overlap from end of current chunk
                let overlap_words = self.get_overlap_words(&current, overlap_tokens);
                current = overlap_words;
                current_tokens = self.estimate_tokens(&current);
            }

            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
            current_tokens += word_tokens;
        }

        if !current.trim().is_empty() {
            chunks.push(current.trim().to_string());
        }

        chunks
    }

    fn get_overlap_words(&self, text: &str, target_tokens: usize) -> String {
        if target_tokens == 0 {
            return String::new();
        }
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut overlap = Vec::new();
        let mut tokens = 0usize;

        for word in words.iter().rev() {
            let wt = self.estimate_tokens(word);
            if tokens + wt > target_tokens {
                break;
            }
            overlap.push(*word);
            tokens += wt;
        }

        overlap.reverse();
        overlap.join(" ")
    }
}

impl Default for TokenSplitter {
    fn default() -> Self {
        Self::new(256, 20)
    }
}

impl Splitter for TokenSplitter {
    fn split(&self, documents: Vec<Document>) -> Vec<Chunk> {
        let mut chunks = Vec::new();

        for doc in documents {
            if self.estimate_tokens(&doc.text) <= self.max_tokens {
                let mut chunk = Chunk::new(&doc.id, &doc.text);
                chunk.metadata = doc.metadata.clone();
                chunk
                    .metadata
                    .insert("chunk_index".to_string(), "0".to_string());
                chunks.push(chunk);
                continue;
            }

            let text_chunks = self.split_text(&doc.text);
            for (i, text) in text_chunks.into_iter().enumerate() {
                let mut chunk = Chunk::new(&doc.id, text);
                chunk.metadata = doc.metadata.clone();
                chunk
                    .metadata
                    .insert("chunk_index".to_string(), i.to_string());
                chunks.push(chunk);
            }
        }

        chunks
    }
}

// =============================================================================
// Sentence-boundary splitter (semantic-aware)
// =============================================================================

/// Splits on sentence boundaries, grouping sentences into chunks that fit max_chars.
/// This is a lightweight alternative to embedding-based semantic splitting.
pub struct SentenceSplitter {
    max_chars: usize,
    overlap_sentences: usize,
}

impl SentenceSplitter {
    pub fn new(max_chars: usize, overlap_sentences: usize) -> Self {
        Self {
            max_chars,
            overlap_sentences,
        }
    }

    /// Simple sentence boundary detection
    fn split_sentences(text: &str) -> Vec<String> {
        let mut sentences = Vec::new();
        let mut current = String::new();

        for ch in text.chars() {
            current.push(ch);
            if (ch == '.' || ch == '!' || ch == '?') && current.len() > 1 {
                // Check if next char is whitespace or end (avoid splitting "Dr." etc.)
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() {
                    sentences.push(trimmed);
                    current = String::new();
                }
            }
        }

        let trimmed = current.trim().to_string();
        if !trimmed.is_empty() {
            sentences.push(trimmed);
        }

        sentences
    }
}

impl Default for SentenceSplitter {
    fn default() -> Self {
        Self::new(1000, 1)
    }
}

impl Splitter for SentenceSplitter {
    fn split(&self, documents: Vec<Document>) -> Vec<Chunk> {
        let mut chunks = Vec::new();

        for doc in documents {
            if doc.text.len() <= self.max_chars {
                let mut chunk = Chunk::new(&doc.id, &doc.text);
                chunk.metadata = doc.metadata.clone();
                chunks.push(chunk);
                continue;
            }

            let sentences = Self::split_sentences(&doc.text);
            let mut current_group: Vec<String> = Vec::new();
            let mut current_len = 0usize;
            let mut chunk_index = 0usize;

            for sentence in &sentences {
                if current_len + sentence.len() > self.max_chars && !current_group.is_empty() {
                    let text = current_group.join(" ");
                    let mut chunk = Chunk::new(&doc.id, text);
                    chunk.metadata = doc.metadata.clone();
                    chunk
                        .metadata
                        .insert("chunk_index".to_string(), chunk_index.to_string());
                    chunks.push(chunk);
                    chunk_index += 1;

                    // Keep overlap_sentences from end
                    let keep = self.overlap_sentences.min(current_group.len());
                    let overlap: Vec<String> = current_group[current_group.len() - keep..].to_vec();
                    current_len = overlap.iter().map(|s| s.len()).sum();
                    current_group = overlap;
                }

                current_len += sentence.len();
                current_group.push(sentence.clone());
            }

            if !current_group.is_empty() {
                let text = current_group.join(" ");
                let mut chunk = Chunk::new(&doc.id, text);
                chunk.metadata = doc.metadata.clone();
                chunk
                    .metadata
                    .insert("chunk_index".to_string(), chunk_index.to_string());
                chunks.push(chunk);
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
        let docs = vec![Document::new(
            "This is a longer text that needs to be split into multiple chunks.",
        )];

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

    #[test]
    fn test_token_splitter() {
        // ~4 chars per token, so 10 tokens = ~40 chars
        let splitter = TokenSplitter::new(10, 2);
        let docs = vec![Document::new(
            "The quick brown fox jumps over the lazy dog and then runs back home to sleep",
        )];

        let chunks = splitter.split(docs);
        assert!(chunks.len() > 1);

        // Each chunk should respect word boundaries
        for chunk in &chunks {
            assert!(!chunk.text.starts_with(' '));
            assert!(!chunk.text.ends_with(' '));
        }
    }

    #[test]
    fn test_token_splitter_short() {
        let splitter = TokenSplitter::new(100, 10);
        let docs = vec![Document::new("Short text")];

        let chunks = splitter.split(docs);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].text, "Short text");
        assert_eq!(chunks[0].metadata.get("chunk_index").unwrap(), "0");
    }

    #[test]
    fn test_sentence_splitter() {
        let splitter = SentenceSplitter::new(50, 1);
        let docs = vec![Document::new(
            "First sentence here. Second sentence here. Third sentence that is also here. Fourth one.",
        )];

        let chunks = splitter.split(docs);
        assert!(chunks.len() > 1);

        // Verify chunk_index metadata
        for (i, chunk) in chunks.iter().enumerate() {
            assert_eq!(chunk.metadata.get("chunk_index").unwrap(), &i.to_string());
        }
    }

    #[test]
    fn test_sentence_splitter_short() {
        let splitter = SentenceSplitter::new(1000, 1);
        let docs = vec![Document::new("One short sentence.")];

        let chunks = splitter.split(docs);
        assert_eq!(chunks.len(), 1);
    }
}
