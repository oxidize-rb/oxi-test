# frozen_string_literal: true

require_relative "lib/oxi/test/version"

java_p = RUBY_PLATFORM.include?("java")

Gem::Specification.new do |spec|
  spec.name = "oxi-test"
  spec.version = Oxi::Test::VERSION
  spec.authors = ["Ian Ker-Seymer"]
  spec.email = ["hello@ianks.com"]

  spec.summary = "Just a Rust test gem"
  spec.description = "Used to integration test rb-sys"
  spec.homepage = "https://github.com/oxidize-rb/oxi-test"
  spec.license = "MIT"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = spec.homepage
  spec.required_ruby_version = ">= 2.3.0"

  ext_files = %w[rs rb]
  lib_files = %w[rb]
  lib_files << "jar" if java_p
  spec.files = Dir["lib/**/*.{#{lib_files.join(",")}}", "ext/**/*.{#{ext_files.join(",")}}",
                   "Cargo.lock", "Cargo.toml", "ext/Cargo.toml", "LICENSE.txt", "README.md"]
  spec.bindir = "exe"
  spec.executables = []
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/extconf.rb"]

  spec.add_dependency "rb_sys", "~> 0.9"
end
