use lz_str::decompress;

#[test]
fn valid_decompress_raw() {
    let valid_data: &[(Vec<u16>, Vec<u16>)] = &[
        ("red123".encode_utf16().collect(), vec![0x80, 0x80]),
        ("腆퍂蚂荂".encode_utf16().collect(), vec![0xD8A0]),
        (
            "\u{0485}\u{3036}\u{60F6}@\u{EA90}\u{2730}\u{04C8}\0\0"
                .encode_utf16()
                .collect(),
            "Hello World".encode_utf16().collect(),
        ),
        (
            vec![1157, 12342, 24822, 3753, 1254, 154, 33335, 37504],
            "HelloWorldHelloWorld".encode_utf16().collect(),
        ),
        (
            vec![
                1157, 12342, 24822, 3753, 1254, 154, 33335, 37461, 22585, 58107, 30043, 3315,
                18252, 51496, 33326, 12541, 52526, 55914, 30187, 6378, 34748, 48830, 23153, 61401,
                39713, 710, 62741, 50299, 24962, 42440, 37665, 27889, 54246, 19288, 48466, 36299,
                13482, 55972, 61875, 17966, 6832, 26366, 47994, 7394, 0,
            ],
            "HelloWorldHelloWorldHelloWorldHelloWorldHelloWorldHelloWorld"
                .repeat(10)
                .encode_utf16()
                .collect(),
        ),
    ];
    for (data, expected) in valid_data {
        let decompressed =
            decompress(data.as_slice()).expect(&format!("Valid Decompress of {:?}", expected));
        assert_eq!(&decompressed, expected);
    }
}

#[test]
fn valid_round_raw() {
    let valid_data: &[Vec<u16>] = &[
        "`ó«¯¯ ".encode_utf16().collect(),
        "  	".encode_utf16().collect(),
    ];

    for data in valid_data {
        let compressed = lz_str::compress(data);
        let decompressed = decompress(compressed.as_slice()).unwrap();
        assert_eq!(&decompressed, data);
    }
}
