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

  describe ".rust_parse_csv" do
    subject(:csv) { described_class.rust_parse_csv(file) }

    let(:file) { "spec/fixtures/organizations_small.csv" }

    it do
      expect(csv.first.to_h).to eq({
                                     "Index" => "1",
                                     "Organization Id" => "391dAA77fea9EC1",
                                     "Name" => "Daniel-Mcmahon",
                                     "Website" => "https://stuart-rios.biz/",
                                     "Country" => "Cambodia",
                                     "Description" => "Focused eco-centric help-desk",
                                     "Founded" => "2013",
                                     "Industry" => "Sports",
                                     "Number of employees" => "1878"
                                   })
    end
  end

  describe ".rb_parse_csv" do
    subject(:csv) { described_class.rb_parse_csv(file) }

    let(:file) { "spec/fixtures/organizations_small.csv" }

    it do
      expect(csv.first.to_h).to eq({
                                     "Index" => "1",
                                     "Organization Id" => "391dAA77fea9EC1",
                                     "Name" => "Daniel-Mcmahon",
                                     "Website" => "https://stuart-rios.biz/",
                                     "Country" => "Cambodia",
                                     "Description" => "Focused eco-centric help-desk",
                                     "Founded" => "2013",
                                     "Industry" => "Sports",
                                     "Number of employees" => "1878"
                                   })
    end
  end

  describe ".rust_count_employees" do
    subject { described_class.rust_count_employees(file) }

    let(:file) { "spec/fixtures/organizations_small.csv" }

    it { is_expected.to eq 48_727 }
  end

  describe ".rb_count_employees" do
    subject { described_class.rb_count_employees(file) }

    let(:file) { "spec/fixtures/organizations_small.csv" }

    it { is_expected.to eq 48_727 }
  end
end
