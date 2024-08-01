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

    /// The number of characters in each input file is written to the standard output.
    #[arg(short)]
    m: bool,

    /// file
    #[arg(value_parser)]
    file: Input,
}

fn filepath(file: &Input) -> &str {
    file.path().to_str().unwrap()
}

fn content(file: &mut Input) -> io::Result<String> {
    let mut content = String::new();
    file.get_file().unwrap().read_to_string(&mut content)?;
    Ok(content)
}

fn main() -> io::Result<()> {
    let mut args = Args::parse();

    if args.l {
        let content = content(&mut args.file)?;
        print!("{:8}", content.lines().count());
    }

    if args.w {
        let content = content(&mut args.file)?;
        print!("{:8}", content.split_whitespace().count());
    }

    if args.m {
        let content = content(&mut args.file)?;
        print!("{:8}", content.chars().count());
    } else if args.c {
        let len = args.file.len().unwrap();
        print!("{:8}", len);
    }

    println!(" {}", filepath(&args.file));
    Ok(())
}
