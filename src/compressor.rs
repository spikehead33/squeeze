pub mod huffman;
pub mod lz77;


pub trait Compressor {
    fn compress(&self, input: &[u8])
        -> Result<Vec<u8>, CompressorRuntimeError>;

    fn decompress(&self, input: &[u8])
        -> Result<Vec<u8>, CompressorRuntimeError>;
}

#[derive(Debug)]
pub struct CompressorRuntimeError(String);

impl std::fmt::Display for CompressorRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}", self.0)
    }
}

impl std::error::Error for CompressorRuntimeError {
    fn description(&self) -> &str {
        &self.0
    }
}
