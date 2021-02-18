use lz_str::decompress;

#[test]
fn invalid_decompress() {
    let invalid_data = &["bed123", "zed123", "ed[[[[d1d[[[[dF9]", "腆퍂蚂荂"];

    for data in invalid_data {
        let arr: Vec<u16> = data.encode_utf16().collect();
        assert!(decompress(&arr).is_none());
    }
}
