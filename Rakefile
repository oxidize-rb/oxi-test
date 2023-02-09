# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/testtask"
require "rb_sys/extensiontask"

GEMSPEC = Gem::Specification.load("oxi-test.gemspec") || abort("Could not load oxi-test.gemspec")

RbSys::ExtensionTask.new("oxi-test", GEMSPEC) do |ext|
  ext.lib_dir = "lib/oxi/test"
end

task :fmt do
  sh "cargo", "fmt"
end

Rake::TestTask.new(:ruby_test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList["test/**/*_test.rb"]
end

desc "Build native extension for a given platform (i.e. `rake 'native[x86_64-linux]'`)"
task :native, [:platform] do |_t, platform:|
  sh "bundle", "exec", "rb-sys-dock", "--platform", platform, "--build"
end

task :cargo_test do
  sh "cargo test"
end

task test: [:ruby_test, :cargo_test]

task build: :compile

task default: %i[compile test]
