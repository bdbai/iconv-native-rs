#![no_std]

extern crate alloc;

#[cfg(any(windows, feature = "win32"))]
mod encoding;
mod error;
mod sys;

pub use error::ConvertLossyError;

use alloc::{string::String, vec::Vec};

pub fn convert_lossy(
    input: impl AsRef<[u8]>,
    from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertLossyError> {
    sys::convert_lossy(input, from_encoding, to_encoding)
}

pub fn decode_lossy(input: impl AsRef<[u8]>, encoding: &str) -> Result<String, ConvertLossyError> {
    sys::decode_lossy(input, encoding)
}
