mod cli;
mod compressor;

use clap::Parser;
use std::{fs, path::PathBuf};


fn read_file_vec(filepath: PathBuf) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = fs::read(filepath)?;
    Ok(data)
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = cli::Cli::parse();

    let oufile_path = if let Some(oufile) = args.output {
        oufile
    } else {
        let mut ou_base_name = args.input.to_owned();

        match args.is_uncompress {
            true => ou_base_name.set_extension(""),
            false => ou_base_name.set_extension(".sq")
        };

        ou_base_name
    };

    let symbols = match read_file_vec(args.input) {
        Ok(bytes) => bytes,
        Err(e) => return Err(e)
    };

    let cli::Compressors(compressors) = args.compressors;

    let result = compressors.iter().try_fold(symbols, |input_stream, compressor| {
        compressor.compress(input_stream.as_slice())
    });

    match result {
        Ok(data) => {
            std::fs::write(oufile_path, data).or_else(|e| {
                return Err(Box::new(e))
            });
            Ok(())
        },
        Err(e) => Err(Box::new(e))
    }
}
