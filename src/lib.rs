#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! A port of [lz-string](https://github.com/pieroxy/lz-string) to Rust.
//!
//!
//! # Example
//! ```rust
//! # // The demonstrated functions correspond with `LZString.compress` and `LZString.decompress` from the JS version.
//! # fn main() {
//!     let data = "The quick brown fox jumps over the lazy dog";
//! 
//!     // Compress the data. This cannot fail.
//!     let compressed_data = lz_str::compress(data);
//! 
//!     // Decompress the data.
//!     // This may return `Option::None` if it fails.
//!     // Make sure to do error-checking in a real application to prevent crashes!
//!     let decompressed_data =
//!         lz_str::decompress(compressed_data).expect("`compressed_data` is invalid");
//! 
//!     // The decompressed_data should be the same as data, except encoded as UTF16.
//!     // We undo that here.
//!     // In a real application,
//!     // you will want to do error checking to prevent users from causing crashes with invalid data.
//!     let decompressed_data =
//!         String::from_utf16(&decompressed_data).expect("`decompressed_data` is not valid UTF16");
//! 
//!     assert!(data == decompressed_data);
//! # }
//! ```
//! 
//! # Passing and Recieving Data
//! The original library uses invalid UTF16 strings to represent data.
//! To maintain compatability, this library uses a [`Vec`] of [`u16`]s instead of Rust strings where applicable.
//! The [`IntoWideIter`] trait exists to ease the passing of data into functions.
//! Most functions accept this generic parameter instead of a concrete type.
//! Look at this trait's documentation to see what types this trait is implemented for.

mod compress;
mod constants;
mod decompress;

pub use crate::compress::compress;
pub use crate::compress::compress_internal;
pub use crate::compress::compress_to_base64;
pub use crate::compress::compress_to_encoded_uri_component;
pub use crate::compress::compress_to_uint8_array;
pub use crate::compress::compress_to_utf16;
pub use crate::decompress::decompress;
pub use crate::decompress::decompress_from_base64;
pub use crate::decompress::decompress_from_encoded_uri_component;
pub use crate::decompress::decompress_from_uint8_array;
pub use crate::decompress::decompress_from_utf16;
pub use crate::decompress::decompress_internal;

/// A trait to make it easier to pass arguments to functions.
pub trait IntoWideIter {
    /// The Iterator type
    type Iter: Iterator<Item = u16>;

    /// Convert this object into something that yields possibly invalid wide characters.
    fn into_wide_iter(self) -> Self::Iter;
}

impl<'a> IntoWideIter for &'a str {
    type Iter = std::str::EncodeUtf16<'a>;

    #[inline]
    fn into_wide_iter(self) -> Self::Iter {
        self.encode_utf16()
    }
}

impl<'a> IntoWideIter for &&'a str {
    type Iter = std::str::EncodeUtf16<'a>;

    #[inline]
    fn into_wide_iter(self) -> Self::Iter {
        self.encode_utf16()
    }
}

impl<'a> IntoWideIter for &'a [u16] {
    type Iter = std::iter::Copied<std::slice::Iter<'a, u16>>;

    #[inline]
    fn into_wide_iter(self) -> Self::Iter {
        self.iter().copied()
    }
}

impl IntoWideIter for Vec<u16> {
    type Iter = std::vec::IntoIter<u16>;

    #[inline]
    fn into_wide_iter(self) -> Self::Iter {
        self.into_iter()
    }
}

impl<'a> IntoWideIter for &'a Vec<u16> {
    type Iter = std::iter::Copied<std::slice::Iter<'a, u16>>;

    #[inline]
    fn into_wide_iter(self) -> Self::Iter {
        self.iter().copied()
    }
}
