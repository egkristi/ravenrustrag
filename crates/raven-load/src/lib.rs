use raven_core::{Document, Result};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use tracing::warn;
use walkdir::WalkDir;

// =============================================================================
// Plugin registry for custom loaders
// =============================================================================

type LoaderFn = Box<dyn Fn(&Path) -> Result<Document> + Send + Sync>;

static CUSTOM_LOADERS: Mutex<Option<HashMap<String, LoaderFn>>> = Mutex::new(None);

/// Register a custom loader for a file extension
pub fn register_loader(extension: &str, loader: impl Fn(&Path) -> Result<Document> + Send + Sync + 'static) {
    let ext = normalize_ext(extension);
    let mut guard = CUSTOM_LOADERS.lock().unwrap();
    let map = guard.get_or_insert_with(HashMap::new);
    map.insert(ext, Box::new(loader));
}

/// Get registered custom extensions
pub fn get_registered_extensions() -> Vec<String> {
    let guard = CUSTOM_LOADERS.lock().unwrap();
    guard.as_ref().map(|m| m.keys().cloned().collect()).unwrap_or_default()
}

fn normalize_ext(ext: &str) -> String {
    let e = ext.to_lowercase();
    if e.starts_with('.') { e } else { format!(".{}", e) }
}

// =============================================================================
// Main loader
// =============================================================================

pub struct Loader;

impl Loader {
    /// Load a single file, auto-detecting format
    pub fn from_file(path: impl AsRef<Path>) -> Result<Document> {
        let path = path.as_ref();
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| format!(".{}", e.to_lowercase()))
            .unwrap_or_default();

        // Check custom loaders first
        {
            let guard = CUSTOM_LOADERS.lock().unwrap();
            if let Some(map) = guard.as_ref() {
                if let Some(loader) = map.get(&ext) {
                    return loader(path);
                }
            }
        }

        // Built-in loaders by extension
        match ext.as_str() {
            ".md" | ".markdown" => Self::load_markdown(path),
            ".csv" => Self::load_csv(path),
            ".json" => Self::load_json(path),
            ".jsonl" | ".ndjson" => Self::load_jsonl(path),
            ".html" | ".htm" => Self::load_html(path),
            _ => Self::load_text(path), // Fallback: plain text
        }
    }

    /// Load all files from a directory
    pub fn from_directory(
        path: impl AsRef<Path>,
        extensions: Option<&[&str]>,
    ) -> Result<Vec<Document>> {
        let path = path.as_ref();
        let canonical_root = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

        let exts: Option<Vec<String>> = extensions.map(|e| {
            e.iter().map(|ext| normalize_ext(ext)).collect()
        });

        let mut documents = Vec::new();

        for entry in WalkDir::new(path).follow_links(false) {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    warn!("Walk error: {}", e);
                    continue;
                }
            };

            if !entry.file_type().is_file() {
                continue;
            }

            let file_path = entry.path();

            // Symlink safety: ensure resolved path is within root
            if let Ok(canonical) = file_path.canonicalize() {
                if !canonical.starts_with(&canonical_root) {
                    warn!("Skipping symlink outside root: {}", file_path.display());
                    continue;
                }
            }

            // Extension filter
            if let Some(ref extensions) = exts {
                let file_ext = file_path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| format!(".{}", e.to_lowercase()));

                if let Some(ext) = file_ext {
                    if !extensions.contains(&ext) {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            match Self::from_file(file_path) {
                Ok(doc) => documents.push(doc),
                Err(e) => {
                    warn!("Failed to load {}: {}", file_path.display(), e);
                }
            }
        }

        Ok(documents)
    }

    // --- Built-in loaders ---

    fn load_text(path: &Path) -> Result<Document> {
        let text = std::fs::read_to_string(path)?;
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(Document::new(text)
            .with_metadata("source", path.to_string_lossy())
            .with_metadata("filename", file_name)
            .with_metadata("format", "text"))
    }

    fn load_markdown(path: &Path) -> Result<Document> {
        let raw = std::fs::read_to_string(path)?;
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut doc = Document::new(&raw)
            .with_metadata("source", path.to_string_lossy())
            .with_metadata("filename", file_name)
            .with_metadata("format", "markdown");

        // Parse YAML frontmatter if present
        if raw.starts_with("---") {
            if let Some(end) = raw[3..].find("---") {
                let frontmatter = &raw[3..3 + end].trim();
                // Simple key: value parsing (no full YAML dependency)
                for line in frontmatter.lines() {
                    if let Some((key, value)) = line.split_once(':') {
                        let k = key.trim().to_string();
                        let v = value.trim().trim_matches('"').trim_matches('\'').to_string();
                        if !k.is_empty() && !v.is_empty() {
                            doc = doc.with_metadata(k, v);
                        }
                    }
                }
                // Strip frontmatter from text
                let body = &raw[3 + end + 3..];
                doc.text = body.trim_start().to_string();
            }
        }

        Ok(doc)
    }

    fn load_csv(path: &Path) -> Result<Document> {
        let raw = std::fs::read_to_string(path)?;
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Convert CSV rows to readable text
        let lines: Vec<&str> = raw.lines().collect();
        let text = if lines.len() > 1 {
            let headers: Vec<&str> = lines[0].split(',').map(|h| h.trim()).collect();
            let mut parts = Vec::new();
            for line in &lines[1..] {
                let values: Vec<&str> = line.split(',').map(|v| v.trim()).collect();
                let pairs: Vec<String> = headers
                    .iter()
                    .zip(values.iter())
                    .map(|(h, v)| format!("{}: {}", h, v))
                    .collect();
                parts.push(pairs.join(", "));
            }
            parts.join("\n")
        } else {
            raw.clone()
        };

        Ok(Document::new(text)
            .with_metadata("source", path.to_string_lossy())
            .with_metadata("filename", file_name)
            .with_metadata("format", "csv")
            .with_metadata("rows", (lines.len().saturating_sub(1)).to_string()))
    }

    fn load_json(path: &Path) -> Result<Document> {
        let raw = std::fs::read_to_string(path)?;
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Try to pretty-print for readability
        let text = if let Ok(value) = serde_json::from_str::<serde_json::Value>(&raw) {
            serde_json::to_string_pretty(&value).unwrap_or(raw)
        } else {
            raw
        };

        Ok(Document::new(text)
            .with_metadata("source", path.to_string_lossy())
            .with_metadata("filename", file_name)
            .with_metadata("format", "json"))
    }

    fn load_jsonl(path: &Path) -> Result<Document> {
        let raw = std::fs::read_to_string(path)?;
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut lines_out = Vec::new();
        for line in raw.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(line) {
                lines_out.push(serde_json::to_string_pretty(&value).unwrap_or_else(|_| line.to_string()));
            } else {
                lines_out.push(line.to_string());
            }
        }

        Ok(Document::new(lines_out.join("\n---\n"))
            .with_metadata("source", path.to_string_lossy())
            .with_metadata("filename", file_name)
            .with_metadata("format", "jsonl")
            .with_metadata("records", lines_out.len().to_string()))
    }

    fn load_html(path: &Path) -> Result<Document> {
        let raw = std::fs::read_to_string(path)?;
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Simple HTML tag stripping (no external dependency)
        let text = strip_html_tags(&raw);

        Ok(Document::new(text)
            .with_metadata("source", path.to_string_lossy())
            .with_metadata("filename", file_name)
            .with_metadata("format", "html"))
    }
}

/// Simple HTML tag stripper
fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;
    let mut in_script = false;
    let mut in_style = false;

    let lower = html.to_lowercase();
    let chars: Vec<char> = html.chars().collect();
    let lower_chars: Vec<char> = lower.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        if !in_tag && chars[i] == '<' {
            // Check for script/style
            let remaining: String = lower_chars[i..].iter().collect();
            if remaining.starts_with("<script") {
                in_script = true;
            } else if remaining.starts_with("<style") {
                in_style = true;
            } else if remaining.starts_with("</script") {
                in_script = false;
            } else if remaining.starts_with("</style") {
                in_style = false;
            }
            in_tag = true;
        } else if in_tag && chars[i] == '>' {
            in_tag = false;
            // Add space after closing tag to separate text content
            result.push(' ');
        } else if !in_tag && !in_script && !in_style {
            result.push(chars[i]);
        }
        i += 1;
    }

    // Collapse whitespace
    let collapsed: String = result
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    collapsed.trim().to_string()
}

// =============================================================================
// Export/import JSONL
// =============================================================================

pub fn export_jsonl(documents: &[Document], path: impl AsRef<Path>) -> Result<usize> {
    use std::io::Write;
    let file = std::fs::File::create(path)?;
    let mut writer = std::io::BufWriter::new(file);
    let mut count = 0;

    for doc in documents {
        let line = serde_json::to_string(doc).map_err(raven_core::RavenError::Serde)?;
        writeln!(writer, "{}", line)?;
        count += 1;
    }

    writer.flush()?;
    Ok(count)
}

pub fn import_jsonl(path: impl AsRef<Path>) -> Result<Vec<Document>> {
    use std::io::BufRead;
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut documents = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match serde_json::from_str::<Document>(line) {
            Ok(doc) => {
                if !doc.text.is_empty() {
                    documents.push(doc);
                }
            }
            Err(e) => {
                warn!("Skipping invalid JSONL row: {}", e);
            }
        }
    }

    Ok(documents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_load_file() {
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        write!(temp_file, "Hello world").unwrap();

        let doc = Loader::from_file(temp_file.path()).unwrap();
        assert_eq!(doc.text, "Hello world");
        assert!(doc.metadata.contains_key("source"));
    }

    #[test]
    fn test_load_directory() {
        let temp_dir = tempfile::tempdir().unwrap();

        let file1 = temp_dir.path().join("a.txt");
        std::fs::write(&file1, "Doc 1").unwrap();
        let file2 = temp_dir.path().join("b.txt");
        std::fs::write(&file2, "Doc 2").unwrap();

        let docs = Loader::from_directory(temp_dir.path(), None).unwrap();
        assert_eq!(docs.len(), 2);
    }

    #[test]
    fn test_load_markdown_frontmatter() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file = temp_dir.path().join("test.md");
        std::fs::write(&file, "---\ntitle: My Doc\nauthor: Test\n---\n# Hello\n\nContent here.").unwrap();

        let doc = Loader::from_file(&file).unwrap();
        assert_eq!(doc.metadata.get("title"), Some(&"My Doc".to_string()));
        assert_eq!(doc.metadata.get("author"), Some(&"Test".to_string()));
        assert!(doc.text.starts_with("# Hello"));
    }

    #[test]
    fn test_load_csv() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file = temp_dir.path().join("test.csv");
        std::fs::write(&file, "name,age\nAlice,30\nBob,25").unwrap();

        let doc = Loader::from_file(&file).unwrap();
        assert!(doc.text.contains("name: Alice"));
        assert!(doc.text.contains("name: Bob"));
        assert_eq!(doc.metadata.get("format"), Some(&"csv".to_string()));
    }

    #[test]
    fn test_strip_html() {
        let html = "<html><body><h1>Title</h1><p>Paragraph</p><script>var x=1;</script></body></html>";
        let text = strip_html_tags(html);
        assert_eq!(text, "Title Paragraph");
    }

    #[test]
    fn test_export_import_jsonl() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file = temp_dir.path().join("export.jsonl");

        let docs = vec![
            Document::new("First doc").with_metadata("source", "a.txt"),
            Document::new("Second doc").with_metadata("source", "b.txt"),
        ];

        let count = export_jsonl(&docs, &file).unwrap();
        assert_eq!(count, 2);

        let imported = import_jsonl(&file).unwrap();
        assert_eq!(imported.len(), 2);
        assert_eq!(imported[0].text, "First doc");
        assert_eq!(imported[1].text, "Second doc");
    }
}