use alloc::{string::String, vec::Vec};

mod codepage;
mod utf32;
mod wide;

use crate::wide::{try_decode_utf16_lossy, try_decode_utf32_lossy};
use crate::ConvertLossyError;

pub fn convert_lossy(
    input: impl AsRef<[u8]>,
    from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertLossyError> {
    wide::convert_lossy(input, from_encoding, to_encoding, false)
}

pub fn decode_lossy(input: impl AsRef<[u8]>, encoding: &str) -> Result<String, ConvertLossyError> {
    if let Some(str) = try_decode_utf16_lossy(input.as_ref(), encoding)
        .or_else(|| try_decode_utf32_lossy(input.as_ref(), encoding, false))
    {
        return Ok(str);
    }
    let buf = wide::convert_lossy(input, encoding, "utf-8", true)?;
    unsafe { Ok(String::from_utf8_unchecked(buf)) }
}
