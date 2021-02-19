use lz_str::decompress;

#[test]
fn valid_decompress() {
    let valid_data: &[(&str, Vec<u16>)] =
        &[("red123", vec![0x80, 0x80]), ("腆퍂蚂荂", vec![0xD8A0])];
    for (data, expected) in valid_data {
        let arr: Vec<u16> = data.encode_utf16().collect();
        let decompressed = decompress(&arr).expect("Valid Decompress");
        assert_eq!(&decompressed, expected);
    }
}
