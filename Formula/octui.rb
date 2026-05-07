class Octui < Formula
  desc "GitHub contribution graph in your terminal"
  homepage "https://github.com/abosnjakovic/octui"
  version "0.1.1"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/abosnjakovic/octui/releases/download/v0.1.1/octui-0.1.1-aarch64-apple-darwin.tar.gz"
      sha256 "1c1b90ba5152000a18951b991473c6374060a5a99463ee3633462ca53e53de0f"
    else
      url "https://github.com/abosnjakovic/octui/releases/download/v0.1.1/octui-0.1.1-x86_64-apple-darwin.tar.gz"
      sha256 "c74a0c042b082e4da718f7a007d403d229be0c5c65d6b6116d2546a5419a71f2"
    end
  end

  on_linux do
    url "https://github.com/abosnjakovic/octui/releases/download/v0.1.1/octui-0.1.1-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "eddde56f96cf78d2fd02b5782a72a6cd83e594b76a5fee5a945be377d67bc241"
  end

  def install
    bin.install "octui"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/octui --version")
  end
end
