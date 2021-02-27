use crate::{
    constants::{
        BASE64_KEY,
        CHAR_CODE,
        CLOSE_CODE,
        URI_KEY,
        WIDE_CHAR_CODE,
    },
    IntoWideIter,
};

/// The starting dictionary size.
/// This is 3 because there are 3 starting codes: `CHAR_CODE`, `WIDE_CHAR_CODE`, and `END_CODE`.
const START_DICT_SIZE: u32 = 3;

#[cfg(not(feature = "rustc-hash"))]
type HashMap<K, V> = std::collections::HashMap<K, V>;

#[cfg(not(feature = "rustc-hash"))]
type HashSet<T> = std::collections::HashSet<T>;

#[cfg(feature = "rustc-hash")]
type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;

#[cfg(feature = "rustc-hash")]
type HashSet<T> = rustc_hash::FxHashSet<T>;

#[derive(Debug)]
pub(crate) struct CompressContext<F> {
    dictionary: HashMap<Vec<u16>, u16>,
    dictionary_to_create: HashSet<u16>,

    w: Vec<u16>,

    enlarge_in: usize,

    /// Dictionary size. Cannot exceed u16::MAX.
    /// This is a [`u32`] as this implementation relies on being able to add 1 to this value before using it.
    /// So this can have an invalid value of u16::MAX + 1, which is fine so long as it is never used.
    dict_size: u32,

    /// The length of a code in bits.
    code_length: usize,

    /// Output Data
    output: Vec<u16>,

    /// Bit buffer
    val: u16,

    /// Bit position
    position: u8,

    /// Max # of bits per char
    bits_per_char: u8,

    /// Transform function for translating u16s before output.
    to_char: F,

    /// Whether w is actually w + c right now.
    w_is_wc: bool,
}

impl<F> CompressContext<F>
where
    F: Fn(u16) -> u16,
{
    /// Make a new [`CompressContext`] that is ready for use.
    #[inline(always)]
    pub fn new(bits_per_char: u8, to_char: F) -> Self {
        CompressContext {
            dictionary: Default::default(),
            dictionary_to_create: Default::default(),

            w: Vec::new(),

            enlarge_in: 2,
            dict_size: START_DICT_SIZE,
            code_length: 2,
            output: Vec::new(),
            val: 0,
            position: 0,
            bits_per_char,
            to_char,

            w_is_wc: false,
        }
    }

    #[inline(always)]
    fn produce_w(&mut self) {
        // wc is always either w or a subset of wc.
        // Make sure to determine what case it is and select correct bounds.
        // We add 1 if w is wc because wc is always 1 char greater than w.
        if self.w.len() == (1 + usize::from(self.w_is_wc))
            && self.dictionary_to_create.remove(&self.w[0])
        {
            let first_w_char = self.w[0];
            if first_w_char < 256 {
                self.write_bits(self.code_length, CHAR_CODE);
                self.write_bits(8, first_w_char);
            } else {
                self.write_bits(self.code_length, WIDE_CHAR_CODE);
                self.write_bits(16, first_w_char);
            }
            self.decrement_enlarge_in();
        } else {
            // See above comment about selecting appropriate bounds based on whether w is wc.
            let end_w_index = self.w.len() - usize::from(self.w_is_wc);
            self.write_bits(
                self.code_length,
                self.dictionary
                    .get(&self.w[..end_w_index])
                    .copied()
                    .expect("Missing W entry"),
            );
        }
        self.decrement_enlarge_in();
    }

    /// Write n bits from value
    #[inline(always)]
    fn write_bits(&mut self, n: usize, mut value: u16) {
        for _ in 0..n {
            self.val = (self.val << 1) | (value & 1);
            if self.position == self.bits_per_char - 1 {
                self.position = 0;
                let char_data = (self.to_char)(self.val);
                self.output.push(char_data);
                self.val = 0;
            } else {
                self.position += 1;
            }
            value >>= 1;
        }
    }

    #[inline(always)]
    fn decrement_enlarge_in(&mut self) {
        self.enlarge_in -= 1;
        if self.enlarge_in == 0 {
            self.enlarge_in = 1 << self.code_length;
            self.code_length += 1;
        }
    }

    #[inline(always)]
    pub fn reserve_dictionary_space(&mut self, size: usize) {
        self.dictionary.reserve(size);
    }

    #[inline(always)]
    pub fn reserve_output_space(&mut self, size: usize) {
        self.output.reserve(size);
    }

    /// Compress a `u16`. This represents a wide char.
    ///
    #[inline(always)]
    pub fn write_u16(&mut self, c: u16) {
        if !self.dictionary.contains_key(std::slice::from_ref(&c)) {
            self.dictionary.insert(vec![c], self.dict_size as u16);
            self.dict_size += 1;
            self.dictionary_to_create.insert(c);
        }

        // wc = w + c
        // This already has w + c from the last iteration, which became the w value for this iteration.
        // Therefore, just add the new c.
        // w_is_wc is set to indicate that w is now wc.
        self.w.push(c);
        self.w_is_wc = true;

        if self.dictionary.contains_key(&self.w) {
            // At this point, wc = w + c.
            // In order to make w into wc, just add c.
            // We just pass here and let the w_is_wc become unset, transforming wc into w.
        } else {
            self.produce_w();
            // Add wc to the dictionary.
            self.dictionary
                .insert(self.w.clone(), self.dict_size as u16);
            self.dict_size += 1;

            // w and wc are both cleared here and set to c, so nuke w and let wc turn into w.
            self.w.clear();
            self.w.push(c);

            // Pre-add w to the wc value for the next iteration.
            // The w value is just c, as it is set above.
            // We just pass here and let the w_is_wc become unset, transforming wc into w.
        }

        // w_is_wc is unset to indicate that wc is now w.
        self.w_is_wc = false;
    }

    /// Reset internal state. This preserves buffers.
    #[inline]
    pub fn reset(&mut self) {
        self.dictionary.clear();
        self.dictionary_to_create.clear();

        self.w.clear();
        
        self.enlarge_in = 2;
        self.dict_size = START_DICT_SIZE;
        self.code_length = 2;
        self.output.clear();
        self.val = 0;
        self.position = 0;
        
        self.w_is_wc = false;
    }

    /// Finish the stream and get the final result.
    ///
    #[inline]
    pub fn finish(&mut self) -> Vec<u16> {
        // Output the code for w.
        if !self.w.is_empty() {
            self.produce_w();
        }

        // Mark the end of the stream
        self.write_bits(self.code_length, CLOSE_CODE);

        // Flush the last char
        self.val = self.val << 1; // Why is this needed?
        self.val <<= (self.bits_per_char - 1) - self.position;
        let char_data = (self.to_char)(self.val);
        self.output.push(char_data);

        // Reset state and return
        let ret = std::mem::take(&mut self.output);
        self.reset();
        ret
    }
}

/// Compress a string into a [`Vec<u16>`].
///
/// The resulting [`Vec`] may contain invalid UTF16.
///
#[inline]
pub fn compress(input: impl IntoWideIter) -> Vec<u16> {
    compress_internal(input.into_wide_iter(), 16, std::convert::identity)
}

/// Compress a string as a valid [`String`].
///
/// This function converts the result back into a Rust [`String`] since it is guaranteed to be valid UTF16.
///
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
///
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
///
#[inline]
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
///
#[inline]
pub fn compress_to_uint8_array(data: impl IntoWideIter) -> Vec<u8> {
    let compressed = compress(data);

    let mut buf = Vec::with_capacity(compressed.len() * 2);

    for val in compressed.into_iter() {
        buf.push((val >> 8) as u8);
        buf.push((val & 0xFF) as u8);
    }

    buf
}

/// The internal function for compressing data.
///
/// All other compression functions are built on top of this.
/// It generally should not be used directly.
/// This function looks at the maximum value of `Iterator::size_hint` to allocate its memory.
///
pub fn compress_internal<I: Iterator<Item = u16>, F: Fn(u16) -> u16>(
    uncompressed: I,
    bits_per_char: u8,
    to_char: F,
) -> Vec<u16> {
    let mut ctx = CompressContext::new(bits_per_char, to_char);
    // Reserving the max theoretical size up front prevents allocations.
    // Use 200 as a fallback.
    let size_hint = uncompressed.size_hint().1.unwrap_or(200);

    // This is probably too large a size for the dictionary.
    // While wasteful, it does wonders for perf.
    // This might have to be a library-wide opt-in option.
    ctx.reserve_dictionary_space(size_hint);
    
    // w buffer cannot exceed `size_hint`. This is an overestimate.
    // Disabled for now since it looks like allocating a large buffer 
    // for every compression is not actually speeding it up much.
    ctx.w.reserve(size_hint);

    // This might actually be too small as some cases will create a larger output that the input.
    // Generally though, this is too large.
    //
    // See comments for reserving dictionary space.
    ctx.reserve_output_space(size_hint);

    for c in uncompressed {
        ctx.write_u16(c);
    }

    ctx.finish()
}
