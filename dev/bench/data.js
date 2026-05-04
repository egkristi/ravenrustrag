window.BENCHMARK_DATA = {
  "lastUpdate": 1777937590058,
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
      }
    ]
  }
}