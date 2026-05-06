# File Formats

RavenRustRAG supports loading documents in several formats. The loader is selected automatically based on file extension.

## Supported Formats

| Extension | Loader | Notes |
|-----------|--------|-------|
| `.txt` | Text | Plain text, also used as fallback for unknown extensions |
| `.md` | Markdown | YAML frontmatter parsed into metadata |
| `.csv` | CSV | Each row becomes a document; headers become metadata keys |
| `.json` | JSON | Pretty-printed content for chunking |
| `.jsonl` | JSONL | Each line treated as a separate record |
| `.html` | HTML | Tags stripped, scripts/styles removed |

## Markdown

Markdown files are loaded with full content preserved. If the file contains YAML frontmatter, it is parsed into document metadata:

```markdown
---
title: My Document
author: John Doe
tags: [rust, rag]
---

# Content starts here
```

The frontmatter fields (`title`, `author`, `tags`) become searchable metadata.

## CSV

Each row in a CSV file becomes a separate document. Column headers are used as metadata keys:

```csv
title,content,category
Introduction,Welcome to the guide,basics
Setup,Install with cargo,getting-started
```

## JSON

JSON files are pretty-printed and treated as a single document for chunking. Nested structures are preserved in the text representation.

## HTML

HTML files have all tags stripped. `<script>` and `<style>` blocks are completely removed. The resulting plain text is then chunked normally.

## Filtering by Extension

When indexing a directory, use `--extensions` to limit which files are processed:

```bash
# Only index markdown and text files
ravenrag index ./docs --extensions md,txt

# Include HTML files
ravenrag index ./web --extensions html,md,txt
```

The default extensions are `txt,md`.
