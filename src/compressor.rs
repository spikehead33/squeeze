pub mod huffman;
pub mod lz77;


pub trait Compressor {
    fn compress(&self, input: &[u8])
        -> Result<Vec<u8>, CompressorRuntimeError>;

    fn uncompress(&self, input: &[u8])
        -> Result<Vec<u8>, CompressorRuntimeError>;
}


#[derive(Debug)]
pub struct CompressorRuntimeError(String);
