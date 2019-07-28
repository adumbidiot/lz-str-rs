mod context;

use crate::constants::URI_KEY;
use context::DecompressContext;
use std::{
    collections::HashMap,
    convert::TryFrom,
};

pub fn decompress_uri(compressed: &[u32]) -> Option<String> {
    //let compressed = compressed.replace(" ", "+"); //Is this even necessary?
    let compressed: Option<Vec<u32>> = compressed
        .iter()
        .map(|c| {
            URI_KEY
                .bytes()
                .position(|k| u8::try_from(*c) == Ok(k))
                .map(|n| u32::try_from(n).unwrap())
        })
        .collect();
    decompress(&compressed?, 6)
}

pub fn decompress_str(compressed: &[u32]) -> Option<String> {
    decompress(&compressed, 16)
}

pub fn decompress(compressed: &[u32], bits_per_char: usize) -> Option<String> {
    let reset_val = 2_usize.pow(u32::try_from(bits_per_char).unwrap() - 1);
    if compressed.is_empty() {
        return None;
    }

    let mut ctx = DecompressContext::new(compressed, reset_val);
    let mut dictionary: HashMap<u32, String> = HashMap::new();
    for i in 0_u8..3_u8 {
        dictionary.insert(u32::from(i), (i as char).to_string());
    }

    let next = ctx.read_bits(2);
    let first_entry = match next {
        0 | 1 => {
            let bits_to_read = (next * 8) + 8;
            let bits = ctx.read_bits(bits_to_read as usize);
            unsafe { std::char::from_u32_unchecked(bits) }
        }
        2 => return Some(String::new()),
        _v => return None,
    };
    dictionary.insert(3, first_entry.to_string());

    let mut w = first_entry.to_string();
    let mut result = first_entry.to_string();
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
                let c = unsafe { std::char::from_u32_unchecked(bits) };
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
                entry.push(w.chars().next().expect("next"));
            } else {
                return None;
            }
        }

        result += &entry;

        // Add w+entry[0] to the dictionary.
        let mut to_be_inserted = w.clone();
        to_be_inserted.push(entry.chars().next().expect("next"));
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
