use alloc::{string::String, vec::Vec};

use utf32::string_to_utf32_lossy;
use web_sys::wasm_bindgen::{self, prelude::*, JsCast};
use web_sys::{js_sys, TextDecoder, TextDecoderOptions};

mod utf16;
mod utf32;

use crate::encoding::{
    is_encoding_byte_order_ambiguous, match_encoding_parts_exact, trim_encoding_prefix,
};
use crate::wide::{
    try_decode_utf16_lossy, try_decode_utf32_lossy, ByteOrderMark, ByteOrderMarkExt,
};
use crate::ConvertLossyError;
use utf16::{adjust_utf16_params, string_to_utf16_lossy};

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends =  js_sys :: Object , js_name = TextDecoder , typescript_type = "TextDecoder")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type TextDecoderImmutable;
    # [wasm_bindgen (catch , method , structural , js_class = "TextDecoder" , js_name = decode)]
    pub fn decode_with_u8_array(
        this: &TextDecoderImmutable,
        input: &[u8],
    ) -> Result<String, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "TextDecoder" , js_name = decode)]
    pub fn decode_raw_with_u8_array(
        this: &TextDecoderImmutable,
        input: &[u8],
    ) -> Result<JsValue, JsValue>;

    # [wasm_bindgen (extends = js_sys :: Object , js_name = TextDecoderOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type TextDecoderOptionsIgnoreBOM;
    #[wasm_bindgen(method, getter = "ignoreBOM")]
    pub fn get_ignoreBOM(this: &TextDecoderOptionsIgnoreBOM) -> Option<bool>;
    #[wasm_bindgen(method, setter = "ignoreBOM")]
    pub fn set_ignoreBOM(this: &TextDecoderOptionsIgnoreBOM, val: bool);

    # [wasm_bindgen (extends = js_sys :: Object , js_name = TextEncoder , typescript_type = "TextEncoder")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type TextEncoderNonStandard;
    #[wasm_bindgen(catch, constructor, js_class = "TextEncoder")]
    pub fn new_with_label(
        label: &str,
        options: js_sys::Object,
    ) -> Result<TextEncoderNonStandard, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "TextEncoder" , js_name = encode)]
    pub fn encode_with_raw_input(this: &TextEncoderNonStandard, input: JsValue) -> Vec<u8>;
    #[wasm_bindgen(method, getter = "encoding")]
    pub fn get_encoding(this: &TextEncoderNonStandard) -> Option<String>;
}

pub fn convert_lossy(
    input: impl AsRef<[u8]>,
    mut from_encoding: &str,
    to_encoding: &str,
) -> Result<Vec<u8>, ConvertLossyError> {
    #[cfg(not(feature = "wasm_nonstandard_allow_legacy_encoding"))]
    if crate::encoding::trim_encoding_prefix(to_encoding, "utf").is_none() {
        return Err(ConvertLossyError::UnknownToEncoding);
    }

    if from_encoding.eq_ignore_ascii_case(to_encoding)
        || (match_encoding_parts_exact(from_encoding, &["utf", "8"])
            && match_encoding_parts_exact(to_encoding, &["utf", "8"]))
    {
        return Ok(input.as_ref().to_vec());
    }

    let from_ambiguous = is_encoding_byte_order_ambiguous(from_encoding);
    let to_ambiguous = is_encoding_byte_order_ambiguous(to_encoding);
    let mut input = input.as_ref();
    adjust_utf16_params(&mut from_encoding, &mut input);

    let decoded = if let Some(str) =
        try_decode_utf32_lossy(input, from_encoding, to_ambiguous || !from_ambiguous)
    {
        JsValue::from_str(&str)
    } else {
        let options = TextDecoderOptions::new();
        {
            options
                .unchecked_ref::<TextDecoderOptionsIgnoreBOM>()
                .set_ignoreBOM(true);
        }
        let decoder = TextDecoder::new_with_label_and_options(from_encoding, &options)
            .map_err(|_| ConvertLossyError::UnknownFromEncoding)?;
        let decoder = TextDecoderImmutable::unchecked_from_js(decoder.into());
        decoder
            .decode_raw_with_u8_array(input)
            .expect("TextDecoder.decode returned an error without fatal being set")
    };

    if let Some(encoding) = trim_encoding_prefix(to_encoding, "utf") {
        let decoded = decoded.as_string().unwrap_or_default();
        let is_le = !(to_encoding.ends_with("be") || to_encoding.ends_with("BE"));
        let add_bom = to_ambiguous || decoded.as_bytes().get_utf8_bom().is_present();
        if encoding.starts_with("32") {
            if is_le {
                Ok(string_to_utf32_lossy(decoded, add_bom, u32::to_le_bytes))
            } else {
                Ok(string_to_utf32_lossy(decoded, add_bom, u32::to_be_bytes))
            }
        } else if encoding.starts_with("16") {
            if is_le {
                Ok(string_to_utf16_lossy(decoded, add_bom, u16::to_le_bytes))
            } else {
                Ok(string_to_utf16_lossy(decoded, add_bom, u16::to_be_bytes))
            }
        } else if encoding == "8" {
            let mut res = decoded.into_bytes();
            match res.get_utf8_bom() {
                ByteOrderMark::NotPresent if add_bom => {
                    res.splice(0..0, [0xEF, 0xBB, 0xBF]);
                }
                bom if bom.is_present() && !add_bom => {
                    res.drain(0..3);
                }
                _ => {}
            }
            Ok(res)
        } else {
            Err(ConvertLossyError::UnknownToEncoding)
        }
    } else {
        let options = js_sys::Object::new();
        #[cfg(feature = "wasm_nonstandard_allow_legacy_encoding")]
        {
            Reflect::set(
                &options,
                &"NONSTANDARD_allowLegacyEncoding".into(),
                &true.into(),
            )
            .expect("failed to set NONSTANDARD_allowLegacyEncoding");
        }
        let encoder = TextEncoderNonStandard::new_with_label(to_encoding, options)
            .map_err(|_| ConvertLossyError::UnknownToEncoding)?;
        #[cfg(feature = "wasm_nonstandard_allow_legacy_encoding")]
        {
            if !match_encoding_parts_exact(to_encoding, &["utf", "8"])
                && encoder.get_encoding().as_deref() == Some("utf-8")
            {
                // Maybe using a non-polyfilled TextEncoder
                return Err(ConvertLossyError::UnknownToEncoding);
            }
        }
        Ok(encoder.encode_with_raw_input(decoded))
    }
}

pub fn decode_lossy(input: impl AsRef<[u8]>, encoding: &str) -> Result<String, ConvertLossyError> {
    if let Some(str) = try_decode_utf16_lossy(input.as_ref(), encoding)
        .or_else(|| try_decode_utf32_lossy(input.as_ref(), encoding, false))
    {
        return Ok(str);
    }

    let input = input.as_ref();
    let decoder = TextDecoder::new_with_label(encoding)
        .map_err(|_| ConvertLossyError::UnknownFromEncoding)?;
    let decoder = TextDecoderImmutable::unchecked_from_js(decoder.into());
    let str = decoder
        .decode_with_u8_array(input)
        .expect("TextDecoder.decode returned an error without fatal being set");
    Ok(str)
}
