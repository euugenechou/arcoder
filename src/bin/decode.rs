use arcoder::{ArDecoder, Result};
use byteorder::WriteBytesExt;
use std::io;

fn main() -> Result<()> {
    let mut coder = ArDecoder::new()?;

    while let Ok(ch) = coder.decode_symbol() {
        io::stdout().write_u8(ch)?;
    }

    Ok(())
}
