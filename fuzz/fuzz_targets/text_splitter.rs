#![no_main]
use libfuzzer_sys::fuzz_target;
use raven_core::Document;
use raven_split::{Splitter, TextSplitter};

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        if text.is_empty() {
            return;
        }
        // Fuzz with various chunk sizes
        for chunk_size in [32, 128, 512] {
            let splitter = TextSplitter::new(chunk_size, chunk_size / 4);
            let docs = vec![Document::new(text)];
            let chunks = splitter.split(docs);
            // Must not panic and must produce at least one chunk
            assert!(!chunks.is_empty());
            for chunk in &chunks {
                assert!(!chunk.text.is_empty());
            }
        }
    }
});
