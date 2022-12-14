use std::env;
use std::fs::read;

use squeeze::compressor::Compressor;

fn text_data() -> Vec<u8> {
    let file = env::current_dir().unwrap().join("tests/test.txt");
    read(file).unwrap()
}

#[test]
fn text_compression_test() {
    let data = text_data();
    let huffman = squeeze::compressor::huffman::HuffmanCompressor;

    assert_eq!(
        data,
        huffman.decompress(huffman.compress(&data).as_ref().unwrap())
               .unwrap()
    );
}
