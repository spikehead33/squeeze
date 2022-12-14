use super::compressor;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// input file path
    #[clap(short, long)]
    #[clap(parse(from_os_str))]
    pub input: std::path::PathBuf,

    /// input file path
    #[clap(short, long)]
    #[clap(parse(from_os_str))]
    pub output: Option<std::path::PathBuf>,

    /// decompress mode
    #[clap(short = 'u', long)]
    pub decompress: bool,

    /// compression algorithms; separate by spaces if pipelining compressors
    #[clap(short, long)]
    pub compressors: Compressors
}

pub struct Compressors(pub Vec<Box<dyn compressor::Compressor>>);

impl std::str::FromStr for Compressors {
    type Err = clap::error::Error;

    fn from_str(inputs: &str) -> Result<Self, Self::Err> {
        let mut compressors: Vec<Box<dyn compressor::Compressor>> = vec![];
        
        for input in inputs.split(',') {
            match input {
                "huffman" | "hfm" => compressors.push(Box::new(compressor::huffman::HuffmanCompressor)),
                "lz77" | "z7" => compressors.push(Box::new(compressor::lz77::Lz77Compressor)),
                _ => {
                    return Err(
                        clap::error::Error::raw(
                            clap::error::ErrorKind::InvalidValue,
                            format!("compressor {} has not been supported yet!", &input)
                        )
                    )
                }
            }
        }

        Ok(Compressors(compressors))
    }
}
