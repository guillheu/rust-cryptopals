#![doc = include_str!("../README.md")]

pub mod set1;

extern crate thiserror;

use thiserror::Error;

/// This alias will make it easier to handle bytes for cryptographic operations.
type Bytes = Vec<u8>;

#[derive(Error, Debug)]
#[error("{encoding} : {message}")]
/// Generic error structure for all formatting errors
struct FormattingError{
    encoding: String,
    message: String,
}
