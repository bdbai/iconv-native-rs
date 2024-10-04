use alloc::string::String;
use widestring::{decode_utf16_lossy, decode_utf32_lossy};

use crate::encoding::match_encoding_parts;

pub(crate) fn try_decode_utf32_lossy(
    input: impl AsRef<[u8]>,
    encoding: &str,
    mut keep_bom: bool,
) -> Option<String> {
    let mut is_le = true;
    let mut input = input.as_ref();
    if let Some(encoding) = match_encoding_parts(encoding, &["utf", "32"]) {
        match input.get_utf32_bom() {
            ByteOrderMark::Le => {
                is_le = true;
                input = &input[4..];
            }
            ByteOrderMark::Be => {
                is_le = false;
                input = &input[4..];
            }
            ByteOrderMark::NotPresent => {
                keep_bom = false;
                if encoding.eq_ignore_ascii_case("be") {
                    is_le = false;
                }
            }
        }
    } else {
        return None;
    }
    let mut input_iter = input.chunks_exact(4);
    let output_bom = keep_bom.then_some('\u{feff}').into_iter();
    let mut res = if is_le {
        output_bom
            .chain(decode_utf32_lossy(input_iter.by_ref().map(|x| {
                u32::from_le_bytes(unsafe { x.try_into().unwrap_unchecked() })
            })))
            .collect::<String>()
    } else {
        output_bom
            .chain(decode_utf32_lossy(input_iter.by_ref().map(|x| {
                u32::from_be_bytes(unsafe { x.try_into().unwrap_unchecked() })
            })))
            .collect::<String>()
    };
    if !input_iter.remainder().is_empty() {
        res.push('\u{FFFD}');
    }
    Some(res)
}

pub(crate) fn try_decode_utf16_lossy(input: impl AsRef<[u8]>, encoding: &str) -> Option<String> {
    let mut is_le = true;
    let mut input = input.as_ref();
    if let Some(encoding) = match_encoding_parts(encoding, &["utf", "16"]) {
        match input.get_utf16_bom() {
            ByteOrderMark::Le => {
                is_le = true;
                input = &input[2..];
            }
            ByteOrderMark::Be => {
                is_le = false;
                input = &input[2..];
            }
            ByteOrderMark::NotPresent => {
                if encoding.eq_ignore_ascii_case("be") {
                    is_le = false;
                }
            }
        }
    } else {
        return None;
    }
    let mut input_iter = input.chunks_exact(2);
    let mut res = if is_le {
        decode_utf16_lossy(
            input_iter
                .by_ref()
                .map(|x| u16::from_le_bytes(unsafe { x.try_into().unwrap_unchecked() })),
        )
        .collect::<String>()
    } else {
        decode_utf16_lossy(
            input_iter
                .by_ref()
                .map(|x| u16::from_be_bytes(unsafe { x.try_into().unwrap_unchecked() })),
        )
        .collect::<String>()
    };
    if !input_iter.remainder().is_empty() {
        res.push('\u{FFFD}');
    }
    Some(res)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ByteOrderMark {
    Le,
    Be,
    NotPresent,
}

impl ByteOrderMark {
    #[allow(dead_code)]
    pub(crate) fn is_present(&self) -> bool {
        *self != ByteOrderMark::NotPresent
    }
}

#[allow(dead_code)]
pub(crate) trait ByteOrderMarkExt {
    fn get_utf8_bom(&self) -> ByteOrderMark;
    fn get_utf16_bom(&self) -> ByteOrderMark;
    fn get_utf32_bom(&self) -> ByteOrderMark;
}

impl ByteOrderMarkExt for [u8] {
    fn get_utf8_bom(&self) -> ByteOrderMark {
        if self.get(0..3) == Some(&[0xEF, 0xBB, 0xBF]) {
            ByteOrderMark::Le
        } else {
            ByteOrderMark::NotPresent
        }
    }

    fn get_utf16_bom(&self) -> ByteOrderMark {
        if self.get(0..2) == Some(&[0xFF, 0xFE]) {
            ByteOrderMark::Le
        } else if self.get(0..2) == Some(&[0xFE, 0xFF]) {
            ByteOrderMark::Be
        } else {
            ByteOrderMark::NotPresent
        }
    }

    fn get_utf32_bom(&self) -> ByteOrderMark {
        if self.get(0..4) == Some(&[0xFF, 0xFE, 0, 0]) {
            ByteOrderMark::Le
        } else if self.get(0..4) == Some(&[0, 0, 0xFE, 0xFF]) {
            ByteOrderMark::Be
        } else {
            ByteOrderMark::NotPresent
        }
    }
}
