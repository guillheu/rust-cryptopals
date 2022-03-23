pub mod set1;
#[cfg(test)]
mod tests;

extern crate thiserror;

use thiserror::Error;

#[derive(Error, Debug)]
#[error("{encoding} : {message}")]
struct FormatError{
    encoding: String,
    message: String,
}
