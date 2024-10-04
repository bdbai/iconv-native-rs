use alloc::{string::String, vec::Vec};

use widestring::encode_utf32;

use crate::wide::ByteOrderMarkExt;

pub(super) fn string_to_utf32_lossy(
    input: String,
    add_bom: bool,
    mut bytes_to_num: impl FnMut(u32) -> [u8; 4],
) -> Vec<u8> {
    let mut res = Vec::new();
    if add_bom && !input.as_bytes().get_utf8_bom().is_present() {
        res.extend_from_slice(&bytes_to_num(0xFEFF));
    }
    res.extend(encode_utf32(input.chars()).flat_map(bytes_to_num));
    res
}
