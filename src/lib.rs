#![forbid(unsafe_code)]

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

pub fn str_to_u32_vec(buf: &str) -> Vec<u32> {
    buf.chars().map(u32::from).collect()
}
