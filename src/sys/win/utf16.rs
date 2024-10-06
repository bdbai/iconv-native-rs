use alloc::vec::Vec;

pub(super) fn utf16_to_wide_lossy(
    input: &[u8],
    mut bytes_to_num: impl FnMut([u8; 2]) -> u16,
) -> Vec<u16> {
    input
        .chunks_exact(2)
        .map(|x| bytes_to_num([x[0], x[1]]))
        .collect()
}
