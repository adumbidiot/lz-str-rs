use crate::constants::{
    BASE64_KEY,
    URI_KEY,
};
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct CompressContext<F: Fn(u32) -> u32> {
    dictionary: HashMap<String, u32>,
    dictionary_to_create: HashMap<String, bool>, // TODO: Hashset?
    wc: String,
    w: String,
    enlarge_in: usize,
    dict_size: usize,
    num_bits: usize,
    result: String,
    // Data
    output: Vec<u32>,
    val: u32,
    position: usize,
    // Limits
    bits_per_char: usize,
    to_char: F,
}

impl<F: Fn(u32) -> u32> CompressContext<F> {
    #[inline]
    pub fn new(bits_per_char: usize, to_char: F) -> Self {
        CompressContext {
            dictionary: HashMap::new(),
            dictionary_to_create: HashMap::new(),
            wc: String::new(),
            w: String::new(),
            enlarge_in: 2,
            dict_size: 3,
            num_bits: 2,
            result: String::new(),
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
            let first_w_char = self.w.chars().next().unwrap() as u32;
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
    pub fn write_bit(&mut self, value: u32) {
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
    pub fn write_bits(&mut self, n: usize, mut value: u32) {
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
pub fn compress(input: &str) -> Vec<u32> {
    compress_internal(input, 16, |n| n)
}

/// Compress a [`&str`] as a valid [`String`].
///
#[inline]
pub fn compress_to_utf16(input: &str) -> String {
    let buf = compress_internal(input, 15, |n| n + 32);
    buf.into_iter()
        .map(|i| std::char::from_u32(i).expect("`compress_to_utf16 output was not valid unicode`"))
        .chain(std::iter::once(' '))
        .collect()
}

/// Compress a [`&str`] into a [`String`], which can be safely used in a uri.
///
#[inline]
pub fn compress_to_encoded_uri_component(data: &str) -> String {
    compress_internal(data, 6, |n| {
        u32::from(
            URI_KEY
                .chars()
                .nth(n as usize)
                .expect("Invalid index into `URI_KEY` in `compress_to_encoded_uri_component`"),
        )
    })
    .into_iter()
    .map(|c| {
        std::char::from_u32(c)
            .expect("`compress_to_encoded_uri_component` output was not valid unicode`")
    })
    .collect()
}

/// Compress a [`&str`] into a [`String`], which is valid base64.
///
pub fn compress_to_base64(data: &str) -> String {
    let mut compressed = compress_internal(data, 6, |n| {
        u32::from(
            BASE64_KEY
                .chars()
                .nth(n as usize)
                .expect("Invalid index into `BASE64_KEY` in `compress_to_base64`"),
        )
    });

    for _ in 0..(compressed.len() % 4) {
        compressed.push(u32::from('='));
    }

    compressed
        .into_iter()
        .map(|c| {
            std::char::from_u32(c).expect("`compress_to_base64` output was not valid unicode`")
        })
        .collect()
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
pub fn compress_internal<F: Fn(u32) -> u32>(
    uncompressed: &str,
    bits_per_char: usize,
    to_char: F,
) -> Vec<u32> {
    let mut ctx = CompressContext::new(bits_per_char, to_char);
    uncompressed.chars().for_each(|c| {
        let c_str = c.to_string();
        if !ctx.dictionary.contains_key(&c_str) {
            ctx.dictionary.insert(c_str.clone(), ctx.dict_size as u32);
            ctx.dict_size += 1;
            ctx.dictionary_to_create.insert(c_str.clone(), true);
        }

        ctx.wc = ctx.w.clone() + &c_str;
        if ctx.dictionary.contains_key(&ctx.wc) {
            ctx.w = ctx.wc.clone();
        } else {
            ctx.produce_w();
            // Add wc to the dictionary.
            ctx.dictionary.insert(ctx.wc.clone(), ctx.dict_size as u32);
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
