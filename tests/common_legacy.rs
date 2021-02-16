use lz_str::{
    compress_str,
    compress_to_utf16,
    compress_uri,
    decompress_from_utf16,
    decompress_str,
    decompress_uri,
    str_to_u32_vec,
};

const TEST_STR: &str = "The quick brown fox jumps over the lazy dog";
const TEST_STR_COMPRESSED: &[u32] = &[
    2688, 45222, 64, 36362, 57494, 1584, 13700, 1120, 9987, 55325, 49270, 4108, 54016, 15392, 2758,
    364, 112, 6594, 19459, 29469, 2049, 30466, 108, 1072, 3008, 10116, 38, 38915, 39168,
];

#[test]
pub fn round_test_uri() {
    let compressed = compress_uri(&TEST_STR);
    assert_eq!(
        &compressed,
        &str_to_u32_vec(
            "CoCwpgBAjgrglgYwNYQEYCcD2B3AdhAM0wA8IArGAWwAcBnCTANzHQgBdwIAbAQwC8AnhAAmmAOZA"
        )
    );
    let decompressed = decompress_uri(&compressed).expect("`round_test_uri` valid decompress");
    assert_eq!(TEST_STR, decompressed);
}

#[test]
pub fn round_red() {
    let compressed = compress_str(&TEST_STR);
    let decompressed = decompress_str(&compressed).unwrap();
    assert_eq!(TEST_STR, decompressed);
}

#[test]
pub fn compress_test() {
    let compressed = compress_str(&TEST_STR);
    assert_eq!(TEST_STR_COMPRESSED, compressed);
}

#[test]
pub fn compress_test_to_utf16() {
    let compressed = compress_to_utf16(&TEST_STR);
    assert_eq!("ՠⱉ䀨ऀ圤堸悋Ф〳䄖Ϙށ䰠硠૦Ö<͘ⓠ᮸瑀̎Ƞ㘢ఢ砤硠Ŕ怮㈠", compressed);
}

#[test]
pub fn decompress_test_to_utf16() {
    let decompressed = decompress_from_utf16("ՠⱉ䀨ऀ圤堸悋Ф〳䄖Ϙށ䰠硠૦Ö<͘ⓠ᮸瑀̎Ƞ㘢ఢ砤硠Ŕ怮㈠ ")
        .expect("Valid Decompress");
    assert_eq!(TEST_STR, decompressed);
}

#[test]
pub fn compress_repeat() {
    let data = "aaaaabaaaaacaaaaadaaaaaeaaaaa";
    let compressed = compress_uri(&data);
    assert_eq!(&compressed, &str_to_u32_vec("IYkI1EGNOATWBTWQ"));
}

#[test]
pub fn decompress_test() {
    let decompressed = decompress_str(TEST_STR_COMPRESSED).unwrap();
    assert_eq!(TEST_STR, decompressed);
}
