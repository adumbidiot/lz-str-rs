use lz_string::{
    decompress_str,
    string_to_u32_array,
};

// These are actually valid...
/*
#[test]
fn decompress_red() {
    let arr = string_to_u32_array("red123");
    assert!(dbg!(decompress_str(&arr)).is_err());
}

*/

#[test]
fn decompress_bed123() {
    let arr = string_to_u32_array("bed123");
    assert!(decompress_str(&arr).is_err());
}
