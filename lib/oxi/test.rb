# frozen_string_literal: true

require "oxi/test/version"

begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "#{$1}/oxi/test/ext"
rescue LoadError
  require "oxi/test/ext"
end

module Oxi
  module Test
    class Error < StandardError; end
    # Your code goes here...
  end
end
