use core::str::FromStr;

use alloc::{string::String, vec::Vec};

use crate::utf::UtfEncoding;
use crate::{ConvertError, ConvertLossyError};

pub mod ffi;

/// Computes the initial output buffer capacity for a conversion.
///
/// Conversions from ambiguous UTF-16/UTF-32 rely on the platform `iconv` to
/// detect the byte order from the BOM, which is only consulted on the first
/// `iconv()` call. Size the buffer to fit the whole output so the conversion
/// completes in a single call and never depends on byte-order state surviving
/// across resumed calls.
fn initial_capacity(input_len: usize, from_encoding: &str) -> usize {
    if UtfEncoding::from_str(from_encoding).is_ok_and(|utf| utf.is_ambiguous()) {
        // A UTF-16/32 unit expands to at most 4 bytes, plus room for a BOM.
        input_len.saturating_mul(4).saturating_add(32)
    } else {
        input_len + 4
    }
}

pub fn convert(
    input: &[u8],
    from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertError> {
    let capacity = initial_capacity(input.len(), from_encoding);
    let mut iconv = ffi::Iconv::new(from_encoding, to_encoding)?;
    iconv.convert(input, capacity)
}

pub fn convert_lossy(
    input: &[u8],
    from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertLossyError> {
    let capacity = initial_capacity(input.len(), from_encoding);
    let mut iconv = ffi::LossyIconv::new(from_encoding, to_encoding)?;
    let buf = iconv.convert(input, capacity);
    Ok(buf)
}

pub fn decode(input: &[u8], encoding: &str) -> Result<String, ConvertError> {
    let capacity = initial_capacity(input.len(), encoding);
    let mut iconv = ffi::Iconv::new(encoding, "UTF-8")?;
    let buf = iconv.convert(input, capacity)?;
    // Safety: relies on the correctness of iconv implementation
    unsafe { Ok(String::from_utf8_unchecked(buf)) }
}

pub fn decode_lossy(input: &[u8], encoding: &str) -> Result<String, ConvertLossyError> {
    let capacity = initial_capacity(input.len(), encoding);
    let mut iconv = ffi::LossyIconv::new(encoding, "UTF-8")?;
    let buf = iconv.convert(input, capacity);
    // Safety: relies on the correctness of iconv implementation
    unsafe { Ok(String::from_utf8_unchecked(buf)) }
}
