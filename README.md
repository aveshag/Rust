# Rust 
## Installation
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at: `/home/<user>/.rustup`. This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at: `/home/<user>/.cargo`. This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to Cargo's bin directory, located at: `/home/<user>/.cargo/bin`. This path will then be added to your PATH environment variable by modifying the profile files located at:
`/home/<user>/.profile` and `/home/<user>/.bashrc`

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

3. Rust files always end with the .rs extension. If you’re using more than one word in your filename, the convention is to use an underscore to separate them

4. Rust’s naming convention for constants is to use all uppercase with underscores between words.

5. The tuple without any values has a special name, unit. This value and its corresponding type are both written () and represent an empty value or an empty return type. 