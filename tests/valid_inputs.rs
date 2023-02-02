use lz_str::decompress;

#[test]
fn valid_decompress() {
    let valid_data: &[(&str, Vec<u16>)] =
        &[("red123", vec![0x80, 0x80]), ("腆퍂蚂荂", vec![0xD8A0])];
    for (data, expected) in valid_data {
        let arr: Vec<u16> = data.encode_utf16().collect();
        let decompressed = decompress(arr).expect("decompression failed");
        assert_eq!(&decompressed, expected);
    }
}

#[test]
fn valid_long_input_round() {
    // let buffer = [];
    // for(let i = 0; i < 100000; i++){
    //     buffer.push(i % 65_535);
    // }
    // result = LZString144.compress(String.fromCharCode(...buffer));
    // Array.from(result).map((v) => v.charCodeAt(0));
    let data: Vec<u16> = (0u64..100_000u64)
        .map(|val| (val % u64::from(std::u16::MAX)) as u16)
        .collect();

    let compressed = lz_str::compress(&data);

    let js_compressed = include_str!("../test_data/long_compressed_js.txt")
        .split(',')
        .map(|s| s.trim().parse::<u16>().unwrap())
        .collect::<Vec<u16>>();

    for (i, (a, b)) in compressed.iter().zip(js_compressed.iter()).enumerate() {
        if a != b {
            assert_eq!(a, b, "[index={i}] {a} != {b}");
        }
    }
    assert_eq!(compressed, js_compressed);

    let decompressed = lz_str::decompress(&compressed).expect("decompression failed");
    assert_eq!(decompressed, data);
}
