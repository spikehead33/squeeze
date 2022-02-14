pub mod compressor;
pub mod model;
// use clap::{App, Arg, ArgMatches, crate_version};

fn main() {
}


// fn command_line_flags() -> ArgMatches<'static> {
//     App::new("Command Line Compression Utility")
//         .version(crate_version!()) 
//         .author("z78078") 
//         .about("Simple implementation of Text Compression") 
//         .after_help("Welcome!") 
//         .arg(
//             Arg::with_name("infilename")
//             .help("input file name")
//             .short("f")
//             .long("file")
//             .takes_value(true)
//             .required(true)
//         )
//         .arg(
//             Arg::with_name("oufilename")
//             .help("output file name")
//             .short("o")
//             .long("output")
//             .takes_value(true)
//             .required(false)
//         )
//         .arg(
//             Arg::with_name("algo") 
//             .help("The compression algorithm that's going to be used on the Text file")
//             .short("a")
//             .long("algo")
//             .takes_value(true)
//             .default_value("huffman")
//         )
//         .arg(
//             Arg::with_name("uncompress")
//             .help("Action: Compress") 
//             .short("u") 
//             .long("uncompress") 
//             .takes_value(false)
//             .required(false)
//         ).get_matches()
// }