use crate::{
    constants::{
        BASE64_KEY,
        CHAR_CODE,
        CLOSE_CODE,
        URI_KEY,
        WIDE_CHAR_CODE,
    },
    IntoWideIter,
};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct DecompressContext<I> {
    val: u16,
    compressed_data: I,
    position: usize,
    reset_val: usize,

    dictionary: Vec<Vec<u16>>,
    enlarge_in: u32,
    num_bits: u32,

    w: Vec<u16>,
    entry: Vec<u16>,
}

impl<I> DecompressContext<I>
where
    I: Iterator<Item = u16>,
{
    /// Make a new [`DecompressContext`].
    ///
    /// # Errors
    /// Returns `None` if the iterator is empty.
    ///
    /// # Panics
    /// Panics if `bits_per_char` is greater than the number of bits in a `u16`.
    ///
    #[inline]
    pub fn new(mut compressed_data: I, bits_per_char: u8) -> Option<Self> {
        assert!(usize::from(bits_per_char) <= std::mem::size_of::<u16>() * 8);

        let reset_val = 1 << (bits_per_char - 1);
        let val = compressed_data.next()?;

        Some(DecompressContext {
            val,
            compressed_data,
            position: reset_val,
            reset_val,

            // Init dictionary with default codes
            dictionary: vec![vec![CHAR_CODE], vec![WIDE_CHAR_CODE], vec![CLOSE_CODE]],

            enlarge_in: 5,
            num_bits: 2,

            w: Vec::with_capacity(256),
            entry: Vec::with_capacity(256),
        })
    }

    #[inline(always)]
    fn read_bit(&mut self) -> Option<bool> {
        let res = self.val & (self.position as u16);
        self.position >>= 1;

        if self.position == 0 {
            self.position = self.reset_val;
            self.val = self.compressed_data.next()?;
        }

        Some(res != 0)
    }

    #[inline(always)]
    fn read_bits(&mut self, n: u32) -> Option<u16> {
        let mut res = 0;
        let max_power: u32 = 1 << n;
        let mut power = 1;
        while power != max_power {
            res |= u16::from(self.read_bit()?) * power as u16;
            power <<= 1;
        }

        Some(res)
    }

    #[inline]
    fn add_to_dictionary(&mut self, data: Vec<u16>) {
        self.dictionary.push(data);
        self.enlarge_in -= 1;
        if self.enlarge_in == 0 {
            self.enlarge_in = 1 << self.num_bits;
            self.num_bits += 1;
        }
    }

    #[inline(always)]
    fn read_string(&mut self) -> Option<bool> {
        let code = self.read_bits(self.num_bits)?;
        let dictionary_len = self.dictionary.len();

        match code {
            CHAR_CODE => {
                let string = self.read_bits(8)?;
                self.add_to_dictionary(vec![string]);

                // entry = string
                self.entry.clear();
                self.entry.push(string);

                Some(false)
            }
            WIDE_CHAR_CODE => {
                let string = self.read_bits(16)?;
                self.add_to_dictionary(vec![string]);

                // entry = string
                self.entry.clear();
                self.entry.push(string);

                Some(false)
            }
            CLOSE_CODE => Some(true),
            code if usize::from(code) < dictionary_len => {
                let entry_value = self.dictionary.get(usize::from(code))?;

                // entry = entry_value
                self.entry.clear();
                self.entry.extend(entry_value);

                Some(false)
            }
            code if usize::from(code) == dictionary_len => {
                // entry = w + w[0]
                self.entry.clear();
                self.entry.extend(&self.w);
                self.entry.push(*self.w.get(0)?);

                Some(false)
            }
            _ => None,
        }
    }
}

/// Decompress a string into a [`Vec<u16>`].
/// The result contains possibly invalid UTF16.
///
/// # Errors
/// Returns `None` if the decompression fails.
///
#[inline]
pub fn decompress(compressed: impl IntoWideIter) -> Option<Vec<u16>> {
    decompress_internal(compressed.into_wide_iter(), 16)
}

/// Decompress a [`&str`] compressed with [`crate::compress_to_utf16`].
///
/// # Errors
/// Returns an error if the compressed data could not be decompressed.
///
#[inline]
pub fn decompress_from_utf16(compressed: &str) -> Option<Vec<u16>> {
    decompress_internal(compressed.encode_utf16().map(|c| c - 32), 15)
}

/// Decompress a [`&str`] compressed with [`crate::compress_to_encoded_uri_component`].
///
/// # Errors
/// Returns an error if the compressed data could not be decompressed.
///
#[inline]
pub fn decompress_from_encoded_uri_component(compressed: &str) -> Option<Vec<u16>> {
    let compressed: Option<Vec<u16>> = compressed
        .encode_utf16()
        .map(|c| {
            if c == u16::from(b' ') {
                u16::from(b'+')
            } else {
                c
            }
        })
        .map(u32::from)
        .map(|c| {
            URI_KEY
                .iter()
                .position(|k| u8::try_from(c) == Ok(*k))
                .map(|n| u16::try_from(n).ok())
        })
        .flatten()
        .collect();

    decompress_internal(compressed?.into_iter(), 6)
}

/// Decompress a [`&str`] compressed with [`crate::compress_to_base64`].
///
/// # Errors
/// Returns an error if the compressed data could not be decompressed.
///
#[inline]
pub fn decompress_from_base64(compressed: &str) -> Option<Vec<u16>> {
    let compressed: Option<Vec<u16>> = compressed
        .encode_utf16()
        .map(|c| {
            BASE64_KEY
                .iter()
                .position(|k| u8::try_from(c) == Ok(*k))
                .map(|n| u16::try_from(n).ok())
        })
        .flatten()
        .collect();

    decompress_internal(compressed?.into_iter(), 6)
}

/// Decompress a byte slice compressed with [`crate::compress_to_uint8_array`].
///
/// # Errors
/// Returns an error if the compressed data could not be decompressed.
///
#[inline]
pub fn decompress_from_uint8_array(compressed: &[u8]) -> Option<Vec<u16>> {
    let mut buf = Vec::with_capacity(compressed.len() / 2);
    for i in 0..(compressed.len() / 2) {
        buf.push(u16::from(compressed[i * 2]) * 256 + u16::from(compressed[i * 2 + 1]));
    }

    decompress(&buf)
}

/// The internal decompress function.
///
/// All other decompress functions are built on top of this one.
/// It generally should not be used directly.
///
/// # Errors
/// Returns an error if the compressed data could not be decompressed.
///
/// # Panics
/// Panics if `bits_per_char` is greater than the number of bits in a `u16`.
///
#[inline]
pub fn decompress_internal<I>(compressed: I, bits_per_char: u8) -> Option<Vec<u16>>
where
    I: Iterator<Item = u16>,
{
    let size_hint = compressed.size_hint();
    let max_input_len = size_hint.1.unwrap_or(200);
    let mut ctx = match DecompressContext::new(compressed, bits_per_char) {
        Some(ctx) => ctx,
        None => return Some(Vec::new()),
    };
    let mut result = Vec::with_capacity(max_input_len);
    ctx.dictionary.reserve(max_input_len);

    if ctx.read_string()? {
        return Some(result);
    }

    let first_entry = *ctx.entry.get(0)?;
    result.push(first_entry);
    ctx.w.push(first_entry);
    ctx.num_bits += 1;

    loop {
        if ctx.read_string()? {
            return Some(result);
        }

        result.extend(&ctx.entry);

        // Add w+entry[0] to the dictionary.
        let to_be_inserted = {
            let mut vec = Vec::with_capacity(ctx.w.len() + 1);
            vec.extend(&ctx.w);
            vec.push(*ctx.entry.get(0)?);
            vec
        };
        ctx.add_to_dictionary(to_be_inserted);

        // w = entry
        ctx.w.clear();
        ctx.w.extend(&ctx.entry);
    }
}
