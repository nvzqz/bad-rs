# bad-rs

<div align="center">
    <a href="https://crates.io/crates/bad">
        <img src="https://img.shields.io/crates/v/bad.svg" alt="Crates.io">
        <img src="https://img.shields.io/crates/d/bad.svg" alt="Downloads">
    </a>
    <a href="https://travis-ci.com/nvzqz/bad-rs">
        <img src="https://travis-ci.com/nvzqz/bad-rs.svg?branch=master" alt="Build Status">
    </a>
    <img src="https://img.shields.io/badge/rustc-^1.31.0-blue.svg" alt="rustc ^1.31.0">
    <br>
    <a href="https://www.patreon.com/nvzqz">
        <img src="https://c5.patreon.com/external/logo/become_a_patron_button.png" alt="Become a Patron!" height="35">
    </a>
    <a href="https://www.paypal.me/nvzqz">
        <img src="https://buymecoffee.intm.org/img/button-paypal-white.png" alt="Buy me a coffee" height="35">
    </a>
</div>

A collection of (bad) ideas in Rust that you may or may not want to make use of
in your next big project. Courtesy of
[Nikolai Vazquez](https://twitter.com/NikolaiVazquez).

[Documentation](https://docs.rs/bad/)

## Installation

This crate is available [on crates.io](https://crates.io/crates/bad) and can be
used by adding the following to your project's
[`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html):

```toml
[dependencies]
bad = "0.1.1"
```

and optionally add this to your crate root (`main.rs` or `lib.rs`):

```rust
extern crate bad;
```

## Usage

This crate exposes the following functionality:

- [`StalinSort`]:

  A single pass, no-nonsense sorting algorithm with O(n) complexity that removes
  elements until the value is sorted.

- [`Never`]:

  A type alias to [`!` (never)][never] that works in places `!` doesn't
  currently in stable Rust.

## License

This project is released under either:

- [The Unlicense](https://github.com/nvzqz/bad-rs/blob/master/UNLICENSE)

- [MIT License](https://github.com/nvzqz/bad-rs/blob/master/LICENSE-MIT)

at your choosing.

[never]: https://doc.rust-lang.org/std/primitive.never.html

[`StalinSort`]: https://docs.rs/bad/0.1.1/bad/trait.StalinSort.html
[`Never`]:      https://docs.rs/bad/0.1.1/bad/type.Never.html
