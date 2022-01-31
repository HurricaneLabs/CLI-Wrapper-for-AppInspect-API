# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# PLEASE REMOVE ALL GENERATED COMMENTS BEFORE SUBMITTING YOUR PULL REQUEST!
class Appinspect < Formula
  desc "AppInspect API: Now on the CLI" 
  homepage ""
  url "" :using => GitHubPrivateRepositoryReleaseDownloadStrategy
  sha256 ""
  license ""
  version "0.1.7"

  def install
    bin.install "target/release/appinspect"
  end

end
