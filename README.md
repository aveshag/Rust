# Rust 
## Installation

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at: `/home/avesh/.rustup`. This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at: `/home/avesh/.cargo`. This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to Cargo's bin directory, located at: `/home/avesh/.cargo/bin`. This path will then be added to your PATH environment variable by modifying the profile files located at:
`/home/avesh/.profile` and `/home/avesh/.bashrc`

## Update
> rustup update

## Uninstall
> rustup self uninstall

## Commands
```
cargo new
cargo init

cargo build
cargo run
cargo check
cargo build --release

cargo doc --open
```

## Code
1. hello_world
2. hello_world_cargo
3. guessing_game

## Notes
1. By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude, and you can see everything in it in the standard library documentation.


2. If the user types 5 and presses enter, guess looks like this: 5\n. The \n represents “newline”. (On Windows, pressing enter results in a carriage return and a newline, \r\n). The trim method eliminates \n or \r\n, resulting in just 5.