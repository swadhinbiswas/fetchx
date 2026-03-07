class Fetchx < Formula
  desc "A blazing-fast, feature-rich system information tool written in Rust"
  homepage "https://github.com/swadhinbiswas/fetchx"
  url "https://github.com/swadhinbiswas/fetchx/releases/download/v0.2.0/fetchx-0.2.0-x86_64-apple-darwin.tar.gz"
  version "0.2.0"
  sha256 "REPLACE_WITH_ACTUAL_SHA256"
  license "MIT"
  head "https://github.com/swadhinbiswas/fetchx.git"

  depends_on "rust" => :build

  def install
    # Build from source if using head
    if build.head?
      system "cargo", "build", "--release", "--locked"
      bin.install "target/release/fetchx"
    else
      bin.install "fetchx"
    end
  end

  test do
    system "#{bin}/fetchx", "--version"
  end
end
