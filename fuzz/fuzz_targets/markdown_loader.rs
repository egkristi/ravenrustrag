#![no_main]
use libfuzzer_sys::fuzz_target;
use raven_load::Loader;
use std::io::Write;
use tempfile::NamedTempFile;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        // Write to a temp .md file and try to load it
        if let Ok(mut tmp) = NamedTempFile::with_suffix(".md") {
            if tmp.write_all(text.as_bytes()).is_ok() {
                // Should not panic regardless of input
                let _ = Loader::from_file(tmp.path());
            }
        }
    }
});
