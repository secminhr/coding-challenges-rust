use clap::Parser;
use clio::Input;

/// coding challenges wc
#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// The number of bytes in each input file is written to the standard output.
    #[arg(short)]
    c: bool,

    /// The number of lines in each input file is written to the standard output.
    #[arg(short)]
    l: bool,

    /// file
    #[arg(value_parser)]
    file: Input,
}

fn main() {
    let args = Args::parse();
    let len = args.file.len().unwrap();
    // println!(
    //     "{} {}",
    //     len,
    //     args.file.path().file_name().unwrap().to_str().unwrap()
    // );
}
