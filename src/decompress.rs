use crate::{
    constants::{
        BASE64_KEY,
        CLOSE_CODE,
        URI_KEY,
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
    #[inline]
    pub fn new(mut compressed_data: I, reset_val: usize) -> Option<Self> {
        let val = compressed_data.next()?;

        Some(DecompressContext {
            val,
            compressed_data,
            position: reset_val,
            reset_val,

            // Init dictionary with default codes
            dictionary: vec![vec![0], vec![1], vec![CLOSE_CODE]],
            enlarge_in: 4,
            num_bits: 3,
        })
    }

    #[inline]
    pub fn read_bit(&mut self) -> Option<bool> {
        let res = self.val & (self.position as u16);
        self.position >>= 1;

        if self.position == 0 {
            self.position = self.reset_val;
            self.val = self.compressed_data.next()?;
        }

        Some(res != 0)
    }

    #[inline]
    pub fn read_bits(&mut self, n: usize) -> Option<u16> {
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
pub fn decompress_internal<I>(compressed: I, bits_per_char: usize) -> Option<Vec<u16>>
where
    I: Iterator<Item = u16>,
{
    assert!(bits_per_char <= std::mem::size_of::<u16>() * 8);

    let size_hint = compressed.size_hint();
    let max_input_len = size_hint.1.unwrap_or(200);

    let reset_val = 1 << (bits_per_char - 1);
    let mut ctx = match DecompressContext::new(compressed, reset_val) {
        Some(ctx) => ctx,
        None => return Some(Vec::new()),
    };
    ctx.dictionary.reserve(max_input_len);

    let next = ctx.read_bits(2)?;
    let first_entry: u16 = match next {
        0 | 1 => {
            let bits_to_read = (next * 8) + 8;
            ctx.read_bits(bits_to_read.into())?
        }
        CLOSE_CODE => return Some(Vec::new()),
        _ => return None,
    };
    ctx.dictionary.insert(3, vec![first_entry]);

    let mut w = vec![first_entry];
    let mut result = vec![first_entry];
    let mut entry: Vec<u16> = Vec::new();

    result.reserve(max_input_len);
    loop {
        let mut cc = ctx.read_bits(ctx.num_bits as usize)? as usize;
        match cc as u16 {
            0 | 1 => {
                let bits_to_read = (cc * 8) + 8;
                // if cc == 0 {
                // if (errorCount++ > 10000) return "Error"; // TODO: Error logic
                // }

                let bits = ctx.read_bits(bits_to_read as usize)? as u16;
                ctx.add_to_dictionary(vec![bits]);

                cc = ctx.dictionary.len() - 1;
            }
            CLOSE_CODE => return Some(result),
            _ => {}
        }

        if let Some(entry_value) = ctx.dictionary.get(cc as usize) {
            // entry = entry_value.clone()
            entry.clear();
            entry.extend(entry_value);
        } else if usize::from(cc) == ctx.dictionary.len() {
            // entry = w.clone();
            // entry.push(*w.get(0)?);
            entry.clear();
            entry.extend(&w);
            entry.push(*w.get(0)?);
        } else {
            return None;
        }

        result.extend(&entry);

        // Add w+entry[0] to the dictionary.
        let mut to_be_inserted = w.clone();
        to_be_inserted.push(*entry.get(0)?);
        ctx.add_to_dictionary(to_be_inserted);

        // w = entry
        w.clear();
        w.extend(&entry);
    }
}
