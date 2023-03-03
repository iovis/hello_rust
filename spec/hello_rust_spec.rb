# frozen_string_literal: true

RSpec.describe HelloRust do
  describe ".hello" do
    subject { described_class.hello(arg) }

    let(:arg) { "Francis" }

    it { is_expected.to eq "Hello from Rust, Francis!" }
  end
end
