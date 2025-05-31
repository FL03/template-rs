/*
    appellation: error <module>
    authors: @FL03
*/
#[cfg(feature = "alloc")]
use alloc::string::String;

/// a type alias for [`Result`] that uses the custom [`Error`] type
pub type Result<T = ()> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An error occurred")]
    SomeError,
    #[cfg(feature = "anyhow")]
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(#[from] alloc::boxed::Box<dyn core::error::Error + Send + Sync>),
    #[cfg(feature = "json")]
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[cfg(feature = "std")]
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[cfg(feature = "alloc")]
    #[error("Unknown error: {0}")]
    UnknownError(String),
}

#[cfg(feature = "alloc")]
impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::UnknownError(value)
    }
}

#[cfg(feature = "alloc")]
impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::UnknownError(String::from(value))
    }
}