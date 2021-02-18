use lz_str::decompress;

#[test]
fn valid_decompress() {
    let arr: Vec<u16> = "red123".encode_utf16().collect();
    assert_eq!(decompress(&arr).expect("Valid Decompress"), "\u{80}\u{80}");
}
