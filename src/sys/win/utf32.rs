use alloc::vec::Vec;

use widestring::{
    decode_utf16, decode_utf16_lossy, decode_utf32, decode_utf32_lossy, encode_utf32, U16String,
};

use crate::ConvertError;

pub(super) fn utf32_to_wide_lossy(
    input: &[u8],
    mut bytes_to_num: impl FnMut([u8; 4]) -> u32,
) -> Vec<u16> {
    let mut input_iter = input.chunks_exact(4);
    let iter = decode_utf32_lossy(
        input_iter
            .by_ref()
            .map(|x| bytes_to_num(x.try_into().unwrap())),
    );
    let mut res = U16String::from_iter(iter).into_vec();
    if !input_iter.remainder().is_empty() {
        res.push(0xFFFD);
    }
    res
}

pub(super) fn utf32_to_wide(
    input: &[u8],
    mut bytes_to_num: impl FnMut([u8; 4]) -> u32,
) -> Result<Vec<u16>, ConvertError> {
    let input_iter = input.chunks_exact(4);
    if !input_iter.remainder().is_empty() {
        return Err(ConvertError::InvalidInput);
    }
    let res = decode_utf32(input_iter.map(|x| bytes_to_num(x.try_into().unwrap())))
        .collect::<Result<U16String, _>>()
        .map_err(|_| ConvertError::InvalidInput)?;
    Ok(res.into_vec())
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

pub(super) fn wide_to_utf32(
    input: &[u16],
    bytes_to_num: impl FnMut(u32) -> [u8; 4],
) -> Result<Vec<u8>, ConvertError> {
    decode_utf16(input.iter().copied())
        .try_for_each(|x| x.map(|_| ()))
        .map_err(|_| ConvertError::InvalidInput)?;
    let mut res = Vec::with_capacity(input.len() * 2);
    res.extend(
        encode_utf32(decode_utf16(input.iter().copied()).map(|c|
            // Safety: `input` is immutable, plus we just went through a decoding over it,
            // and `decode_utf16` should be pure.
            unsafe { c.unwrap_unchecked() }))
        .flat_map(bytes_to_num),
    );
    Ok(res)
}
