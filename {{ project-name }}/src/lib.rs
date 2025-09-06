//! # {{ project-name }}
#![allow(
    non_snake_case,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::self_named_constructors,
    clippy::upper_case_acronyms,
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]

#[cfg(not(all(feature = "alloc", feature = "std")))]
compiler_error! {
    "Either the `alloc` or `std` feature must be enabled for the crate to compile."
}

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use {{ project-name }}_core::*;

#[doc(hidden)]
pub mod prelude {
    #[allow(unused_imports)]
    pub use {{ project-name }}_core::prelude::*;
}
