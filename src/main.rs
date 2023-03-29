mod cli;
mod compressor;

use clap::Parser;
use std::{fs, path::PathBuf};

fn read_file_vec(filepath: PathBuf) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = fs::read(filepath)?;
    Ok(data)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Cli::parse();

    let outfile_path = match args.output {
        Some(p) => p,
        None => {
            let mut ou_base_name = args.input.to_owned();
            if !args.decompress {
                ou_base_name.set_extension("sq");
            }
            ou_base_name
        }
    };

    let symbols = read_file_vec(args.input)?;

    let cli::CompressorOptions(compressors) = args.compressors;

    let result = match args.decompress {
        false => compressors
            .iter()
            .try_fold(symbols, |input_stream, compressor| {
                compressor.compress(input_stream.as_slice())
            }),
        true => compressors
            .iter()
            .rev()
            .try_fold(symbols, |input_stream, compressor| {
                compressor.decompress(input_stream.as_slice())
            }),
    };

    match result {
        Ok(data) => {
            std::fs::write(outfile_path, data)?;
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}
