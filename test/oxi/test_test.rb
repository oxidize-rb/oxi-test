# frozen_string_literal: true

require 'test_helper'

module Oxi
  class TestTest < Minitest::Test
    def test_that_it_has_a_version_number
      assert ::Oxi::Test.const_defined?(:VERSION)
    end

    def test_it_does_something_useful
      assert_equal 'Hello, Ian', Oxi::Test.hello('Ian')
    end
  end
end
