# frozen_string_literal: true

require "jruby"

def oxi
  Java::Oxi
end

# like require_relative
lib_path = File.join(
  File.absolute_path(File.dirname(__FILE__)),
  "oxi_test.#{RbConfig::MAKEFILE_CONFIG["DLEXT"]}"
)
oxi.test.OxiTestService.systemLoad(lib_path)
oxi.test.OxiTestService.new.basicLoad(JRuby.runtime)
