#![forbid(unsafe_code)]

mod compress;
mod constants;
mod decompress;
#[cfg(feature = "wasm-bindgen-support")]
pub mod wasm_bindgen_support;

pub use crate::{
    compress::{
        compress,
        compress_internal,
        compress_to_base64,
        compress_to_encoded_uri_component,
        compress_to_uint8_array,
        compress_to_utf16,
    },
    decompress::{
        decompress,
        decompress_from_base64,
        decompress_from_encoded_uri_component,
        decompress_from_uint8_array,
        decompress_from_utf16,
        decompress_internal,
    },
};

/// A utility function for converting a [`&str`] into a [`Vec`] of [`u32`]s,
/// where each [`u32`] is a valid [`char`].
///
pub fn str_to_u32_vec(buf: &str) -> Vec<u32> {
    buf.chars().map(u32::from).collect()
}
