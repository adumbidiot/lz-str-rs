use std::collections::HashMap;
#[derive(Debug)]
pub struct DecompressContext {
    val: u32,
    string: Vec<char>,
    position: usize,
    index: usize,
    reset_val: usize,
}

impl DecompressContext {
    pub fn new(string: &str, reset_val: usize) -> Self {
        let mut string: Vec<char> = string.chars().collect();
        string.push(0 as char); //Js version seems to rely on being able to load a nonexistent byte, so just pad it here...? Maybe a bug in my impl?
        DecompressContext {
            val: string[0] as u32,
            string,
            position: reset_val,
            index: 1,
            reset_val,
        }
    }

    pub fn read_bit(&mut self) -> bool {
        let res = self.val & (self.position as u32);
        self.position >>= 1;

        if self.position == 0 {
            self.position = self.reset_val;
            self.val = self.string[self.index] as u32;
            self.index += 1;
        }

        res != 0
    }

    pub fn read_bits(&mut self, n: usize) -> u32 {
        let mut res = 0;
        let max_power = 2_u32.pow(n as u32);
        let mut power = 1;
        while power != max_power {
            res |= self.read_bit() as u32 * power;
            power <<= 1;
        }

        res
    }
}

pub fn decompress_uri(compressed: &str) -> Option<String> {
    let key = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-$";
    //let compressed = compressed.replace(" ", "+"); //Is this even necessary?
    let compressed: Option<String> = compressed
        .chars()
        .map(|c| {
            key.chars()
                .position(|k| c == k)
                .map(|n| std::char::from_u32(n as u32).unwrap())
        })
        .collect();
    decompress(&compressed?, 32)
}

pub fn decompress(compressed: &str, reset_val: usize) -> Option<String> {
    let mut ctx = DecompressContext::new(compressed, reset_val); //32768
    let mut dictionary: HashMap<u32, String> = HashMap::new();
    for i in 0_u8..3_u8 {
        dictionary.insert(i as u32, (i as char).to_string());
    }

    let next = ctx.read_bits(2);
    let first_entry = match next {
        0 | 1 => {
            let bits_to_read = (next * 8) + 8;
            let bits = ctx.read_bits(bits_to_read as usize);
            std::char::from_u32(bits)
        }
        _ => None,
    };
    dictionary.insert(3, first_entry?.to_string());

    let mut w = first_entry?.to_string();
    let mut result = first_entry?.to_string();
    let mut num_bits = 3;
    let mut enlarge_in = 4;
    let mut dict_size = 4;
    let mut entry;
    loop {
        let mut cc = ctx.read_bits(num_bits);
        match cc {
            0 | 1 => {
                let bits_to_read = (cc * 8) + 8;
                if cc == 0 {
                    //if (errorCount++ > 10000) return "Error"; //TODO: Error logic 
                }

                let bits = ctx.read_bits(bits_to_read as usize);
                let c = std::char::from_u32(bits).unwrap();
                dictionary.insert(dict_size, c.to_string());
                dict_size += 1;
                cc = dict_size - 1;
                enlarge_in -= 1;
            }
            2 => {
                return Some(result);
            }
            _ => {}
        }

        if enlarge_in == 0 {
            enlarge_in = 2_u32.pow(num_bits as u32);
            num_bits += 1;
        }

        if dictionary.contains_key(&cc) {
            entry = dictionary[&cc].clone();
        } else {
            if cc == dict_size {
                entry = w.clone();
                entry.push(w.chars().next()?);
            } else {
                return None;
            }
        }

        result += &entry;

        // Add w+entry[0] to the dictionary.
        let mut to_be_inserted = w.clone();
        to_be_inserted.push(entry.chars().next()?);
        dictionary.insert(dict_size, to_be_inserted);
        dict_size += 1;
        enlarge_in -= 1;

        w = entry;

        if enlarge_in == 0 {
            enlarge_in = 2_u32.pow(num_bits as u32);
            num_bits += 1;
        }
    }
}
