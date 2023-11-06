use thiserror::Error;

pub mod module;
pub mod windows;

#[derive(Error, Debug)]
pub enum PlatformError {
    #[error("{0}")]
    WindowsError(#[from] ::windows::core::Error),
}

pub type Result<T> = std::result::Result<T, PlatformError>;
