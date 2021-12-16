use super::Result;
use byteorder::ReadBytesExt;
use std::io;

pub struct BitReader {
    buffer: u8,
    buffered: isize,
    garbage: u64,
}

impl BitReader {
    pub fn new() -> Self {
        Self {
            buffer: 0,
            buffered: 0,
            garbage: 0,
        }
    }

    pub fn read(&mut self) -> Result<usize> {
        if self.buffered == 0 {
            if let Ok(byte) = io::stdin().read_u8() {
                self.buffer = byte;
            } else {
                // Allow garbage bits, but track them.
                // The decoder stops when it sees the stop symbol.
                self.garbage += 1;
            }
            self.buffered += 8;
        }

        let bit = (self.buffer & 1) as usize;
        self.buffer >>= 1;
        self.buffered -= 1;
        Ok(bit)
    }
}
