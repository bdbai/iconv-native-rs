use core::str::FromStr;

use alloc::{string::String, vec::Vec};

use utf32::string_to_utf32;
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::{js_sys, TextDecoder, TextDecoderOptions};

mod ffi;
mod utf16;
mod utf32;

use crate::utf::{decode_utf_lossy, UtfEncoding, UtfType};
use crate::ConvertLossyError;
use ffi::*;
use utf16::string_to_utf16;

pub fn convert_lossy(
    input: &[u8],
    from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertLossyError> {
    let from_utf = UtfEncoding::from_str(from_encoding).ok();
    let to_utf = UtfEncoding::from_str(to_encoding).ok();

    #[cfg(not(feature = "wasm-nonstandard-allow-legacy-encoding"))]
    if to_utf.is_none() {
        return Err(ConvertLossyError::UnknownConversion);
    }

    if from_encoding.eq_ignore_ascii_case(to_encoding)
        || (from_utf == to_utf && from_utf.map_or(false, |u| u.is_utf8()))
    {
        return Ok(input.to_vec());
    }

    let decoded = if let Some(str) = from_utf
        // TextDecoder tends to remove BOMs. Use widestring to preserve them.
        .map(|u| decode_utf_lossy(input, u))
    {
        JsValue::from_str(&str)
    } else {
        let options = TextDecoderOptions::new();
        {
            options
                .unchecked_ref::<TextDecoderOptionsIgnoreBOM>()
                .set_ignoreBOM(false);
        }
        let decoder = TextDecoder::new_with_label_and_options(from_encoding, &options)
            .map_err(|_| ConvertLossyError::UnknownConversion)?;
        let decoder = TextDecoderImmutable::unchecked_from_js(decoder.into());
        decoder
            .decode_raw_with_u8_array(input)
            .expect("TextDecoder.decode returned an error without fatal being set")
    };

    if let Some(to_utf) = to_utf {
        let decoded = decoded.as_string().unwrap_or_default();
        let add_bom = to_utf.is_ambiguous();
        Ok(match (to_utf.r#type(), to_utf.byte_order().is_le(true)) {
            (UtfType::Utf8, _) => decoded.into_bytes(),
            (UtfType::Utf16, true) => string_to_utf16(decoded, add_bom, u16::to_le_bytes),
            (UtfType::Utf16, false) => string_to_utf16(decoded, add_bom, u16::to_be_bytes),
            (UtfType::Utf32, true) => string_to_utf32(decoded, add_bom, u32::to_le_bytes),
            (UtfType::Utf32, false) => string_to_utf32(decoded, add_bom, u32::to_be_bytes),
        })
    } else {
        let options = js_sys::Object::new();
        #[cfg(feature = "wasm-nonstandard-allow-legacy-encoding")]
        {
            Reflect::set(
                &options,
                &"NONSTANDARD_allowLegacyEncoding".into(),
                &true.into(),
            )
            .expect("failed to set NONSTANDARD_allowLegacyEncoding");
        }
        let encoder = TextEncoderNonStandard::new_with_label(to_encoding, options)
            .map_err(|_| ConvertLossyError::UnknownConversion)?;
        #[cfg(feature = "wasm-nonstandard-allow-legacy-encoding")]
        {
            if !to_utf.map_or(false, |u| u.is_utf8())
                && encoder.get_encoding().as_deref() == Some("utf-8")
            {
                // Maybe using a non-polyfilled TextEncoder
                return Err(ConvertLossyError::UnknownConversion);
            }
        }
        Ok(encoder.encode_with_raw_input(decoded))
    }
}

pub fn decode_lossy(input: &[u8], encoding: &str) -> Result<String, ConvertLossyError> {
    if let Ok(utf) = UtfEncoding::from_str(encoding) {
        return Ok(decode_utf_lossy(input, utf));
    }

    let decoder =
        TextDecoder::new_with_label(encoding).map_err(|_| ConvertLossyError::UnknownConversion)?;
    let decoder = TextDecoderImmutable::unchecked_from_js(decoder.into());
    let str = decoder
        .decode_with_u8_array(input)
        .expect("TextDecoder.decode returned an error without fatal being set");
    Ok(str)
}
