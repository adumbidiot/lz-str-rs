use lz_str::{
    decompress,
    str_to_u32_vec,
};

#[test]
fn invalid_decompress() {
    let invalid_data = &["bed123", "zed123", "ed[[[[d1d[[[[dF9]", "腆퍂蚂荂"];

    for data in invalid_data {
        let arr = str_to_u32_vec(data);
        assert!(decompress(&arr).is_none());
    }
}
