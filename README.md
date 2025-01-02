# Bevy Colony Simulator Game

This is a learning / experimental project to develop
a [colony simulator](https://en.wikipedia.org/wiki/Construction_and_management_simulation) style game using
the [Bevy](https://bevyengine.org/) game engine.

## Language

[Rust](https://www.rust-lang.org/)

- <https://doc.rust-lang.org/stable/book/>
- <https://doc.rust-lang.org/stable/rust-by-example/>
- <https://rust-unofficial.github.io/patterns/-> <https://doc.rust-lang.org/stable/book/>
- <https://doc.rust-lang.org/stable/rust-by-example/>
- <https://doc.rust-lang.org/reference/introduction.html>

See also:

- [https://en.wikipedia.org/wiki/Rust_(programming_language)](https://en.wikipedia.org/wiki/Rust_(programming_language))

### Install

<https://www.rust-lang.org/tools/install>

### Update

`rustup update`

### Uninstall

`rustup self uninstall`

### Local Documentation

`rustup doc`

### Cargo

When you install Rustup you’ll also get the latest stable version of the Rust build tool and package manager, also known
as Cargo. Cargo does lots of things:

- Check your project (for errors) with `cargo check` (faster than `cargo build`)
- Build your project with `cargo build`
- Run your project with `cargo run`
- Test your project with `cargo test`
- Remove the target directory with `cargo clean`
- Build documentation for your project with `cargo doc`
- Publish a library to [crates.io](https://crates.io) with `cargo publish`
- Reformat your code according to rust style guidelines with `cargo fmt`
- Find issues with the content of your code with `cargo clippy`

To test that you have Rust and Cargo installed, you can run this in your terminal of choice:

`cargo --version`

## Tooling

### clippy

A collection of lints to catch common mistakes and improve your Rust code.

To see suggestions: `cargo clippy`

To automatically apply suggestions: `cargo clippy --fix`

1. <https://github.com/rust-lang/rust-clippy>

### rustfmt

A tool for formatting Rust code according to style guidelines.

1. <https://github.com/rust-lang/rustfmt>
2. <https://github.com/rust-lang/rustfmt/blob/master/intellij.md> (For use with CLion's Rust Plugin)

## CI / CD

TODO

## Dependencies

See `DEPENDENCIES.md` for more information.
