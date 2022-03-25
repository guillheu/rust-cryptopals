#![doc = include_str!("../README.md")]

pub mod set1;

extern crate thiserror;

use thiserror::Error;
use std::ops::Index;
use std::slice;
use std::fmt;

type Byte = u8;

/// This newtype will make it easier to handle bytes for cryptographic operations.
/// A newtype is preferable to an alias, to avoid having bytes-specific methods apply too freely with other Vec<u8>-based data types, and vice-versa.
/// e.g. a method for Bytes will not work for instance on a `type Ascii = Vec<u8>`, which is a good thing : )
pub struct Bytes(Vec<Byte>);

pub enum EncodingType {
    Base64,
    Hexadecimal,
}

#[derive(Error, Debug)]
#[error("{encoding} : {message}")]
/// Generic error structure for all encoding errors
pub struct EncodingError{
    encoding: EncodingType,
    message: String,
}

/// This is for Bytes to be indexable
impl Index<usize> for Bytes {
    type Output = Byte;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl EncodingType {
    pub fn to_string(&self) -> String {
        match self {
            EncodingType::Base64          => "base 64".to_string(),
            EncodingType::Hexadecimal     => "hexadecimal".to_string()
        }
    }
}

impl fmt::Display for EncodingType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for EncodingType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Probably a lot (if not all) of these could be replaced with trait implementation of Vec methods... I'll get to that eventually
impl Bytes {
    pub fn new() -> Bytes {
        Bytes (Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Bytes {
        Bytes (Vec::with_capacity(capacity))
    }

    pub fn push(&mut self, b: Byte) {
        self.0.push(b);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    //This is not good XD gonna have to fix that iterator thing later... in a few weeks
    pub fn iter(&self) -> slice::Iter<Byte> {
        self.0.iter()
    }
}