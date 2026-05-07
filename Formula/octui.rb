class Octui < Formula
  desc "GitHub contribution graph in your terminal"
  homepage "https://github.com/abosnjakovic/octui"
  version "0.0.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/abosnjakovic/octui/releases/download/v0.0.0/octui-0.0.0-aarch64-apple-darwin.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    else
      url "https://github.com/abosnjakovic/octui/releases/download/v0.0.0/octui-0.0.0-x86_64-apple-darwin.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    end
  end

  on_linux do
    url "https://github.com/abosnjakovic/octui/releases/download/v0.0.0/octui-0.0.0-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "0000000000000000000000000000000000000000000000000000000000000000"
  end

  def install
    bin.install "octui"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/octui --version")
  end
end
