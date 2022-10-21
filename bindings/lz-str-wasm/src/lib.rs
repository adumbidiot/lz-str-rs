use js_sys::JsString;
use js_sys::Uint16Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(module = "/src/inline.js")]
extern "C" {
    #[wasm_bindgen(js_name = "convertUint16ArrayToString")]
    fn convert_uint16_array_to_string(array: &Uint16Array) -> JsString;
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
    let array: Uint16Array = compressed.as_slice().into();
    convert_uint16_array_to_string(&array).into()
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
        .map(|decompressed| {
            let array: Uint16Array = decompressed.as_slice().into();
            convert_uint16_array_to_string(&array).into()
        })
        .unwrap_or(JsValue::NULL)
}
