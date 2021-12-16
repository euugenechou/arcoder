use super::BitReader;
use super::Model;
use super::Result;
use super::STOP_SYMBOL;
use super::{CODE_BITS, CODE_FIRST_QTR, CODE_HALF, CODE_MAX, CODE_THIRD_QTR};

pub struct ArDecoder {
    low: usize,
    high: usize,
    curr: usize,
    model: Model,
    reader: BitReader,
}

impl ArDecoder {
    pub fn new() -> Result<Self> {
        let mut reader = BitReader::new();

        let mut curr = 0;
        for _ in 1..=CODE_BITS {
            curr = 2 * curr + reader.read()?;
        }

        Ok(Self {
            low: 0,
            high: CODE_MAX,
            curr,
            model: Model::new(),
            reader,
        })
    }

    pub fn decode_symbol(&mut self) -> Result<u8> {
        let range = (self.high - self.low) + 1;
        let freq = ((self.curr - self.low + 1) * self.model.freq_range[0] - 1) / range;

        let mut sym = 1;
        while self.model.freq_range[sym] > freq {
            sym += 1;
        }

        self.high =
            self.low + (range * self.model.freq_range[sym - 1]) / self.model.freq_range[0] - 1;
        self.low += (range * self.model.freq_range[sym]) / self.model.freq_range[0];

        loop {
            if self.high < CODE_HALF {
                // Do nothing.
            } else if self.low >= CODE_HALF {
                self.curr -= CODE_HALF;
                self.low -= CODE_HALF;
                self.high -= CODE_HALF;
            } else if self.low >= CODE_FIRST_QTR && self.high < CODE_THIRD_QTR {
                self.curr -= CODE_FIRST_QTR;
                self.low -= CODE_FIRST_QTR;
                self.high -= CODE_FIRST_QTR;
            } else {
                break;
            }
            self.low *= 2;
            self.high = 2 * self.high + 1;
            self.curr = 2 * self.curr + self.reader.read()?;
        }

        if sym == STOP_SYMBOL {
            return Err("EOF".into());
        }

        let ch = self.model.index_to_char[sym];
        self.model.update(sym);
        Ok(ch as u8)
    }
}
