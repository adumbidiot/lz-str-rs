extern crate lz_string;

use lz_string::{
    compress,
    compress_uri,
    decompress,
    decompress_uri,
};

const DECOMPRESSED_URI_DATA: &'static str = r#"{"userGuid":"d8e58de4-f9c6-4f20-83e0-61a1fc038ca3","outlineFile":"pcpro2018v6.xml","scoringPath":"pp6\\sims\\typescriptv1\\pcpro2018v6\\packages\\pcpro2018v6windowspackage\\l_wl_profile1_pp6.js","actualScore":0,"possibleScore":2,"secondsInResource":37,"details":"0¯2¯37.379¯0-+0¯NaN¯1"}"#;

const COMPRESSED_URI_DATA: &'static str = "N4IgrgzgpgTg4mAlgExALhMgHFArF5KAFgFoAzATgGMA2EosgJgAYSsBmKVmgRgEMeZKs3ZYqfdiAA0IAPZgALgBtEAOygAxREqjoQAByr6YsljywA3GgDoAHgFsl0kBCqyYagOYAFPgoAWevr6NAA6oRCI9hDhCgCe+lCuHvoKFjzhhsamzOZWmXxUANZ8nkmZRiZmljQA7mrIsrUQ+oUlZeFKAPq13dlk2lA8XcE2AFYQzoUKYHxKAMpuMLpozDL6shCRAEY6i+4rjDLQbqrIEACSqgBKSfIwVCvsAOwyhAp82pMYzAD1jL8XtYXhRfqwANR-AByfChvx4IAAvkA";

const RED_STR: &'static str = "red";
//#[test]
//pub fn common() {}

#[test]
pub fn round_uri_data() {
    let compressed = compress_uri(&DECOMPRESSED_URI_DATA).unwrap();
    assert_eq!(&compressed, &COMPRESSED_URI_DATA);
    let decompressed = decompress_uri(&compressed).unwrap();
    assert_eq!(&decompressed, &DECOMPRESSED_URI_DATA);
}

#[test]
pub fn round_red_uri() {
    let compressed = compress_uri(&RED_STR).unwrap();
    assert_eq!(&compressed, "E4UwJkA");
    let decompressed = decompress_uri(&compressed).unwrap();
    assert_eq!(RED_STR, decompressed);
}

#[test]
pub fn round_red() {
    let compressed = compress(&RED_STR, 16, std::char::from_u32);
    let decompressed = decompress(&compressed, 32_768).unwrap();
    assert_eq!(RED_STR, decompressed);
}

#[test]
pub fn compress_red() {
    let compressed = compress(&RED_STR, 16, std::char::from_u32);
    assert_eq!("ᎅ〦䀀", compressed);
}

#[test]
pub fn compress_repeat() {
    let data = "aaaaabaaaaacaaaaadaaaaaeaaaaa";
    let compressed = compress_uri(&data).unwrap();
    assert_eq!(&compressed, "IYkI1EGNOATWBTWQ");
}

#[test]
pub fn decompress_red() {
    let compressed = "ᎅ〦䀀";
    let decompressed = decompress(&compressed, 32_768).unwrap();
    assert_eq!(RED_STR, decompressed);
}

#[test]
fn decompress_uri_data() {
    let decompressed = decompress_uri(&COMPRESSED_URI_DATA).unwrap();
    assert_eq!(DECOMPRESSED_URI_DATA, decompressed);
}
