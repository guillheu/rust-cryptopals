#![doc = include_str!("../README.md")]

pub mod set1;

extern crate thiserror;

use thiserror::Error;

/// This newtype will make it easier to handle bytes for cryptographic operations.
/// A newtype is preferable to an alias, to avoid having bytes-specific methods apply too freely with other Vec<u8>-based data types, and vice-versa.
/// e.g. a method for Bytes will not work for instance on a `type Ascii = Vec<u8>`, which is a good thing : )
struct Bytes(Vec<u8>);

#[derive(Error, Debug)]
#[error("{encoding} : {message}")]
/// Generic error structure for all formatting errors
struct FormattingError{
    encoding: String,
    message: String,
}
