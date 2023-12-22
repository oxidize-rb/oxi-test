# frozen_string_literal: true

require "mkmf"
require "rb_sys/mkmf"

java_p = RUBY_PLATFORM.include?("java")

create_rust_makefile("oxi/test/oxi_test") do |r|
  r.features = java_p ? %w[jruby] : %w[mri]
end