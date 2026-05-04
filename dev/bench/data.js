window.BENCHMARK_DATA = {
  "lastUpdate": 1777930943953,
  "repoUrl": "https://github.com/egkristi/ravenrustrag",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "Erling.Kristiansen@skatteetaten.no",
            "name": "Kristiansen, Erling Gustav Moland",
            "username": "egkristi"
          },
          "committer": {
            "email": "Erling.Kristiansen@skatteetaten.no",
            "name": "Kristiansen, Erling Gustav Moland",
            "username": "egkristi"
          },
          "distinct": true,
          "id": "3778c400042316b9364e186a1bb5e46cc7e5f727",
          "message": "perf: DashMap lock-free cache, mmap SQLite, CI concurrency groups\n\n- Replace Mutex<HashMap> with DashMap + AtomicU64 in EmbeddingCache (#47)\n- Enable PRAGMA mmap_size=256MB for zero-copy reads in SqliteStore (#48)\n- Add concurrency groups to CI/Docker workflows (#46)\n- Fix unwrap in semaphore acquire (clippy compliance)\n\nCloses #46, closes #47, closes #48",
          "timestamp": "2026-05-04T23:28:49+02:00",
          "tree_id": "c162c47c5f6cff6bf73279eb5bf41621a9af009e",
          "url": "https://github.com/egkristi/ravenrustrag/commit/3778c400042316b9364e186a1bb5e46cc7e5f727"
        },
        "date": 1777930348247,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 103,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 587,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1162,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 55551,
            "range": "± 616",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 586909,
            "range": "± 2774",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 96968,
            "range": "± 2738",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 43682,
            "range": "± 124",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 101804,
            "range": "± 370",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "Erling.Kristiansen@skatteetaten.no",
            "name": "Kristiansen, Erling Gustav Moland",
            "username": "egkristi"
          },
          "committer": {
            "email": "Erling.Kristiansen@skatteetaten.no",
            "name": "Kristiansen, Erling Gustav Moland",
            "username": "egkristi"
          },
          "distinct": true,
          "id": "6dbab25d26b4d88b98f0ee7984ea71b0a0e79b7c",
          "message": "docs: add mkdocs + mdbook documentation site with GitHub Pages deployment\n\n- Add docs/ with 17 pages: architecture, CLI reference, API, MCP,\n  configuration, troubleshooting, migration guide, performance tuning,\n  hybrid search, knowledge graph, Docker, testing, contributing, etc.\n- Add mkdocs.yml (Material theme) for primary docs site\n- Add book.toml at root for mdbook compatibility (same source)\n- Add .github/workflows/docs.yml to publish to GitHub Pages via mkdocs\n- Add site/ and book/ to .gitignore\n\nCloses #49, closes #50",
          "timestamp": "2026-05-04T23:38:46+02:00",
          "tree_id": "bb0a8b921b56179f9c1771baec51a17fcb6f96d4",
          "url": "https://github.com/egkristi/ravenrustrag/commit/6dbab25d26b4d88b98f0ee7984ea71b0a0e79b7c"
        },
        "date": 1777930943088,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 100,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 547,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1086,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 55423,
            "range": "± 490",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 572445,
            "range": "± 2149",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 95640,
            "range": "± 450",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39395,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 111400,
            "range": "± 653",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}