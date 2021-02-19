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
}

impl<I: Iterator<Item = u16>> DecompressContext<I> {
    /// Make a new [`DecompressContext`].
    ///
    /// # Errors
    /// Returns `None` if the iterator is empty.
    ///
    #[inline]
    pub fn new(mut compressed_data: I, reset_val: usize) -> Option<Self> {
        Some(DecompressContext {
            val: compressed_data.next()?,
            compressed_data,
            position: reset_val,
            reset_val,
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
    pub fn read_bits(&mut self, n: usize) -> Option<u32> {
        let mut res = 0;
        let max_power = 2_u32.pow(n as u32);
        let mut power = 1;
        while power != max_power {
            res |= u32::from(self.read_bit()?) * power;
            power <<= 1;
        }

        Some(res)
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

    let reset_val_pow = u32::try_from(bits_per_char).ok()? - 1;
    let reset_val = 2_usize.pow(reset_val_pow);
    let mut ctx = match DecompressContext::new(compressed, reset_val) {
        Some(ctx) => ctx,
        None => return Some(Vec::new()),
    };

    let mut dictionary: Vec<Vec<u16>> = Vec::with_capacity(3);
    for i in 0_u16..3_u16 {
        dictionary.push(vec![i]);
    }

    let next = ctx.read_bits(2)?;
    let first_entry: u16 = match next as u16 {
        0 | 1 => {
            let bits_to_read = (next * 8) + 8;
            ctx.read_bits(bits_to_read as usize)? as u16
        }
        CLOSE_CODE => return Some(Vec::new()),
        _ => return None,
    };
    dictionary.insert(3, vec![first_entry]);

    let mut w = vec![first_entry];
    let mut result = vec![first_entry];
    let mut num_bits = 3;
    let mut enlarge_in = 4;
    let mut dict_size = 4;
    let mut entry;
    loop {
        let mut cc = ctx.read_bits(num_bits)? as usize;
        match cc as u16 {
            0 | 1 => {
                let bits_to_read = (cc * 8) + 8;
                // if cc == 0 {
                // if (errorCount++ > 10000) return "Error"; // TODO: Error logic
                // }

                let bits = ctx.read_bits(bits_to_read as usize)? as u16;
                dictionary.push(vec![bits]);
                dict_size += 1;
                cc = dict_size - 1;
                enlarge_in -= 1;
            }
            CLOSE_CODE => return Some(result),
            _ => {}
        }

        if enlarge_in == 0 {
            enlarge_in = 2_u32.pow(num_bits as u32);
            num_bits += 1;
        }

        if let Some(entry_value) = dictionary.get(cc as usize) {
            entry = entry_value.clone();
        } else if cc == dict_size {
            entry = w.clone();
            entry.push(*w.get(0)?);
        } else {
            return None;
        }

        result.extend(&entry);

        // Add w+entry[0] to the dictionary.
        let mut to_be_inserted = w.clone();
        to_be_inserted.push(*entry.get(0)?);
        dictionary.push(to_be_inserted);
        dict_size += 1;
        enlarge_in -= 1;

        w = entry;

        if enlarge_in == 0 {
            enlarge_in = 2_u32.pow(num_bits as u32);
            num_bits += 1;
        }
    }
}
