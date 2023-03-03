# frozen_string_literal: true

require_relative "hello_rust/version"
require_relative "hello_rust/hello_rust"

module HelloRust
  class Error < StandardError; end

  def self.rb_fib(number)
    return 0 if number.zero?
    return 1 if number == 1

    rb_fib(number - 1) + rb_fib(number - 2)
  end
end
