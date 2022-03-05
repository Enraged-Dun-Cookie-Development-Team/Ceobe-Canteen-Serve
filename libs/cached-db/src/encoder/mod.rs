mod decoder;
mod encoder;

#[derive(Debug)]
pub enum EncodeError<T> {
    Inner(T),
    Encode(bincode::Error),
}

impl<T> From<bincode::Error> for EncodeError<T> {
    fn from(err: bincode::Error) -> Self {
        Self::Encode(err)
    }
}

pub use decoder::{Decoder,DecodeReq};
pub use encoder::Encoder;
