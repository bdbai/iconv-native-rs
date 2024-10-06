use alloc::{string::String, vec::Vec};

use crate::ConvertLossyError;

pub mod ffi;

pub fn convert_lossy(
    input: impl AsRef<[u8]>,
    from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertLossyError> {
    let iconv = ffi::LossyIconv::new(from_encoding, to_encoding)?;
    let buf = iconv.convert(input.as_ref());
    Ok(buf)
}

pub fn decode_lossy(input: impl AsRef<[u8]>, encoding: &str) -> Result<String, ConvertLossyError> {
    let iconv = ffi::LossyIconv::new(encoding, "UTF-8")?;
    let buf = iconv.convert(input.as_ref());
    unsafe { Ok(String::from_utf8_unchecked(buf)) }
}
