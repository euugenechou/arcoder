const CHARS: usize = 256;
const SYMBOLS: usize = CHARS + 1;
const MAX_FREQ: usize = (1 << 14) - 1;
pub const STOP_SYMBOL: usize = CHARS + 1;

pub struct Model {
    pub sym_freq: [usize; SYMBOLS + 1],
    pub freq_range: [usize; SYMBOLS + 1],
    pub char_to_index: [usize; CHARS],
    pub index_to_char: [usize; SYMBOLS + 1],
}

impl Model {
    pub fn new() -> Self {
        let mut sym_freq = [0; SYMBOLS + 1];
        let mut freq_range = [0; SYMBOLS + 1];
        let mut char_to_index = [0; CHARS];
        let mut index_to_char = [0; SYMBOLS + 1];

        for i in 0..CHARS {
            char_to_index[i] = i + 1;
            index_to_char[i + 1] = i;
        }

        for i in 0..SYMBOLS {
            sym_freq[i] = 1;
            freq_range[i] = SYMBOLS - i;
        }

        sym_freq[0] = 0;

        Self {
            sym_freq,
            freq_range,
            char_to_index,
            index_to_char,
        }
    }

    pub fn update(&mut self, sym: usize) {
        if self.freq_range[0] == MAX_FREQ {
            let mut freq = 0;
            for i in (0..SYMBOLS + 1).rev() {
                self.sym_freq[i] = (self.sym_freq[i] + 1) / 2;
                self.freq_range[i] = freq;
                freq += self.sym_freq[i];
            }
        }

        let mut i = sym;
        while self.sym_freq[i] == self.sym_freq[i - 1] {
            i -= 1;
        }

        if i < sym {
            let ch_i = self.index_to_char[i];
            let ch_sym = self.index_to_char[sym];
            self.index_to_char[i] = ch_sym;
            self.index_to_char[sym] = ch_i;
            self.char_to_index[ch_i] = sym;
            self.char_to_index[ch_sym] = i;
        }

        self.sym_freq[i] += 1;
        while i > 0 {
            i -= 1;
            self.freq_range[i] += 1;
        }
    }
}
