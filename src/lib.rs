pub mod compress;
pub mod constants;
pub mod decompress;

#[cfg(target_arch = "wasm32")]
mod wasm;

pub use compress::{
    compress,
    compress_str,
    compress_uri,
};
pub use decompress::{
    decompress,
    decompress_str,
    decompress_uri,
};

pub unsafe fn u32_array_to_string(buf: &[u32]) -> String {
    buf.iter()
        .map(|n| std::char::from_u32_unchecked(*n))
        .collect()
}

pub fn string_to_u32_array(buf: &str) -> Vec<u32> {
    buf.chars().map(|c| u32::from(c)).collect()
}
