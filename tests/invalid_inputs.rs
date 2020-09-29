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
fn invalid_decompress() {
    let invalid_data = &["bed123", "zed123", "ed[[[[d1d[[[[dF9]"];

    for data in invalid_data {
        let arr = string_to_u32_array(data);
        assert!(decompress_str(&arr).is_err());
    }
}
