use lz_string::{compress_str, compress_uri, decompress_str, decompress_uri, str_to_u32_vec, compress_to_utf16};

const RED_STR: &str = "red";

#[test]
pub fn round_red_uri() {
    let compressed = compress_uri(&RED_STR);
    assert_eq!(&compressed, &str_to_u32_vec("E4UwJkA"));
    let decompressed = decompress_uri(&compressed).unwrap();
    assert_eq!(RED_STR, decompressed);
}

#[test]
pub fn round_red() {
    let compressed = compress_str(&RED_STR);
    let decompressed = decompress_str(&compressed).unwrap();
    assert_eq!(RED_STR, decompressed);
}

#[test]
pub fn compress_red() {
    let compressed = compress_str(&RED_STR);
    assert_eq!(str_to_u32_vec("ᎅ〦䀀"), compressed);
}

#[test]
pub fn compress_red_to_utf16() {
    let compressed = compress_to_utf16(&RED_STR);
    assert_eq!("\u{9e2}䰩䠠".to_string(), compressed);
}

#[test]
pub fn compress_repeat() {
    let data = "aaaaabaaaaacaaaaadaaaaaeaaaaa";
    let compressed = compress_uri(&data);
    assert_eq!(&compressed, &str_to_u32_vec("IYkI1EGNOATWBTWQ"));
}

#[test]
pub fn decompress_red() {
    let compressed = "ᎅ〦䀀".chars().map(u32::from).collect::<Vec<_>>();
    let decompressed = decompress_str(&compressed).unwrap();
    assert_eq!(RED_STR, decompressed);
}
