use alloc::borrow::Cow;

use alloc::{string::String, vec::Vec};

use crate::bom::ByteOrderMarkExt;
use crate::utf::UtfEncoding;
use crate::{ConvertError, ConvertLossyError};

pub mod ffi;

/// Resolve an ambiguous UTF-16/UTF-32 source encoding before calling `iconv`.
///
/// Some platform `iconv` implementations only consult the BOM on the first
/// `iconv()` call and forget the detected byte order when the conversion is
/// resumed after `E2BIG`. Strip a present BOM here and rewrite the source
/// encoding to an explicit LE/BE variant, so resumed calls stay correct.
/// Without a BOM the original encoding is kept and the platform default byte
/// order still applies.
fn explicit_source<'a>(input: &'a [u8], from_encoding: &'a str) -> (Cow<'a, str>, &'a [u8]) {
    if let Ok(utf) = from_encoding.parse::<UtfEncoding>() {
        if utf.is_utf16() && utf.is_ambiguous() {
            match input.get_utf16_bom() {
                crate::bom::ByteOrderMark::Le => {
                    return (Cow::Borrowed("UTF-16LE"), &input[2..]);
                }
                crate::bom::ByteOrderMark::Be => {
                    return (Cow::Borrowed("UTF-16BE"), &input[2..]);
                }
                _ => {}
            }
        } else if utf.is_utf32() && utf.is_ambiguous() {
            match input.get_utf32_bom() {
                crate::bom::ByteOrderMark::Le => {
                    return (Cow::Borrowed("UTF-32LE"), &input[4..]);
                }
                crate::bom::ByteOrderMark::Be => {
                    return (Cow::Borrowed("UTF-32BE"), &input[4..]);
                }
                _ => {}
            }
        }
        return (Cow::Borrowed(from_encoding), input);
    }
    (Cow::Borrowed(from_encoding), input)
}

pub fn convert(
    input: &[u8],
    from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertError> {
    let (from_encoding, input) = explicit_source(input, from_encoding);
    let mut iconv = ffi::Iconv::new(&from_encoding, to_encoding)?;
    iconv.convert(input)
}

pub fn convert_lossy(
    input: &[u8],
    from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertLossyError> {
    let (from_encoding, input) = explicit_source(input, from_encoding);
    let mut iconv = ffi::LossyIconv::new(&from_encoding, to_encoding)?;
    let buf = iconv.convert(input);
    Ok(buf)
}

pub fn decode(input: &[u8], encoding: &str) -> Result<String, ConvertError> {
    let (encoding, input) = explicit_source(input, encoding);
    let mut iconv = ffi::Iconv::new(&encoding, "UTF-8")?;
    let buf = iconv.convert(input)?;
    // Safety: relies on the correctness of iconv implementation
    unsafe { Ok(String::from_utf8_unchecked(buf)) }
}

pub fn decode_lossy(input: &[u8], encoding: &str) -> Result<String, ConvertLossyError> {
    let (encoding, input) = explicit_source(input, encoding);
    let mut iconv = ffi::LossyIconv::new(&encoding, "UTF-8")?;
    let buf = iconv.convert(input);
    // Safety: relies on the correctness of iconv implementation
    unsafe { Ok(String::from_utf8_unchecked(buf)) }
}
