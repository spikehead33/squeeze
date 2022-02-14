pub mod huffman;
pub mod lz77;


pub trait Compressor {
    type Error;
    fn compress(&self, input: &[u8]) -> Result<Vec<u8>, Self::Error>;
    fn uncompress(&self, input: &[u8]) -> Result<Vec<u8>, Self::Error>;
}
