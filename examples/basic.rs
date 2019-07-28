use lz_string::{
    compress_str,
    decompress_str,
};

const RED_STR: &'static str = "red";

fn main() {
    let compressed = compress_str(&RED_STR);
    println!("{:?}", compressed);

    let decompressed = decompress_str(&compressed).unwrap();
    println!("{:?}", decompressed);

    assert_eq!(RED_STR, decompressed);
}
