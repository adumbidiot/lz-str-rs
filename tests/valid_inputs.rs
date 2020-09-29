use lz_string::{
    decompress_str,
    string_to_u32_array,
};

#[test]
fn decompress_red() {
    let arr = string_to_u32_array("red123");
    assert_eq!(decompress_str(&arr).unwrap(), "\u{80}\u{80}");
}
