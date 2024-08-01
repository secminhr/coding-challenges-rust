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

    /// The number of words in each input file is written to the standard output.
    #[arg(short)]
    w: bool,

    /// file
    #[arg(value_parser)]
    file: Input,
}

fn filepath(file: &Input) -> &str {
    file.path().to_str().unwrap()
}

fn main() -> io::Result<()> {
    let mut args = Args::parse();

    if args.l {
        let mut content = String::new();
        args.file.get_file().unwrap().read_to_string(&mut content)?;
        let lines = content.lines().count();
        print!("{:8}", lines);
    }

    if args.w {
        let mut content = String::new();
        args.file.get_file().unwrap().read_to_string(&mut content)?;
        let words = content.split_whitespace().count();
        print!("{:8}", words);
    }

    if args.c {
        let len = args.file.len().unwrap();
        print!("{:8}", len);
    }

    println!(" {}", filepath(&args.file));
    Ok(())
}
