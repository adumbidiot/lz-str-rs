use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compress_uri(data: &str) -> String {
    let compressed = crate::compress_uri(data);
    unsafe { crate::u32_array_to_string(&compressed) }
}

#[wasm_bindgen]
pub fn decompress_uri(data: &str) -> Option<String> {
    let arr = crate::string_to_u32_array(data);
    crate::decompress_uri(&arr)
}
