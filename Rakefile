# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/testtask"

GEMSPEC = Gem::Specification.load("oxi-test.gemspec") || abort("Could not load oxi-test.gemspec")

java_p = RUBY_PLATFORM.include?("java")

if java_p
  require_relative "rake_ext"
  require "rake/javaextensiontask"

  # Note: not "oxi-test"
  Rake::JavaExtensionTask.new("oxi_test", GEMSPEC) do |ext|
    ext.lib_dir = "lib/oxi/test"
    ext.ext_dir = "ext"
    ext.source_version = "11" # or "8"
    ext.target_version = "11" # or "8"
  end

  Rake::Task[:compile].rename(:java_compile)

  require "rb_sys/extensiontask"

  RbSys::ExtensionTask.new("oxi-test", GEMSPEC) do |ext|
    ext.lib_dir = "lib/oxi/test"
  end

  task compile: :java_compile
else
  require "rb_sys/extensiontask"

  RbSys::ExtensionTask.new("oxi-test", GEMSPEC) do |ext|
    ext.lib_dir = "lib/oxi/test"
  end
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
  if java_p
    env = {
      # For libjvm.so
      "LD_LIBRARY_PATH" => File.join(
        java.lang.System.getProperty("java.home"), "lib", "server"
      )
    }
    sh env, "cargo test --features jruby_dev"
  else
    sh "cargo test --features mri_dev"
  end
end

task test: [:ruby_test, :cargo_test]

task build: :compile

task default: %i[compile test]
