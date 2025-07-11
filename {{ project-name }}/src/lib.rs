/*
    appellation: gvf <library>
    authors: @FL03
*/
//! # gvf
//!
//! A generative visual framework for the scsys-io platform
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
    "Either the `alloc` or `std` feature must be enabled for the `gvf` crate."
}

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use gvf_core::*;

#[doc(hidden)]
pub mod prelude {
    #[allow(unused_imports)]
    pub use gvf_core::prelude::*;
}
