mod compress;
mod constants;
mod decompress;

pub use crate::{
    compress::{
        compress,
        compress_str,
        compress_uri,
    },
    decompress::{
        decompress,
        decompress_str,
        decompress_uri,
    },
};

pub unsafe fn u32_array_to_string(buf: &[u32]) -> String {
    buf.iter()
        .map(|n| std::char::from_u32_unchecked(*n))
        .collect()
}

pub fn string_to_u32_array(buf: &str) -> Vec<u32> {
    buf.chars().map(|c| u32::from(c)).collect()
}