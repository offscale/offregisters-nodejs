offregisters-nodejs
===================

offregisters-nodejs is a library which works similarly to [`nvm`](https://github.com/creationix/nvm) & [`n`](https://github.com/tj/n), but is written in Rust, and is released as a single static binary.

## Purpose
The main purpose is to handle `FROM scratch` Docker builds, and enable buildpack-style but native installs for developer & server installations.
Secondary purposes include a cross-platform [`nvm`](https://github.com/creationix/nvm) & [`n`](https://github.com/tj/n) alternative.
Final purpose is to serve as a model for other installers, e.g.: for Ruby, Python, R, &etc.

## Developer guide

Install the latest version of [Rust](https://www.rust-lang.org). We tend to use nightly versions. [CLI tool for installing Rust](https://rustup.rs).

We use [rust-clippy](https://github.com/rust-lang-nursery/rust-clippy) linters to improve code quality.

There are plenty of [IDEs](https://areweideyet.com) and other [Rust development tools to consider](https://github.com/rust-unofficial/awesome-rust#development-tools).

### Step-by-step guide
Run this after you have cloned/downloaded this repository and `cd`'d to its directory;
```bash
# Install Rust (nightly)
$ curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
# Install cargo-make (cross-platform feature-rich reimplementation of Make)
$ cargo install --force cargo-make
# Install rustfmt (Rust formatter)
$ rustup component add rustfmt
# Run tests
$ cargo test
# Format, download dependencies, build and test
$ cargo make
```
