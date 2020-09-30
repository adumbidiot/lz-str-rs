use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compress(data: &str) -> Result<JsString, JsValue> {
    // TODO: Lossless conversion
    // let data: Vec<u32> = data.iter().map(u32::from).collect();

    let compressed = crate::compress_str(&data);

    JsString::from_code_point(&compressed)
}

#[wasm_bindgen]
pub fn decompress(data: JsString) -> Result<JsString, JsValue> {
    // Returning a String crashes?
    let data: Vec<u32> = data.iter().map(u32::from).collect();
    let compressed = crate::decompress_str(&data).map_err(|e| e.to_string())?;

    // TODO: Lossless conversion
    // JsString::from_code_point(&compressed)

    Ok(JsString::from(compressed))
}
