# HelloRust

Experiments creating Ruby extensions in Rust

## Installation

You may need `git-lfs` and the rust toolchain

```sh
$ brew install git-lfs
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  # Install Rust toolchain
```

Install the gem and add to the application's Gemfile by executing:

```sh
$ bundle install
$ bundle exec rake
```

## Development

```sh
$ bin/console  # Open `pry` with the library linked
$ bin/benches  # Run benchmarks
```
