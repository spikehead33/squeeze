use bit_vec::BitVec;
use itertools::Itertools;
use std::{
    fmt,
    collections::{HashMap, BinaryHeap},
    cmp::{Reverse, Ordering},
    convert::TryFrom
};
use super::Compressor;


/// This is the only struct expose to the public
pub struct HuffmanCompressor;

/// The Variant stands for different phase of the 
/// Huffman Compressor
#[derive(Debug)]
pub enum HuffmanCompressorError {
    // Error occur when creating Symbol Frequency Table
    SymbolFrequencyTableCreation,
    // Error occur when creating Huffman Tree from symbol frequency table
    HuffmanTreeCreation,
    // Error occur when creating the Codebook from Huffman Tree
    CodeBookCreation,
    // Error when using the codebook
    CodeBookConsumption,
    //
    UncompressionCodeWordNotFound,
    // 
    UncompressionBufferNotEmpty,
}

impl fmt::Display for HuffmanCompressorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = "HuffmanCompressor error:";

        match *self {
            Self::SymbolFrequencyTableCreation => {
                write!(f, "{prefix} failed to build SFT from the input")
            },
            Self::HuffmanTreeCreation => {
                write!(f, "{prefix} failed to build HuffmanTree from SFT")
            },
            Self::CodeBookCreation => {
                write!(f, "{prefix} failed to build CodeBook from the HuffmanTree")
            },
            Self::CodeBookConsumption => {
                write!(f, "{prefix} failed to apply Huffman CodeBook to the input")
            },
            Self::UncompressionCodeWordNotFound => {
                write!(f, "{prefix} failed to apply Huffman Reverse CodeBook to the input")
            },
            Self::UncompressionBufferNotEmpty => {
                write!(f, "{prefix} failed to uncompress data due to buffer is not clear")
            }
        }
    }
}

/// RawSymbol refer to the unencoded data
type RawSymbol = u8;

/// encoded symbol refer to the symbols that were encoded before
type EncodedSymbol = u8;

/// The first item is the symbol
struct CodeLengthTable(HashMap<u8, u8>);

struct CodeBook(HashMap<RawSymbol, BitVec>);

struct RevCodeBook(HashMap<BitVec, RawSymbol>);

struct SymbolFrequencyTable(HashMap<RawSymbol, u32>);

/// This is the Huffman tree that used to generate
/// the variable-length code
#[derive(Debug)]
enum HuffmanTree {
    /// Frequency, Symbol
    Leaf(u32, RawSymbol),
    /// Frequency, LeftTree, RightTree
    Tree(u32, Box<HuffmanTree>, Box<HuffmanTree>),
}

fn code_length_table(input: &[RawSymbol]) -> Result<CodeLengthTable, HuffmanCompressorError> {
    SymbolFrequencyTable::try_from(input)
        .and_then(HuffmanTree::try_from)
        .and_then(CodeLengthTable::try_from)
}

/// Use the CodeBook to compress the input content
fn apply_codebook(input: &[RawSymbol], cb: &CodeBook) -> BitVec {
    let mut bitvec = BitVec::new();
    input.iter()
         .map(|symbol| cb.0.get(symbol).unwrap().clone())
         .for_each(|mut bv| bitvec.append(&mut bv));
    bitvec
}

/// Make the Reverse Codebook from the input
fn reverse_codebook(cb: &CodeBook) -> RevCodeBook { 
    let mut rev_cb = HashMap::new();
    for (&symbol, bv) in cb.0.iter() {
        rev_cb.insert(bv.clone(), symbol);
    }

    RevCodeBook(rev_cb)
}

/// cb_size: CodeBook Size number of bytes
/// npad: The padding bits behind the compressed/encoded data
/// codebook key(Symbol)-value(CodeLength) pairs.
fn header(table: &CodeLengthTable, npad: u8) -> Vec<u8> {
    let mut header = vec![];
    let mut cb_size = ((table.0.len() * 2) as u16)
                .to_be_bytes()
                .to_vec();
    let mut cb = table.0
                .iter()
                .flat_map(|(&a, &b)| [a, b])
                .collect();

    header.push(npad);
    header.append(&mut cb_size);
    header.append(&mut cb);
    header
}

impl Compressor for HuffmanCompressor {
    type Error = HuffmanCompressorError;

    fn compress(&self, input: &[RawSymbol]) -> Result<Vec<EncodedSymbol>, Self::Error> {
        let cl = code_length_table(input)?;
        let cb = CodeBook::from(&cl);
        let data = apply_codebook(input, &cb);

        Ok(header(&cl, (data.len() % 8) as u8)
            .into_iter()
            .chain(data.to_bytes().into_iter())
            .collect())
    }

    fn uncompress(&self, input: &[EncodedSymbol]) -> Result<Vec<RawSymbol>, Self::Error> {
        let mut npad: u8 = 0;
        let mut cb_size: u16 = 0;

        for (index, &symbol) in input.iter().enumerate() {
            match index {
                0 => npad = 8 - symbol,
                1 => cb_size = (symbol as u16) << 8,
                2 => cb_size |= symbol as u16,
                _ =>  break,
            };
        }

        // This part construct the code-length-table
        let mut code_length_table = HashMap::new();
        for chunk in &input
                .into_iter()
                .skip(1 + 2)
                .take(cb_size as usize)
                .chunks(2)
        {
            if let &[&symbol, &code_length] = chunk.collect::<Vec<_>>().as_slice() {
                code_length_table.insert(symbol, code_length);
            };
        }

        // This part generate the reverse codebook
        let cb = CodeBook::from(&CodeLengthTable(code_length_table));
        let rcb = reverse_codebook(&cb);

        let mut payload = BitVec::from_bytes(
            input.iter()
                 .skip(1 + 2 + cb_size as usize)
                 .copied()
                 .collect::<Vec<_>>()
                 .as_slice()
        );

        // need to remove the padding bits on the payload before
        // doing the compression
        payload.truncate(payload.len() - npad as usize);

        let mut content: Vec<RawSymbol> = vec![];
        let mut buffer = BitVec::new();
        for bit in payload.iter() {
            buffer.push(bit);

            if rcb.0.contains_key(&buffer) {
                content.push(*rcb.0.get(&buffer).ok_or(
                    Self::Error::UncompressionCodeWordNotFound)?);
                buffer.truncate(0);
            }
        }

        if !buffer.is_empty() {
            return Err(Self::Error::UncompressionBufferNotEmpty);
        }
        
        Ok(content)
    }
}

impl TryFrom<&[RawSymbol]> for SymbolFrequencyTable {
    type Error = HuffmanCompressorError;

    fn try_from(symbols: &[RawSymbol]) -> Result<Self, Self::Error> {
        let mut table = HashMap::new();
        
        for &symbol in symbols.iter() {
            *table.entry(symbol).or_insert(0) += 1;
        }

        if table.is_empty() {
            return Err(Self::Error::SymbolFrequencyTableCreation);
        }

        Ok(SymbolFrequencyTable(table))
    }
}

impl TryFrom<SymbolFrequencyTable> for HuffmanTree {
    type Error = HuffmanCompressorError;

    fn try_from(table: SymbolFrequencyTable) -> Result<Self, Self::Error> {
        let mut heap = BinaryHeap::new();

        for (&byte, &freq) in table.0.iter() {
            // since BinaryHeap is a max-heap by default
            // and Huffman Tree is build by a min-heap.
            // Therefore, Reversing the node is necessary
            heap.push(Reverse(HuffmanTree::Leaf(freq, byte)));
        }

        while heap.len() > 1 {
            let Reverse(smaller) = heap
                .pop()
                .ok_or(Self::Error::HuffmanTreeCreation)?;
            
            let Reverse(bigger) = heap
                .pop()
                .ok_or(Self::Error::HuffmanTreeCreation)?;

            heap.push(
                Reverse(
                    HuffmanTree::Tree(
                        smaller.frequency() + bigger.frequency(), 
                        Box::new(smaller), 
                        Box::new(bigger)
                    )
                )
            );
        }

        let Reverse(tree) = heap
            .pop()
            .ok_or(Self::Error::HuffmanTreeCreation)?;
        Ok(tree)
    }
}

impl TryFrom<HuffmanTree> for CodeLengthTable {
    type Error = HuffmanCompressorError;

    fn try_from(hfm_tree: HuffmanTree) -> Result<Self, Self::Error> {
        let mut table = HashMap::new();
        // stack store both subtree and code-length
        let mut stack: Vec<(Box<HuffmanTree>, u8)> = vec![(Box::new(hfm_tree), 0)];

        while let Some((tree, code_length)) = stack.pop() {
            match *tree {
                HuffmanTree::Leaf(_, symbol) => {
                    table.insert(symbol, code_length);
                },
                HuffmanTree::Tree(_, left, right) => {
                    stack.push((right, code_length + 1));
                    stack.push((left, code_length + 1));
                }
            }
        }
        Ok(CodeLengthTable(table))
    }
}

impl From<&CodeLengthTable> for CodeBook {
    fn from(table: &CodeLengthTable) -> Self {
        // increment the bitvec by one
        let inc_bitvec = |bv: &mut BitVec| {
            let n = bv.len() - 1;

            let p = bv.iter().rev().position(|b| !b).unwrap();
            let i = n - p;

            bv.set(i, true);
            bv.truncate(i+1);
            bv.grow(n - i, false);
        };

        let mut codebook = HashMap::new();

        let mut sorted_code_length = Vec::from_iter(table.0.iter());
        sorted_code_length.sort_by_key(|(&a, &b)| (b, a));

        let mut code = BitVec::new();

        for (&symbol, &cl) in sorted_code_length.iter() {
            let npad = if codebook.is_empty() {
                cl as usize
            } else {
                inc_bitvec(&mut code);
                cl as usize - code.len()
            };
            // if npad == 0 -> no growth
            code.grow(npad, false);
            codebook.insert(symbol, code.clone());
        }

        CodeBook(codebook)
    }
}

impl HuffmanTree {
    fn frequency(&self) -> u32 {
        match *self {
            HuffmanTree::Leaf(x, _) => x,
            HuffmanTree::Tree(x, _, _) => x,
        }
    }
}

impl PartialEq for HuffmanTree {
    fn eq(&self, other: &Self) -> bool {
        self.frequency() == other.frequency()
    }
}

impl Eq for HuffmanTree {}

impl PartialOrd for HuffmanTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.frequency().partial_cmp(&other.frequency())
    }
}

impl Ord for HuffmanTree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency().cmp(&other.frequency())
    }
}

#[cfg(test)]
mod tests {
    use super::HashMap;

    use super::SymbolFrequencyTable;
    #[test]
    fn symbol_frequency_table() {
        let bytes = &Vec::from(
            "aaaaaabccdddeefffffgggghhh".as_bytes()
        );
        let expect: HashMap<u8, u32> = HashMap::from([
            (b'a', 6), (b'b', 1), (b'c', 2), (b'd', 3),
            (b'e', 2), (b'f', 5), (b'g', 4), (b'h', 3),
        ]);

        let test = SymbolFrequencyTable::try_from(bytes.as_slice());
        assert_eq!(expect, test.unwrap().0);
    }

}
