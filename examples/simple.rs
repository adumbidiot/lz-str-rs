// The demonstrated functions correspond with `LZString.compress` and `LZString.decompress` from the JS version.
fn main() {
    let data = "The quick brown fox jumps over the lazy dog";

    // Compress the data. This cannot fail.
    let compressed_data = lz_str::compress(data);

    // Decompress the data.
    // This may return `Option::None` if it fails.
    // Make sure to do error-checking in a real application to prevent crashes!
    let decompressed_data =
        lz_str::decompress(compressed_data).expect("the compressed data is invalid");

    // The decompressed_data should be the same as data, except encoded as UTF16.
    // We undo that here.
    // In a real application, you will want to do error checking to prevent users from causing crashes with invalid data.
    let decompressed_data =
        String::from_utf16(&decompressed_data).expect("`decompressed_data` was not valid UTF16");

    assert!(data == decompressed_data);
}
