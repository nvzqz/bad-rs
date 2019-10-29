//! A collection of (bad) ideas that you may or may not want use in your next
//! big project. Courtesy of
//! [Nikolai Vazquez](https://twitter.com/NikolaiVazquez).
//!
//! ## Installation
//!
//! This crate is available [on crates.io](https://crates.io/crates/bad) and can be
//! used by adding the following to your project's
//! [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html):
//!
//! ```toml
//! [dependencies]
//! bad = "0.1.0"
//! ```
//!
//! and optionally add this to your crate root (`main.rs` or `lib.rs`):
//!
//! ```rust
//! extern crate bad;
//! ```
//!
//! ## License
//!
//! This project is released under either:
//!
//! - [The Unlicense](https://github.com/nvzqz/bad-rs/blob/master/UNLICENSE)
//!
//! - [MIT License](https://github.com/nvzqz/bad-rs/blob/master/LICENSE-MIT)
//!
//! at your choosing.
//!
//! [`StalinSort`]: trait.StalinSort.html

#![deny(missing_docs)]
#![deny(private_in_public)]

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(test, nightly), feature(test))]

#[cfg(all(test, nightly))]
extern crate test;

#[cfg(feature = "alloc")]
extern crate alloc;

mod never;
mod stalin_sort;
pub use self::{
    stalin_sort::StalinSort,
    never::Never,
};
