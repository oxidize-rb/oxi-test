# frozen_string_literal: true

require_relative "test/version"

begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "#{$1}/test/ext"
rescue LoadError
  require "test/ext"
end

module Oxi
  module Test
    class Error < StandardError; end
    # Your code goes here...
  end
end
