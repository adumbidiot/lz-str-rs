# lz-str-rs
[![crates.io](https://img.shields.io/crates/v/lz-str.svg)](https://crates.io/crates/lz-str)
[![Documentation](https://docs.rs/lz-str/badge.svg)](https://docs.rs/lz-str)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/lz-str.svg)](./LICENSE-APACHE)
![Rust](https://github.com/adumbidiot/lz-str-rs/workflows/Rust/badge.svg)

A port of [lz-string](https://github.com/pieroxy/lz-string) to Rust. 

### Installing

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
lz-str = "0.1.0"
```

## Getting Started

```rust
// The demonstrated functions correspond with `LZString.compress` and `LZString.decompress` from the JS version.
fn main() {
    let data = "The quick brown fox jumps over the lazy dog";

    // Compress the data. This cannot fail.
    let compressed_data = lz_str::compress(data);

    // Decompress the data.
    // This may return `Option::None` if it fails.
    // Make sure to do error-checking in a real application to prevent crashes!
    let decompressed_data =
        lz_str::decompress(compressed_data).expect("`compressed_data` is invalid");

    // The decompressed_data should be the same as data, except encoded as UTF16.
    // We undo that here.
    // In a real application,
    // you will want to do error checking to prevent users from causing crashes with invalid data.
    let decompressed_data =
        String::from_utf16(&decompressed_data).expect("`decompressed_data` is not valid UTF16");

    assert!(data == decompressed_data);
}
```


## Testing
```bash
cargo test
```

## Benching
```bash
cargo bench
```

## Authors
adumbidiot (Nathaniel Daniel)

## License
Licensed under either of
 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.