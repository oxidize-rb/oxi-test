# frozen_string_literal: true

require 'bundler/gem_tasks'
require 'rb_sys/extensiontask'
require "minitest/test_task"

GEMSPEC = Gem::Specification.load('oxi-test.gemspec') || abort('Could not load oxi-test.gemspec')

RbSys::ExtensionTask.new('oxi-test', GEMSPEC) do |ext|
  ext.lib_dir = 'lib/oxi/test'
end

Minitest::TestTask.create :ruby_test

task :fmt do
  sh 'cargo', 'fmt'
end

desc "Build native extension for a given platform (i.e. `rake 'native[x86_64-linux]'`)"
task :native, [:platform] do |_t, platform:|
  sh 'bundle', 'exec', 'rb-sys-dock', '--platform', platform, '--build'
end

task :cargo_test do
  sh 'cargo test'
end

task test: %i[ruby_test cargo_test]

task build: :compile

task default: %i[compile test]
