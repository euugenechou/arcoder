use arcoder::{ArEncoder, Result};
use byteorder::ReadBytesExt;
use std::io;

fn main() -> Result<()> {
    let mut coder = ArEncoder::new();

    while let Ok(ch) = io::stdin().read_u8() {
        coder.encode_symbol(ch.into())?;
    }

    coder.flush()?;
    Ok(())
}
