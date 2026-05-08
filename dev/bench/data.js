window.BENCHMARK_DATA = {
  "lastUpdate": 1778233714224,
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
          "id": "a441ea4530bf25b73550b0b529fd5ec5ddec42a8",
          "message": "api: define stable public API surface through ravenrustrag crate re-exports closes #83",
          "timestamp": "2026-05-05T10:32:14+02:00",
          "tree_id": "14d5b34e2167d1af47abd75be8c9a5a09eda9b78",
          "url": "https://github.com/egkristi/ravenrustrag/commit/a441ea4530bf25b73550b0b529fd5ec5ddec42a8"
        },
        "date": 1777970237409,
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
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 54964,
            "range": "± 1104",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 580579,
            "range": "± 1809",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 95028,
            "range": "± 743",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39605,
            "range": "± 156",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 115487,
            "range": "± 341",
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
          "id": "a98e76ca56885cd0abaf1fd6468add04436ff5e7",
          "message": "feat: POST /ask streaming citations — event: source + event: token + event: done SSE pattern",
          "timestamp": "2026-05-05T10:40:03+02:00",
          "tree_id": "3eabc425b35eaf3302b1ff947b3080796284f631",
          "url": "https://github.com/egkristi/ravenrustrag/commit/a98e76ca56885cd0abaf1fd6468add04436ff5e7"
        },
        "date": 1777970695001,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 105,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 587,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1162,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 55460,
            "range": "± 243",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 580725,
            "range": "± 1685",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 96470,
            "range": "± 733",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 43268,
            "range": "± 317",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 101128,
            "range": "± 218",
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
          "id": "08896ce8f4dd53907160d6b0b1ec108c19ea2f61",
          "message": "docs: update changelog and PLAN.md with all Phase 4+5 completions",
          "timestamp": "2026-05-05T10:57:42+02:00",
          "tree_id": "db53194652690d63e51b83d5accf45da0a922e8a",
          "url": "https://github.com/egkristi/ravenrustrag/commit/08896ce8f4dd53907160d6b0b1ec108c19ea2f61"
        },
        "date": 1777971760632,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 103,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 587,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1162,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 56297,
            "range": "± 409",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 589019,
            "range": "± 1820",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 95964,
            "range": "± 264",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 43037,
            "range": "± 158",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 101013,
            "range": "± 1903",
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
          "id": "00b2ce95042e1a8991d739699595f6dbecbf8e95",
          "message": "docs: comprehensive documentation update reflecting current project state",
          "timestamp": "2026-05-05T11:12:20+02:00",
          "tree_id": "e35b384873732e3621468b3e339a7a35d0292caa",
          "url": "https://github.com/egkristi/ravenrustrag/commit/00b2ce95042e1a8991d739699595f6dbecbf8e95"
        },
        "date": 1777972627948,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 90,
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
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 58585,
            "range": "± 767",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 644639,
            "range": "± 78936",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 95837,
            "range": "± 1104",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 38786,
            "range": "± 408",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 86280,
            "range": "± 299",
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
          "id": "28624e78ee60801451a48a8e43eeab5a03354b6d",
          "message": "release: v1.0.0 — bump MSRV to 1.88, fix CI (tarpaulin --locked), fix clippy lint",
          "timestamp": "2026-05-05T12:46:45+02:00",
          "tree_id": "a2db54fe50adf02f36873df44aa79375c271dda4",
          "url": "https://github.com/egkristi/ravenrustrag/commit/28624e78ee60801451a48a8e43eeab5a03354b6d"
        },
        "date": 1777978450302,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 558,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1105,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 54933,
            "range": "± 462",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 585247,
            "range": "± 5636",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 96300,
            "range": "± 405",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39343,
            "range": "± 289",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 114744,
            "range": "± 1022",
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
          "id": "12c63f134189e0efe197d212ed583c93a00a777d",
          "message": "ci: fix release workflow — use macos-13 for x86_64, disable fail-fast",
          "timestamp": "2026-05-05T17:01:57+02:00",
          "tree_id": "6e6ae25e010a798743b230cf17ae83986ac7c201",
          "url": "https://github.com/egkristi/ravenrustrag/commit/12c63f134189e0efe197d212ed583c93a00a777d"
        },
        "date": 1777993611219,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 103,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 552,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1091,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 56819,
            "range": "± 211",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 591049,
            "range": "± 4789",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 98390,
            "range": "± 418",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 43485,
            "range": "± 607",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 100952,
            "range": "± 2770",
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
          "id": "e56430adbe97268586663a333bafbea8ac559000",
          "message": "feat: implement Phase 5 remaining features\n\n- Incremental BM25: add remove_by_doc_id() for efficient document deletion\n- Async SQLite: wrap heavy operations (add, search, all) in spawn_blocking\n- Binary embedding storage: F32/F16/Uint8 quantized storage (75% size reduction)\n- Config hot-reload: watch raven.toml and update rate limiter at runtime\n- Quantized ONNX: support int8/fp16 models in OnnxEmbedder/OnnxReranker\n- Update PLAN.md status to v1.0.0 released, mark all features done\n- Update README roadmap to reflect v1.0.0 release\n- Create GitHub issues #84-#87 for remaining packaging tasks",
          "timestamp": "2026-05-05T17:25:09+02:00",
          "tree_id": "2cca360854b5785a568d25956984dff2edb43ae7",
          "url": "https://github.com/egkristi/ravenrustrag/commit/e56430adbe97268586663a333bafbea8ac559000"
        },
        "date": 1777995060900,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 557,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1107,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 54582,
            "range": "± 255",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 589048,
            "range": "± 5072",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 99168,
            "range": "± 422",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39548,
            "range": "± 444",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 117043,
            "range": "± 733",
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
          "id": "27523be67a0b4b09d2d90de1bee6eaa58b983fe6",
          "message": "docs: add Phase 6 distribution plan — all platform package managers\n\nWindows: winget, choco, scoop, exe, msi\nmacOS: brew, dmg, pkg\nLinux: apt, dnf, pacman, zypper, apk, flatpak, snap\nCross-platform: cargo install, static binaries, Docker",
          "timestamp": "2026-05-05T17:37:45+02:00",
          "tree_id": "0d8afbb4dc6ed89d92e78c946fc2992cad70dd3b",
          "url": "https://github.com/egkristi/ravenrustrag/commit/27523be67a0b4b09d2d90de1bee6eaa58b983fe6"
        },
        "date": 1777995765247,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 100,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 557,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1105,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 54689,
            "range": "± 2163",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 583735,
            "range": "± 3995",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 95855,
            "range": "± 951",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39594,
            "range": "± 131",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 117198,
            "range": "± 907",
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
          "id": "56d72d5ed02fb0784d53e14e79824964ad69f0b4",
          "message": "fix: add missing mut on ONNX session builder variable",
          "timestamp": "2026-05-05T20:17:45+02:00",
          "tree_id": "773ffc92b0e6101215b1239e90cb9444e16aa2d4",
          "url": "https://github.com/egkristi/ravenrustrag/commit/56d72d5ed02fb0784d53e14e79824964ad69f0b4"
        },
        "date": 1778005403702,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 557,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1105,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 54435,
            "range": "± 281",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 576269,
            "range": "± 2758",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 95995,
            "range": "± 400",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39575,
            "range": "± 159",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 114840,
            "range": "± 1655",
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
          "id": "6280f23be0f0a8f7a71096ca69503bc2f3a4756a",
          "message": "docs: expand Phase 6 distribution targets\n\nAdded: portable ZIP, Nix/nixpkgs, AppImage, F-Droid, TestFlight,\nHelm chart, curl install script. Reorganized sections.",
          "timestamp": "2026-05-05T20:39:23+02:00",
          "tree_id": "62496eafc37a4bbbbc6683d2e38a4ba56ff52745",
          "url": "https://github.com/egkristi/ravenrustrag/commit/6280f23be0f0a8f7a71096ca69503bc2f3a4756a"
        },
        "date": 1778006653511,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 92,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 517,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1028,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 58351,
            "range": "± 689",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 640361,
            "range": "± 5423",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 97434,
            "range": "± 598",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 38695,
            "range": "± 195",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 88109,
            "range": "± 461",
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
          "id": "c3c9badbe933187b998e11d6c6ffbef56e345d0a",
          "message": "fix: portable ZIP creation for Windows runners",
          "timestamp": "2026-05-05T20:48:50+02:00",
          "tree_id": "9ee9b727f4ffebeae0b103d6102bbde924d8e543",
          "url": "https://github.com/egkristi/ravenrustrag/commit/c3c9badbe933187b998e11d6c6ffbef56e345d0a"
        },
        "date": 1778007215830,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 82,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 438,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 864,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 46515,
            "range": "± 556",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 489566,
            "range": "± 1737",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 76465,
            "range": "± 206",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 33961,
            "range": "± 164",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 76387,
            "range": "± 1766",
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
          "id": "8b592e68bd08858f2ea6a8bb53fa1e88ce17c3ce",
          "message": "fix: tolerate tarpaulin spurious exit codes in coverage job",
          "timestamp": "2026-05-05T20:59:12+02:00",
          "tree_id": "db47354faabd3563075cb8ccb22fe9955bb22090",
          "url": "https://github.com/egkristi/ravenrustrag/commit/8b592e68bd08858f2ea6a8bb53fa1e88ce17c3ce"
        },
        "date": 1778007830500,
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
            "value": 558,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1105,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 53670,
            "range": "± 187",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 578205,
            "range": "± 7586",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 95462,
            "range": "± 664",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39615,
            "range": "± 303",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 114432,
            "range": "± 1941",
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
          "id": "b968e07dc6e5730a2f08b08b7ebec52481f3f50f",
          "message": "fix: exclude stress tests from tarpaulin coverage (ptrace overhead)",
          "timestamp": "2026-05-05T21:10:47+02:00",
          "tree_id": "3ceba9e4aefafa4bd5c6eb6b24871d35038125d4",
          "url": "https://github.com/egkristi/ravenrustrag/commit/b968e07dc6e5730a2f08b08b7ebec52481f3f50f"
        },
        "date": 1778008537246,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 557,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1106,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 53961,
            "range": "± 505",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 581541,
            "range": "± 3420",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 94602,
            "range": "± 660",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39422,
            "range": "± 135",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 114601,
            "range": "± 1539",
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
          "id": "e5a31e017685fc62adc3a20d2db35693e5f940e3",
          "message": "fix: lower coverage threshold to 65% (tarpaulin underreports async code)",
          "timestamp": "2026-05-05T21:22:09+02:00",
          "tree_id": "1e8e52d1f01916c5e1b81b08b50ad7826168cb20",
          "url": "https://github.com/egkristi/ravenrustrag/commit/e5a31e017685fc62adc3a20d2db35693e5f940e3"
        },
        "date": 1778009210961,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 92,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 517,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1028,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 58974,
            "range": "± 1572",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 651889,
            "range": "± 3231",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 98917,
            "range": "± 896",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 38769,
            "range": "± 138",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 84353,
            "range": "± 344",
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
          "id": "c89537e3b732d6205fb1ee7e29185df6f25f157a",
          "message": "docs: update PLAN.md and README.md — Phase 6 done, #87 closed, #94 tracked",
          "timestamp": "2026-05-05T21:42:42+02:00",
          "tree_id": "1b91d9268d595f1f977b3853acbe89d6541760ce",
          "url": "https://github.com/egkristi/ravenrustrag/commit/c89537e3b732d6205fb1ee7e29185df6f25f157a"
        },
        "date": 1778010455940,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 557,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1106,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 55887,
            "range": "± 231",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 590046,
            "range": "± 5016",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 93800,
            "range": "± 270",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39551,
            "range": "± 112",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 115793,
            "range": "± 507",
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
          "id": "e20115837695d33e2477d9871f4db459b5508d3c",
          "message": "fix: add workflow_dispatch to packages.yml, fix tag resolution",
          "timestamp": "2026-05-05T22:42:08+02:00",
          "tree_id": "9c59bef8897af75da713a62db23b8e0dacfe3c59",
          "url": "https://github.com/egkristi/ravenrustrag/commit/e20115837695d33e2477d9871f4db459b5508d3c"
        },
        "date": 1778013997082,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 557,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1105,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 56774,
            "range": "± 538",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 592033,
            "range": "± 5430",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 94401,
            "range": "± 842",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39480,
            "range": "± 167",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 112770,
            "range": "± 379",
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
          "id": "68d7e184dbc1a69ba689bba6c2193ca2170701f6",
          "message": "fix: add tag resolution to cargo-deb job in packages workflow",
          "timestamp": "2026-05-05T22:48:17+02:00",
          "tree_id": "4442ed055fdac5086debc92fc74b32c93c3e6809",
          "url": "https://github.com/egkristi/ravenrustrag/commit/68d7e184dbc1a69ba689bba6c2193ca2170701f6"
        },
        "date": 1778014380323,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 557,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1105,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 54890,
            "range": "± 380",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 582977,
            "range": "± 10502",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 96185,
            "range": "± 940",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39871,
            "range": "± 259",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 117469,
            "range": "± 946",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "egkristi@gmail.com",
            "name": "Erling Gustav Moland Kristiansen",
            "username": "egkristi"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2d61681579e34ca0f41d77159e5d4dababf7f1d3",
          "message": "Merge pull request #95 from egkristi/MUNIN-94-winget-initial-manifest\n\nMUNIN-94: Add winget manifest for initial submission",
          "timestamp": "2026-05-06T00:03:09+02:00",
          "tree_id": "d298968c9a8d75d15abee7a587d0f5a008f20a46",
          "url": "https://github.com/egkristi/ravenrustrag/commit/2d61681579e34ca0f41d77159e5d4dababf7f1d3"
        },
        "date": 1778018824729,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 92,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 517,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1028,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 58776,
            "range": "± 992",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 650090,
            "range": "± 4386",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 99339,
            "range": "± 1420",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 38676,
            "range": "± 337",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 85597,
            "range": "± 173",
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
          "id": "d31115d54747950932188d4497c2ed514f60b399",
          "message": "fix: make winget publish non-blocking in release workflow, update PLAN.md",
          "timestamp": "2026-05-06T00:07:27+02:00",
          "tree_id": "ce47fb8b247a66a74374d6e9c81b58bd865f6ff6",
          "url": "https://github.com/egkristi/ravenrustrag/commit/d31115d54747950932188d4497c2ed514f60b399"
        },
        "date": 1778019136219,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 92,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 517,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1028,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 58661,
            "range": "± 2563",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 656919,
            "range": "± 2752",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 97514,
            "range": "± 893",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39020,
            "range": "± 170",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 84018,
            "range": "± 277",
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
          "id": "8565cca5d708fecaf892ba6181eadf3c3c268a7a",
          "message": "docs: mark Phase 4 as complete in PLAN.md",
          "timestamp": "2026-05-06T00:18:53+02:00",
          "tree_id": "a4689b50c091d7e2be526de4087a906ba5f452ae",
          "url": "https://github.com/egkristi/ravenrustrag/commit/8565cca5d708fecaf892ba6181eadf3c3c268a7a"
        },
        "date": 1778019802392,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 92,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 517,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1028,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 60435,
            "range": "± 500",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 663085,
            "range": "± 3519",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 98104,
            "range": "± 1464",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 38708,
            "range": "± 345",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 84800,
            "range": "± 190",
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
          "id": "7ef7b3f2ca5de1a0114217c4fd39d3ac3bbf4a4a",
          "message": "docs: clean up PLAN.md — separate closed issues, update timestamp",
          "timestamp": "2026-05-06T07:17:47+02:00",
          "tree_id": "64ce1cf2b5d97f7a86666e2eb3cda9941315fa5f",
          "url": "https://github.com/egkristi/ravenrustrag/commit/7ef7b3f2ca5de1a0114217c4fd39d3ac3bbf4a4a"
        },
        "date": 1778044952369,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 557,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1106,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 54218,
            "range": "± 596",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 575147,
            "range": "± 1785",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 95025,
            "range": "± 665",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39572,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 115551,
            "range": "± 403",
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
          "id": "f8b17e21e3e9eb4077442500e0867853fd18dce8",
          "message": "docs: move completed items from PLAN.md to changelog, simplify roadmap",
          "timestamp": "2026-05-06T07:35:24+02:00",
          "tree_id": "6c78baa38bc88879e860e9944f69a397aeee7911",
          "url": "https://github.com/egkristi/ravenrustrag/commit/f8b17e21e3e9eb4077442500e0867853fd18dce8"
        },
        "date": 1778045991586,
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
            "value": 557,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1107,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 54418,
            "range": "± 379",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 580811,
            "range": "± 2376",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 97388,
            "range": "± 364",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39288,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 113756,
            "range": "± 531",
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
          "id": "3876e8b74bb3a8671d90de2399c549e3fe59af47",
          "message": "docs: mark Homebrew tap as live in PLAN.md",
          "timestamp": "2026-05-06T07:48:34+02:00",
          "tree_id": "d8b9e86b086f886f1d6fa0fd7266c861785d82ef",
          "url": "https://github.com/egkristi/ravenrustrag/commit/3876e8b74bb3a8671d90de2399c549e3fe59af47"
        },
        "date": 1778046794441,
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
            "value": 557,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1107,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 54307,
            "range": "± 713",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 578696,
            "range": "± 2671",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 96657,
            "range": "± 3127",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39174,
            "range": "± 196",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 115923,
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
          "id": "f497f2afcfbb30feffed98496e9ff7196ae0b74f",
          "message": "ci: auto-update Homebrew tap on release",
          "timestamp": "2026-05-06T08:30:54+02:00",
          "tree_id": "66a9c4cbf93c129884523a722c9b33ee4f2ef508",
          "url": "https://github.com/egkristi/ravenrustrag/commit/f497f2afcfbb30feffed98496e9ff7196ae0b74f"
        },
        "date": 1778049322477,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 557,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1105,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 55425,
            "range": "± 706",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 577791,
            "range": "± 10652",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 96014,
            "range": "± 512",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39626,
            "range": "± 129",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 116178,
            "range": "± 797",
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
          "id": "875f0335a04c5105c3a5fb33b3a8a222284c6e14",
          "message": "feat: rename binary from raven to ravenrag\n\nAvoids naming conflicts with other tools. The installed command\nis now 'ravenrag' across all platforms (brew, scoop, chocolatey,\nsnap, flatpak, AUR, nix, AppImage, Docker).\n\nRelease asset filenames remain unchanged (raven-darwin-arm64 etc.)\nbut all installers map to the 'ravenrag' command name.",
          "timestamp": "2026-05-06T22:22:15+02:00",
          "tree_id": "723ca51140ce2b2368961c0a0c32cbb6afeba081",
          "url": "https://github.com/egkristi/ravenrustrag/commit/875f0335a04c5105c3a5fb33b3a8a222284c6e14"
        },
        "date": 1778099299165,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 92,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 517,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1028,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 59058,
            "range": "± 584",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 654579,
            "range": "± 2378",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 97435,
            "range": "± 2827",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 38623,
            "range": "± 290",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 83676,
            "range": "± 256",
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
          "id": "0ddd0143e490bc32f23e358277dc0b74ca3143a0",
          "message": "fix: bump version to 1.0.1 for ravenrag binary rename",
          "timestamp": "2026-05-08T09:27:51+02:00",
          "tree_id": "7e108236ff31f23832934620c5d4141e220fa996",
          "url": "https://github.com/egkristi/ravenrustrag/commit/0ddd0143e490bc32f23e358277dc0b74ca3143a0"
        },
        "date": 1778225579655,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 101,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 552,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1092,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 55914,
            "range": "± 1171",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 595522,
            "range": "± 3983",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 97314,
            "range": "± 378",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 43752,
            "range": "± 162",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 100760,
            "range": "± 349",
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
          "id": "6286d6757d5227320e14b94efe9e6939af229107",
          "message": "fix: add Docker latest tag on releases, use PAT for homebrew-tap push",
          "timestamp": "2026-05-08T09:58:42+02:00",
          "tree_id": "2a7abdaf0a8addd4f2aa53eb2073eee7ceaf3f15",
          "url": "https://github.com/egkristi/ravenrustrag/commit/6286d6757d5227320e14b94efe9e6939af229107"
        },
        "date": 1778227421008,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 118,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 645,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1274,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 67861,
            "range": "± 195",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 701276,
            "range": "± 1842",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 112630,
            "range": "± 2341",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 50523,
            "range": "± 250",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 118836,
            "range": "± 769",
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
          "id": "aa5b66d6c113d303266ca5f0cfddb6b5927a2439",
          "message": "fix: bump to 1.0.2 to verify homebrew pipeline",
          "timestamp": "2026-05-08T10:20:56+02:00",
          "tree_id": "9d3c98b6687820b7a41fdd1d93793bd3217d0f1d",
          "url": "https://github.com/egkristi/ravenrustrag/commit/aa5b66d6c113d303266ca5f0cfddb6b5927a2439"
        },
        "date": 1778228762904,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 106,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 552,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1092,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 56000,
            "range": "± 802",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 590535,
            "range": "± 2171",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 98364,
            "range": "± 1418",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 43474,
            "range": "± 261",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 101585,
            "range": "± 421",
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
          "id": "1af5dba8dd2bf1a4f49e36a2dc5beb916578422d",
          "message": "fix: update winget identifier to egkristi.ravenrag",
          "timestamp": "2026-05-08T10:36:48+02:00",
          "tree_id": "2acc21daa9de157852850826d4d33add560b1353",
          "url": "https://github.com/egkristi/ravenrustrag/commit/1af5dba8dd2bf1a4f49e36a2dc5beb916578422d"
        },
        "date": 1778229695137,
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
            "value": 557,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1105,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 54588,
            "range": "± 469",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 584200,
            "range": "± 1943",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 95629,
            "range": "± 1596",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39300,
            "range": "± 205",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 117275,
            "range": "± 421",
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
          "id": "cd88c759e4091253628f2949e81ae5faa78e29f3",
          "message": "docs: update README.md and PLAN.md for ravenrag rename and current state",
          "timestamp": "2026-05-08T11:00:03+02:00",
          "tree_id": "9626d3f0f3e3e386ec2939513faf929e7bb388cb",
          "url": "https://github.com/egkristi/ravenrustrag/commit/cd88c759e4091253628f2949e81ae5faa78e29f3"
        },
        "date": 1778231082838,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 557,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1106,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 53922,
            "range": "± 1056",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 572403,
            "range": "± 2566",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 95077,
            "range": "± 443",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 40152,
            "range": "± 315",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 113095,
            "range": "± 1111",
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
          "id": "661f9fbd00ef9c202d1f6c1a1e418e350073464c",
          "message": "docs: fix version refs to 1.0.2, fix MCP description (no ChromaDB)",
          "timestamp": "2026-05-08T11:10:01+02:00",
          "tree_id": "7fea14c93a2b43ebc9c1b6b7d7b1dcf748f09f2b",
          "url": "https://github.com/egkristi/ravenrustrag/commit/661f9fbd00ef9c202d1f6c1a1e418e350073464c"
        },
        "date": 1778231688517,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 557,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1106,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 54738,
            "range": "± 165",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 582223,
            "range": "± 2737",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 94890,
            "range": "± 413",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 39713,
            "range": "± 317",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 117333,
            "range": "± 1538",
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
          "id": "f1d2a4b4c5e1980f4faa030b91a804d62fc12040",
          "message": "docs: add brew tap + install instructions for macOS",
          "timestamp": "2026-05-08T11:21:08+02:00",
          "tree_id": "d41b880038cf94ee0927b748ad41d6ba49bf0e70",
          "url": "https://github.com/egkristi/ravenrustrag/commit/f1d2a4b4c5e1980f4faa030b91a804d62fc12040"
        },
        "date": 1778232446867,
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
            "value": 552,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1091,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 57941,
            "range": "± 1069",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 612841,
            "range": "± 1648",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 98080,
            "range": "± 374",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 43517,
            "range": "± 163",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 104117,
            "range": "± 1149",
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
          "id": "a512c764ac00f6351c25aa8895d5df6f7169ba6a",
          "message": "docs: add documentation link to top of README",
          "timestamp": "2026-05-08T11:24:02+02:00",
          "tree_id": "464417a08aa8a6eb90e6574eef297320bd1c8d98",
          "url": "https://github.com/egkristi/ravenrustrag/commit/a512c764ac00f6351c25aa8895d5df6f7169ba6a"
        },
        "date": 1778232715868,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 102,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 552,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1092,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 55616,
            "range": "± 179",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 582144,
            "range": "± 11468",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 96381,
            "range": "± 302",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 43319,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 101579,
            "range": "± 824",
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
          "id": "e8c60fdbcabee6b759f6f1ea4eb21c8d5c2e0acf",
          "message": "docs: add Docker version tags (latest, semver, sha) to README",
          "timestamp": "2026-05-08T11:28:42+02:00",
          "tree_id": "8ca2ab12db67250065a1bf0a4c029841f06e8fc6",
          "url": "https://github.com/egkristi/ravenrustrag/commit/e8c60fdbcabee6b759f6f1ea4eb21c8d5c2e0acf"
        },
        "date": 1778233164966,
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
            "value": 552,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1091,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 55290,
            "range": "± 299",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 586284,
            "range": "± 8282",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 97846,
            "range": "± 328",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 43613,
            "range": "± 1180",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 101284,
            "range": "± 2435",
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
          "id": "0e8aa0e52c681b35a220d6360b7b595b8793f24b",
          "message": "docs: update installation page with Homebrew, ravenrag binary name, Docker tags",
          "timestamp": "2026-05-08T11:36:40+02:00",
          "tree_id": "0d4756b758d3a9315367192e1e7efccbdf8f2779",
          "url": "https://github.com/egkristi/ravenrustrag/commit/0e8aa0e52c681b35a220d6360b7b595b8793f24b"
        },
        "date": 1778233713585,
        "tool": "cargo",
        "benches": [
          {
            "name": "cosine_128d",
            "value": 103,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_768d",
            "value": 552,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cosine_1536d",
            "value": 1091,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_100docs",
            "value": 56177,
            "range": "± 1017",
            "unit": "ns/iter"
          },
          {
            "name": "query_1000docs",
            "value": 596823,
            "range": "± 2623",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_query_100docs",
            "value": 97468,
            "range": "± 782",
            "unit": "ns/iter"
          },
          {
            "name": "index_10docs",
            "value": 43564,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "bm25_search_1000",
            "value": 101346,
            "range": "± 571",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}