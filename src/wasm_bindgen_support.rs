use js_sys::JsString;
use wasm_bindgen::prelude::*;

/// Compress a [`JsString`].
///
#[wasm_bindgen]
pub fn compress(data: JsString) -> Result<JsString, JsValue> {
    let data: Vec<u16> = data.iter().collect();
    let compressed = crate::compress(&data);
    Ok(JsString::from_char_code(&compressed))
}

/// Decompress a [`JsString`].
///
#[wasm_bindgen]
pub fn decompress(data: JsString) -> JsValue {
    // Returning a String crashes?
    let data: Vec<u16> = data.iter().collect();
    crate::decompress(&data)
        .map(|s| JsString::from_char_code(&s))
        .map(Into::into)
        .unwrap_or(JsValue::NULL)
}
