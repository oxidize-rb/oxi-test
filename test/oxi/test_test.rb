# frozen_string_literal: true

require "test_helper"

describe Oxi::Test do
  it "has a VERSION" do
    assert ::Oxi::Test.const_defined?(:VERSION)
  end

  it "does something useful" do
    assert_equal "Hello, Ian", Oxi::Test.hello("Ian")
  end
end
