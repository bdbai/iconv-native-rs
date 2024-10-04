use alloc::{string::String, vec::Vec};

mod codepage;
mod wide;

use crate::ConvertLossyError;

pub fn convert_lossy(
    input: impl AsRef<[u8]>,
    from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertLossyError> {
    wide::convert_lossy(input, from_encoding, to_encoding, false)
}

pub fn decode_lossy(input: impl AsRef<[u8]>, encoding: &str) -> Result<String, ConvertLossyError> {
    let buf = wide::convert_lossy(input, encoding, "utf-8", true)?;
    unsafe { Ok(String::from_utf8_unchecked(buf)) }
}
