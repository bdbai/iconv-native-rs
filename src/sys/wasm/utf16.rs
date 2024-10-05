use alloc::{string::String, vec::Vec};
use widestring::encode_utf16;

use crate::bom::ByteOrderMarkExt;
use crate::encoding::match_encoding_parts_exact;

pub(super) fn adjust_utf16_params(encoding: &mut &str, input: &mut &[u8]) {
    if !match_encoding_parts_exact(encoding, &["utf", "16"]) {
        return;
    }
    if input.get(0..2) == Some(&[0xFF, 0xFE]) {
        *input = &input[2..];
        *encoding = "utf-16le";
    } else if input.get(0..2) == Some(&[0xFE, 0xFF]) {
        *input = &input[2..];
        *encoding = "utf-16be";
    }
}

pub(super) fn string_to_utf16_lossy(
    input: String,
    add_bom: bool,
    mut bytes_to_num: impl FnMut(u16) -> [u8; 2],
) -> Vec<u8> {
    let mut res = Vec::new();
    if add_bom && !input.as_bytes().get_utf8_bom().is_present() {
        res.extend_from_slice(&bytes_to_num(0xFEFF));
    }
    res.extend(encode_utf16(input.chars()).flat_map(bytes_to_num));
    res
}
