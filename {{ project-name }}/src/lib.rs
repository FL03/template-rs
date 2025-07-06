/*
    appellation: {{ project-name }} <library>
    authors: @FL03
*/
//! # {{ project-name }}
//!
//! {{ description }}
#![allow(
    non_snake_case,
    slippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
)]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::error::{Error, Result};

pub mod error;

#[doc(hidden)]
pub mod prelude {}
