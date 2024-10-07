use alloc::string::{String, ToString};

use widestring::{decode_utf16, decode_utf32};

use crate::ConvertError;

use super::{UtfEncoding, UtfType};

fn decode_utf16_inner(
    input: &[u8],
    mut bytes_to_num: impl FnMut([u8; 2]) -> u16,
) -> Result<String, ConvertError> {
    let input_iter = input.chunks_exact(2);
    if !input_iter.remainder().is_empty() {
        return Err(ConvertError::InvalidInput);
    }
    let res = decode_utf16(input_iter.map(|x| bytes_to_num(x.try_into().unwrap())))
        .collect::<Result<String, _>>()
        .map_err(|_| ConvertError::InvalidInput)?;
    Ok(res)
}

fn decode_utf32_inner(
    input: &[u8],
    mut bytes_to_num: impl FnMut([u8; 4]) -> u32,
) -> Result<String, ConvertError> {
    let input_iter = input.chunks_exact(4);
    if !input_iter.remainder().is_empty() {
        return Err(ConvertError::InvalidInput);
    }
    let res = decode_utf32(input_iter.map(|x| bytes_to_num(x.try_into().unwrap())))
        .collect::<Result<String, _>>()
        .map_err(|_| ConvertError::InvalidInput)?;
    Ok(res)
}

pub(crate) fn decode_utf(mut input: &[u8], encoding: UtfEncoding) -> Result<String, ConvertError> {
    let byte_order = encoding.consume_input_bom(&mut input);
    let is_le = byte_order.is_le(true);
    match (encoding.r#type, is_le) {
        (UtfType::Utf16, true) => decode_utf16_inner(input, u16::from_le_bytes),
        (UtfType::Utf16, false) => decode_utf16_inner(input, u16::from_be_bytes),
        (UtfType::Utf32, true) => decode_utf32_inner(input, u32::from_le_bytes),
        (UtfType::Utf32, false) => decode_utf32_inner(input, u32::from_be_bytes),
        (UtfType::Utf8, _) => match alloc::str::from_utf8(input) {
            Ok(s) => Ok(s.to_string()),
            Err(_) => Err(ConvertError::InvalidInput),
        },
    }
}
