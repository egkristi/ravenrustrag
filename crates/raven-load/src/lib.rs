use raven_core::{Document, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Loader;

impl Loader {
    /// Load a single text file
    pub fn from_file(path: impl AsRef<Path>) -> Result<Document> {
        let path = path.as_ref();
        let text = std::fs::read_to_string(path)?;
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(Document::new(text)
            .with_metadata("source", path.to_string_lossy())
            .with_metadata("filename", file_name))
    }

    /// Load all text files from a directory
    pub fn from_directory(
        path: impl AsRef<Path>,
        extensions: Option<&[&str]>,
    ) -> Result<Vec<Document>> {
        let path = path.as_ref();
        let exts: Option<Vec<String>> = extensions.map(|e| {
            e.iter().map(|ext| {
                if ext.starts_with('.') {
                    ext.to_string()
                } else {
                    format!(".{}", ext)
                }
            }).collect()
        });

        let mut documents = Vec::new();

        for entry in WalkDir::new(path).follow_links(true) {
            let entry = entry.map_err(|e| raven_core::RavenError::Load(format!("Walk error: {}", e)))?;
            
            if !entry.file_type().is_file() {
                continue;
            }

            let file_path = entry.path();
            
            // Check extension filter
            if let Some(ref extensions) = exts {
                let file_ext = file_path.extension()
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
                    eprintln!("Warning: failed to load {}: {}", file_path.display(), e);
                }
            }
        }

        Ok(documents)
    }
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
        assert_eq!(doc.metadata.get("source"), Some(&temp_file.path().to_string_lossy().to_string()));
    }

    #[test]
    fn test_load_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        let mut file1 = tempfile::NamedTempFile::new_in(temp_dir.path()).unwrap();
        write!(file1, "Doc 1").unwrap();
        
        let mut file2 = tempfile::NamedTempFile::new_in(temp_dir.path()).unwrap();
        write!(file2, "Doc 2").unwrap();

        let docs = Loader::from_directory(temp_dir.path(), None).unwrap();
        assert_eq!(docs.len(), 2);
    }
}