#[derive(Debug)]
pub struct DecompressContext<'a> {
    val: u32,
    compressed_data: &'a [u32],
    position: usize,
    index: usize,
    reset_val: usize,
}

impl<'a> DecompressContext<'a> {
    pub fn new(compressed_data: &'a [u32], reset_val: usize) -> Self {
        //Js version seems to rely on being able to load a nonexistent byte, so just pad it here...? Maybe a bug in my impl?
        DecompressContext {
            val: compressed_data[0],
            compressed_data,
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
            self.val = self.compressed_data[self.index];
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
