use js_sys::JsString;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "convertU16SliceToString")]
    fn convert_u16_slice_to_string(slice: &[u16]) -> JsString;
}

/// Compress a [`JsString`].
#[wasm_bindgen]
pub fn compress(data: &JsValue) -> JsValue {
    let data: &JsString = match data.dyn_ref::<JsString>() {
        Some(data) => data,
        None => {
            return JsValue::NULL;
        }
    };
    let data: Vec<u16> = data.iter().collect();
    let compressed = lz_str::compress(&data);
    convert_u16_slice_to_string(&compressed).into()
}

/// Decompress a [`JsString`].
#[wasm_bindgen]
pub fn decompress(data: &JsValue) -> JsValue {
    let data: &JsString = match data.dyn_ref::<JsString>() {
        Some(data) => data,
        None => {
            return JsValue::NULL;
        }
    };
    let data: Vec<u16> = data.iter().collect();
    lz_str::decompress(&data)
        .map(|decompressed| convert_u16_slice_to_string(&decompressed).into())
        .unwrap_or(JsValue::NULL)
}
