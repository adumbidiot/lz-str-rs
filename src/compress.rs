use crate::constants::{
    BASE64_KEY,
    URI_KEY,
};
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct CompressContext<F: Fn(u16) -> u16> {
    dictionary: HashMap<Vec<u16>, u16>,
    dictionary_to_create: HashMap<Vec<u16>, bool>, // TODO: Hashset?
    wc: Vec<u16>,
    w: Vec<u16>, // String,
    enlarge_in: usize,
    dict_size: usize,
    num_bits: usize,
    result: Vec<u16>,
    // Data
    output: Vec<u16>,
    val: u16,
    position: usize,
    // Limits
    bits_per_char: usize,
    to_char: F,
}

impl<F: Fn(u16) -> u16> CompressContext<F> {
    #[inline]
    pub fn new(bits_per_char: usize, to_char: F) -> Self {
        CompressContext {
            dictionary: HashMap::new(),
            dictionary_to_create: HashMap::new(),
            wc: Vec::new(),
            w: Vec::new(),
            enlarge_in: 2,
            dict_size: 3,
            num_bits: 2,
            result: Vec::new(),
            output: Vec::new(),
            val: 0,
            position: 0,
            bits_per_char,
            to_char,
        }
    }

    #[inline]
    pub fn produce_w(&mut self) {
        if self.dictionary_to_create.contains_key(&self.w) {
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
}

/// Compress a [`&str`] into a [`Vec`] of [`u32`]s, which represent possibly invalid UTF16.
///
#[inline]
pub fn compress(input: &str) -> Vec<u16> {
    compress_internal(input.encode_utf16(), 16, |n| n)
}

/// Compress a [`&str`] as a valid [`String`].
///
#[inline]
pub fn compress_to_utf16(input: &str) -> String {
    let compressed = compress_internal(input.encode_utf16(), 15, |n| n + 32);
    let mut compressed =
        String::from_utf16(&compressed).expect("`compress_to_utf16 output was not valid unicode`");
    compressed.push(' ');

    compressed
}

/// Compress a [`&str`] into a [`String`], which can be safely used in a uri.
///
#[inline]
pub fn compress_to_encoded_uri_component(data: &str) -> String {
    let compressed = compress_internal(data.encode_utf16(), 6, |n| {
        u16::from(
            *URI_KEY
                .get(n as usize)
                .expect("Invalid index into `URI_KEY` in `compress_to_encoded_uri_component`"),
        )
    });

    String::from_utf16(&compressed)
        .expect("`compress_to_encoded_uri_component` output was not valid unicode`")
}

/// Compress a [`&str`] into a [`String`], which is valid base64.
///
pub fn compress_to_base64(data: &str) -> String {
    let mut compressed = compress_internal(data.encode_utf16(), 6, |n| {
        u16::from(
            *BASE64_KEY
                .get(n as usize)
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

/// Compress a [`&str`] into a [`Vec`] of [`u8`].
///
pub fn compress_to_uint8_array(data: &str) -> Vec<u8> {
    let compressed = compress(data);

    let mut buf = Vec::with_capacity(compressed.len() * 2);

    for val in compressed.into_iter() {
        buf.push((val >> 8) as u8);
        buf.push((val & 0xFF) as u8);
    }

    buf
}

/// The internal function for compressing data.
/// All other compression functions are built on top of this.
/// It generally should not be used directly.
///
#[inline]
pub fn compress_internal<I: Iterator<Item = u16>, F: Fn(u16) -> u16>(
    uncompressed: I,
    bits_per_char: usize,
    to_char: F,
) -> Vec<u16> {
    let mut ctx = CompressContext::new(bits_per_char, to_char);
    uncompressed.for_each(|c| {
        let c_str = vec![c]; //c.to_string();
        if !ctx.dictionary.contains_key(&c_str) {
            ctx.dictionary.insert(c_str.clone(), ctx.dict_size as u16);
            ctx.dict_size += 1;
            ctx.dictionary_to_create.insert(c_str.clone(), true);
        }

        ctx.wc = ctx.w.clone();
        ctx.wc.extend(&c_str);
        if ctx.dictionary.contains_key(&ctx.wc) {
            ctx.w = ctx.wc.clone();
        } else {
            ctx.produce_w();
            // Add wc to the dictionary.
            ctx.dictionary.insert(ctx.wc.clone(), ctx.dict_size as u16);
            ctx.dict_size += 1;
            ctx.w = c_str;
        }
    });

    // Output the code for w.
    if !ctx.w.is_empty() {
        ctx.produce_w();
    }

    // Mark the end of the stream
    ctx.write_bits(ctx.num_bits, 2);

    let str_len = ctx.output.len();
    // Flush the last char
    while ctx.output.len() == str_len {
        ctx.write_bit(0);
    }

    ctx.output
}
