use std::{
    fmt::Debug,
    io::{self, Read},
};

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

fn filepath(file: &Input) -> &str {
    file.path().to_str().unwrap()
}

fn main() -> io::Result<()> {
    let mut args = Args::parse();

    let mut content = String::new();
    args.file.get_file().unwrap().read_to_string(&mut content)?;
    let lines = content.lines().count();
    if args.l {
        print!("{:8}", lines);
    }

    let len = args.file.len().unwrap();
    if args.c {
        print!("{:8}", len);
    }

    println!(" {}", filepath(&args.file));
    Ok(())
}
