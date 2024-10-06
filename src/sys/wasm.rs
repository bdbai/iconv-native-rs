use core::str::FromStr;

use alloc::{string::String, vec::Vec};

use web_sys::wasm_bindgen::JsCast;
use web_sys::{TextDecoder, TextDecoderOptions};

mod convert;
mod ffi;
mod utf16;
mod utf32;

use crate::utf::{decode_utf, decode_utf_lossy, UtfEncoding};
use crate::{ConvertError, ConvertLossyError};
use ffi::*;

pub fn convert(
    input: &[u8],
    from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertError> {
    convert::convert_inner(input, from_encoding, to_encoding, false)
}

pub fn convert_lossy(
    input: &[u8],
    from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertLossyError> {
    match convert::convert_inner(input, from_encoding, to_encoding, true) {
        Ok(v) => Ok(v),
        Err(ConvertError::UnknownConversion) => Err(ConvertLossyError::UnknownConversion),
        Err(ConvertError::InvalidInput) => {
            panic!("TextDecoder.decode returned an error without fatal being set")
        }
    }
}

pub fn decode(input: &[u8], encoding: &str) -> Result<String, ConvertError> {
    if let Ok(utf) = UtfEncoding::from_str(encoding) {
        return decode_utf(input, utf);
    }

    let option = TextDecoderOptions::new();
    option.set_fatal(true);
    let decoder = TextDecoder::new_with_label_and_options(encoding, &option)
        .map_err(|_| ConvertError::UnknownConversion)?;
    let decoder = TextDecoderImmutable::unchecked_from_js(decoder.into());
    let str = decoder
        .decode_with_u8_array(input)
        .map_err(|_| ConvertError::InvalidInput)?;
    Ok(str)
}

pub fn decode_lossy(input: &[u8], encoding: &str) -> Result<String, ConvertLossyError> {
    if let Ok(utf) = UtfEncoding::from_str(encoding) {
        return Ok(decode_utf_lossy(input, utf));
    }

    let decoder =
        TextDecoder::new_with_label(encoding).map_err(|_| ConvertLossyError::UnknownConversion)?;
    let decoder = TextDecoderImmutable::unchecked_from_js(decoder.into());
    let str = decoder
        .decode_with_u8_array(input)
        .expect("TextDecoder.decode returned an error without fatal being set");
    Ok(str)
}
