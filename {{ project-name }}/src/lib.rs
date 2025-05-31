//! # {{ project-name }}
//!
//! {{ description }}
#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::error::{Error, Result};

pub mod error;

pub mod prelude {
    pub use crate::error::{Error, Result};
}
