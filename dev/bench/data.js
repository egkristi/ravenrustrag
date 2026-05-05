window.BENCHMARK_DATA = {
  "lastUpdate": 1777969579181,
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
          "id": "895721c163877c4719990131d86794fad3be9a28",
          "message": "docs: finalize PLAN.md — mark completed issues, update status\n\n- Phase 3 complete, Phase 4 in progress\n- Mark #45, #46, #47, #48, #49, #50, #51, #54 as resolved\n- Update open issues table (4 remaining: #43, #44, #52, #53)\n- ONNX (#43, #44) marked as deferred due to MSRV conflict\n- GitHub Releases already working via release.yml\n- 148 checked items, 11 remaining across 4 issues",
          "timestamp": "2026-05-04T23:42:27+02:00",
          "tree_id": "96c0d97746347c02beddd65f2049a53cdddd20c9",
          "url": "https://github.com/egkristi/ravenrustrag/commit/895721c163877c4719990131d86794fad3be9a28"
        },
        "date": 1777931143019,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 100,
            "range": "± 3",
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
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 54162,
            "range": "± 1257",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 574675,
            "range": "± 2777",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 95619,
            "range": "± 392",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39083,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 113343,
            "range": "± 437",
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
          "id": "1a861b76ab96d50b7087b6a44869bd1c2f44bd48",
          "message": "fix: docs workflow — deploy to gh-pages branch instead of Pages API\n\nThe repo uses gh-pages branch for GitHub Pages (shared with benchmarks).\nSwitch from deploy-pages action to git worktree push, preserving the\ndev/bench/ directory used by benchmark-action.",
          "timestamp": "2026-05-04T23:48:55+02:00",
          "tree_id": "c7c0e2d9d21376416eb7816b3fe6f7a967fe8d06",
          "url": "https://github.com/egkristi/ravenrustrag/commit/1a861b76ab96d50b7087b6a44869bd1c2f44bd48"
        },
        "date": 1777931565039,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 99,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 547,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1085,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 54250,
            "range": "± 206",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 575831,
            "range": "± 1764",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 94417,
            "range": "± 657",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39465,
            "range": "± 207",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 115799,
            "range": "± 821",
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
          "id": "9889ba073d9598c28dab00d54d166e9c70817f29",
          "message": "docs: comprehensive documentation update\n\n- README: mark implemented features (HNSW, graph, semantic split, streaming,\n  lock-free cache, mmap), fix command count (13), add /ready endpoint,\n  fix --extensions format (no dots), add Documentation section with link\n  to GitHub Pages, update roadmap to phase-based, fix config example\n- docs/index.md: expand feature list, update crate descriptions\n- docs/quickstart.md: fix default port (8484 not 3000)\n- docs/configuration.md: use correct TOML section names matching Config\n  struct (embedder/splitter/pipeline/server), add all server options\n- docs/api.md: fix auth documentation (stats/metrics require auth by\n  default unless public_stats=true), fix rate limit (per second not minute)",
          "timestamp": "2026-05-04T23:59:54+02:00",
          "tree_id": "e6f8b9a801ce534194344f0a9bc4064d542fa6ba",
          "url": "https://github.com/egkristi/ravenrustrag/commit/9889ba073d9598c28dab00d54d166e9c70817f29"
        },
        "date": 1777932201356,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 99,
            "range": "± 5",
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
            "value": 1085,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 54655,
            "range": "± 826",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 578814,
            "range": "± 2301",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 96166,
            "range": "± 640",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39129,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 114105,
            "range": "± 703",
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
          "id": "53c83293d417dd62fc537a16f8c93ea18deae96b",
          "message": "feat: SQLite schema versioning and automatic migrations closes #60\n\n- Added schema_version table to track database version\n- Migration system: checks current version on open, applies pending migrations\n- migrate_to_v1() creates all tables in a transaction\n- SqliteStore::schema_version() method for inspection\n- Doctor command reports schema version\n- Existing databases (pre-migration) auto-migrate on next open\n- Future migrations: add migrate_to_v2(), etc. in sequence",
          "timestamp": "2026-05-05T00:11:46+02:00",
          "tree_id": "dfd8af3bb2a40bd5b382ff49e1406bd62ceeb24d",
          "url": "https://github.com/egkristi/ravenrustrag/commit/53c83293d417dd62fc537a16f8c93ea18deae96b"
        },
        "date": 1777932946235,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 99,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 547,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1086,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 54789,
            "range": "± 440",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 573523,
            "range": "± 13072",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 96145,
            "range": "± 465",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39143,
            "range": "± 151",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 112240,
            "range": "± 459",
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
          "id": "88bc1c38a3656e4856f0959c11000b6a1c2a3db9",
          "message": "test: add fuzz testing infrastructure (cargo-fuzz) closes #57\n\nAdded 6 fuzz targets in fuzz/ directory:\n- fuzz_text_splitter: various chunk sizes on arbitrary UTF-8\n- fuzz_markdown_loader: malformed markdown/frontmatter\n- fuzz_csv_loader: malformed CSV\n- fuzz_html_loader: malformed HTML with tags/scripts\n- fuzz_json_loader: malformed JSON and JSONL\n- fuzz_cosine_similarity: arbitrary f32 vectors\n\nUsage (requires nightly):\n  cargo +nightly fuzz run fuzz_text_splitter -- -max_total_time=60\n  cargo +nightly fuzz run fuzz_cosine_similarity -- -max_total_time=60\n\nWorkspace excludes fuzz/ to avoid MSRV conflicts with libfuzzer-sys.",
          "timestamp": "2026-05-05T00:19:28+02:00",
          "tree_id": "c892c4a5078b2fbdcdb848ca519851281084c0ff",
          "url": "https://github.com/egkristi/ravenrustrag/commit/88bc1c38a3656e4856f0959c11000b6a1c2a3db9"
        },
        "date": 1777933417716,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 99,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 547,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1086,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 54921,
            "range": "± 599",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 583120,
            "range": "± 9776",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 95352,
            "range": "± 2836",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39338,
            "range": "± 166",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 111572,
            "range": "± 692",
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
          "id": "69d0ed9add4ebbdbb77e4c8a223d442208a0ee36",
          "message": "test: concurrent stress tests and 10k+ scaling closes #59\n\nAdded stress_tests.rs integration test suite (7 tests):\n- test_10k_document_indexing: 10k docs indexed at 20k+ docs/sec\n- test_10k_query_latency: sub-500ms avg in debug, sub-10ms in release\n- test_concurrent_indexing: 8 parallel threads indexing simultaneously\n- test_concurrent_query_while_indexing: mixed read/write workload\n- test_bm25_scaling: BM25 search at 10k chunks\n- test_many_small_documents: 50k tiny docs at 190k+ docs/sec (release)\n- test_large_document: single 500KB+ doc splits correctly\n\nAll pass in both debug and release mode.",
          "timestamp": "2026-05-05T00:24:13+02:00",
          "tree_id": "608602ad44d3c705dd8d4b535177c21bbe1efc81",
          "url": "https://github.com/egkristi/ravenrustrag/commit/69d0ed9add4ebbdbb77e4c8a223d442208a0ee36"
        },
        "date": 1777933717239,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 99,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 547,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1086,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 53836,
            "range": "± 592",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 583853,
            "range": "± 3899",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 95046,
            "range": "± 323",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39355,
            "range": "± 187",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 114326,
            "range": "± 915",
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
          "id": "cfbe76c812fba8806cdf34b77c007752523f45a4",
          "message": "feat: LLM generation integration (raven ask) closes #63",
          "timestamp": "2026-05-05T00:31:07+02:00",
          "tree_id": "b7782cda3dba034e6f3b71f1683cfbe1445c5de1",
          "url": "https://github.com/egkristi/ravenrustrag/commit/cfbe76c812fba8806cdf34b77c007752523f45a4"
        },
        "date": 1777934133253,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 99,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 547,
            "range": "± 8",
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
            "value": 54906,
            "range": "± 203",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 582084,
            "range": "± 2952",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 95458,
            "range": "± 417",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39179,
            "range": "± 487",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 115197,
            "range": "± 1293",
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
          "id": "117afb809f0d0874b2ac02d7b83012cd3270f9c5",
          "message": "feat: POST /ask SSE endpoint + crates.io metadata prep closes #52",
          "timestamp": "2026-05-05T00:42:31+02:00",
          "tree_id": "2dcc15ba1fb33e9a2828449e0d57b3452aebd37a",
          "url": "https://github.com/egkristi/ravenrustrag/commit/117afb809f0d0874b2ac02d7b83012cd3270f9c5"
        },
        "date": 1777934841509,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 99,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 547,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1085,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 53941,
            "range": "± 594",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 574949,
            "range": "± 3401",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 94315,
            "range": "± 2553",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39052,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 112768,
            "range": "± 663",
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
          "id": "5688c7af6ca8e62beec7e6b10a85996acbb5cb3c",
          "message": "feat: add tests, Homebrew formula, AUR PKGBUILD, CI instructions closes #55 closes #56",
          "timestamp": "2026-05-05T01:28:47+02:00",
          "tree_id": "679a608bec826eb9cb43838d2e0060fb27df6fe5",
          "url": "https://github.com/egkristi/ravenrustrag/commit/5688c7af6ca8e62beec7e6b10a85996acbb5cb3c"
        },
        "date": 1777937589288,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 99,
            "range": "± 3",
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
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 54202,
            "range": "± 931",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 578565,
            "range": "± 3022",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 95780,
            "range": "± 756",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 40000,
            "range": "± 104",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 116774,
            "range": "± 552",
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
          "id": "cd0f8402a739dc790e06deabeab8c618d362b7a1",
          "message": "fix: upgrade reqwest 0.11→0.12, pdf-extract 0.7→0.10 (resolve all security advisories)",
          "timestamp": "2026-05-05T02:00:23+02:00",
          "tree_id": "bfbf7c90f8d5dd5f2fb6dcd48c57398f299aa44b",
          "url": "https://github.com/egkristi/ravenrustrag/commit/cd0f8402a739dc790e06deabeab8c618d362b7a1"
        },
        "date": 1777939550302,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 91,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 508,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1010,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 58704,
            "range": "± 860",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 647029,
            "range": "± 3387",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 98638,
            "range": "± 884",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 38484,
            "range": "± 224",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 81678,
            "range": "± 1218",
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
          "id": "2932ccca9ba40f854a9c088d2a1ff5933160c54a",
          "message": "docs: deep analysis — create issues #64-#78 for production quality improvements",
          "timestamp": "2026-05-05T02:09:04+02:00",
          "tree_id": "9b21845257a91a5169921c74360f8719a22f163a",
          "url": "https://github.com/egkristi/ravenrustrag/commit/2932ccca9ba40f854a9c088d2a1ff5933160c54a"
        },
        "date": 1777940001990,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 105,
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
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 56040,
            "range": "± 379",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 580188,
            "range": "± 1773",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 97998,
            "range": "± 2266",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 43232,
            "range": "± 213",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 101580,
            "range": "± 1157",
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
          "id": "5bcfaf247b694c6cfecf033257e34f44456fa41a",
          "message": "fix: remove hardcoded 768 dimension — use embedder.dimension() closes #67",
          "timestamp": "2026-05-05T02:13:37+02:00",
          "tree_id": "0275f1907f8753c33160b7b80093c7863441e141",
          "url": "https://github.com/egkristi/ravenrustrag/commit/5bcfaf247b694c6cfecf033257e34f44456fa41a"
        },
        "date": 1777940272305,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 99,
            "range": "± 0",
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
            "value": 1085,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 53803,
            "range": "± 479",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 581390,
            "range": "± 4303",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 93217,
            "range": "± 1169",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39110,
            "range": "± 138",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 112333,
            "range": "± 2416",
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
          "id": "8c6ef1e69bfd21392ae499b0f1d0a924de872040",
          "message": "perf: replace random cache eviction with moka LRU closes #68",
          "timestamp": "2026-05-05T02:27:25+02:00",
          "tree_id": "73418f72172c2ac2e42496b00334d9e80b789d4c",
          "url": "https://github.com/egkristi/ravenrustrag/commit/8c6ef1e69bfd21392ae499b0f1d0a924de872040"
        },
        "date": 1777941145785,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 84,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 470,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 930,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 43671,
            "range": "± 289",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 458387,
            "range": "± 2114",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 76334,
            "range": "± 322",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 34019,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 76922,
            "range": "± 1093",
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
          "id": "a291fded35d7392061437d0abf0eb3787a9fee8a",
          "message": "feat: add top-level ravenrustrag library crate with unified API closes #75",
          "timestamp": "2026-05-05T07:25:29+02:00",
          "tree_id": "0b9d503032447536c98911c783b7fc8f96168cbb",
          "url": "https://github.com/egkristi/ravenrustrag/commit/a291fded35d7392061437d0abf0eb3787a9fee8a"
        },
        "date": 1777959014865,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 99,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 547,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1085,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 53841,
            "range": "± 330",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 570463,
            "range": "± 3277",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 95162,
            "range": "± 991",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39279,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 114461,
            "range": "± 1288",
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
          "id": "3499866b35311a5bde576dfbb92344def6301fbf",
          "message": "fix: add dummy backend to create_cached_embedder factory",
          "timestamp": "2026-05-05T09:04:11+02:00",
          "tree_id": "fed7054aa16957c19454cf27f8311dac21400fd6",
          "url": "https://github.com/egkristi/ravenrustrag/commit/3499866b35311a5bde576dfbb92344def6301fbf"
        },
        "date": 1777964987364,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 102,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 547,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1089,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 53854,
            "range": "± 124",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 571293,
            "range": "± 2571",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 93841,
            "range": "± 255",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39313,
            "range": "± 615",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 116483,
            "range": "± 1693",
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
          "id": "8a0442dd6978c6906c0ff16f5223ce655dd31653",
          "message": "docs: add 9 new features to PLAN.md (streaming citations, MCP resources/prompts, embeddings versioning, read-only mode, query explain, backup, MCP filter, JSON Schema, marketplace)",
          "timestamp": "2026-05-05T10:02:07+02:00",
          "tree_id": "6d7d0885c10eb0b154c4265f1ac5bad98e2b2541",
          "url": "https://github.com/egkristi/ravenrustrag/commit/8a0442dd6978c6906c0ff16f5223ce655dd31653"
        },
        "date": 1777968420088,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 102,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 547,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1089,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 53386,
            "range": "± 270",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 568441,
            "range": "± 1399",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 94728,
            "range": "± 311",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39477,
            "range": "± 1057",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 112364,
            "range": "± 544",
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
          "id": "06e05d68970cbb986702f99a0aed574eb117c77f",
          "message": "feat: embeddings versioning — store model+dimensions, reject dimension mismatch on index",
          "timestamp": "2026-05-05T10:09:31+02:00",
          "tree_id": "bc5d6ffc92c4c89db0480a198a1f417f050670e7",
          "url": "https://github.com/egkristi/ravenrustrag/commit/06e05d68970cbb986702f99a0aed574eb117c77f"
        },
        "date": 1777968868391,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 102,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 547,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1089,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 53881,
            "range": "± 167",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 572507,
            "range": "± 2684",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 95229,
            "range": "± 383",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39618,
            "range": "± 885",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 114282,
            "range": "± 1968",
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
          "id": "39c9638aff23c01bd7622ac2aba774890b8580c1",
          "message": "docs: document ONNX MSRV split (1.88+) and add CI gate for onnx feature closes #80",
          "timestamp": "2026-05-05T10:21:30+02:00",
          "tree_id": "4ac619de0f8a3a63d50ed4c7eacc0eb97da07b66",
          "url": "https://github.com/egkristi/ravenrustrag/commit/39c9638aff23c01bd7622ac2aba774890b8580c1"
        },
        "date": 1777969578144,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 84,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 470,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 929,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 43805,
            "range": "± 176",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 457930,
            "range": "± 1597",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 75239,
            "range": "± 3057",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 33484,
            "range": "± 141",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 77071,
            "range": "± 216",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}