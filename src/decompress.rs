use crate::constants::BASE64_KEY;
use crate::constants::CLOSE_CODE;
use crate::constants::START_CODE_BITS;
use crate::constants::U16_CODE;
use crate::constants::U8_CODE;
use crate::constants::URI_KEY;
use crate::IntoWideIter;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
pub struct DecompressContext<I> {
    val: u16,
    compressed_data: I,
    position: usize,
    reset_val: usize,
}

impl<I> DecompressContext<I>
where
    I: Iterator<Item = u16>,
{
    /// Make a new [`DecompressContext`].
    ///
    /// # Errors
    /// Returns `None` if the iterator is empty.
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
        let res: u16 = self.val & (u16::try_from(self.position).unwrap());
        self.position >>= 1;

        if self.position == 0 {
            self.position = self.reset_val;
            self.val = self.compressed_data.next()?;
        }

        Some(res != 0)
    }

    /// Read n bits.
    ///
    /// `u32` is the return type as we expect all possible codes to be within that type's range.
    #[inline]
    pub fn read_bits(&mut self, n: u8) -> Option<u32> {
        let mut res = 0;
        let max_power: u32 = 1 << n;
        let mut power: u32 = 1;
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
#[inline]
pub fn decompress(compressed: impl IntoWideIter) -> Option<Vec<u16>> {
    decompress_internal(compressed.into_wide_iter(), 16)
}

/// Decompress a [`&str`] compressed with [`crate::compress_to_utf16`].
///
/// # Errors
/// Returns an error if the compressed data could not be decompressed.
#[inline]
pub fn decompress_from_utf16(compressed: &str) -> Option<Vec<u16>> {
    decompress_internal(compressed.encode_utf16().map(|c| c - 32), 15)
}

/// Decompress a [`&str`] compressed with [`crate::compress_to_encoded_uri_component`].
///
/// # Errors
/// Returns an error if the compressed data could not be decompressed.
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
        .flat_map(|c| {
            URI_KEY
                .iter()
                .position(|k| u8::try_from(c) == Ok(*k))
                .map(|n| u16::try_from(n).ok())
        })
        .collect();

    decompress_internal(compressed?.into_iter(), 6)
}

/// Decompress a [`&str`] compressed with [`crate::compress_to_base64`].
///
/// # Errors
/// Returns an error if the compressed data could not be decompressed.
#[inline]
pub fn decompress_from_base64(compressed: &str) -> Option<Vec<u16>> {
    let compressed: Option<Vec<u16>> = compressed
        .encode_utf16()
        .flat_map(|c| {
            BASE64_KEY
                .iter()
                .position(|k| u8::try_from(c) == Ok(*k))
                .map(|n| u16::try_from(n).ok())
        })
        .collect();

    decompress_internal(compressed?.into_iter(), 6)
}

/// Decompress a byte slice compressed with [`crate::compress_to_uint8_array`].
///
/// # Errors
/// Returns an error if the compressed data could not be decompressed.
#[inline]
pub fn decompress_from_uint8_array(compressed: &[u8]) -> Option<Vec<u16>> {
    // The buffer is a UCS2 big endian encoded string.
    // If it is not a multiple of 2, it is invalid.
    let compressed_len = compressed.len();
    if compressed_len & 1 == 1 {
        return None;
    }

    let buffer: Vec<u16> = compressed
        .chunks(2)
        .map(|slice| {
            // The slice is always guaranteed to be 2 here.
            // We check to see if the length is a multiple of 2 earlier.
            u16::from_be_bytes(slice.try_into().unwrap())
        })
        .collect();

    decompress(&buffer)
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
#[inline]
pub fn decompress_internal<I>(compressed: I, bits_per_char: u8) -> Option<Vec<u16>>
where
    I: Iterator<Item = u16>,
{
    assert!(usize::from(bits_per_char) <= std::mem::size_of::<u16>() * 8);

    // u16::MAX < u32::MAX
    let reset_val_pow = bits_per_char - 1;
    let reset_val = 1 << reset_val_pow;
    let mut ctx = match DecompressContext::new(compressed, reset_val) {
        Some(ctx) => ctx,
        None => return Some(Vec::new()),
    };

    let mut dictionary: Vec<Vec<u16>> = Vec::with_capacity(16);
    for i in 0_u16..3_u16 {
        dictionary.push(vec![i]);
    }

    // u8::MAX > u2::MAX
    let code = u8::try_from(ctx.read_bits(START_CODE_BITS)?).unwrap();
    let first_entry = match code {
        U8_CODE | U16_CODE => {
            let bits_to_read = (code * 8) + 8;
            // bits_to_read == 8 or 16 <= 16
            u16::try_from(ctx.read_bits(bits_to_read)?).unwrap()
        }
        CLOSE_CODE => return Some(Vec::new()),
        _ => return None,
    };
    dictionary.push(vec![first_entry]);

    let mut w = vec![first_entry];
    let mut result = vec![first_entry];
    let mut num_bits: u8 = 3;
    let mut enlarge_in: u64 = 4;
    let mut entry;
    loop {
        let mut code = ctx.read_bits(num_bits)?;
        match u8::try_from(code) {
            Ok(code_u8 @ (U8_CODE | U16_CODE)) => {
                let bits_to_read = (code_u8 * 8) + 8;
                // if cc == 0 {
                // if (errorCount++ > 10000) return "Error"; // TODO: Error logic
                // }

                // bits_to_read == 8 or 16 <= 16
                let bits = u16::try_from(ctx.read_bits(bits_to_read)?).unwrap();
                dictionary.push(vec![bits]);
                code = u32::try_from(dictionary.len() - 1).ok()?;
                enlarge_in -= 1;
            }
            Ok(CLOSE_CODE) => return Some(result),
            _ => {}
        }

        if enlarge_in == 0 {
            enlarge_in = 1 << num_bits;
            num_bits += 1;
        }

        // Return error if code cannot be converted to dictionary index
        let code_usize = usize::try_from(code).ok()?;
        if let Some(entry_value) = dictionary.get(code_usize) {
            entry = entry_value.clone();
        } else if code_usize == dictionary.len() {
            entry = w.clone();
            entry.push(*w.first()?);
        } else {
            return None;
        }

        result.extend(&entry);

        // Add w+entry[0] to the dictionary.
        let mut to_be_inserted = w.clone();
        to_be_inserted.push(*entry.first()?);
        dictionary.push(to_be_inserted);
        enlarge_in -= 1;

        w = entry;

        if enlarge_in == 0 {
            enlarge_in = 1 << num_bits;
            num_bits += 1;
        }
    }
}
