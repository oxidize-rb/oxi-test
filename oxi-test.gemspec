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

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject do |f|
      (f == __FILE__) || f.match(%r{\A(?:(?:bin|test|spec|features)/|\.(?:git|travis|circleci)|appveyor)})
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/Cargo.toml"]

  spec.add_dependency "rb_sys"
end
