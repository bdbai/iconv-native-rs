use alloc::vec::Vec;
use widestring::decode_utf16;

use crate::ConvertError;

pub(super) fn utf16_to_wide_lossy(
    input: &[u8],
    mut bytes_to_num: impl FnMut([u8; 2]) -> u16,
) -> Vec<u16> {
    let mut input_iter = input.chunks_exact(2);
    let mut res: Vec<u16> = input_iter
        .by_ref()
        .map(|x| bytes_to_num([x[0], x[1]]))
        .collect();
    if !input_iter.remainder().is_empty() {
        res.push(0xFFFD);
    }
    res
}

pub(super) fn utf16_to_wide(
    input: &[u8],
    mut bytes_to_num: impl FnMut([u8; 2]) -> u16,
) -> Result<Vec<u16>, ConvertError> {
    let input_iter = input.chunks_exact(2);
    if !input_iter.remainder().is_empty() {
        return Err(ConvertError::InvalidInput);
    }
    let res: Vec<u16> = input_iter.map(|x| bytes_to_num([x[0], x[1]])).collect();
    decode_utf16(res.iter().copied())
        .try_for_each(|x| x.map(|_| ()))
        .map_err(|_| ConvertError::InvalidInput)?;
    Ok(res)
}
