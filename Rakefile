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

task ruby_test: [:compile]

desc "Build native extension for a given platform (i.e. `rake 'native[x86_64-linux]'`)"
task :native, [:platform] do |_t, platform:|
  sh "bundle", "exec", "rb-sys-dock", "--platform", platform, "--build"
end

task :cargo_test, [:compile] do
  if java_p
    java_home = java.lang.System.getProperty("java.home")
    class_path = [
      ENV["CLASSPATH"],
      File.join(RbConfig::CONFIG["libdir"], RbConfig::CONFIG["LIBRUBY"]), # jruby.jar
      File.join(
        File.absolute_path(File.dirname(__FILE__)),
        "lib", "oxi", "test", "oxi_test.jar"
      )
    ]
    if Gem.win_platform?
      bin_path = [
        File.join(java_home, "bin", "server"),
        File.join(java_home, "bin"),
      ]
      env = { "PATH" => "#{ENV["PATH"]};#{bin_path.join(";")}", "CLASSPATH" => class_path.join(";") }
    else
      lib_path = [
        File.join(java_home, "lib", "server"), # For libjvm.so
        File.join(java_home, "lib"), # For libjli.dylib
      ]
      var_name = RbConfig::MAKEFILE_CONFIG["RUBY_PLATFORM"] =~ /darwin/ ? "DYLD_FALLBACK_LIBRARY_PATH" : "LD_LIBRARY_PATH"
      env = { var_name => "#{ENV[var_name]}:#{lib_path.join(":")}", "CLASSPATH" => class_path.join(":") }
    end

    sh env, "cargo test --features jruby_dev"
  else
    sh "cargo test --features mri_dev"
  end
end

task test: [:ruby_test, :cargo_test]

task build: :compile

task default: %i[compile test]
