use alloc::{string::String, vec::Vec};

use widestring::encode_utf16;

pub(super) fn string_to_utf16(
    input: String,
    add_bom: bool,
    mut bytes_to_num: impl FnMut(u16) -> [u8; 2],
) -> Vec<u8> {
    let mut res = Vec::new();
    if add_bom {
        res.extend_from_slice(&bytes_to_num(0xFEFF));
    }
    res.extend(encode_utf16(input.chars()).flat_map(bytes_to_num));
    res
}
