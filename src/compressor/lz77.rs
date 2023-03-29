use super::{Compressor, CompressorRuntimeError};

pub struct Lz77Compressor;

#[derive(Debug)]
struct Output(usize, usize, u8);

impl Output {
    fn offset(&self) -> usize {
        self.0
    }

    fn length(&self) -> usize {
        self.1
    }

    fn nextchar(&self) -> u8 {
        self.2
    }
}

struct Lz77State {
    search_buffer_max_size: usize,
    lookah_buffer_max_size: usize,
    coding_position: usize,
    lookah_buffer: (usize, usize),
    search_buffer: (usize, usize),
    output_buffer: Vec<Output>,
}

impl Lz77State {
    fn new(search_buffer_max_size: usize, lookah_buffer_max_size: usize) -> Self {
        Self {
            search_buffer_max_size,
            lookah_buffer_max_size,
            coding_position: 0,
            output_buffer: Vec::new(),
            search_buffer: (0, 0),
            lookah_buffer: (0, 0),
        }
    }
}

impl Compressor for Lz77Compressor {
    fn compress(&self, input: &[u8]) -> Result<Vec<u8>, CompressorRuntimeError> {
        let mut state = Lz77State::new(8096, 256);

        while state.coding_position < input.len() {
            // set the boundry for the lookahead buffer
            println!("Here");
            state.lookah_buffer = (
                state.coding_position,
                std::cmp::min(
                    input.len(),
                    state.coding_position + state.lookah_buffer_max_size,
                ),
            );

            // set the boundry for the search buffer
            state.search_buffer = (
                if state.coding_position < state.search_buffer_max_size + 1 {
                    0
                } else {
                    state.coding_position - state.search_buffer_max_size
                },
                if state.coding_position == 0 {
                    0
                } else {
                    state.coding_position
                },
            );
            println!("coding posit  {:?}", state.coding_position);
            println!("lookah_buffer {:?}", state.lookah_buffer);
            println!("search_buffer {:?}", state.search_buffer);
            let search_buffer_str =
                std::str::from_utf8(&input[state.search_buffer.0..state.search_buffer.1])
                    .map_err(|e| CompressorRuntimeError(format!("{}", e)))?;
            println!("search_buffer: {:?}", search_buffer_str);
            let lookah_buffer_str =
                std::str::from_utf8(&input[state.lookah_buffer.0..state.lookah_buffer.1])
                    .map_err(|e| CompressorRuntimeError(format!("{}", e)))?;
            println!("lookah_buffer: {:?}", lookah_buffer_str);
            let longest_match = longest_match(lookah_buffer_str, search_buffer_str);
            println!("after longest_match");
            let output = match longest_match {
                (0, 0) => Output(0, 0, input[state.coding_position + 1]),
                (offset, length) => {
                    Output(offset, length, input[state.coding_position + length + 1])
                }
            };
            println!("after matching output");
            // increment the coding position by L + 1
            state.coding_position += output.length() + 1;
            println!("after incrementing the coding position");
            state.output_buffer.push(output);
            println!("after pushing to the output buffer");
            println!();
        }
        println!("{:?}", state.output_buffer);

        Ok(vec![])
    }

    fn decompress(&self, input: &[u8]) -> Result<Vec<u8>, CompressorRuntimeError> {
        unimplemented!()
    }
}

/// return the tuple of offset and the length of the matches
fn longest_match(pattern: &str, search_space: &str) -> (usize, usize) {
    println!("0");
    for i in (0usize..pattern.len()).rev() {
        println!("{:?}", pattern);
        println!("i: {}", i);
        let p = &pattern[..i + 1];
        println!("p: {:?}", p);
        let offset = search_space
            .rfind(p)
            .and_then(|i| Some(search_space.len() - i));
        println!("2");
        match offset {
            None => continue,
            Some(o) => return (o, p.len()),
        }
    }
    println!("999999");
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_match() {
        let pattern = "Peter";
        let search_space = "Hi, Peter, The weather is really good Peter";
        let expected = (5usize, pattern.len() as usize);

        assert_eq!(longest_match(pattern, search_space), expected);
    }
}
