use core::str::FromStr;

use alloc::{string::String, vec::Vec};

mod codepage;
mod utf16;
mod utf32;
mod wide;

use crate::utf::{decode_utf, decode_utf_lossy, UtfEncoding};
use crate::{ConvertError, ConvertLossyError};
use codepage::{encoding_to_codepage, CODEPAGE_UTF8};

pub fn convert(
    input: &[u8],
    from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertError> {
    let from_codepage =
        encoding_to_codepage(from_encoding).ok_or(ConvertLossyError::UnknownConversion)?;
    let to_codepage =
        encoding_to_codepage(to_encoding).ok_or(ConvertLossyError::UnknownConversion)?;
    if from_codepage == to_codepage {
        return Ok(input.to_vec());
    }
    wide::convert(
        input,
        from_encoding,
        to_encoding,
        from_codepage,
        to_codepage,
    )
}

pub fn convert_lossy(
    input: &[u8],
    from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertLossyError> {
    let from_codepage =
        encoding_to_codepage(from_encoding).ok_or(ConvertLossyError::UnknownConversion)?;
    let to_codepage =
        encoding_to_codepage(to_encoding).ok_or(ConvertLossyError::UnknownConversion)?;
    if from_codepage == to_codepage {
        return Ok(input.to_vec());
    }
    wide::convert_lossy(
        input,
        from_encoding,
        to_encoding,
        from_codepage,
        to_codepage,
    )
}

pub fn decode(input: &[u8], encoding: &str) -> Result<String, ConvertError> {
    if let Ok(utf) = UtfEncoding::from_str(encoding) {
        return decode_utf(input, utf);
    };

    let from_codepage =
        encoding_to_codepage(encoding).ok_or(ConvertLossyError::UnknownConversion)?;
    let buf = wide::convert(input, encoding, "utf-8", from_codepage, CODEPAGE_UTF8)?;
    // Safety: UTF-8 related conversions are done by system mb2wc-wc2mb functions.
    unsafe { Ok(String::from_utf8_unchecked(buf)) }
}

pub fn decode_lossy(input: &[u8], encoding: &str) -> Result<String, ConvertLossyError> {
    if let Ok(utf) = UtfEncoding::from_str(encoding) {
        return Ok(decode_utf_lossy(input, utf));
    };

    let from_codepage =
        encoding_to_codepage(encoding).ok_or(ConvertLossyError::UnknownConversion)?;
    let buf = wide::convert_lossy(input, encoding, "utf-8", from_codepage, CODEPAGE_UTF8)?;
    // Safety: UTF-8 related conversions are done by system mb2wc-wc2mb functions.
    unsafe { Ok(String::from_utf8_unchecked(buf)) }
}
