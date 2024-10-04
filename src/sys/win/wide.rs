use core::ptr::null_mut;

use alloc::{vec, vec::Vec};

use super::codepage::{
    encoding_to_codepage, CODEPAGE_UTF16, CODEPAGE_UTF16BE, CODEPAGE_UTF32, CODEPAGE_UTF32BE,
    CODEPAGE_UTF8,
};
use widestring::{decode_utf16_lossy, decode_utf32_lossy, encode_utf32, U16String};
use windows_sys::Win32::Globalization::{MultiByteToWideChar, WideCharToMultiByte};

use crate::{encoding::is_encoding_byte_order_ambiguous, ConvertLossyError};

fn utf32_to_wide_lossy(
    input: impl AsRef<[u8]>,
    mut bytes_to_num: impl FnMut([u8; 4]) -> u32,
) -> Vec<u16> {
    let input = input.as_ref();
    let mut input_iter = input.chunks_exact(4);
    let iter = decode_utf32_lossy(
        input_iter
            .by_ref()
            .map(|x| bytes_to_num(unsafe { x.try_into().unwrap_unchecked() })),
    );
    let mut res = U16String::from_iter(iter).into_vec();
    let mut shim = [0; 4];
    for (i, x) in input_iter.remainder().iter().enumerate() {
        shim[i] = *x;
    }
    if shim != [0; 4] {
        res.extend_from_slice(
            U16String::from_iter(decode_utf32_lossy(core::iter::once(bytes_to_num(shim))))
                .as_slice(),
        );
    }
    res
}

fn wide_to_utf32_lossy(
    input: impl AsRef<[u16]>,
    bytes_to_num: impl FnMut(u32) -> [u8; 4],
) -> Vec<u8> {
    let iter = decode_utf16_lossy(input.as_ref().iter().copied());
    let mut res = Vec::with_capacity(input.as_ref().len() * 2);
    res.extend(encode_utf32(iter).flat_map(bytes_to_num));
    res
}

fn decode_wide_lossy(
    input: impl AsRef<[u8]>,
    codepage: u32,
    preserve_bom: &mut bool,
) -> Result<Vec<u16>, ConvertLossyError> {
    let mut input = input.as_ref();
    if codepage == CODEPAGE_UTF8 {
        if input.get(0..3) == Some(&[0xEF, 0xBB, 0xBF]) {
            *preserve_bom &= true;
            input = &input[3..];
        } else {
            *preserve_bom = false;
        }
    } else if let CODEPAGE_UTF16 | CODEPAGE_UTF16BE = codepage {
        let mut is_le = codepage == CODEPAGE_UTF16;
        if input.get(0..2) == Some(&[0xFF, 0xFE]) {
            *preserve_bom &= true;
            input = &input[2..];
        } else if input.get(0..2) == Some(&[0xFE, 0xFF]) {
            is_le = false;
            *preserve_bom &= true;
            input = &input[2..];
        } else {
            *preserve_bom = false;
        }

        return Ok(if is_le {
            input
                .chunks_exact(2)
                .map(|x| u16::from_le_bytes([x[0], x[1]]))
                .collect()
        } else {
            input
                .chunks_exact(2)
                .map(|x| u16::from_be_bytes([x[0], x[1]]))
                .collect()
        });
    } else if let CODEPAGE_UTF32 | CODEPAGE_UTF32BE = codepage {
        let mut is_le = codepage == CODEPAGE_UTF32;
        if input.get(0..4) == Some(&[0xFF, 0xFE, 0x00, 0x00]) {
            *preserve_bom &= true;
            input = &input[4..];
        } else if input.get(0..4) == Some(&[0x00, 0x00, 0xFE, 0xFF]) {
            is_le = false;
            *preserve_bom &= true;
            input = &input[4..];
        } else {
            *preserve_bom = false;
        }

        return Ok(if is_le {
            utf32_to_wide_lossy(input, u32::from_le_bytes)
        } else {
            utf32_to_wide_lossy(input, u32::from_be_bytes)
        });
    } else {
        *preserve_bom = false;
    }

    if input.is_empty() {
        // If the input is empty, calling MultiByteToWideChar would return an error.
        return Ok(Vec::new());
    }
    let mut output = vec![];
    let input_len = input.len().try_into().unwrap();
    unsafe {
        let size = MultiByteToWideChar(codepage, 0, input.as_ptr(), input_len, null_mut(), 0);
        if size <= 0 {
            return Err(ConvertLossyError::UnknownFromEncoding);
        }
        output.reserve_exact(size as usize);
        let cap = output.capacity().try_into().unwrap();
        let res = MultiByteToWideChar(
            codepage,
            0,
            input.as_ptr(),
            input_len,
            output.as_mut_ptr(),
            cap,
        );
        if res <= 0 {
            panic!("MultiByteToWideChar with buffer failed");
        }
        output.set_len(res as usize);
    }
    Ok(output)
}

fn encode_wide(
    input: impl AsRef<[u16]>,
    codepage: u32,
    add_bom: bool,
) -> Result<Vec<u8>, ConvertLossyError> {
    let input = input.as_ref();
    if let CODEPAGE_UTF16 | CODEPAGE_UTF16BE = codepage {
        let is_le = codepage == CODEPAGE_UTF16;
        let mut output = Vec::with_capacity(input.len() * 2 + if add_bom { 2 } else { 0 });
        if is_le {
            if add_bom {
                output.extend_from_slice(&[0xFF, 0xFE]);
            }
            output.extend(input.iter().flat_map(|x| x.to_le_bytes()));
        } else {
            if add_bom {
                output.extend_from_slice(&[0xFE, 0xFF]);
            }
            output.extend(input.iter().flat_map(|x| x.to_be_bytes()));
        };
        return Ok(output);
    }
    if let CODEPAGE_UTF32 | CODEPAGE_UTF32BE = codepage {
        let is_le = codepage == CODEPAGE_UTF32;
        let mut output = Vec::with_capacity(input.len() * 4 + if add_bom { 4 } else { 0 });
        if is_le {
            if add_bom {
                output.extend_from_slice(&[0xFF, 0xFE, 0x00, 0x00]);
            }
            output.extend(wide_to_utf32_lossy(input, u32::to_le_bytes));
        } else {
            if add_bom {
                output.extend_from_slice(&[0x00, 0x00, 0xFE, 0xFF]);
            }
            output.extend(wide_to_utf32_lossy(input, u32::to_be_bytes));
        };
        return Ok(output);
    }

    let mut output = vec![];
    if codepage == CODEPAGE_UTF8 && add_bom {
        output.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
    }
    if input.is_empty() {
        // If the input is empty, calling WideCharToMultiByte would return an error.
        return Ok(Vec::new());
    }
    let input_len = input.len().try_into().unwrap();
    unsafe {
        let size = WideCharToMultiByte(
            codepage,
            0,
            input.as_ptr(),
            input_len,
            null_mut(),
            0,
            null_mut(),
            null_mut(),
        );
        if size <= 0 {
            return Err(ConvertLossyError::UnknownToEncoding);
        }
        output.reserve_exact(size as usize);
        let cap = output.capacity().try_into().unwrap();
        let res = WideCharToMultiByte(
            codepage,
            0,
            input.as_ptr(),
            input_len,
            output.spare_capacity_mut().as_mut_ptr() as _,
            cap,
            null_mut(),
            null_mut(),
        );
        if res <= 0 {
            panic!("WideCharToMultiByte with buffer failed");
        }
        output.set_len(output.len() + res as usize);
    }
    Ok(output)
}

pub fn convert_lossy(
    input: impl AsRef<[u8]>,
    from_encoding: &str,
    to_encoding: &str,
    ignore_bom: bool,
) -> Result<Vec<u8>, ConvertLossyError> {
    let from_codepage =
        encoding_to_codepage(from_encoding).ok_or(ConvertLossyError::UnknownFromEncoding)?;
    let to_codepage =
        encoding_to_codepage(to_encoding).ok_or(ConvertLossyError::UnknownToEncoding)?;
    if from_codepage == to_codepage {
        return Ok(input.as_ref().to_vec());
    }
    let mut preserve_bom = !is_encoding_byte_order_ambiguous(from_encoding);
    let wide = decode_wide_lossy(input, from_codepage, &mut preserve_bom)?;
    let add_bom = preserve_bom || is_encoding_byte_order_ambiguous(to_encoding);
    encode_wide(wide, to_codepage, add_bom && !ignore_bom)
}
