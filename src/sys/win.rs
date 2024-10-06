use core::str::FromStr;

use alloc::{string::String, vec::Vec};

mod codepage;
mod utf16;
mod utf32;
mod wide;

use crate::utf::{decode_utf_lossy, UtfEncoding};
use crate::ConvertLossyError;

pub fn convert_lossy(
    input: impl AsRef<[u8]>,
    from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertLossyError> {
    wide::convert_lossy(input, from_encoding, to_encoding)
}

pub fn decode_lossy(input: &[u8], encoding: &str) -> Result<String, ConvertLossyError> {
    if let Ok(utf) = UtfEncoding::from_str(encoding) {
        return Ok(decode_utf_lossy(input, utf));
    };

    let buf = wide::convert_lossy(input, encoding, "utf-8")?;
    unsafe { Ok(String::from_utf8_unchecked(buf)) }
}
