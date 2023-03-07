# frozen_string_literal: true

require_relative "hello_rust/version"
require_relative "hello_rust/hello_rust"

require "csv"

module HelloRust
  class Error < StandardError; end

  def self.rb_fib(number)
    return 0 if number.zero?
    return 1 if number == 1

    rb_fib(number - 1) + rb_fib(number - 2)
  end

  def self.rb_parse_csv(path)
    file = File.read(path)
    CSV.parse(file, headers: true)
  end

  def self.rb_count_employees(path)
    file = File.read(path)
    csv = CSV.parse(file, headers: true)
    csv.sum { _1["Number of employees"].to_i }
  end
end
