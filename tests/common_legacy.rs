use lz_string::{
    compress_str,
    compress_uri,
    decompress_str,
    decompress_uri,
    u32_array_to_string,
};

const RED_STR: &'static str = "red";

#[test]
pub fn round_red_uri() {
    let compressed = compress_uri(&RED_STR);
    let compressed_str = unsafe { u32_array_to_string(&compressed) };
    assert_eq!(&compressed_str, "E4UwJkA");
    let decompressed = decompress_uri(&compressed).unwrap();
    assert_eq!(RED_STR, decompressed);
}

#[test]
pub fn round_red() {
    let compressed = compress_str(&RED_STR);
    let _compressed_str = unsafe { u32_array_to_string(&compressed) };
    let decompressed = decompress_str(&compressed).unwrap();
    assert_eq!(RED_STR, decompressed);
}

#[test]
pub fn compress_red() {
    let compressed = compress_str(&RED_STR);
    let compressed_str = unsafe { u32_array_to_string(&compressed) };
    assert_eq!("ᎅ〦䀀", compressed_str);
}

#[test]
pub fn compress_repeat() {
    let data = "aaaaabaaaaacaaaaadaaaaaeaaaaa";
    let compressed = compress_uri(&data);
    let compressed_str = unsafe { u32_array_to_string(&compressed) };
    assert_eq!(&compressed_str, "IYkI1EGNOATWBTWQ");
}

#[test]
pub fn decompress_red() {
    let compressed = "ᎅ〦䀀".chars().map(|c| c as u32).collect::<Vec<_>>();
    let decompressed = decompress_str(&compressed).unwrap();
    assert_eq!(RED_STR, decompressed);
}
