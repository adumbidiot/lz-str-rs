use std::collections::HashMap;

#[derive(Debug)]
pub struct CompressContext<F: Fn(u32) -> u32> {
    pub dictionary: HashMap<String, u32>,
    pub dictionary_to_create: HashMap<String, bool>, //TODO: Hashset?
    pub wc: String,
    pub w: String,
    enlarge_in: usize,
    pub dict_size: usize,
    pub num_bits: usize,
    result: String,
    //Data
    pub output: Vec<u32>,
    val: u32,
    position: usize,
    //Limits
    bits_per_char: usize,
    transform_fn: F,
}

impl<F: Fn(u32) -> u32> CompressContext<F> {
    pub fn new(bits_per_char: usize, transform_fn: F) -> Self {
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
            transform_fn,
        }
    }

    pub fn produce_w(&mut self) {
        if self.dictionary_to_create.contains_key(&self.w) {
            let first_w_char = self.w.chars().next().expect("Next Char") as u32;
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
            self.write_bits(self.num_bits, *self.dictionary.get(&self.w).expect("dict"));
        }
        self.decrement_enlarge_in();
    }

    pub fn write_bit(&mut self, value: u32) {
        self.val = (self.val << 1) | value;
        if self.position == self.bits_per_char - 1 {
            self.position = 0;
            let n = (self.transform_fn)(self.val);
            self.output.push(n);
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
