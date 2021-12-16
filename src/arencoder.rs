use super::BitWriter;
use super::Model;
use super::Result;
use super::STOP_SYMBOL;
use super::{CODE_FIRST_QTR, CODE_HALF, CODE_MAX, CODE_THIRD_QTR};

pub struct ArEncoder {
    low: usize,
    high: usize,
    pending: usize,
    model: Model,
    writer: BitWriter,
}

impl ArEncoder {
    pub fn new() -> Self {
        Self {
            low: 0,
            high: CODE_MAX,
            pending: 0,
            model: Model::new(),
            writer: BitWriter::new(),
        }
    }

    pub fn encode_symbol(&mut self, ch: usize) -> Result<()> {
        let sym = if ch == STOP_SYMBOL {
            STOP_SYMBOL
        } else {
            self.model.char_to_index[ch]
        };

        let range = (self.high - self.low) + 1;
        self.high =
            self.low + (range * self.model.freq_range[sym - 1]) / self.model.freq_range[0] - 1;
        self.low += (range * self.model.freq_range[sym]) / self.model.freq_range[0];

        loop {
            if self.high < CODE_HALF {
                self.bit_plus_follow(0)?;
            } else if self.low >= CODE_HALF {
                self.bit_plus_follow(1)?;
                self.low -= CODE_HALF;
                self.high -= CODE_HALF;
            } else if self.low >= CODE_FIRST_QTR && self.high < CODE_THIRD_QTR {
                self.pending += 1;
                self.low -= CODE_FIRST_QTR;
                self.high -= CODE_FIRST_QTR;
            } else {
                break;
            }
            self.low *= 2;
            self.high = 2 * self.high + 1;
        }

        self.model.update(sym);
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.encode_symbol(STOP_SYMBOL)?;

        self.pending += 1;
        if self.low < CODE_FIRST_QTR {
            self.bit_plus_follow(0)?;
        } else {
            self.bit_plus_follow(1)?;
        }

        self.writer.flush()
    }

    fn bit_plus_follow(&mut self, bit: usize) -> Result<()> {
        self.writer.write(bit as u8)?;

        while self.pending > 0 {
            if bit == 0 {
                self.writer.write(1)?;
            } else {
                self.writer.write(0)?;
            }
            self.pending -= 1;
        }

        Ok(())
    }
}
