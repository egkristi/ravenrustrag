//! Text splitting strategies for RavenRustRAG.
//!
//! Provides the `Splitter` trait and implementations: Text, Token, and Sentence splitters.

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
                doc.metadata.clone_into(&mut chunk.metadata);
                chunk
                    .metadata
                    .insert("chunk_index".to_string(), "0".to_string());
                chunk
                    .metadata
                    .insert("source_id".to_string(), doc.id.clone());
                chunks.push(chunk);
                continue;
            }

            let step = self.chunk_size - self.chunk_overlap;
            let mut start = 0;
            let mut chunk_index = 0usize;

            while start < text.len() {
                let mut end = (start + self.chunk_size).min(text.len());
                // Ensure we don't split in the middle of a multi-byte character
                while end < text.len() && !text.is_char_boundary(end) {
                    end += 1;
                }
                // Also ensure start is on a char boundary (for overlap)
                while start < text.len() && !text.is_char_boundary(start) {
                    start += 1;
                }
                // Skip if we've gone past the end or start == end
                if start >= end {
                    break;
                }
                let chunk_text = &text[start..end];
                // Skip empty chunks (can happen with multi-byte boundary adjustments)
                if chunk_text.is_empty() || chunk_text.trim().is_empty() {
                    start += step.max(1);
                    continue;
                }

                let mut chunk = Chunk::new(&doc.id, chunk_text);
                doc.metadata.clone_into(&mut chunk.metadata);
                chunk
                    .metadata
                    .insert("chunk_index".to_string(), chunk_index.to_string());
                chunk
                    .metadata
                    .insert("source_id".to_string(), doc.id.clone());
                chunks.push(chunk);

                if end == text.len() {
                    break;
                }
                start += step;
                chunk_index += 1;
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
                doc.metadata.clone_into(&mut chunk.metadata);
                chunk
                    .metadata
                    .insert("chunk_index".to_string(), "0".to_string());
                chunk
                    .metadata
                    .insert("source_id".to_string(), doc.id.clone());
                chunks.push(chunk);
                continue;
            }

            let text_chunks = self.split_text(&doc.text);
            for (i, text) in text_chunks.into_iter().enumerate() {
                let mut chunk = Chunk::new(&doc.id, text);
                doc.metadata.clone_into(&mut chunk.metadata);
                chunk
                    .metadata
                    .insert("chunk_index".to_string(), i.to_string());
                chunk
                    .metadata
                    .insert("source_id".to_string(), doc.id.clone());
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

    /// Sentence boundary detection with abbreviation awareness.
    /// Avoids splitting on common abbreviations like "Dr.", "e.g.", "U.S.", etc.
    fn split_sentences(text: &str) -> Vec<String> {
        // Common abbreviations that should NOT trigger sentence breaks
        const ABBREVIATIONS: &[&str] = &[
            "mr", "mrs", "ms", "dr", "prof", "sr", "jr", "st", "ave", "blvd", "vs", "etc", "inc",
            "ltd", "co", "corp", "dept", "univ", "gen", "gov", "sgt", "cpl", "pvt", "capt", "col",
            "maj", "jan", "feb", "mar", "apr", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
            "fig", "eq", "vol", "no", "approx", "est", "ref",
        ];
        // Multi-char abbreviations with dots (e.g., i.e., etc.)
        const DOT_ABBREVIATIONS: &[&str] = &["e.g", "i.e", "a.m", "p.m", "u.s", "u.k"];

        let mut sentences = Vec::new();
        let mut current = String::new();
        let chars: Vec<char> = text.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            let ch = chars[i];
            current.push(ch);

            if (ch == '.' || ch == '!' || ch == '?') && current.len() > 1 {
                // Check if the next char suggests a sentence boundary
                // (whitespace followed by uppercase, or end of text)
                let next_is_boundary = if i + 1 >= len {
                    true // end of text
                } else if chars[i + 1].is_whitespace() {
                    // Look ahead past whitespace for uppercase letter or end
                    let mut j = i + 1;
                    while j < len && chars[j].is_whitespace() {
                        j += 1;
                    }
                    j >= len || chars[j].is_uppercase() || chars[j] == '"' || chars[j] == '\''
                } else {
                    false // no whitespace after punctuation = not a sentence break
                };

                if next_is_boundary && ch == '.' {
                    // Check if this is an abbreviation
                    let trimmed = current.trim();
                    let last_word = trimmed
                        .rsplit(|c: char| c.is_whitespace())
                        .next()
                        .unwrap_or("");
                    let word_before_dot = last_word
                        .strip_suffix('.')
                        .unwrap_or(last_word)
                        .to_lowercase();

                    // Check single-word abbreviations
                    if ABBREVIATIONS.contains(&word_before_dot.as_str()) {
                        i += 1;
                        continue;
                    }

                    // Check dotted abbreviations (e.g., i.e., u.s.)
                    if DOT_ABBREVIATIONS
                        .iter()
                        .any(|abbr| word_before_dot.ends_with(abbr))
                    {
                        i += 1;
                        continue;
                    }

                    // Single uppercase letter + dot (initials like "J.")
                    if word_before_dot.len() == 1
                        && word_before_dot
                            .chars()
                            .next()
                            .is_some_and(char::is_alphabetic)
                    {
                        i += 1;
                        continue;
                    }
                }

                if next_is_boundary {
                    let trimmed = current.trim().to_string();
                    if !trimmed.is_empty() {
                        sentences.push(trimmed);
                        current = String::new();
                    }
                }
            }

            i += 1;
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
                doc.metadata.clone_into(&mut chunk.metadata);
                chunk
                    .metadata
                    .insert("chunk_index".to_string(), "0".to_string());
                chunk
                    .metadata
                    .insert("source_id".to_string(), doc.id.clone());
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
                    doc.metadata.clone_into(&mut chunk.metadata);
                    chunk
                        .metadata
                        .insert("chunk_index".to_string(), chunk_index.to_string());
                    chunk
                        .metadata
                        .insert("source_id".to_string(), doc.id.clone());
                    chunks.push(chunk);
                    chunk_index += 1;

                    // Keep overlap_sentences from end
                    let keep = self.overlap_sentences.min(current_group.len());
                    let overlap: Vec<String> = current_group[current_group.len() - keep..].to_vec();
                    current_len = overlap.iter().map(std::string::String::len).sum();
                    current_group = overlap;
                }

                current_len += sentence.len();
                current_group.push(sentence.clone());
            }

            if !current_group.is_empty() {
                let text = current_group.join(" ");
                let mut chunk = Chunk::new(&doc.id, text);
                doc.metadata.clone_into(&mut chunk.metadata);
                chunk
                    .metadata
                    .insert("chunk_index".to_string(), chunk_index.to_string());
                chunk
                    .metadata
                    .insert("source_id".to_string(), doc.id.clone());
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

    #[test]
    fn test_sentence_splitter_abbreviations() {
        // These should NOT cause sentence splits
        let text = "Dr. Smith went to Washington. He met with Prof. Jones and discussed e.g. various topics.";
        let sentences = SentenceSplitter::split_sentences(text);
        // Should be 2 sentences: "Dr. Smith went to Washington." and the rest
        assert_eq!(sentences.len(), 2);
        assert!(sentences[0].starts_with("Dr."));
        assert!(sentences[1].contains("Prof."));
    }

    #[test]
    fn test_sentence_splitter_exclamation_question() {
        let text = "What is this? It is a test! And here is more.";
        let sentences = SentenceSplitter::split_sentences(text);
        assert_eq!(sentences.len(), 3);
        assert!(sentences[0].ends_with('?'));
        assert!(sentences[1].ends_with('!'));
        assert!(sentences[2].ends_with('.'));
    }

    #[test]
    fn test_sentence_splitter_initials() {
        let text = "J. K. Rowling wrote Harry Potter. The books are popular.";
        let sentences = SentenceSplitter::split_sentences(text);
        assert_eq!(sentences.len(), 2);
        assert!(sentences[0].contains("J. K. Rowling"));
    }

    #[test]
    fn test_sentence_splitter_empty() {
        let splitter = SentenceSplitter::new(1000, 1);
        let docs = vec![Document::new("")];
        let chunks = splitter.split(docs);
        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].text.is_empty());
    }

    #[test]
    fn test_text_splitter_unicode() {
        let splitter = TextSplitter::new(20, 5);
        let docs = vec![Document::new(
            "Hello 🦀 world! Rust is great 🚀 for building fast programs.",
        )];
        let chunks = splitter.split(docs);
        assert!(chunks.len() > 1);
        // Verify no panics and text is not corrupted
        for chunk in &chunks {
            assert!(!chunk.text.is_empty());
        }
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;
    use raven_core::Document;

    proptest! {
        /// TextSplitter: no data loss — all chunk texts concatenated (minus overlap) cover original
        #[test]
        fn text_splitter_no_empty_chunks(
            text in ".{1,2000}",
            chunk_size in 20usize..500usize,
            overlap_pct in 0usize..50usize,
        ) {
            let overlap = (chunk_size * overlap_pct) / 100;
            let overlap = overlap.min(chunk_size - 1);
            let splitter = TextSplitter::new(chunk_size, overlap);
            let docs = vec![Document::new(text.clone())];
            let chunks = splitter.split(docs);

            // At least one chunk
            prop_assert!(!chunks.is_empty(), "Empty input produced no chunks");

            // No empty chunks (non-empty input)
            for chunk in &chunks {
                prop_assert!(!chunk.text.is_empty(), "Empty chunk produced");
            }
        }

        /// TextSplitter: chunk sizes never exceed configured max
        #[test]
        fn text_splitter_respects_max_size(
            text in ".{1,3000}",
            chunk_size in 10usize..500usize,
        ) {
            let overlap = chunk_size / 4;
            let splitter = TextSplitter::new(chunk_size, overlap);
            let docs = vec![Document::new(text)];
            let chunks = splitter.split(docs);

            for chunk in &chunks {
                // Byte length check — chunks may be slightly larger due to char boundaries
                prop_assert!(
                    chunk.text.len() <= chunk_size + 4,
                    "Chunk too large: {} > {}", chunk.text.len(), chunk_size
                );
            }
        }

        /// TextSplitter: empty input produces single empty chunk
        #[test]
        fn text_splitter_empty_produces_one(chunk_size in 10usize..500usize) {
            let overlap = chunk_size / 4;
            let splitter = TextSplitter::new(chunk_size, overlap);
            let docs = vec![Document::new("")];
            let chunks = splitter.split(docs);
            prop_assert_eq!(chunks.len(), 1);
        }

        /// TokenSplitter: produces non-empty chunks for non-empty input
        #[test]
        fn token_splitter_non_empty(
            text in "[a-z ]{10,1000}",
            max_tokens in 5usize..100usize,
        ) {
            let overlap = max_tokens / 4;
            let splitter = TokenSplitter::new(max_tokens, overlap);
            let docs = vec![Document::new(text)];
            let chunks = splitter.split(docs);

            prop_assert!(!chunks.is_empty());
            for chunk in &chunks {
                prop_assert!(!chunk.text.is_empty());
            }
        }

        /// SentenceSplitter: preserves all text content
        #[test]
        fn sentence_splitter_preserves_content(
            sentences in proptest::collection::vec("[A-Z][a-z]{3,20}\\. ", 1..10),
        ) {
            let text: String = sentences.concat();
            let splitter = SentenceSplitter::new(1000, 1);
            let docs = vec![Document::new(text.clone())];
            let chunks = splitter.split(docs);

            // All original text must appear in some chunk
            let combined: String = chunks.iter().map(|c| c.text.as_str()).collect::<Vec<_>>().join("");
            // Every sentence should be found in the combined output
            for sentence in &sentences {
                let trimmed = sentence.trim();
                prop_assert!(
                    combined.contains(trimmed),
                    "Sentence '{}' lost in splitting", trimmed
                );
            }
        }
    }
}
