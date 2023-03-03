# frozen_string_literal: true

RSpec.describe HelloRust do
  describe ".hello" do
    subject { described_class.hello(arg) }

    let(:arg) { "Francis" }

    it { is_expected.to eq "Hello from Rust, Francis!" }
  end

  describe ".rust_fib" do
    it { expect(described_class.rust_fib(1)).to eq 1 }
    it { expect(described_class.rust_fib(3)).to eq 2 }
    it { expect(described_class.rust_fib(7)).to eq 13 }
    it { expect(described_class.rust_fib(8)).to eq 21 }
  end

  describe ".rb_fib" do
    it { expect(described_class.rb_fib(1)).to eq 1 }
    it { expect(described_class.rb_fib(3)).to eq 2 }
    it { expect(described_class.rb_fib(7)).to eq 13 }
    it { expect(described_class.rb_fib(8)).to eq 21 }
  end
end
