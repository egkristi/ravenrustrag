#![no_main]
use libfuzzer_sys::fuzz_target;
use raven_load::Loader;
use std::io::Write;
use tempfile::NamedTempFile;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        // Test .json loading
        if let Ok(mut tmp) = NamedTempFile::with_suffix(".json") {
            if tmp.write_all(text.as_bytes()).is_ok() {
                let _ = Loader::from_file(tmp.path());
            }
        }
        // Test .jsonl loading
        if let Ok(mut tmp) = NamedTempFile::with_suffix(".jsonl") {
            if tmp.write_all(text.as_bytes()).is_ok() {
                let _ = Loader::from_file(tmp.path());
            }
        }
    }
});
