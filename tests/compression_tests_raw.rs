use lz_string::{
    compress_str,
    decompress_str,
    u32_array_to_string,
};
use rand::prelude::*;
use std::io::Write;

fn round(data: &str) {
    let compressed = compress_str(&data);
    let compressed_str = unsafe { u32_array_to_string(&compressed) };
    assert_ne!(data, compressed_str);
    let decompressed = decompress_str(&compressed).unwrap();
    assert_eq!(data, decompressed);
}

#[test]
fn round_hello_world() {
    round("Hello world!");
}

#[test]
fn round_empty_string() {
    let compressed = compress_str("");
    let compressed_str = unsafe { u32_array_to_string(&compressed) };
    assert_ne!("", compressed_str);

    let decompressed = decompress_str(&compressed).unwrap();
    assert_eq!("", decompressed);

    /*
         it('compresses and decompresses an empty string', function() {
        var compressed = compress('');
        if (uint8array_mode===false){
            expect(compressed).not.toBe('');
            expect(typeof compressed).toBe(typeof '');
        } else {    //uint8array
            expect(compressed instanceof Uint8Array).toBe(true);
            expect(compressed.length).not.toBe(0);  //not an empty array when compress
        }
        var decompressed = decompress(compressed);
        expect(decompressed).toBe('');
    });
    */
}

#[test]
fn round_all_utf16() {
    let mut test_string = String::new();

    for i in 32..127 {
        test_string.push(std::char::from_u32(i).unwrap());
    }

    for i in (128 + 32)..55_296 {
        test_string.push(std::char::from_u32(i).unwrap());
    }

    for i in 63_744..65_536 {
        test_string.push(std::char::from_u32(i).unwrap());
    }

    round(&test_string);
}

#[test]
fn round_repeating_string() {
    let test_string = "aaaaabaaaaacaaaaadaaaaaeaaaaa";
    let compressed = compress_str(&test_string);
    let compressed_str = unsafe { u32_array_to_string(&compressed) };
    assert_ne!(test_string, compressed_str);
    assert!(test_string.len() > compressed_str.len());
    let decompressed = decompress_str(&compressed).unwrap();
    assert_eq!(test_string, decompressed);
}

#[test]
fn round_long_string() {
    let mut rng = rand::thread_rng();
    let test_string = String::from_utf8((0..1000).map(|_| rng.gen::<f32>()).fold(
        Vec::new(),
        |mut vec, n| {
            write!(vec, "{} ", n).unwrap();
            vec
        },
    ))
    .unwrap();

    let compressed = compress_str(&test_string);
    let compressed_str = unsafe { u32_array_to_string(&compressed) };
    assert_ne!(test_string, compressed_str);
    assert!(test_string.len() > compressed.len());
    let decompressed = decompress_str(&compressed).unwrap();
    assert_eq!(test_string, decompressed);
}
