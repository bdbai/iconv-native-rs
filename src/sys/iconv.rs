use alloc::{string::String, vec::Vec};

use crate::{ConvertError, ConvertLossyError};

pub mod ffi;

pub fn convert(
    input: &[u8],
    from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertError> {
    let mut iconv = ffi::Iconv::new(from_encoding, to_encoding)?;
    iconv.convert(input)
}

pub fn convert_lossy(
    input: &[u8],
    from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertLossyError> {
    let mut iconv = ffi::LossyIconv::new(from_encoding, to_encoding)?;
    let buf = iconv.convert(input);
    Ok(buf)
}

pub fn decode(input: &[u8], encoding: &str) -> Result<String, ConvertError> {
    let mut iconv = ffi::Iconv::new(encoding, "UTF-8")?;
    let buf = iconv.convert(input)?;
    unsafe { Ok(String::from_utf8_unchecked(buf)) }
}

pub fn decode_lossy(input: &[u8], encoding: &str) -> Result<String, ConvertLossyError> {
    let mut iconv = ffi::LossyIconv::new(encoding, "UTF-8")?;
    let buf = iconv.convert(input);
    unsafe { Ok(String::from_utf8_unchecked(buf)) }
}
