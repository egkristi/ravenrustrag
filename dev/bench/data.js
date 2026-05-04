window.BENCHMARK_DATA = {
  "lastUpdate": 1777930348718,
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
      }
    ]
  }
}