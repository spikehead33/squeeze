use std::cell::RefCell;
use super::compressor::{Compressor, huffman::HuffmanCompressor}; 
use std::time::{Duration, Instant};


/// Model will perform compression and uncompression
/// to the input symbols.
/// It also stores some related attributes of the
/// Model.
pub struct Model {
    // compressor: Box<dyn Compressor>,
}

impl Model {
    
    // pub fn new(alg: &str) -> Self {
    //     Model {
    //         compressor: match alg {
    //             "huffman" => Box::new(HuffmanCompressor{}),
    //             "lz77" => Box::new(Lz77Compressor{}),
    //             _ => unimplemented!(),
    //         }
    //     }
    // }

    // pub fn perform_compression(&self, symbols: &Vec<u8>) 
    //     -> Result<Vec<u8>, CompressorRuntimeError>
    // {
    //     (*self.compressor).compress(symbols)
    // }

    // pub fn perform_uncompression(&self, encoded_symbols: &Vec<u8>)
    //     -> Result<Vec<u8>, CompressorRuntimeError>
    // {
    //     (*self.compressor).uncompress(encoded_symbols)
    // }
}
