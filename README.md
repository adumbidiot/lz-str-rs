# lz-string-rs

A port of [this](https://github.com/pieroxy/lz-string) to rust.

### Installing

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
lz-string = { git = "https://github.com/adumbidiot/lz-string-rs" }
```

## Getting Started

```rust
use lz_string::{
    compress_str,
    decompress_str,
};

const RED_STR: &'static str = "red";

fn main(){
    let compressed = compress_str(&RED_STR);
    let decompressed = decompress_str(&compressed).unwrap();
    assert_eq!(RED_STR, decompressed);
}
```


## Testing
```bash
cargo test
```

## Contributing
I literally have no standards so anything you contribute will be an improvement. Just open a pull request.

## Authors
adumbidiot (Nathaniel Daniel)

## License
This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details