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
    let content = content(&mut args.file)?;

    if !args.l && !args.w && !args.m && !args.c {
        print_l(&content);
        print_w(&content);
        print_c(&args);
    } else {
        if args.l {
            print_l(&content);
        }

        if args.w {
            print_w(&content);
        }

        if args.m {
            print_m(&content);
        } else if args.c {
            print_c(&args);
        }
    }

    println!(" {}", filepath(&args.file));
    Ok(())
}

fn print_c(args: &Args) {
    let len = args.file.len().unwrap();
    print!("{:8}", len);
}

fn print_m(content: &String) {
    print!("{:8}", content.chars().count());
}

fn print_w(content: &String) {
    print!("{:8}", content.split_whitespace().count());
}

fn print_l(content: &String) {
    print!("{:8}", content.lines().count());
}
