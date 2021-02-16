#![forbid(unsafe_code)]

mod compress;
mod constants;
mod decompress;
#[cfg(feature = "wasm-bindgen-support")]
pub mod wasm_bindgen_support;

pub use crate::{
    compress::{
        compress,
        compress_str,
        compress_to_utf16,
        compress_uri,
    },
    decompress::{
        decompress,
        decompress_from_utf16,
        decompress_str,
        decompress_uri,
    },
};

/// A utility function for converting a [`&str`] into a [`Vec`] of [`u32`]s,
/// where each [`u32`] is a valid [`char`].
///
pub fn str_to_u32_vec(buf: &str) -> Vec<u32> {
    buf.chars().map(u32::from).collect()
}
