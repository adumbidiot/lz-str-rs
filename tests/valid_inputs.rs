use lz_str::{
    decompress,
    str_to_u32_vec,
};

#[test]
fn decompress_red() {
    let arr = str_to_u32_vec("red123");
    assert_eq!(decompress(&arr).expect("Valid Decompress"), "\u{80}\u{80}");
}
