//! handles chunked encoding and decoding

mod decoder;
mod encoder;

pub use decoder::ChunkedDecoder;
pub(crate) use encoder::ChunkedEncoder;
