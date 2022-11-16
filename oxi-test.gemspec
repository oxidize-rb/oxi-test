# frozen_string_literal: true

require_relative "lib/oxi/test/version"

Gem::Specification.new do |spec|
  spec.name = "oxi-test"
  spec.version = Oxi::Test::VERSION
  spec.authors = ["Ian Ker-Seymer"]
  spec.email = ["hello@ianks.com"]

  spec.summary = "Just a test gem"
  spec.description = "Just a test gem"
  spec.homepage = "https://github.com/oxidize-rb/oxi-test"
  spec.license = "MIT"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = spec.homepage
  spec.required_ruby_version = ">= 2.3.0"

  spec.files = Dir["lib/**/*", "ext/**/*.{rs,rb}", "**/Cargo.*", "LICENSE.txt", "README.md"]
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/extconf.rb"]

  spec.add_dependency "rb_sys", "~> 0.9.41"
end
