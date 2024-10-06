use alloc::vec::Vec;

use widestring::{decode_utf16_lossy, decode_utf32_lossy, encode_utf32, U16String};

pub(super) fn utf32_to_wide_lossy(
    input: &[u8],
    mut bytes_to_num: impl FnMut([u8; 4]) -> u32,
) -> Vec<u16> {
    let mut input_iter = input.chunks_exact(4);
    let iter = decode_utf32_lossy(
        input_iter
            .by_ref()
            .map(|x| bytes_to_num(unsafe { x.try_into().unwrap_unchecked() })),
    );
    let mut res = U16String::from_iter(iter).into_vec();
    if !input_iter.remainder().is_empty() {
        res.push(0xFFFD);
    }
    res
}

pub(super) fn wide_to_utf32_lossy(
    input: &[u16],
    bytes_to_num: impl FnMut(u32) -> [u8; 4],
) -> Vec<u8> {
    let iter = decode_utf16_lossy(input.iter().copied());
    let mut res = Vec::with_capacity(input.len() * 2);
    res.extend(encode_utf32(iter).flat_map(bytes_to_num));
    res
}
