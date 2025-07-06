/*
    appellation: {{ project-name }}-core <library>
    authors: @FL03
*/
//! The core modules of the {{ project-name }} project
#![allow(
    non_snake_case,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]

#[cfg(not(all(feature = "alloc", feature = "std")))]
compiler_error! {
    "Either the `alloc` or `std` feature must be enabled for the `gvf-core` crate."
}

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::error::{Error, Result};

pub mod error;

pub mod traits {
    //! the core traits for the {{ project-name }} project
}

#[doc(hidden)]
pub mod prelude {}
