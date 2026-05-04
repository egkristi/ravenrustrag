#![no_main]
use libfuzzer_sys::fuzz_target;
use raven_load::Loader;
use std::io::Write;
use tempfile::NamedTempFile;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        if let Ok(mut tmp) = NamedTempFile::with_suffix(".csv") {
            if tmp.write_all(text.as_bytes()).is_ok() {
                let _ = Loader::from_file(tmp.path());
            }
        }
    }
});
