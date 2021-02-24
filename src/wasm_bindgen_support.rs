use crate::IntoWideIter;
use js_sys::{
    JsString,
    Uint8Array,
};
use wasm_bindgen::{
    prelude::*,
    JsCast,
};

const JS_STRING_CHUNK_SIZE: usize = 4096;

/// An iterator over chars in a [`JsString`].
///
pub struct JsStringIter<'a> {
    string: &'a JsString,
    current: u32,
    length: u32,
}

impl<'a> JsStringIter<'a> {
    /// Create an iterator over chars in a [`JsString`].
    ///
    pub fn new(string: &'a JsString) -> Self {
        JsStringIter {
            string,
            current: 0,
            length: string.length(),
        }
    }
}

impl<'a> Iterator for JsStringIter<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.length {
            return None;
        }

        let val = self.string.char_code_at(self.current) as u16;
        self.current += 1;

        Some(val)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.length as usize, Some(self.length as usize))
    }
}

impl<'a> IntoWideIter for &'a JsString {
    type Iter = JsStringIter<'a>;

    fn into_wide_iter(self) -> Self::Iter {
        JsStringIter::new(self)
    }
}

/// Compress a [`JsString`].
///
/// Returns an empty string if the input is null or was not a [`JsString`].
///
#[wasm_bindgen]
pub fn compress(data: &JsValue) -> JsString {
    let mut ret = JsString::from_char_code(&[]);

    let data: &JsString = match data.dyn_ref::<JsString>() {
        Some(data) => data,
        None => return ret,
    };

    let compressed = crate::compress(data);

    // Chunk the return to avoid overflowing the stack space
    for chunk in compressed.chunks(JS_STRING_CHUNK_SIZE) {
        ret = ret.concat(&JsString::from_char_code(chunk));
    }

    ret
}

/// Compress a [`JsString`] to a uri component.
///
/// Returns an empty string if the input is null or was not a [`JsString`].
///
#[wasm_bindgen(js_name = "compressToEncodedURIComponent")]
pub fn compress_to_encoded_uri_component(data: &JsValue) -> String {
    let data: &JsString = match data.dyn_ref::<JsString>() {
        Some(data) => data,
        None => return String::new(),
    };

    crate::compress_to_encoded_uri_component(data)
}

/// Compress a [`JsString`] to UTF16.
///
/// Returns an empty string if the input is null or was not a [`JsString`].
///
#[wasm_bindgen(js_name = "compressToUTF16")]
pub fn compress_to_utf16(data: &JsValue) -> String {
    let data: &JsString = match data.dyn_ref::<JsString>() {
        Some(data) => data,
        None => return String::new(),
    };

    crate::compress_to_utf16(data)
}

/// Compress a [`JsString`] to Base64.
///
/// Returns an empty string if the input is null or was not a [`JsString`].
///
#[wasm_bindgen(js_name = "compressToBase64")]
pub fn compress_to_base64(data: &JsValue) -> String {
    let data: &JsString = match data.dyn_ref::<JsString>() {
        Some(data) => data,
        None => return String::new(),
    };

    crate::compress_to_base64(data)
}

/// Compress a [`JsString`] to a Uint8Array.
///
/// Returns an empty string if the input is null or was not a [`JsString`].
///
#[wasm_bindgen(js_name = "compressToUint8Array")]
pub fn compress_to_uint8_array(data: &JsValue) -> Uint8Array {
    let data: &JsString = match data.dyn_ref::<JsString>() {
        Some(data) => data,
        None => return Uint8Array::new_with_length(0),
    };

    Uint8Array::from(crate::compress_to_uint8_array(data).as_slice())
}

/// Decompress a [`JsString`].
///
/// Returns an empty string if the input is null or was not a [`JsString`].
///
/// # Errors
/// Returns [`None`]/null if decompression failed.
///
#[wasm_bindgen]
pub fn decompress(data: &JsValue) -> Option<JsString> {
    let data: &JsString = match data.dyn_ref::<JsString>() {
        Some(data) => data,
        None => return Some(JsString::from_char_code(&[])),
    };

    if data.length() == 0 {
        return None;
    }

    let decompressed = crate::decompress(data)?;

    Some(JsString::from_char_code(&decompressed))
}
