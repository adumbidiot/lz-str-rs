use lz_str::{
    decompress_str,
    str_to_u32_vec,
};

#[test]
fn decompress_red() {
    let arr = str_to_u32_vec("red123");
    assert_eq!(decompress_str(&arr).unwrap(), "\u{80}\u{80}");
}
