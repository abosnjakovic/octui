class Octui < Formula
  desc "GitHub contribution graph in your terminal"
  homepage "https://github.com/abosnjakovic/octui"
  version "0.1.0"

  if Hardware::CPU.arm?
    url "https://github.com/abosnjakovic/octui/releases/download/v0.1.0/octui-0.1.0-aarch64-apple-darwin.tar.gz"
    sha256 "PLACEHOLDER"
  else
    url "https://github.com/abosnjakovic/octui/releases/download/v0.1.0/octui-0.1.0-x86_64-apple-darwin.tar.gz"
    sha256 "PLACEHOLDER"
  end

  depends_on "gh"

  def install
    bin.install "octui"
  end

  test do
    system "#{bin}/octui", "--help"
  end
end
