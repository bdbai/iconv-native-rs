use alloc::string::String;

use widestring::{decode_utf16_lossy, decode_utf32_lossy};

use super::{UtfEncoding, UtfType};

fn decode_utf16_lossy_inner(input: &[u8], mut bytes_to_num: impl FnMut([u8; 2]) -> u16) -> String {
    let mut input_iter = input.chunks_exact(2);
    let mut res = decode_utf16_lossy(
        input_iter
            .by_ref()
            .map(|x| bytes_to_num(x.try_into().unwrap())),
    )
    .collect::<String>();
    if !input_iter.remainder().is_empty() {
        res.push('\u{FFFD}');
    }
    res
}

fn decode_utf32_lossy_inner(input: &[u8], mut bytes_to_num: impl FnMut([u8; 4]) -> u32) -> String {
    let mut input_iter = input.chunks_exact(4);
    let mut res = decode_utf32_lossy(
        input_iter
            .by_ref()
            .map(|x| bytes_to_num(x.try_into().unwrap())),
    )
    .collect::<String>();
    if !input_iter.remainder().is_empty() {
        res.push('\u{FFFD}');
    }
    res
}

pub(crate) fn decode_utf_lossy(mut input: &[u8], encoding: UtfEncoding) -> String {
    let byte_order = encoding.consume_input_bom(&mut input);
    let is_le = byte_order.is_le(true);
    match (encoding.r#type, is_le) {
        (UtfType::Utf16, true) => decode_utf16_lossy_inner(input, u16::from_le_bytes),
        (UtfType::Utf16, false) => decode_utf16_lossy_inner(input, u16::from_be_bytes),
        (UtfType::Utf32, true) => decode_utf32_lossy_inner(input, u32::from_le_bytes),
        (UtfType::Utf32, false) => decode_utf32_lossy_inner(input, u32::from_be_bytes),
        (UtfType::Utf8, _) => String::from_utf8_lossy(input).into_owned(),
    }
}
