#![cfg_attr(
    any(
        all(windows, feature = "win32"),
        all(target_arch = "wasm32", feature = "web-encoding"),
    ),
    no_std
)]

extern crate alloc;

mod bom;
mod encoding;
mod error;
mod sys;
mod utf;

use core::str::FromStr;

pub use error::{ConvertError, ConvertLossyError};

use alloc::{string::String, vec::Vec};

pub fn convert_lossy(
    input: impl AsRef<[u8]>,
    from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertLossyError> {
    sys::convert_lossy(input.as_ref(), from_encoding, to_encoding)
}

pub fn decode_lossy(input: impl AsRef<[u8]>, encoding: &str) -> Result<String, ConvertLossyError> {
    let mut input = input.as_ref();
    if let Ok(utf) = utf::UtfEncoding::from_str(encoding) {
        utf.strip_bom(&mut input);
    };
    sys::decode_lossy(input, encoding)
}
