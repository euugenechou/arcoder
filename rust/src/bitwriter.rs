use super::Result;
use byteorder::WriteBytesExt;
use std::io;

pub struct BitWriter {
    buffer: u8,
    space: u8,
}

impl BitWriter {
    pub fn new() -> Self {
        Self {
            buffer: 0,
            space: 8,
        }
    }

    pub fn write(&mut self, bit: u8) -> Result<()> {
        self.buffer >>= 1;
        self.buffer |= bit << 7;
        self.space -= 1;
        if self.space == 0 {
            io::stdout().write_u8(self.buffer)?;
            self.space = 8;
        }
        Ok(())
    }

    pub fn flush(&self) -> Result<()> {
        io::stdout().write_u8(self.buffer >> self.space)?;
        Ok(())
    }
}
