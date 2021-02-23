use crate::IntoWideIter;
use js_sys::JsString;
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

/// Decompress a [`JsString`].
///
#[wasm_bindgen]
pub fn decompress(data: &JsValue) -> JsValue {
    let data: &JsString = data.dyn_ref::<JsString>().expect("Valid JsString");
    let data: Vec<u16> = data.iter().collect();
    crate::decompress(&data)
        .map(|s| JsString::from_char_code(&s))
        .map(Into::into)
        .unwrap_or(JsValue::NULL)
}
