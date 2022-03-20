mod cli;
mod compressor;

use clap::Parser;


fn main() {
    let args = cli::Args::parse();
}
