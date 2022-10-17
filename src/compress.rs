use crate::constants::BASE64_KEY;
use crate::constants::CLOSE_CODE;
use crate::constants::URI_KEY;
use crate::IntoWideIter;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;

/// The starting size of a codepoint.
///
/// Compression starts with the following codes:
/// 0: u8
/// 1: u16
/// 2: close stream
const START_NUM_BITS: u8 = 2;

/// The stream code for a `u8`.
const U8_CODE: u32 = 0;

/// The stream code for a `u16`.
const U16_CODE: u32 = 1;

/// The number of "base codes",
/// the default codes of all streams.
///
/// These are U8_CODE, U16_CODE, and CLOSE_CODE.
const NUM_BASE_CODES: usize = 3;

#[derive(Debug)]
pub(crate) struct CompressContext<'a, F> {
    dictionary: HashMap<Vec<u16>, u32>,
    dictionary_to_create: HashSet<Vec<u16>>,

    w: Vec<u16>,
    enlarge_in: usize,

    /// The input buffer.
    input: &'a [u16],

    /// The output buffer.
    output: Vec<u16>,

    /// The bit buffer.
    bit_buffer: u16,

    /// The current number of bits in a code.
    ///
    /// This is a u8,
    /// because we currently assume the max code size is 32 bits.
    /// 32 < u8::MAX
    num_bits: u8,

    /// The current bit position.
    bit_position: u8,

    /// The maximum # of bits per char.
    ///
    /// This value may not exceed 16,
    /// as the reference implementation will also not handle values over 16.
    bits_per_char: u8,

    /// A transformation function to map a u16 to another u16,
    /// before appending it to the output buffer.
    to_char: F,
}

impl<'a, F> CompressContext<'a, F>
where
    F: Fn(u16) -> u16,
{
    /// Make a new [`CompressContext`].
    ///
    /// # Panics
    /// Panics if `bits_per_char` exceeds the number of bits in a u16.
    #[inline]
    pub fn new(input: &'a [u16], bits_per_char: u8, to_char: F) -> Self {
        assert!(usize::from(bits_per_char) <= std::mem::size_of::<u16>() * 8);

        CompressContext {
            dictionary: HashMap::with_capacity(16),
            dictionary_to_create: HashSet::with_capacity(16),

            w: Vec::new(),
            enlarge_in: 2,

            input,
            output: Vec::with_capacity(64),

            bit_buffer: 0,

            num_bits: START_NUM_BITS,

            bit_position: 0,
            bits_per_char,
            to_char,
        }
    }

    #[inline]
    pub fn produce_w(&mut self) {
        if self.dictionary_to_create.contains(&self.w) {
            let first_w_char = self.w[0];
            if first_w_char < 256 {
                self.write_bits(self.num_bits, U8_CODE);
                self.write_bits(8, first_w_char.into());
            } else {
                self.write_bits(self.num_bits, U16_CODE);
                self.write_bits(16, first_w_char.into());
            }
            self.decrement_enlarge_in();
            self.dictionary_to_create.remove(&self.w);
        } else {
            self.write_bits(self.num_bits, *self.dictionary.get(&self.w).unwrap());
        }
        self.decrement_enlarge_in();
    }

    /// Append the bit to the bit buffer.
    #[inline]
    pub fn write_bit(&mut self, bit: bool) {
        self.bit_buffer = (self.bit_buffer << 1) | u16::from(bit);
        self.bit_position += 1;

        if self.bit_position == self.bits_per_char {
            self.bit_position = 0;
            let output_char = (self.to_char)(self.bit_buffer);
            self.bit_buffer = 0;

            self.output.push(output_char);
        }
    }

    #[inline]
    pub fn write_bits(&mut self, n: u8, mut value: u32) {
        for _ in 0..n {
            self.write_bit(value & 1 == 1);
            value >>= 1;
        }
    }

    #[inline]
    pub fn decrement_enlarge_in(&mut self) {
        self.enlarge_in -= 1;
        if self.enlarge_in == 0 {
            self.enlarge_in = 2_usize.pow(self.num_bits.into());
            self.num_bits += 1;
        }
    }

    /// Compress a `u16`. This represents a wide char.
    #[inline]
    pub fn write_u16(&mut self, c: &'a u16) {
        let c = std::slice::from_ref(c);
        if !self.dictionary.contains_key(c) {
            self.dictionary.insert(
                c.to_vec(),
                (self.dictionary.len() + NUM_BASE_CODES).try_into().unwrap(),
            );
            self.dictionary_to_create.insert(c.to_vec());
        }

        let mut wc = self.w.clone();
        wc.extend(c);
        if self.dictionary.contains_key(&wc) {
            self.w = wc;
        } else {
            self.produce_w();
            // Add wc to the dictionary.
            self.dictionary.insert(
                wc.to_vec(),
                (self.dictionary.len() + NUM_BASE_CODES).try_into().unwrap(),
            );
            self.w = c.to_vec();
        }
    }

    /// Finish the stream and get the final result.
    #[inline]
    pub fn finish(mut self) -> Vec<u16> {
        // Output the code for w.
        if !self.w.is_empty() {
            self.produce_w();
        }

        // Mark the end of the stream
        self.write_bits(self.num_bits, CLOSE_CODE.into());

        let str_len = self.output.len();
        // Flush the last char
        while self.output.len() == str_len {
            self.write_bit(false);
        }

        self.output
    }

    /// Perform the compression and return the result.
    pub fn compress(mut self) -> Vec<u16> {
        for c in self.input {
            self.write_u16(c);
        }
        self.finish()
    }
}

/// Compress a string into a [`Vec<u16>`].
///
/// The resulting [`Vec`] may contain invalid UTF16.
#[inline]
pub fn compress(input: impl IntoWideIter) -> Vec<u16> {
    compress_internal(input.into_wide_iter(), 16, std::convert::identity)
}

/// Compress a string as a valid [`String`].
///
/// This function converts the result back into a Rust [`String`] since it is guaranteed to be valid UTF16.
#[inline]
pub fn compress_to_utf16(input: impl IntoWideIter) -> String {
    let compressed = compress_internal(input.into_wide_iter(), 15, |n| n + 32);
    let mut compressed =
        String::from_utf16(&compressed).expect("`compress_to_utf16 output was not valid unicode`");
    compressed.push(' ');

    compressed
}

/// Compress a string into a [`String`], which can be safely used in a uri.
///
/// This function converts the result back into a Rust [`String`] since it is guaranteed to be valid unicode.
#[inline]
pub fn compress_to_encoded_uri_component(data: impl IntoWideIter) -> String {
    let compressed = compress_internal(data.into_wide_iter(), 6, |n| {
        u16::from(URI_KEY[usize::from(n)])
    });

    String::from_utf16(&compressed)
        .expect("`compress_to_encoded_uri_component` output was not valid unicode`")
}

/// Compress a string into a [`String`], which is valid base64.
///
/// This function converts the result back into a Rust [`String`] since it is guaranteed to be valid unicode.
pub fn compress_to_base64(data: impl IntoWideIter) -> String {
    let mut compressed = compress_internal(data.into_wide_iter(), 6, |n| {
        u16::from(BASE64_KEY[usize::from(n)])
    });

    let mod_4 = compressed.len() % 4;

    if mod_4 != 0 {
        for _ in mod_4..(4 + 1) {
            compressed.push(u16::from(b'='));
        }
    }

    String::from_utf16(&compressed).expect("`compress_to_base64` output was not valid unicode`")
}

/// Compress a string into a [`Vec<u8>`].
pub fn compress_to_uint8_array(data: impl IntoWideIter) -> Vec<u8> {
    compress(data)
        .into_iter()
        .flat_map(|value| value.to_be_bytes())
        .collect()
}

/// The internal function for compressing data.
///
/// All other compression functions are built on top of this.
/// It generally should not be used directly.
#[inline]
pub fn compress_internal<I, F>(uncompressed: I, bits_per_char: u8, to_char: F) -> Vec<u16>
where
    I: Iterator<Item = u16>,
    F: Fn(u16) -> u16,
{
    let input: Vec<u16> = uncompressed.collect();
    let ctx = CompressContext::new(&input, bits_per_char, to_char);
    ctx.compress()
}
