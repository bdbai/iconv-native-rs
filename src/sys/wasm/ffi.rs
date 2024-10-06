use alloc::{string::String, vec::Vec};

use web_sys::js_sys;
use web_sys::wasm_bindgen::{self, prelude::*};

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
    # [wasm_bindgen (method , structural, catch , js_class = "TextEncoder" , js_name = encode)]
    pub fn encode_with_raw_input(
        this: &TextEncoderNonStandard,
        input: JsValue,
    ) -> Result<Vec<u8>, JsValue>;
    #[wasm_bindgen(method, getter = "encoding")]
    pub fn get_encoding(this: &TextEncoderNonStandard) -> Option<String>;
}
