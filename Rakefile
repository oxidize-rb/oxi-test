# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/testtask"
require "rake/extensiontask"

GEMSPEC = Bundler.load_gemspec("oxi-test.gemspec")

CROSS_PLATFORMS = [
  "arm-linux",
  "aarch64-linux",
  "arm64-darwin",
  "x64-mingw-ucrt",
  "x64-mingw32",
  "x86_64-darwin",
  "x86_64-linux",
  "x86_64-linux-musl"
]

Rake::TestTask.new(:ruby_test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList["test/**/*_test.rb"]
end

task :cargo_test do
  sh "cargo test"
end

task test: [:ruby_test, :cargo_test]

Rake::ExtensionTask.new("ext", GEMSPEC) do |ext|
  ext.lib_dir = "lib/oxi/test"
  ext.ext_dir = "ext"
  ext.cross_compile = true
  ext.cross_platform = CROSS_PLATFORMS
end

task build: :compile
task default: %i[compile test]
