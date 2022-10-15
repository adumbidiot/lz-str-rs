use js_sys::JsString;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Compress a [`JsString`].
#[wasm_bindgen]
pub fn compress(data: &JsValue) -> JsString {
    let data: &JsString = data.dyn_ref::<JsString>().expect("invalid `JsString`");
    let data: Vec<u16> = data.iter().collect();
    let compressed = lz_str::compress(&data);
    JsString::from_char_code(&compressed)
}

/// Decompress a [`JsString`].
#[wasm_bindgen]
pub fn decompress(data: &JsValue) -> JsValue {
    let data: &JsString = data.dyn_ref::<JsString>().expect("invalid `JsString`");
    let data: Vec<u16> = data.iter().collect();
    lz_str::decompress(&data)
        .map(|s| JsString::from_char_code(&s))
        .map(Into::into)
        .unwrap_or(JsValue::NULL)
}
