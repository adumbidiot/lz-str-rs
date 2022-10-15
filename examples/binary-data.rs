use std::convert::TryInto;

// Sometimes, you may need to compress or decompress binary data.
// This shows a way to handle that.
fn main() {
    let data = b"The quick brown fox jumps over the lazy dog";

    // We pad every byte to make it a `u16`.
    let wide_data: Vec<u16> = data.iter().copied().map(u16::from).collect();

    let compressed_data = lz_str::compress(wide_data);

    // Let's turn the output into bytes
    // so we can show how to decompress from bytes.
    // We make the data native endian.
    let compressed_bytes: Vec<u8> = compressed_data
        .iter()
        .copied()
        .flat_map(u16::to_ne_bytes)
        .collect();

    // We cannot decompress from an arbitrary string of bytes;
    // those byte must be encoded wide characters (u16).

    // First, let's ensure those bytes are a multiple of two,
    // since two u8s form one u16.
    // Make this a check instead of an `assert` in an actual application.
    assert!(compressed_bytes.len() % 2 == 0);

    // Now, let's form a `Vec<u16>` from the bytes.
    // Here, we use u16::from_ne_bytes,
    // since we made this byte array with native endian data.
    // Make sure to use the correct function for your data.
    // As an example, if you get your bytes from a network connection,
    // you likely want to use `u16::from_be_bytes`.
    let compressed_data: Vec<u16> = compressed_bytes
        .chunks(2)
        .map(|slice| {
            // slice is always guaranteed to be 2 long here.
            u16::from_ne_bytes(slice.try_into().unwrap())
        })
        .collect();

    // Finally, call the decompress function.
    // Again, make sure to do proper error checking in a real application.
    let decompressed_data =
        lz_str::decompress(compressed_data).expect("`compressed_bytes` is invalid");

    // Remap the u16 to u8 via truncation.
    // We do this in this example only to prove we get the same bytes back from the beginning.
    let decompressed_data: Vec<u8> = decompressed_data.iter().copied().map(|v| v as u8).collect();

    // We get the same bytes from the beginning.
    assert!(decompressed_data == data);
}
