//! These tests were ported from https://github.com/pieroxy/lz-string/blob/83d7b59ebef47edc4cf0527bc03179b86e064f23/tests/lz-string-spec.js

use rand::Rng;
use std::{
    fmt::Write,
    string::FromUtf16Error,
};

fn compression_tests<C, D>(compress: C, decompress: D, bytearray: bool)
where
    C: Fn(&str) -> ByteString,
    D: Fn(ByteString) -> Option<ByteString>,
{
    eprintln!("Check if it compresses and decompresses  \"Hello world!\"");
    let compressed = compress("Hello world!");
    assert_ne!(compressed, "Hello world!");
    let decompressed = decompress(compressed).expect("Valid Decompress");
    assert_eq!(decompressed, "Hello world!");

    /*
     it('compresses and decompresses null', function() {
        var compressed = compress(null);
        if (uint8array_mode===false){
            expect(compressed).toBe('');
            expect(typeof compressed).toBe(typeof '');
        } else {    //uint8array
            expect(compressed instanceof Uint8Array).toBe(true);
            expect(compressed.length).toBe(0);  //empty array
        }
        var decompressed = decompress(compressed);
        expect(decompressed).toBe(null);
    });
    */

    /*
    it('compresses and decompresses undefined', function() {
        var compressed = compress();
        if (uint8array_mode===false){
            expect(compressed).toBe('');
            expect(typeof compressed).toBe(typeof '');
        } else {    //uint8array
            expect(compressed instanceof Uint8Array).toBe(true);
            expect(compressed.length).toBe(0);  //empty array
        }
        var decompressed = decompress(compressed);
        expect(decompressed).toBe(null);
    });
    */

    /*
    it('decompresses null', function() {
        var decompressed = decompress(null);
        expect(decompressed).toBe('');
    });
    */

    eprintln!("Check if it compresses and decompresses an empty string");
    let compressed = compress("");
    if !bytearray {
        assert_ne!(compressed, "");
        // expect(typeof compressed).toBe(typeof '');
    } else {
        // expect(compressed instanceof Uint8Array).toBe(true);
        assert!(!compressed.is_empty());
    }
    let decompressed = decompress(compressed).expect("Valid Decompress");
    assert_eq!(decompressed, "");

    eprintln!("Check if it compresses and decompresses all printable UTF-16 characters");
    let mut test_string = String::new();
    for i in 32..127 {
        test_string.push(std::char::from_u32(i).expect("Valid Char"));
    }
    for i in (128 + 32)..55296 {
        test_string.push(std::char::from_u32(i).expect("Valid Char"));
    }
    for i in 63744..65536 {
        test_string.push(std::char::from_u32(i).expect("Valid Char"));
    }
    let compressed = compress(&test_string);
    assert_ne!(compressed, test_string.as_str());
    let decompressed = decompress(compressed).expect("Valid Decompress");
    assert_eq!(decompressed, test_string.as_str());

    eprintln!("Check if it compresses and decompresses a string that repeats");
    let test_string = "aaaaabaaaaacaaaaadaaaaaeaaaaa";
    let compressed = compress(test_string);
    assert_ne!(compressed, test_string);
    assert!(compressed.len() < test_string.len());
    let decompressed = decompress(compressed).expect("Valid Decompress");
    assert_eq!(decompressed, test_string);

    eprintln!("Check if it compresses and decompresses a long string");
    let mut test_string = String::new();
    for _ in 0..1000 {
        write!(&mut test_string, "{} ", rand::thread_rng().gen::<f64>())
            .expect("write rand float to string")
    }
    let compressed = compress(&test_string);
    assert_ne!(compressed, test_string.as_str());
    assert!(compressed.len() < test_string.len());
    let decompressed = decompress(compressed).expect("Valid Decompress");
    assert_eq!(decompressed, test_string.as_str());
}

#[test]
fn lz_string_base_64() {
    compression_tests(
        |s| lz_str::compress_to_base64(s).into(),
        |s| {
            lz_str::decompress_from_base64(&s.to_utf8_string().expect("Valid UTF16 String"))
                .map(ByteString::from)
        },
        false,
    );
}

#[test]
fn lz_string_utf_16() {
    compression_tests(
        |s| ByteString::from(lz_str::compress_to_utf16(s)),
        |s| {
            lz_str::decompress_from_utf16(&s.to_utf8_string().expect("Valid UTF16 String"))
                .map(ByteString::from)
        },
        false,
    );
}

#[test]
fn lz_string_uri_encoded() {
    compression_tests(
        |s| lz_str::compress_to_encoded_uri_component(s).into(),
        |s| {
            lz_str::decompress_from_encoded_uri_component(
                &s.to_utf8_string().expect("Valid UTF16 String"),
            )
            .map(ByteString::from)
        },
        false,
    );
}

#[test]
fn lz_string_uint8_array() {
    compression_tests(
        |s| {
            lz_str::compress_to_uint8_array(s)
                .into_iter()
                .map(u16::from)
                .collect::<Vec<u16>>()
                .into()
        },
        |s| {
            lz_str::decompress_from_uint8_array(
                &s.0.into_iter().map(|el| el as u8).collect::<Vec<u8>>(),
            )
            .map(ByteString::from)
        },
        true,
    );
}

#[test]
fn lz_string_raw() {
    compression_tests(
        |s| lz_str::compress(s).into(),
        |s| lz_str::decompress(&s.0).map(ByteString::from),
        false,
    );
}

#[test]
fn specific_url_encoded() {
    eprintln!("check that all chars are URL safe");
    let mut test_string = String::new();
    for _ in 0..1000 {
        write!(&mut test_string, "{} ", rand::thread_rng().gen::<f64>())
            .expect("write rand float to string")
    }
    let compressed = lz_str::compress_to_encoded_uri_component(&test_string);
    assert!(!compressed.contains('='));
    assert!(!compressed.contains('/'));
    let decompressed =
        lz_str::decompress_from_encoded_uri_component(&compressed).expect("Valid Decompress");
    assert_eq!(decompressed, test_string);

    eprintln!("check that + and ' ' are interchangeable in decompression");
    let decompressed = "During tattooing, ink is injected into the skin, initiating an immune response, and cells called \"macrophages\" move into the area and \"eat up\" the ink. The macrophages carry some of the ink to the body\'s lymph nodes, but some that are filled with ink stay put, embedded in the skin. That\'s what makes the tattoo visible under the skin. Dalhousie Uiversity\'s Alec Falkenham is developing a topical cream that works by targeting the macrophages that have remained at the site of the tattoo. New macrophages move in to consume the previously pigment-filled macrophages and then migrate to the lymph nodes, eventually taking all the dye with them. \"When comparing it to laser-based tattoo removal, in which you see the burns, the scarring, the blisters, in this case, we\'ve designed a drug that doesn\'t really have much off-target effect,\" he said. \"We\'re not targeting any of the normal skin cells, so you won\'t see a lot of inflammation. In fact, based on the process that we\'re actually using, we don\'t think there will be any inflammation at all and it would actually be anti-inflammatory.";
    let compressed = "CIVwTglgdg5gBAFwIYIQezdGAaO0DWeAznlAFYCmAxghQCanqIAWFcR 0u0ECEKWOEih4AtqJBQ2YCkQAOaKEQq5hDKhQA2mklSTb6cAESikVMGjnMkMWUbii0ANzbQmCVkJlIhUBkYoUOBA5ew9XKHwAOjgAFU9Tc0trW10kMDAAT3Y0UTY0ADMWCMJ3TwAjNDpMgHISTUzRKzgoKtlccpAEHLyWIPS2AogDBgB3XmZSQiJkbLku3ApRcvo6Q2hi9k4oGPiUOrhR627TfFlN5FQMOCcIIghyzTZJNbBNjmgY4H1mNBB7tgAVQgLjA9wQtRIAEEnlQ4AAxfRnKDWUTEOBrFyaSyCHzoOQQPSaODmQJojxBUZoMD4EjlbLIMC2PiwTaJCxWGznCndawuOAyUzQQxBcLsXj5Ipiy7oNAxAByFFGDjMHJS50c-I2TCoiiIIF6YrkMlufyIDTgBJgeSgCAAtEMRiqkpzUr4GOERKIIDAwCg2GU2A0mpNWmsiIsXLaQPoLchtvBY5tqmxxh5iqIYkYAOqsES6prpQS8RBoOCaJDKMB28qVwwy66C5z6bgiI6EyaZP7sCgBirgJS4MVEPQZLBDiqaO60MGtlh3El13CjCg1fnhn1SBg OhgEDwHkYtCyKA1brebTZPlsCRUSaFAp2xnMuAUAoFagIbD2TxEJAQOgs2zVcZBaNBumfCgWUTKBskKTZWjAUxiQ fMtB0XAiDLLsQEORQzx7NgfGxbp4OgAoK3EARFBiABJEQCjML84FrZQGEUTZjTQDQiBIQ8VxqUCmJjS9gnuWBlzYOh8Ig5gCGKUDxm0FiiNg0gKKQKi A4-plLUPBuipEBNG3GgRItFZfD4O1yMo0x0CyKIgAAA$$";
    let decomp2 =
        lz_str::decompress_from_encoded_uri_component(compressed).expect("Valid Decompress");
    assert_eq!(decompressed, decomp2);
}

fn test_enc_bin_compat<C, I, O>(comp: C, expected_dec: I, expected_comp: O)
where
    C: Fn(I) -> O,
    O: std::fmt::Debug + std::cmp::PartialEq,
{
    let compressed = comp(expected_dec);
    assert_eq!(compressed, expected_comp);
}

fn test_dec_bin_compat<D, I>(dec: D, expected_dec: &str, expected_comp: I)
where
    D: Fn(I) -> Option<String>,
{
    let decompressed = dec(expected_comp).expect("Valid Decompress");
    assert_eq!(decompressed, expected_dec);
}

#[test]
fn binary_encoding_compatibility_tests_optional() {
    eprintln!("base64");
    let base64_decompressed = "During tattooing, ink is injected into the skin, initiating an immune response, and cells called \"macrophages\" move into the area and \"eat up\" the ink. The macrophages carry some of the ink to the body\'s lymph nodes, but some that are filled with ink stay put, embedded in the skin. That\'s what makes the tattoo visible under the skin. Dalhousie Uiversity\'s Alec Falkenham is developing a topical cream that works by targeting the macrophages that have remained at the site of the tattoo. New macrophages move in to consume the previously pigment-filled macrophages and then migrate to the lymph nodes, eventually taking all the dye with them. \"When comparing it to laser-based tattoo removal, in which you see the burns, the scarring, the blisters, in this case, we\'ve designed a drug that doesn\'t really have much off-target effect,\" he said. \"We\'re not targeting any of the normal skin cells, so you won\'t see a lot of inflammation. In fact, based on the process that we\'re actually using, we don\'t think there will be any inflammation at all and it would actually be anti-inflammatory.";
    let base64_compressed = "CIVwTglgdg5gBAFwIYIQezdGAaO0DWeAznlAFYCmAxghQCanqIAWFcR+0u0ECEKWOEih4AtqJBQ2YCkQAOaKEQq5hDKhQA2mklSTb6cAESikVMGjnMkMWUbii0ANzbQmCVkJlIhUBkYoUOBA5ew9XKHwAOjgAFU9Tc0trW10kMDAAT3Y0UTY0ADMWCMJ3TwAjNDpMgHISTUzRKzgoKtlccpAEHLyWIPS2AogDBgB3XmZSQiJkbLku3ApRcvo6Q2hi9k4oGPiUOrhR627TfFlN5FQMOCcIIghyzTZJNbBNjmgY4H1mNBB7tgAVQgLjA9wQtRIAEEnlQ4AAxfRnKDWUTEOBrFyaSyCHzoOQQPSaODmQJojxBUZoMD4EjlbLIMC2PiwTaJCxWGznCndawuOAyUzQQxBcLsXj5Ipiy7oNAxAByFFGDjMHJS50c/I2TCoiiIIF6YrkMlufyIDTgBJgeSgCAAtEMRiqkpzUr4GOERKIIDAwCg2GU2A0mpNWmsiIsXLaQPoLchtvBY5tqmxxh5iqIYkYAOqsES6prpQS8RBoOCaJDKMB28qVwwy66C5z6bgiI6EyaZP7sCgBirgJS4MVEPQZLBDiqaO60MGtlh3El13CjCg1fnhn1SBg+OhgEDwHkYtCyKA1brebTZPlsCRUSaFAp2xnMuAUAoFagIbD2TxEJAQOgs2zVcZBaNBumfCgWUTKBskKTZWjAUxiQ+fMtB0XAiDLLsQEORQzx7NgfGxbp4OgAoK3EARFBiABJEQCjML84FrZQGEUTZjTQDQiBIQ8VxqUCmJjS9gnuWBlzYOh8Ig5gCGKUDxm0FiiNg0gKKQKi+A4/plLUPBuipEBNG3GgRItFZfD4O1yMo0x0CyKIgA";
    test_enc_bin_compat(
        lz_str::compress_to_base64,
        base64_decompressed,
        base64_compressed.to_string(),
    );

    eprintln!("uriEncoding");
    let uri_encoding_decompressed = "During tattooing, ink is injected into the skin, initiating an immune response, and cells called \"macrophages\" move into the area and \"eat up\" the ink. The macrophages carry some of the ink to the body\'s lymph nodes, but some that are filled with ink stay put, embedded in the skin. That\'s what makes the tattoo visible under the skin. Dalhousie Uiversity\'s Alec Falkenham is developing a topical cream that works by targeting the macrophages that have remained at the site of the tattoo. New macrophages move in to consume the previously pigment-filled macrophages and then migrate to the lymph nodes, eventually taking all the dye with them. \"When comparing it to laser-based tattoo removal, in which you see the burns, the scarring, the blisters, in this case, we\'ve designed a drug that doesn\'t really have much off-target effect,\" he said. \"We\'re not targeting any of the normal skin cells, so you won\'t see a lot of inflammation. In fact, based on the process that we\'re actually using, we don\'t think there will be any inflammation at all and it would actually be anti-inflammatory.";
    let uri_encoding_compressed = "CIVwTglgdg5gBAFwIYIQezdGAaO0DWeAznlAFYCmAxghQCanqIAWFcR+0u0ECEKWOEih4AtqJBQ2YCkQAOaKEQq5hDKhQA2mklSTb6cAESikVMGjnMkMWUbii0ANzbQmCVkJlIhUBkYoUOBA5ew9XKHwAOjgAFU9Tc0trW10kMDAAT3Y0UTY0ADMWCMJ3TwAjNDpMgHISTUzRKzgoKtlccpAEHLyWIPS2AogDBgB3XmZSQiJkbLku3ApRcvo6Q2hi9k4oGPiUOrhR627TfFlN5FQMOCcIIghyzTZJNbBNjmgY4H1mNBB7tgAVQgLjA9wQtRIAEEnlQ4AAxfRnKDWUTEOBrFyaSyCHzoOQQPSaODmQJojxBUZoMD4EjlbLIMC2PiwTaJCxWGznCndawuOAyUzQQxBcLsXj5Ipiy7oNAxAByFFGDjMHJS50c-I2TCoiiIIF6YrkMlufyIDTgBJgeSgCAAtEMRiqkpzUr4GOERKIIDAwCg2GU2A0mpNWmsiIsXLaQPoLchtvBY5tqmxxh5iqIYkYAOqsES6prpQS8RBoOCaJDKMB28qVwwy66C5z6bgiI6EyaZP7sCgBirgJS4MVEPQZLBDiqaO60MGtlh3El13CjCg1fnhn1SBg+OhgEDwHkYtCyKA1brebTZPlsCRUSaFAp2xnMuAUAoFagIbD2TxEJAQOgs2zVcZBaNBumfCgWUTKBskKTZWjAUxiQ+fMtB0XAiDLLsQEORQzx7NgfGxbp4OgAoK3EARFBiABJEQCjML84FrZQGEUTZjTQDQiBIQ8VxqUCmJjS9gnuWBlzYOh8Ig5gCGKUDxm0FiiNg0gKKQKi+A4-plLUPBuipEBNG3GgRItFZfD4O1yMo0x0CyKIgA";
    test_enc_bin_compat(
        lz_str::compress_to_encoded_uri_component,
        uri_encoding_decompressed,
        uri_encoding_compressed.into(),
    );

    eprintln!("UInt8Array");
    let uint8_array_decompressed = "During tattooing, ink is injected into the skin, initiating an immune response, and cells called \"macrophages\" move into the area and \"eat up\" the ink. The macrophages carry some of the ink to the body\'s lymph nodes, but some that are filled with ink stay put, embedded in the skin. That\'s what makes the tattoo visible under the skin. Dalhousie Uiversity\'s Alec Falkenham is developing a topical cream that works by targeting the macrophages that have remained at the site of the tattoo. New macrophages move in to consume the previously pigment-filled macrophages and then migrate to the lymph nodes, eventually taking all the dye with them. \"When comparing it to laser-based tattoo removal, in which you see the burns, the scarring, the blisters, in this case, we\'ve designed a drug that doesn\'t really have much off-target effect,\" he said. \"We\'re not targeting any of the normal skin cells, so you won\'t see a lot of inflammation. In fact, based on the process that we\'re actually using, we don\'t think there will be any inflammation at all and it would actually be anti-inflammatory.";
    let uint8_array_compressed = &[
        8, 133, 112, 78, 9, 96, 118, 14, 96, 4, 1, 112, 33, 130, 16, 123, 55, 70, 1, 163, 180, 13,
        103, 128, 206, 121, 64, 21, 128, 166, 3, 24, 33, 64, 38, 167, 168, 128, 22, 21, 196, 126,
        210, 237, 4, 8, 66, 150, 56, 72, 161, 224, 11, 106, 36, 20, 54, 96, 41, 16, 0, 230, 138,
        17, 10, 185, 132, 50, 161, 64, 13, 166, 146, 84, 147, 111, 167, 0, 17, 40, 164, 84, 193,
        163, 156, 201, 12, 89, 70, 226, 139, 64, 13, 205, 180, 38, 9, 89, 9, 148, 136, 84, 6, 70,
        40, 80, 224, 64, 229, 236, 61, 92, 161, 240, 0, 232, 224, 0, 85, 61, 77, 205, 45, 173, 109,
        116, 144, 192, 192, 1, 61, 216, 209, 68, 216, 208, 0, 204, 88, 35, 9, 221, 60, 0, 140, 208,
        233, 50, 1, 200, 73, 53, 51, 68, 172, 224, 160, 171, 101, 113, 202, 64, 16, 114, 242, 88,
        131, 210, 216, 10, 32, 12, 24, 1, 221, 121, 153, 73, 8, 137, 145, 178, 228, 187, 112, 41,
        69, 203, 232, 233, 13, 161, 139, 217, 56, 160, 99, 226, 80, 234, 225, 71, 173, 187, 77,
        241, 101, 55, 145, 80, 48, 224, 156, 32, 136, 33, 203, 52, 217, 36, 214, 193, 54, 57, 160,
        99, 129, 245, 152, 208, 65, 238, 216, 0, 85, 8, 11, 140, 15, 112, 66, 212, 72, 0, 65, 39,
        149, 14, 0, 3, 23, 209, 156, 160, 214, 81, 49, 14, 6, 177, 114, 105, 44, 130, 31, 58, 14,
        65, 3, 210, 104, 224, 230, 64, 154, 35, 196, 21, 25, 160, 192, 248, 18, 57, 91, 44, 131, 2,
        216, 248, 176, 77, 162, 66, 197, 97, 179, 156, 41, 221, 107, 11, 142, 3, 37, 51, 65, 12,
        65, 112, 187, 23, 143, 146, 41, 139, 46, 232, 52, 12, 64, 7, 33, 69, 24, 56, 204, 28, 148,
        185, 209, 207, 200, 217, 48, 168, 138, 34, 8, 23, 166, 43, 144, 201, 110, 127, 34, 3, 78,
        0, 73, 129, 228, 160, 8, 0, 45, 16, 196, 98, 170, 74, 115, 82, 190, 6, 56, 68, 74, 32, 128,
        192, 192, 40, 54, 25, 77, 128, 210, 106, 77, 90, 107, 34, 34, 197, 203, 105, 3, 232, 45,
        200, 109, 188, 22, 57, 182, 169, 177, 198, 30, 98, 168, 134, 36, 96, 3, 170, 176, 68, 186,
        166, 186, 80, 75, 196, 65, 160, 224, 154, 36, 50, 140, 7, 111, 42, 87, 12, 50, 235, 160,
        185, 207, 166, 224, 136, 142, 132, 201, 166, 79, 238, 192, 160, 6, 42, 224, 37, 46, 12, 84,
        67, 208, 100, 176, 67, 138, 166, 142, 235, 67, 6, 182, 88, 119, 18, 93, 119, 10, 48, 160,
        213, 249, 225, 159, 84, 129, 131, 227, 161, 128, 64, 240, 30, 70, 45, 11, 34, 128, 213,
        186, 222, 109, 54, 79, 150, 192, 145, 81, 38, 133, 2, 157, 177, 156, 203, 128, 80, 10, 5,
        106, 2, 27, 15, 100, 241, 16, 144, 16, 58, 11, 54, 205, 87, 25, 5, 163, 65, 186, 103, 194,
        129, 101, 19, 40, 27, 36, 41, 54, 86, 140, 5, 49, 137, 15, 159, 50, 208, 116, 92, 8, 131,
        44, 187, 16, 16, 228, 80, 207, 30, 205, 129, 241, 177, 110, 158, 14, 128, 10, 10, 220, 64,
        17, 20, 24, 128, 4, 145, 16, 10, 51, 11, 243, 129, 107, 101, 1, 132, 81, 54, 99, 77, 0,
        208, 136, 18, 16, 241, 92, 106, 80, 41, 137, 141, 47, 96, 158, 229, 129, 151, 54, 14, 135,
        194, 32, 230, 0, 134, 41, 64, 241, 155, 65, 98, 136, 216, 52, 128, 162, 144, 42, 47, 128,
        227, 250, 101, 45, 67, 193, 186, 42, 68, 4, 209, 183, 26, 4, 72, 180, 86, 95, 15, 131, 181,
        200, 202, 52, 199, 64, 178, 40, 136, 0, 0,
    ];
    test_enc_bin_compat(
        lz_str::compress_to_uint8_array,
        uint8_array_decompressed,
        uint8_array_compressed.to_vec(),
    );

    eprintln!("UTF16");
    let utf16_decompressed = "During tattooing, ink is injected into the skin, initiating an immune response, and cells called \"macrophages\" move into the area and \"eat up\" the ink. The macrophages carry some of the ink to the body\'s lymph nodes, but some that are filled with ink stay put, embedded in the skin. That\'s what makes the tattoo visible under the skin. Dalhousie Uiversity\'s Alec Falkenham is developing a topical cream that works by targeting the macrophages that have remained at the site of the tattoo. New macrophages move in to consume the previously pigment-filled macrophages and then migrate to the lymph nodes, eventually taking all the dye with them. \"When comparing it to laser-based tattoo removal, in which you see the burns, the scarring, the blisters, in this case, we\'ve designed a drug that doesn\'t really have much off-target effect,\" he said. \"We\'re not targeting any of the normal skin cells, so you won\'t see a lot of inflammation. In fact, based on the process that we\'re actually using, we don\'t think there will be any inflammation at all and it would actually be anti-inflammatory.";
    let utf16_compressed = "\u{0462}\u{5C33}\u{414C}\u{0780}\u{7320}\u{1025}\u{6063}\u{0230}\u{3DBB}\u{51A0}\u{3496}\u{40F6}\u{3C26}\u{3A05}K\u{00C6}\u{01AC}\u{0870}\u{04F4}\u{7AA8}\u{00D0}\u{5731}\u{7DC5}\u{6D24}\u{0441}\u{25AE}\u{0934}\u{1E20}\u{5B71}\u{1070}\u{6CE0}\u{2930}\u{0093}\u{22A4}\u{2177}\u{1863}\u{152A}V\u{4D44}\u{54B3}\u{37F3}\u{4024}\u{2534}\u{456C}\u{0D3C}\u{7344}\u{18D2}\u{4702}\u{45C0}\u{0393}\u{36A4}\u{60B5}\u{486C}\u{5241}\u{282C}\u{4648}\u{2890}\u{1059}\u{3DA7}\u{55EA}\u{0FA0}\u{03C3}\u{4020}\u{555D}\u{2706}\u{4B8B}\u{2DCE}\u{492C}\u{0620}\u{0517}\u{31C2}\u{44F8}\u{6820}\u{3336}\u{0481}\u{1DF3}\u{6024}\u{3363}\u{5284}\u{01E8}\u{24BA}\u{4CF1}\u{15BC}\u{0A2A}\u{5B4B}\u{4749}@\u{7312}\u{2C61}\u{74D6}\u{0164}\u{00E1}\u{402E}\u{7606}\u{32B2}\u{08A9}\u{48F9}\u{394E}\u{6E25}\u{147C}\u{5F67}\u{2456}\u{4337}\u{5958}\u{5051}\u{78B4}\u{1D7C}\u{149A}\u{6DFA}\u{37E5}\u{4A8F}\u{1170}\u{1890}\u{2728}\u{1124}\u{1CD3}\u{26E9}\u{137B}\u{028C}\u{39C0}\u{31E0}\u{7D86}\u{1A28}\u{1F0D}\u{4022}\u{5440}\u{1738}\u{0F90}\u{218A}\u{1220}\u{0844}\u{7970}\u{7020}\u{0C7F}\u{2359}\u{20F6}\u{28B8}\u{43A1}\u{564E}\u{26B2}\u{6430}\u{7D08}\u{1CA2}\u{03F2}\u{3490}\u{39B0}\u{1364}\u{3C61}\u{28ED}\u{0323}\u{7044}\u{397B}\u{1661}\u{40D6}\u{1F36}\u{04FA}\u{1236}\u{15A6}\u{6758}\u{29FD}\u{35A5}\u{63A0}\u{64C6}\u{3430}\u{622B}\u{430C}\u{2F3F}\u{1249}\u{45B7}\u{3A2D}\u{01A8}\u{0092}\u{0A48}\u{6103}\u{1859}\u{14D9}\u{6907}\u{7256}\u{2635}\u{08C2}\u{1060}\u{5EB8}\u{5741}\u{498E}\u{3FB1}\u{00F3}\u{4029}\u{183E}\u{2520}\u{2020}\u{5A41}\u{4482}\u{5545}\u{1CF4}\u{57E0}\u{63A4}\u{2271}\u{0223}\u{01A0}\u{2856}\u{0CC6}\u{6054}\u{4D69}\u{55C6}\u{5931}\u{0B37}\u{16F2}\u{0408}\u{1704}\u{1B8F}\u{02E7}\u{1B8A}\u{4DAE}\u{1899}\u{4571}\u{0644}\u{3021}\u{6ACC}\u{08B7}\u{2A8B}\u{52A2}\u{2F31}\u{0361}\u{60BA}\u{1239}\u{2321}\u{6E05}\u{2590}\u{61B7}\u{2EA2}\u{73BF}\u{2700}\u{4467}\u{2152}\u{34E9}\u{7F0C}\u{0520}\u{18CB}\u{406A}\u{2E2C}\u{2A41}\u{7439}\u{1628}\u{38CA}\u{3497}\u{2D2C}\u{0D8C}\u{5897}\u{094E}\u{5DE2}\u{4634}\u{0D7F}\u{4F2C}\u{7D72}\u{0327}\u{63C1}\u{4040}\u{3C27}\u{48E5}\u{50D2}\u{1426}\u{570B}\u{3CFA}\u{366F}\u{4B80}\u{2474}\u{24F0}\u{5049}\u{6DAC}\u{734E}\u{00C0}\u{0A25}\u{3521}\u{06E3}\u{6CBE}\u{1129}\u{00A1}\u{684C}\u{6DBA}\u{5739}\u{02F1}\u{508E}\u{4D18}\u{2836}\u{28B9}\u{208C}\u{4872}\u{3676}\u{4622}\u{4C82}\u{2213}\u{734D}\u{03C2}\u{7042}\u{0679}\u{3B30}\u{0892}\u{1453}\u{63F9}\u{583F}\u{0DAB}\u{3A98}\u{1D20}\u{0A2A}\u{6E40}\u{0465}\u{0330}i\u{08A0}\u{28EC}\u{1807}\u{018B}\u{32A0}\u{6134}\u{26EC}\u{34F0}\u{06A4}\u{2068}\u{2202}\u{5C8A}\u{2834}\u{6283}\u{260C}\u{0A0E}\u{2C2C}\u{5CF8}\u{1D2F}\u{4240}\u{7320}\u{21AA}\u{283E}\u{19D4}\u{0B34}\u{2380}\u{6921}\u{22B0}\u{1537}\u{6058}\u{7F6C}\u{52F4}\u{1E2D}\u{68C9}\u{0829}\u{51D7}\u{0D22}\u{124D}\u{0AEB}\u{7118}\u{1DCE}\u{2348}\u{69AE}\u{40D2}\u{1464}\u{0020}\u{0020}";
    test_enc_bin_compat(
        lz_str::compress_to_utf16,
        utf16_decompressed,
        utf16_compressed.into(),
    );
}

#[test]
fn binary_decoding_compatibility_tests_mandatory() {
    eprintln!("base64 - old encoding");
    let base64_encoding_old_decompressed = "During tattooing, ink is injected into the skin, initiating an immune response, and cells called \"macrophages\" move into the area and \"eat up\" the ink. The macrophages carry some of the ink to the body\'s lymph nodes, but some that are filled with ink stay put, embedded in the skin. That\'s what makes the tattoo visible under the skin. Dalhousie Uiversity\'s Alec Falkenham is developing a topical cream that works by targeting the macrophages that have remained at the site of the tattoo. New macrophages move in to consume the previously pigment-filled macrophages and then migrate to the lymph nodes, eventually taking all the dye with them. \"When comparing it to laser-based tattoo removal, in which you see the burns, the scarring, the blisters, in this case, we\'ve designed a drug that doesn\'t really have much off-target effect,\" he said. \"We\'re not targeting any of the normal skin cells, so you won\'t see a lot of inflammation. In fact, based on the process that we\'re actually using, we don\'t think there will be any inflammation at all and it would actually be anti-inflammatory.";
    let base64_encoding_old_compressed = "CIVwTglgdg5gBAFwIYIQezdGAaO0DWeAznlAFYCmAxghQCanqIAWFcR+0u0ECEKWOEih4AtqJBQ2YCkQAOaKEQq5hDKhQA2mklSTb6cAESikVMGjnMkMWUbii0ANzbQmCVkJlIhUBkYoUOBA5ew9XKHwAOjgAFU9Tc0trW10kMDAAT3Y0UTY0ADMWCMJ3TwAjNDpMgHISTUzRKzgoKtlccpAEHLyWIPS2AogDBgB3XmZSQiJkbLku3ApRcvo6Q2hi9k4oGPiUOrhR627TfFlN5FQMOCcIIghyzTZJNbBNjmgY4H1mNBB7tgAVQgLjA9wQtRIAEEnlQ4AAxfRnKDWUTEOBrFyaSyCHzoOQQPSaODmQJojxBUZoMD4EjlbLIMC2PiwTaJCxWGznCndawuOAyUzQQxBcLsXj5Ipiy7oNAxAByFFGDjMHJS50c/I2TCoiiIIF6YrkMlufyIDTgBJgeSgCAAtEMRiqkpzUr4GOERKIIDAwCg2GU2A0mpNWmsiIsXLaQPoLchtvBY5tqmxxh5iqIYkYAOqsES6prpQS8RBoOCaJDKMB28qVwwy66C5z6bgiI6EyaZP7sCgBirgJS4MVEPQZLBDiqaO60MGtlh3El13CjCg1fnhn1SBg+OhgEDwHkYtCyKA1brebTZPlsCRUSaFAp2xnMuAUAoFagIbD2TxEJAQOgs2zVcZBaNBumfCgWUTKBskKTZWjAUxiQ+fMtB0XAiDLLsQEORQzx7NgfGxbp4OgAoK3EARFBiABJEQCjML84FrZQGEUTZjTQDQiBIQ8VxqUCmJjS9gnuWBlzYOh8Ig5gCGKUDxm0FiiNg0gKKQKi+A4/plLUPBuipEBNG3GgRItFZfD4O1yMo0x0CyKIgAAA==";
    test_dec_bin_compat(
        lz_str::decompress_from_base64,
        base64_encoding_old_decompressed,
        base64_encoding_old_compressed,
    );

    eprintln!("base64");
    let base64_encoding_decompressed = "During tattooing, ink is injected into the skin, initiating an immune response, and cells called \"macrophages\" move into the area and \"eat up\" the ink. The macrophages carry some of the ink to the body\'s lymph nodes, but some that are filled with ink stay put, embedded in the skin. That\'s what makes the tattoo visible under the skin. Dalhousie Uiversity\'s Alec Falkenham is developing a topical cream that works by targeting the macrophages that have remained at the site of the tattoo. New macrophages move in to consume the previously pigment-filled macrophages and then migrate to the lymph nodes, eventually taking all the dye with them. \"When comparing it to laser-based tattoo removal, in which you see the burns, the scarring, the blisters, in this case, we\'ve designed a drug that doesn\'t really have much off-target effect,\" he said. \"We\'re not targeting any of the normal skin cells, so you won\'t see a lot of inflammation. In fact, based on the process that we\'re actually using, we don\'t think there will be any inflammation at all and it would actually be anti-inflammatory.";
    let base64_encoding_compressed = "CIVwTglgdg5gBAFwIYIQezdGAaO0DWeAznlAFYCmAxghQCanqIAWFcR+0u0ECEKWOEih4AtqJBQ2YCkQAOaKEQq5hDKhQA2mklSTb6cAESikVMGjnMkMWUbii0ANzbQmCVkJlIhUBkYoUOBA5ew9XKHwAOjgAFU9Tc0trW10kMDAAT3Y0UTY0ADMWCMJ3TwAjNDpMgHISTUzRKzgoKtlccpAEHLyWIPS2AogDBgB3XmZSQiJkbLku3ApRcvo6Q2hi9k4oGPiUOrhR627TfFlN5FQMOCcIIghyzTZJNbBNjmgY4H1mNBB7tgAVQgLjA9wQtRIAEEnlQ4AAxfRnKDWUTEOBrFyaSyCHzoOQQPSaODmQJojxBUZoMD4EjlbLIMC2PiwTaJCxWGznCndawuOAyUzQQxBcLsXj5Ipiy7oNAxAByFFGDjMHJS50c/I2TCoiiIIF6YrkMlufyIDTgBJgeSgCAAtEMRiqkpzUr4GOERKIIDAwCg2GU2A0mpNWmsiIsXLaQPoLchtvBY5tqmxxh5iqIYkYAOqsES6prpQS8RBoOCaJDKMB28qVwwy66C5z6bgiI6EyaZP7sCgBirgJS4MVEPQZLBDiqaO60MGtlh3El13CjCg1fnhn1SBg+OhgEDwHkYtCyKA1brebTZPlsCRUSaFAp2xnMuAUAoFagIbD2TxEJAQOgs2zVcZBaNBumfCgWUTKBskKTZWjAUxiQ+fMtB0XAiDLLsQEORQzx7NgfGxbp4OgAoK3EARFBiABJEQCjML84FrZQGEUTZjTQDQiBIQ8VxqUCmJjS9gnuWBlzYOh8Ig5gCGKUDxm0FiiNg0gKKQKi+A4/plLUPBuipEBNG3GgRItFZfD4O1yMo0x0CyKIgA";
    test_dec_bin_compat(
        lz_str::decompress_from_base64,
        base64_encoding_decompressed,
        base64_encoding_compressed,
    );

    eprintln!("uriEncoding - old encoding");
    let uri_encoding_old_decompressed = "During tattooing, ink is injected into the skin, initiating an immune response, and cells called \"macrophages\" move into the area and \"eat up\" the ink. The macrophages carry some of the ink to the body\'s lymph nodes, but some that are filled with ink stay put, embedded in the skin. That\'s what makes the tattoo visible under the skin. Dalhousie Uiversity\'s Alec Falkenham is developing a topical cream that works by targeting the macrophages that have remained at the site of the tattoo. New macrophages move in to consume the previously pigment-filled macrophages and then migrate to the lymph nodes, eventually taking all the dye with them. \"When comparing it to laser-based tattoo removal, in which you see the burns, the scarring, the blisters, in this case, we\'ve designed a drug that doesn\'t really have much off-target effect,\" he said. \"We\'re not targeting any of the normal skin cells, so you won\'t see a lot of inflammation. In fact, based on the process that we\'re actually using, we don\'t think there will be any inflammation at all and it would actually be anti-inflammatory.";
    let uri_encoding_old_compressed = "CIVwTglgdg5gBAFwIYIQezdGAaO0DWeAznlAFYCmAxghQCanqIAWFcR+0u0ECEKWOEih4AtqJBQ2YCkQAOaKEQq5hDKhQA2mklSTb6cAESikVMGjnMkMWUbii0ANzbQmCVkJlIhUBkYoUOBA5ew9XKHwAOjgAFU9Tc0trW10kMDAAT3Y0UTY0ADMWCMJ3TwAjNDpMgHISTUzRKzgoKtlccpAEHLyWIPS2AogDBgB3XmZSQiJkbLku3ApRcvo6Q2hi9k4oGPiUOrhR627TfFlN5FQMOCcIIghyzTZJNbBNjmgY4H1mNBB7tgAVQgLjA9wQtRIAEEnlQ4AAxfRnKDWUTEOBrFyaSyCHzoOQQPSaODmQJojxBUZoMD4EjlbLIMC2PiwTaJCxWGznCndawuOAyUzQQxBcLsXj5Ipiy7oNAxAByFFGDjMHJS50c-I2TCoiiIIF6YrkMlufyIDTgBJgeSgCAAtEMRiqkpzUr4GOERKIIDAwCg2GU2A0mpNWmsiIsXLaQPoLchtvBY5tqmxxh5iqIYkYAOqsES6prpQS8RBoOCaJDKMB28qVwwy66C5z6bgiI6EyaZP7sCgBirgJS4MVEPQZLBDiqaO60MGtlh3El13CjCg1fnhn1SBg+OhgEDwHkYtCyKA1brebTZPlsCRUSaFAp2xnMuAUAoFagIbD2TxEJAQOgs2zVcZBaNBumfCgWUTKBskKTZWjAUxiQ+fMtB0XAiDLLsQEORQzx7NgfGxbp4OgAoK3EARFBiABJEQCjML84FrZQGEUTZjTQDQiBIQ8VxqUCmJjS9gnuWBlzYOh8Ig5gCGKUDxm0FiiNg0gKKQKi+A4-plLUPBuipEBNG3GgRItFZfD4O1yMo0x0CyKIgA";
    test_dec_bin_compat(
        lz_str::decompress_from_encoded_uri_component,
        uri_encoding_old_decompressed,
        uri_encoding_old_compressed,
    );

    eprintln!("uriEncoding");
    let uri_encoding_decompressed = "During tattooing, ink is injected into the skin, initiating an immune response, and cells called \"macrophages\" move into the area and \"eat up\" the ink. The macrophages carry some of the ink to the body\'s lymph nodes, but some that are filled with ink stay put, embedded in the skin. That\'s what makes the tattoo visible under the skin. Dalhousie Uiversity\'s Alec Falkenham is developing a topical cream that works by targeting the macrophages that have remained at the site of the tattoo. New macrophages move in to consume the previously pigment-filled macrophages and then migrate to the lymph nodes, eventually taking all the dye with them. \"When comparing it to laser-based tattoo removal, in which you see the burns, the scarring, the blisters, in this case, we\'ve designed a drug that doesn\'t really have much off-target effect,\" he said. \"We\'re not targeting any of the normal skin cells, so you won\'t see a lot of inflammation. In fact, based on the process that we\'re actually using, we don\'t think there will be any inflammation at all and it would actually be anti-inflammatory.";
    let uri_encoding_compressed = "CIVwTglgdg5gBAFwIYIQezdGAaO0DWeAznlAFYCmAxghQCanqIAWFcR+0u0ECEKWOEih4AtqJBQ2YCkQAOaKEQq5hDKhQA2mklSTb6cAESikVMGjnMkMWUbii0ANzbQmCVkJlIhUBkYoUOBA5ew9XKHwAOjgAFU9Tc0trW10kMDAAT3Y0UTY0ADMWCMJ3TwAjNDpMgHISTUzRKzgoKtlccpAEHLyWIPS2AogDBgB3XmZSQiJkbLku3ApRcvo6Q2hi9k4oGPiUOrhR627TfFlN5FQMOCcIIghyzTZJNbBNjmgY4H1mNBB7tgAVQgLjA9wQtRIAEEnlQ4AAxfRnKDWUTEOBrFyaSyCHzoOQQPSaODmQJojxBUZoMD4EjlbLIMC2PiwTaJCxWGznCndawuOAyUzQQxBcLsXj5Ipiy7oNAxAByFFGDjMHJS50c-I2TCoiiIIF6YrkMlufyIDTgBJgeSgCAAtEMRiqkpzUr4GOERKIIDAwCg2GU2A0mpNWmsiIsXLaQPoLchtvBY5tqmxxh5iqIYkYAOqsES6prpQS8RBoOCaJDKMB28qVwwy66C5z6bgiI6EyaZP7sCgBirgJS4MVEPQZLBDiqaO60MGtlh3El13CjCg1fnhn1SBg+OhgEDwHkYtCyKA1brebTZPlsCRUSaFAp2xnMuAUAoFagIbD2TxEJAQOgs2zVcZBaNBumfCgWUTKBskKTZWjAUxiQ+fMtB0XAiDLLsQEORQzx7NgfGxbp4OgAoK3EARFBiABJEQCjML84FrZQGEUTZjTQDQiBIQ8VxqUCmJjS9gnuWBlzYOh8Ig5gCGKUDxm0FiiNg0gKKQKi+A4-plLUPBuipEBNG3GgRItFZfD4O1yMo0x0CyKIgAAA$$";
    test_dec_bin_compat(
        lz_str::decompress_from_encoded_uri_component,
        uri_encoding_decompressed,
        uri_encoding_compressed,
    );

    eprintln!("UInt8Array");
    let uint8_array_decompressed = "During tattooing, ink is injected into the skin, initiating an immune response, and cells called \"macrophages\" move into the area and \"eat up\" the ink. The macrophages carry some of the ink to the body\'s lymph nodes, but some that are filled with ink stay put, embedded in the skin. That\'s what makes the tattoo visible under the skin. Dalhousie Uiversity\'s Alec Falkenham is developing a topical cream that works by targeting the macrophages that have remained at the site of the tattoo. New macrophages move in to consume the previously pigment-filled macrophages and then migrate to the lymph nodes, eventually taking all the dye with them. \"When comparing it to laser-based tattoo removal, in which you see the burns, the scarring, the blisters, in this case, we\'ve designed a drug that doesn\'t really have much off-target effect,\" he said. \"We\'re not targeting any of the normal skin cells, so you won\'t see a lot of inflammation. In fact, based on the process that we\'re actually using, we don\'t think there will be any inflammation at all and it would actually be anti-inflammatory.";
    let uint8_array_compressed = &[
        8, 133, 112, 78, 9, 96, 118, 14, 96, 4, 1, 112, 33, 130, 16, 123, 55, 70, 1, 163, 180, 13,
        103, 128, 206, 121, 64, 21, 128, 166, 3, 24, 33, 64, 38, 167, 168, 128, 22, 21, 196, 126,
        210, 237, 4, 8, 66, 150, 56, 72, 161, 224, 11, 106, 36, 20, 54, 96, 41, 16, 0, 230, 138,
        17, 10, 185, 132, 50, 161, 64, 13, 166, 146, 84, 147, 111, 167, 0, 17, 40, 164, 84, 193,
        163, 156, 201, 12, 89, 70, 226, 139, 64, 13, 205, 180, 38, 9, 89, 9, 148, 136, 84, 6, 70,
        40, 80, 224, 64, 229, 236, 61, 92, 161, 240, 0, 232, 224, 0, 85, 61, 77, 205, 45, 173, 109,
        116, 144, 192, 192, 1, 61, 216, 209, 68, 216, 208, 0, 204, 88, 35, 9, 221, 60, 0, 140, 208,
        233, 50, 1, 200, 73, 53, 51, 68, 172, 224, 160, 171, 101, 113, 202, 64, 16, 114, 242, 88,
        131, 210, 216, 10, 32, 12, 24, 1, 221, 121, 153, 73, 8, 137, 145, 178, 228, 187, 112, 41,
        69, 203, 232, 233, 13, 161, 139, 217, 56, 160, 99, 226, 80, 234, 225, 71, 173, 187, 77,
        241, 101, 55, 145, 80, 48, 224, 156, 32, 136, 33, 203, 52, 217, 36, 214, 193, 54, 57, 160,
        99, 129, 245, 152, 208, 65, 238, 216, 0, 85, 8, 11, 140, 15, 112, 66, 212, 72, 0, 65, 39,
        149, 14, 0, 3, 23, 209, 156, 160, 214, 81, 49, 14, 6, 177, 114, 105, 44, 130, 31, 58, 14,
        65, 3, 210, 104, 224, 230, 64, 154, 35, 196, 21, 25, 160, 192, 248, 18, 57, 91, 44, 131, 2,
        216, 248, 176, 77, 162, 66, 197, 97, 179, 156, 41, 221, 107, 11, 142, 3, 37, 51, 65, 12,
        65, 112, 187, 23, 143, 146, 41, 139, 46, 232, 52, 12, 64, 7, 33, 69, 24, 56, 204, 28, 148,
        185, 209, 207, 200, 217, 48, 168, 138, 34, 8, 23, 166, 43, 144, 201, 110, 127, 34, 3, 78,
        0, 73, 129, 228, 160, 8, 0, 45, 16, 196, 98, 170, 74, 115, 82, 190, 6, 56, 68, 74, 32, 128,
        192, 192, 40, 54, 25, 77, 128, 210, 106, 77, 90, 107, 34, 34, 197, 203, 105, 3, 232, 45,
        200, 109, 188, 22, 57, 182, 169, 177, 198, 30, 98, 168, 134, 36, 96, 3, 170, 176, 68, 186,
        166, 186, 80, 75, 196, 65, 160, 224, 154, 36, 50, 140, 7, 111, 42, 87, 12, 50, 235, 160,
        185, 207, 166, 224, 136, 142, 132, 201, 166, 79, 238, 192, 160, 6, 42, 224, 37, 46, 12, 84,
        67, 208, 100, 176, 67, 138, 166, 142, 235, 67, 6, 182, 88, 119, 18, 93, 119, 10, 48, 160,
        213, 249, 225, 159, 84, 129, 131, 227, 161, 128, 64, 240, 30, 70, 45, 11, 34, 128, 213,
        186, 222, 109, 54, 79, 150, 192, 145, 81, 38, 133, 2, 157, 177, 156, 203, 128, 80, 10, 5,
        106, 2, 27, 15, 100, 241, 16, 144, 16, 58, 11, 54, 205, 87, 25, 5, 163, 65, 186, 103, 194,
        129, 101, 19, 40, 27, 36, 41, 54, 86, 140, 5, 49, 137, 15, 159, 50, 208, 116, 92, 8, 131,
        44, 187, 16, 16, 228, 80, 207, 30, 205, 129, 241, 177, 110, 158, 14, 128, 10, 10, 220, 64,
        17, 20, 24, 128, 4, 145, 16, 10, 51, 11, 243, 129, 107, 101, 1, 132, 81, 54, 99, 77, 0,
        208, 136, 18, 16, 241, 92, 106, 80, 41, 137, 141, 47, 96, 158, 229, 129, 151, 54, 14, 135,
        194, 32, 230, 0, 134, 41, 64, 241, 155, 65, 98, 136, 216, 52, 128, 162, 144, 42, 47, 128,
        227, 250, 101, 45, 67, 193, 186, 42, 68, 4, 209, 183, 26, 4, 72, 180, 86, 95, 15, 131, 181,
        200, 202, 52, 199, 64, 178, 40, 136, 0, 0,
    ];
    test_dec_bin_compat(
        lz_str::decompress_from_uint8_array,
        uint8_array_decompressed,
        uint8_array_compressed,
    );

    eprintln!("UTF16");
    let utf16_encoding_decompressed = "During tattooing, ink is injected into the skin, initiating an immune response, and cells called \"macrophages\" move into the area and \"eat up\" the ink. The macrophages carry some of the ink to the body\'s lymph nodes, but some that are filled with ink stay put, embedded in the skin. That\'s what makes the tattoo visible under the skin. Dalhousie Uiversity\'s Alec Falkenham is developing a topical cream that works by targeting the macrophages that have remained at the site of the tattoo. New macrophages move in to consume the previously pigment-filled macrophages and then migrate to the lymph nodes, eventually taking all the dye with them. \"When comparing it to laser-based tattoo removal, in which you see the burns, the scarring, the blisters, in this case, we\'ve designed a drug that doesn\'t really have much off-target effect,\" he said. \"We\'re not targeting any of the normal skin cells, so you won\'t see a lot of inflammation. In fact, based on the process that we\'re actually using, we don\'t think there will be any inflammation at all and it would actually be anti-inflammatory.";
    let utf16_encoding_compressed = "\u{0462}\u{5C33}\u{414C}\u{0780}\u{7320}\u{1025}\u{6063}\u{0230}\u{3DBB}\u{51A0}\u{3496}\u{40F6}\u{3C26}\u{3A05}K\u{00C6}\u{01AC}\u{0870}\u{04F4}\u{7AA8}\u{00D0}\u{5731}\u{7DC5}\u{6D24}\u{0441}\u{25AE}\u{0934}\u{1E20}\u{5B71}\u{1070}\u{6CE0}\u{2930}\u{0093}\u{22A4}\u{2177}\u{1863}\u{152A}V\u{4D44}\u{54B3}\u{37F3}\u{4024}\u{2534}\u{456C}\u{0D3C}\u{7344}\u{18D2}\u{4702}\u{45C0}\u{0393}\u{36A4}\u{60B5}\u{486C}\u{5241}\u{282C}\u{4648}\u{2890}\u{1059}\u{3DA7}\u{55EA}\u{0FA0}\u{03C3}\u{4020}\u{555D}\u{2706}\u{4B8B}\u{2DCE}\u{492C}\u{0620}\u{0517}\u{31C2}\u{44F8}\u{6820}\u{3336}\u{0481}\u{1DF3}\u{6024}\u{3363}\u{5284}\u{01E8}\u{24BA}\u{4CF1}\u{15BC}\u{0A2A}\u{5B4B}\u{4749}@\u{7312}\u{2C61}\u{74D6}\u{0164}\u{00E1}\u{402E}\u{7606}\u{32B2}\u{08A9}\u{48F9}\u{394E}\u{6E25}\u{147C}\u{5F67}\u{2456}\u{4337}\u{5958}\u{5051}\u{78B4}\u{1D7C}\u{149A}\u{6DFA}\u{37E5}\u{4A8F}\u{1170}\u{1890}\u{2728}\u{1124}\u{1CD3}\u{26E9}\u{137B}\u{028C}\u{39C0}\u{31E0}\u{7D86}\u{1A28}\u{1F0D}\u{4022}\u{5440}\u{1738}\u{0F90}\u{218A}\u{1220}\u{0844}\u{7970}\u{7020}\u{0C7F}\u{2359}\u{20F6}\u{28B8}\u{43A1}\u{564E}\u{26B2}\u{6430}\u{7D08}\u{1CA2}\u{03F2}\u{3490}\u{39B0}\u{1364}\u{3C61}\u{28ED}\u{0323}\u{7044}\u{397B}\u{1661}\u{40D6}\u{1F36}\u{04FA}\u{1236}\u{15A6}\u{6758}\u{29FD}\u{35A5}\u{63A0}\u{64C6}\u{3430}\u{622B}\u{430C}\u{2F3F}\u{1249}\u{45B7}\u{3A2D}\u{01A8}\u{0092}\u{0A48}\u{6103}\u{1859}\u{14D9}\u{6907}\u{7256}\u{2635}\u{08C2}\u{1060}\u{5EB8}\u{5741}\u{498E}\u{3FB1}\u{00F3}\u{4029}\u{183E}\u{2520}\u{2020}\u{5A41}\u{4482}\u{5545}\u{1CF4}\u{57E0}\u{63A4}\u{2271}\u{0223}\u{01A0}\u{2856}\u{0CC6}\u{6054}\u{4D69}\u{55C6}\u{5931}\u{0B37}\u{16F2}\u{0408}\u{1704}\u{1B8F}\u{02E7}\u{1B8A}\u{4DAE}\u{1899}\u{4571}\u{0644}\u{3021}\u{6ACC}\u{08B7}\u{2A8B}\u{52A2}\u{2F31}\u{0361}\u{60BA}\u{1239}\u{2321}\u{6E05}\u{2590}\u{61B7}\u{2EA2}\u{73BF}\u{2700}\u{4467}\u{2152}\u{34E9}\u{7F0C}\u{0520}\u{18CB}\u{406A}\u{2E2C}\u{2A41}\u{7439}\u{1628}\u{38CA}\u{3497}\u{2D2C}\u{0D8C}\u{5897}\u{094E}\u{5DE2}\u{4634}\u{0D7F}\u{4F2C}\u{7D72}\u{0327}\u{63C1}\u{4040}\u{3C27}\u{48E5}\u{50D2}\u{1426}\u{570B}\u{3CFA}\u{366F}\u{4B80}\u{2474}\u{24F0}\u{5049}\u{6DAC}\u{734E}\u{00C0}\u{0A25}\u{3521}\u{06E3}\u{6CBE}\u{1129}\u{00A1}\u{684C}\u{6DBA}\u{5739}\u{02F1}\u{508E}\u{4D18}\u{2836}\u{28B9}\u{208C}\u{4872}\u{3676}\u{4622}\u{4C82}\u{2213}\u{734D}\u{03C2}\u{7042}\u{0679}\u{3B30}\u{0892}\u{1453}\u{63F9}\u{583F}\u{0DAB}\u{3A98}\u{1D20}\u{0A2A}\u{6E40}\u{0465}\u{0330}i\u{08A0}\u{28EC}\u{1807}\u{018B}\u{32A0}\u{6134}\u{26EC}\u{34F0}\u{06A4}\u{2068}\u{2202}\u{5C8A}\u{2834}\u{6283}\u{260C}\u{0A0E}\u{2C2C}\u{5CF8}\u{1D2F}\u{4240}\u{7320}\u{21AA}\u{283E}\u{19D4}\u{0B34}\u{2380}\u{6921}\u{22B0}\u{1537}\u{6058}\u{7F6C}\u{52F4}\u{1E2D}\u{68C9}\u{0829}\u{51D7}\u{0D22}\u{124D}\u{0AEB}\u{7118}\u{1DCE}\u{2348}\u{69AE}\u{40D2}\u{1464}\u{0020}\u{0020}";
    test_dec_bin_compat(
        lz_str::decompress_from_utf16,
        utf16_encoding_decompressed,
        utf16_encoding_compressed,
    );
}

pub struct ByteString(Vec<u16>);

impl ByteString {
    pub fn to_utf8_string(&self) -> Result<String, FromUtf16Error> {
        String::from_utf16(&self.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<&str> for ByteString {
    fn from(s: &str) -> ByteString {
        ByteString(s.encode_utf16().collect())
    }
}

impl From<String> for ByteString {
    fn from(s: String) -> ByteString {
        ByteString(s.encode_utf16().collect())
    }
}

impl From<Vec<u16>> for ByteString {
    fn from(data: Vec<u16>) -> ByteString {
        ByteString(data)
    }
}

impl std::fmt::Debug for ByteString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for c in std::char::decode_utf16(self.0.iter().copied()) {
            c.unwrap_or(std::char::REPLACEMENT_CHARACTER).fmt(f)?;
        }

        Ok(())
    }
}

impl PartialEq<str> for ByteString {
    fn eq(&self, other: &str) -> bool {
        other.encode_utf16().eq(self.0.iter().copied())
    }
}

impl PartialEq<&str> for ByteString {
    fn eq(&self, other: &&str) -> bool {
        other.encode_utf16().eq(self.0.iter().copied())
    }
}
