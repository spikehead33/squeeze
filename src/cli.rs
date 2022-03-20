use super::compressor::Compressor;


#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// input file path
    #[clap(short, long)]
    #[clap(parse(from_os_str))]
    input: std::path::PathBuf,

    /// input file path
    #[clap(short, long)]
    #[clap(parse(from_os_str))]
    output: std::path::PathBuf,

    /// compression algorithms; separate by spaces if pipelining compressors
    // #[clap(short, long)]
    // compressors: Vec<Box<dyn Compressor>>,

    /// uncompress mode
    #[clap(short = 'u', long)]
    is_uncompress: bool,
}
