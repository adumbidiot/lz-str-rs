use lz_str::{
    compress,
    compress_to_encoded_uri_component,
    compress_to_utf16,
    decompress,
    decompress_from_encoded_uri_component,
    decompress_from_utf16,
};

const TEST_STR: &str = "The quick brown fox jumps over the lazy dog";
const TEST_STR_COMPRESSED: &[u16] = &[
    2688, 45222, 64, 36362, 57494, 1584, 13700, 1120, 9987, 55325, 49270, 4108, 54016, 15392, 2758,
    364, 112, 6594, 19459, 29469, 2049, 30466, 108, 1072, 3008, 10116, 38, 38915, 39168,
];

#[test]
pub fn round_test_uri() {
    let compressed =
        compress_to_encoded_uri_component(&TEST_STR.encode_utf16().collect::<Vec<u16>>());
    assert_eq!(
        &compressed,
        "CoCwpgBAjgrglgYwNYQEYCcD2B3AdhAM0wA8IArGAWwAcBnCTANzHQgBdwIAbAQwC8AnhAAmmAOZA"
    );
    let decompressed = decompress_from_encoded_uri_component(&compressed)
        .expect("`round_test_uri` valid decompress");
    assert_eq!(TEST_STR.encode_utf16().collect::<Vec<u16>>(), decompressed);
}

#[test]
pub fn round_test() {
    let compressed = compress(&TEST_STR.encode_utf16().collect::<Vec<u16>>());
    let decompressed = decompress(&compressed).unwrap();
    assert_eq!(TEST_STR.encode_utf16().collect::<Vec<u16>>(), decompressed);
}

#[test]
pub fn compress_test() {
    let compressed = compress(&TEST_STR.encode_utf16().collect::<Vec<u16>>());
    assert_eq!(TEST_STR_COMPRESSED, compressed);
}

#[test]
pub fn compress_test_to_utf16() {
    let compressed = compress_to_utf16(&TEST_STR.encode_utf16().collect::<Vec<u16>>());
    assert_eq!("ՠⱉ䀨ऀ圤堸悋Ф〳䄖Ϙށ䰠硠૦Ö<͘ⓠ᮸瑀̎Ƞ㘢ఢ砤硠Ŕ怮㈠ ", compressed);
}

#[test]
pub fn decompress_test_to_utf16() {
    let decompressed = decompress_from_utf16("ՠⱉ䀨ऀ圤堸悋Ф〳䄖Ϙށ䰠硠૦Ö<͘ⓠ᮸瑀̎Ƞ㘢ఢ砤硠Ŕ怮㈠ ")
        .expect("Valid Decompress");
    assert_eq!(TEST_STR.encode_utf16().collect::<Vec<u16>>(), decompressed);
}

#[test]
pub fn compress_repeat() {
    let data = "aaaaabaaaaacaaaaadaaaaaeaaaaa";
    let compressed = compress_to_encoded_uri_component(&data.encode_utf16().collect::<Vec<u16>>());
    assert_eq!(&compressed, "IYkI1EGNOATWBTWQ");
}

#[test]
pub fn decompress_test() {
    let decompressed = decompress(TEST_STR_COMPRESSED).expect("Valid Decompress");
    assert_eq!(TEST_STR.encode_utf16().collect::<Vec<u16>>(), decompressed);
}
