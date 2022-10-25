#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: &[u16]| {
     let compressed = lz_str::compress(input);
     let decompressed = lz_str::decompress(&compressed).unwrap();
     assert!(input == decompressed);
});
