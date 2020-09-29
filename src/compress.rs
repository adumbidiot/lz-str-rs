use crate::constants::URI_KEY;
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

    pub fn write_bits(&mut self, n: usize, mut value: u32) {
        //if (typeof(value)=="string")
        //	value = value.charCodeAt(0);
        for _i in 0..n {
            self.write_bit(value & 1);
            value >>= 1;
        }
    }

    pub fn decrement_enlarge_in(&mut self) {
        self.enlarge_in -= 1;
        if self.enlarge_in == 0 {
            self.enlarge_in = 2_usize.pow(self.num_bits as u32);
            self.num_bits += 1;
        }
    }
}

pub fn compress_str(input: &str) -> Vec<u32> {
    compress(input, 16, |n| n)
}

pub fn compress_uri(data: &str) -> Vec<u32> {
    compress(&data, 6, |n| {
        u32::from(URI_KEY.chars().nth(n as usize).unwrap())
    })
}

pub fn compress<F: Fn(u32) -> u32>(
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
