# lz-string-rs

A port of [this](https://github.com/pieroxy/lz-string) to rust.

### Installing

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
lz-string = { git = "https://github.com/adumbidiot/lz-string-rs" }
```

and to the top of your `main.rs`:

```rust
extern crate lz_string;
```

## Getting Started

```rust
extern crate lz_string;
use lz_string::{
    compress,
	decompress,
};

const RED_STR: &'static str = "red";

fn main(){
	let compressed = compress(&RED_STR, 16, std::char::from_u32); //The 16 is maximum number of bits per char
    let decompressed = decompress(&compressed, 32_768).unwrap(); //The 32,768 is 2^(16 - 1).
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