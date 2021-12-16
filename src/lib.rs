mod ardecoder;
mod arencoder;
mod bitreader;
mod bitwriter;
mod code;
mod model;

pub use ardecoder::ArDecoder;
pub use arencoder::ArEncoder;
use bitreader::BitReader;
use bitwriter::BitWriter;
use code::*;
use model::*;
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
