mod context;
use crate::constants::URI_KEY;
use context::CompressContext;

pub fn compress_str(input: &str) -> Vec<u32> {
    compress(input, 16, |n| n)
}

pub fn compress_uri(data: &str) -> Vec<u32> {
    compress(&data, 6, |n| {
        URI_KEY.chars().nth(n as usize).unwrap() as u32
    })
}

pub fn compress<F: Fn(u32) -> u32>(
    uncompressed: &str,
    bits_per_char: usize,
    transform_fn: F,
) -> Vec<u32> {
    let mut ctx = CompressContext::new(bits_per_char, transform_fn);
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
