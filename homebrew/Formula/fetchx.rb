require "language/node"

class Fetchx < Formula
  desc "A blazing-fast, feature-rich system information tool written in Rust"
  homepage "https://github.com/swadhinbiswas/fetchx"
  license "MIT"
  version "0.2.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/swadhinbiswas/fetchx/releases/download/v#{version}/fetchx-#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_aarch64"
    end

    if Hardware::CPU.intel?
      url "https://github.com/swadhinbiswas/fetchx/releases/download/v#{version}/fetchx-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_x86_64"
    end
  end

  def install
    bin.install "fetchx"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/fetchx --version")
  end
end
