use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;

fn raven() -> Command {
    Command::cargo_bin("raven").expect("binary exists")
}

// ---------------------------------------------------------------------------
// Help & version
// ---------------------------------------------------------------------------

#[test]
fn test_help() {
    raven()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("raven"));
}

#[test]
fn test_version() {
    raven()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("raven"));
}

// ---------------------------------------------------------------------------
// index command
// ---------------------------------------------------------------------------

#[test]
fn test_index_nonexistent_path() {
    // CLI treats nonexistent paths gracefully (0 files found, warning printed)
    raven()
        .args(["index", "/nonexistent/path/12345"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0 files").or(predicate::str::contains("No documents")));
}

#[test]
fn test_index_dry_run() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("a.txt"), "Hello world").expect("write");
    std::fs::write(dir.path().join("b.md"), "# Title\nContent").expect("write");

    raven()
        .args([
            "index",
            dir.path().to_str().expect("path"),
            "--dry-run",
            "--db",
            dir.path().join("test.db").to_str().expect("db path"),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Dry run"));
}

// ---------------------------------------------------------------------------
// query command (empty db)
// ---------------------------------------------------------------------------

#[test]
fn test_query_empty_db() {
    let dir = tempfile::tempdir().expect("tempdir");
    let db = dir.path().join("empty.db");

    raven()
        .args(["query", "test query", "--db", db.to_str().expect("db path")])
        .assert()
        .success();
}

// ---------------------------------------------------------------------------
// info command
// ---------------------------------------------------------------------------

#[test]
fn test_info() {
    let dir = tempfile::tempdir().expect("tempdir");
    let db = dir.path().join("info.db");

    raven()
        .args(["info", "--db", db.to_str().expect("db path")])
        .assert()
        .success();
}

// ---------------------------------------------------------------------------
// status command
// ---------------------------------------------------------------------------

#[test]
fn test_status() {
    let dir = tempfile::tempdir().expect("tempdir");
    let db = dir.path().join("status.db");

    raven()
        .args(["status", "--db", db.to_str().expect("db path")])
        .assert()
        .success();
}

// ---------------------------------------------------------------------------
// export command (empty db)
// ---------------------------------------------------------------------------

#[test]
fn test_export_empty() {
    let dir = tempfile::tempdir().expect("tempdir");
    let db = dir.path().join("export.db");
    let out = dir.path().join("out.jsonl");

    raven()
        .args([
            "export",
            "--db",
            db.to_str().expect("db path"),
            "-o",
            out.to_str().expect("out path"),
        ])
        .assert()
        .success();

    let content = std::fs::read_to_string(&out).expect("read export");
    assert!(content.is_empty() || content.trim().is_empty());
}

// ---------------------------------------------------------------------------
// import command
// ---------------------------------------------------------------------------

#[test]
fn test_import_empty_file() {
    let dir = tempfile::tempdir().expect("tempdir");
    let db = dir.path().join("import.db");
    let jsonl = dir.path().join("data.jsonl");
    std::fs::write(&jsonl, "").expect("write jsonl");

    raven()
        .args([
            "import",
            jsonl.to_str().expect("jsonl path"),
            "--db",
            db.to_str().expect("db path"),
        ])
        .assert()
        .success();
}

#[test]
fn test_import_valid_jsonl() {
    let dir = tempfile::tempdir().expect("tempdir");
    let db = dir.path().join("import2.db");
    let jsonl = dir.path().join("data.jsonl");

    let mut f = std::fs::File::create(&jsonl).expect("create");
    writeln!(
        f,
        r#"{{"id":"doc1","text":"Hello world","metadata":{{"source":"test.txt"}}}}"#
    )
    .expect("write");
    writeln!(
        f,
        r#"{{"id":"doc2","text":"Second doc","metadata":{{"source":"test2.txt"}}}}"#
    )
    .expect("write");

    raven()
        .args([
            "import",
            jsonl.to_str().expect("jsonl path"),
            "--db",
            db.to_str().expect("db path"),
        ])
        .assert()
        .success();
}

// ---------------------------------------------------------------------------
// doctor command
// ---------------------------------------------------------------------------

#[test]
fn test_doctor() {
    raven().arg("doctor").assert().success();
}

// ---------------------------------------------------------------------------
// clear command
// ---------------------------------------------------------------------------

#[test]
fn test_clear() {
    let dir = tempfile::tempdir().expect("tempdir");
    let db = dir.path().join("clear.db");

    raven()
        .args(["clear", "--db", db.to_str().expect("db path")])
        .assert()
        .success();
}

// ---------------------------------------------------------------------------
// completions command
// ---------------------------------------------------------------------------

#[test]
fn test_completions_bash() {
    raven()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("raven"));
}

#[test]
fn test_completions_zsh() {
    raven()
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("raven"));
}

// ---------------------------------------------------------------------------
// Full round-trip: index → info → query → export
// ---------------------------------------------------------------------------

#[test]
fn test_full_roundtrip() {
    let dir = tempfile::tempdir().expect("tempdir");
    let docs_dir = dir.path().join("docs");
    std::fs::create_dir(&docs_dir).expect("mkdir");
    std::fs::write(
        docs_dir.join("intro.txt"),
        "Rust is a systems programming language focused on safety and speed.",
    )
    .expect("write");
    std::fs::write(
        docs_dir.join("rag.txt"),
        "RAG stands for Retrieval-Augmented Generation. It combines search with LLMs.",
    )
    .expect("write");

    let db = dir.path().join("roundtrip.db");
    let db_str = db.to_str().expect("db path");
    let docs_str = docs_dir.to_str().expect("docs path");

    // Index
    raven()
        .args(["index", docs_str, "--db", db_str, "--extensions", "txt"])
        .assert()
        .success();

    // Info
    raven().args(["info", "--db", db_str]).assert().success();

    // Query
    raven()
        .args(["query", "What is Rust?", "--db", db_str])
        .assert()
        .success();

    // Export
    let export_file = dir.path().join("backup.jsonl");
    raven()
        .args([
            "export",
            "--db",
            db_str,
            "-o",
            export_file.to_str().expect("export path"),
        ])
        .assert()
        .success();

    let exported = std::fs::read_to_string(&export_file).expect("read export");
    assert!(
        !exported.trim().is_empty(),
        "export should contain documents"
    );
}
