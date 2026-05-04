class Ravenrustrag < Formula
  desc "Local-first RAG engine - fast, safe retrieval-augmented generation in Rust"
  homepage "https://egkristi.github.io/ravenrustrag/"
  url "https://github.com/egkristi/ravenrustrag/archive/refs/tags/v0.1.0-alpha.1.tar.gz"
  sha256 "PLACEHOLDER"
  license "AGPL-3.0-or-later"
  head "https://github.com/egkristi/ravenrustrag.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: "crates/raven-cli")

    # Install shell completions
    generate_completions_from_executable(bin/"raven", "completions")
  end

  test do
    assert_match "raven", shell_output("#{bin}/raven --version")

    # Test indexing a simple document
    (testpath/"test.txt").write("Hello, world! This is a test document for RavenRustRAG.")
    system bin/"raven", "index", testpath/"test.txt", "--db", testpath/"test.db", "--backend", "dummy"

    # Test querying
    output = shell_output("#{bin}/raven query 'hello' --db #{testpath}/test.db --backend dummy --top-k 1")
    assert_match "Hello", output
  end
end
