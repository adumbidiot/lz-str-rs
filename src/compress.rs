use crate::constants::BASE64_KEY;
use crate::constants::CLOSE_CODE;
use crate::constants::START_CODE_BITS;
use crate::constants::U16_CODE;
use crate::constants::U8_CODE;
use crate::constants::URI_KEY;
use crate::IntoWideIter;
use std::collections::hash_map::Entry as HashMapEntry;
use std::convert::TryInto;

#[cfg(not(feature = "rustc-hash"))]
type HashMap<K, V> = std::collections::HashMap<K, V>;

#[cfg(not(feature = "rustc-hash"))]
type HashSet<T> = std::collections::HashSet<T>;

#[cfg(feature = "rustc-hash")]
type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;

#[cfg(feature = "rustc-hash")]
type HashSet<T> = rustc_hash::FxHashSet<T>;

/// The number of "base codes",
/// the default codes of all streams.
///
/// These are U8_CODE, U16_CODE, and CLOSE_CODE.
const NUM_BASE_CODES: usize = 3;

#[derive(Debug)]
pub(crate) struct CompressContext<'a, F> {
    dictionary: HashMap<&'a [u16], u32>,
    dictionary_to_create: HashSet<u16>,

    /// The current word, w,
    /// in terms of indexes into the input.
    w_start_idx: usize,
    w_end_idx: usize,

    // The counter for increasing the current number of bits in a code.
    // The max size of this is 1 << max(num_bits) == 1 + u32::MAX, so we use u64.
    enlarge_in: u64,

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
            dictionary: HashMap::default(),
            dictionary_to_create: HashSet::default(),

            w_start_idx: 0,
            w_end_idx: 0,

            enlarge_in: 2,

            input,
            output: Vec::with_capacity(input.len() >> 1), // Lowball, assume we can get a 50% reduction in size.

            bit_buffer: 0,

            num_bits: START_CODE_BITS,

            bit_position: 0,
            bits_per_char,
            to_char,
        }
    }

    #[inline]
    pub fn produce_w(&mut self) {
        let w = &self.input[self.w_start_idx..self.w_end_idx];

        match w
            .first()
            .map(|first_w_char| self.dictionary_to_create.take(first_w_char))
        {
            Some(Some(first_w_char)) => {
                if first_w_char < 256 {
                    self.write_bits(self.num_bits, U8_CODE.into());
                    self.write_bits(8, first_w_char.into());
                } else {
                    self.write_bits(self.num_bits, U16_CODE.into());
                    self.write_bits(16, first_w_char.into());
                }
                self.decrement_enlarge_in();
            }
            None | Some(None) => {
                self.write_bits(self.num_bits, *self.dictionary.get(w).unwrap());
            }
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
            self.enlarge_in = 1 << self.num_bits;
            self.num_bits += 1;
        }
    }

    /// Compress a `u16`. This represents a wide char.
    #[inline]
    pub fn write_u16(&mut self, i: usize) {
        let c = &self.input[i];

        let dictionary_len = self.dictionary.len();
        if let HashMapEntry::Vacant(entry) = self.dictionary.entry(std::slice::from_ref(c)) {
            entry.insert((dictionary_len + NUM_BASE_CODES).try_into().unwrap());
            self.dictionary_to_create.insert(*c);
        }

        // wc = w + c.
        let wc = &self.input[self.w_start_idx..self.w_end_idx + 1];
        if self.dictionary.contains_key(wc) {
            // w = wc.
            self.w_end_idx += 1;
        } else {
            self.produce_w();
            // Add wc to the dictionary.
            self.dictionary.insert(
                wc,
                (self.dictionary.len() + NUM_BASE_CODES).try_into().unwrap(),
            );

            // w = c.
            self.w_start_idx = i;
            self.w_end_idx = i + 1;
        }
    }

    /// Finish the stream and get the final result.
    #[inline]
    pub fn finish(mut self) -> Vec<u16> {
        let w = &self.input[self.w_start_idx..self.w_end_idx];

        // Output the code for w.
        if !w.is_empty() {
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
        for i in 0..self.input.len() {
            self.write_u16(i);
        }
        self.finish()
    }
}

/// Compress a string into a [`Vec<u16>`].
///
/// The resulting [`Vec`] may contain invalid UTF16.
#[inline]
pub fn compress(data: impl IntoWideIter) -> Vec<u16> {
    let data: Vec<u16> = data.into_wide_iter().collect();
    compress_internal(&data, 16, std::convert::identity)
}

/// Compress a string as a valid [`String`].
///
/// This function converts the result back into a Rust [`String`] since it is guaranteed to be valid UTF16.
#[inline]
pub fn compress_to_utf16(data: impl IntoWideIter) -> String {
    let data: Vec<u16> = data.into_wide_iter().collect();
    let compressed = compress_internal(&data, 15, |n| n + 32);
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
    let data: Vec<u16> = data.into_wide_iter().collect();
    let compressed = compress_internal(&data, 6, |n| u16::from(URI_KEY[usize::from(n)]));

    String::from_utf16(&compressed)
        .expect("`compress_to_encoded_uri_component` output was not valid unicode`")
}

/// Compress a string into a [`String`], which is valid base64.
///
/// This function converts the result back into a Rust [`String`] since it is guaranteed to be valid unicode.
pub fn compress_to_base64(data: impl IntoWideIter) -> String {
    let data: Vec<u16> = data.into_wide_iter().collect();
    let mut compressed = compress_internal(&data, 6, |n| u16::from(BASE64_KEY[usize::from(n)]));

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
pub fn compress_internal<F>(data: &[u16], bits_per_char: u8, to_char: F) -> Vec<u16>
where
    F: Fn(u16) -> u16,
{
    let ctx = CompressContext::new(data, bits_per_char, to_char);
    ctx.compress()
}
