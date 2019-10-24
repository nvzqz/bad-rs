//! Unlicensed bad ideas.

#![deny(missing_docs)]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod stalin_sort;
pub use self::{
    stalin_sort::StalinSort,
};
