class Octui < Formula
  desc "GitHub contribution graph in your terminal"
  homepage "https://github.com/abosnjakovic/octui"
  version "0.3"

  if Hardware::CPU.arm?
    url "https://github.com/abosnjakovic/octui/releases/download/v0.3/octui-0.3-aarch64-apple-darwin.tar.gz"
    sha256 "80668488c01c8785dbe2e1ec09c1406838759dc01d195e186d2cd86e81e437ba"
  else
    url "https://github.com/abosnjakovic/octui/releases/download/v0.3/octui-0.3-x86_64-apple-darwin.tar.gz"
    sha256 "4417432cadeeb359639920bba7ecf8eac99179b85f3f2bb56d42aec801d3e79a"
  end

  depends_on "gh"

  def install
    bin.install "octui"
  end

  test do
    system "#{bin}/octui", "--help"
  end
end
