use crate::{
    constants::{BASE64_KEY, CLOSE_CODE, URI_KEY},
    IntoWideIter,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub(crate) struct CompressContext<F> {
    dictionary: HashMap<Vec<u16>, u16>,
    dictionary_to_create: HashSet<Vec<u16>>,
    wc: Vec<u16>,
    w: Vec<u16>,
    enlarge_in: usize,
    dict_size: usize,
    num_bits: usize,

    // result: Vec<u16>,

    // Data
    output: Vec<u16>,
    val: u16,
    position: usize,
    // Limits
    bits_per_char: usize,
    to_char: F,
}

impl<F> CompressContext<F>
where
    F: Fn(u16) -> u16,
{
    #[inline]
    pub fn new(bits_per_char: usize, to_char: F) -> Self {
        CompressContext {
            dictionary: Default::default(),
            dictionary_to_create: HashSet::new(),
            wc: Vec::new(),
            w: Vec::new(),
            enlarge_in: 2,
            dict_size: 3,
            num_bits: 2,

            // result: Vec::new(),
            output: Vec::new(),
            val: 0,
            position: 0,
            bits_per_char,
            to_char,
        }
    }

    #[inline]
    pub fn produce_w(&mut self) {
        if self.dictionary_to_create.contains(&self.w) {
            let first_w_char = self.w[0];
            if first_w_char < 256 {
                self.write_bits(self.num_bits, 0);
                self.write_bits(8, first_w_char);
            } else {
                self.write_bits(self.num_bits, 1);
                self.write_bits(16, first_w_char);
            }
            self.decrement_enlarge_in();
            self.dictionary_to_create.remove(&self.w);
        } else {
            self.write_bits(self.num_bits, *self.dictionary.get(&self.w).unwrap());
        }
        self.decrement_enlarge_in();
    }

    #[inline]
    pub fn write_bit(&mut self, value: u16) {
        self.val = (self.val << 1) | value;
        if self.position == self.bits_per_char - 1 {
            self.position = 0;
            let char_data = (self.to_char)(self.val);
            self.output.push(char_data);
            self.val = 0;
        } else {
            self.position += 1;
        }
    }

    #[inline]
    pub fn write_bits(&mut self, n: usize, mut value: u16) {
        for _ in 0..n {
            self.write_bit(value & 1);
            value >>= 1;
        }
    }

    #[inline]
    pub fn decrement_enlarge_in(&mut self) {
        self.enlarge_in -= 1;
        if self.enlarge_in == 0 {
            self.enlarge_in = 2_usize.pow(self.num_bits as u32);
            self.num_bits += 1;
        }
    }

    /// Compress a `u16`. This represents a wide char.
    #[inline]
    pub fn write_u16(&mut self, c: u16) {
        let c = vec![c];
        if !self.dictionary.contains_key(&c) {
            self.dictionary.insert(c.clone(), self.dict_size as u16);
            self.dict_size += 1;
            self.dictionary_to_create.insert(c.clone());
        }

        self.wc = self.w.clone();
        self.wc.extend(&c);
        if self.dictionary.contains_key(&self.wc) {
            self.w = std::mem::take(&mut self.wc);
        } else {
            self.produce_w();
            // Add wc to the dictionary.
            self.dictionary
                .insert(self.wc.clone(), self.dict_size as u16);
            self.dict_size += 1;
            self.w = c;
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
        self.write_bits(self.num_bits, CLOSE_CODE);

        let str_len = self.output.len();
        // Flush the last char
        while self.output.len() == str_len {
            self.write_bit(0);
        }

        self.output
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
        u16::from(
            *URI_KEY
                .get(usize::from(n))
                .expect("Invalid index into `URI_KEY` in `compress_to_encoded_uri_component`"),
        )
    });

    String::from_utf16(&compressed)
        .expect("`compress_to_encoded_uri_component` output was not valid unicode`")
}

/// Compress a string into a [`String`], which is valid base64.
///
/// This function converts the result back into a Rust [`String`] since it is guaranteed to be valid unicode.
pub fn compress_to_base64(data: impl IntoWideIter) -> String {
    let mut compressed = compress_internal(data.into_wide_iter(), 6, |n| {
        u16::from(
            *BASE64_KEY
                .get(usize::from(n))
                .expect("Invalid index into `BASE64_KEY` in `compress_to_base64`"),
        )
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
pub fn compress_internal<I, F>(uncompressed: I, bits_per_char: usize, to_char: F) -> Vec<u16>
where
    I: Iterator<Item = u16>,
    F: Fn(u16) -> u16,
{
    let mut ctx = CompressContext::new(bits_per_char, to_char);
    uncompressed.for_each(|c| ctx.write_u16(c));
    ctx.finish()
}
