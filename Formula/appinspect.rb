# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# PLEASE REMOVE ALL GENERATED COMMENTS BEFORE SUBMITTING YOUR PULL REQUEST!
class Appinspect < Formula
  desc "AppInspect API: Now on the CLI" 
  homepage "https://code.hurricanelabs.net/ian/homebrew-appinspect-api-cli-wrapper"
  url "https://code.hurricanelabs.net/api/v4/projects/1052/repository/archive.tar.gz?sha=v0.1.6"
  sha256 "37d40cc57e6f592a07c9820e1844b34a07c6666bf3a83965ebc183d2eb5ed89f"
  license ""
  version "0.1.6"

  def install
    bin.install "target/release/appinspect"
  end

end
