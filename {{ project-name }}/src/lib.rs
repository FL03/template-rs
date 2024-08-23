/*
    Appellation: {{ project-name }} <library>
    Contrib: {{ authors }}
*/
//! # {{ project-name }}
//!
//! {{ description }}
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "{{ crate_name }}"]

#[cfg(feature = "alloc")]
extern crate alloc;


pub mod prelude {}
