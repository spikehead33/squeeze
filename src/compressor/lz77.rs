use super::{Compressor, CompressorRuntimeError};
pub struct Lz77Compressor;

impl Compressor for Lz77Compressor {
    fn compress(&self, input: &[u8])
        -> Result<Vec<u8>, CompressorRuntimeError>
    {
        unimplemented!()
    }

    fn uncompress(&self, input: &[u8])
        -> Result<Vec<u8>, CompressorRuntimeError>
    {
        unimplemented!()
    }
}