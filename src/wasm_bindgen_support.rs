use crate::IntoWideIter;
use js_sys::JsString;
use wasm_bindgen::{
    prelude::*,
    JsCast,
};

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
}

impl<'a> IntoWideIter for &'a JsString {
    type Iter = JsStringIter<'a>;

    fn into_wide_iter(self) -> Self::Iter {
        JsStringIter::new(self)
    }
}

/// Compress a [`JsString`].
///
#[wasm_bindgen]
pub fn compress(data: &JsValue) -> JsString {
    let data: &JsString = data.dyn_ref::<JsString>().expect("Valid JsString");
    let data: Vec<u16> = data.iter().collect();
    let compressed = crate::compress(&data);
    JsString::from_char_code(&compressed)
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
