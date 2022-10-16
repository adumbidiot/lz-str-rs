// Sometimes, you may need to compress or decompress binary data.
// This shows a way to handle that.
fn main() {
    let data = b"The quick brown fox jumps over the lazy dog";

    // We pad every byte to make it a `u16`.
    let wide_data: Vec<u16> = data.iter().copied().map(u16::from).collect();

    // Use the `compress_to_uint8_array` function to compress data to a `Vec<u8>`.
    // This has the same space efficiency as the `lz_str::compress` function.
    let compressed_data = lz_str::compress_to_uint8_array(wide_data);

    // Call the decompress function.
    // We use the `lz_str::decompress_from_uint8_array`,
    // as that is the correct pairing for the `lz_str::compress_from_uint8_array` function.
    // Make sure to do proper error checking in a real application.
    let decompressed_data = lz_str::decompress_from_uint8_array(&compressed_data)
        .expect("`compressed_data` is invalid");

    // Remap the u16 to u8 via truncation.
    // We do this in this example only to prove we get the same bytes back from the beginning.
    let decompressed_data: Vec<u8> = decompressed_data.iter().copied().map(|v| v as u8).collect();

    // We get the same bytes from the beginning.
    assert!(decompressed_data == data);
}
