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
    #[arg(value_parser, default_value = "-")]
    file: Input,
}

fn filepath(file: &Input) -> String {
    if file.is_std() {
        String::new()
    } else {
        file.path().to_str().unwrap().to_owned()
    }
}

fn content(file: &mut Input) -> io::Result<String> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn data(mut file: Input) -> io::Result<(String, String)> {
    let content = content(&mut file)?;
    Ok((content, filepath(&file)))
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let (content, filepath) = data(args.file)?;

    if !args.l && !args.w && !args.m && !args.c {
        print_l(&content);
        print_w(&content);
        print_c(&content);
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
            print_c(&content);
        }
    }

    println!(" {}", filepath);
    Ok(())
}

fn print_c(content: &String) {
    print!("{:8}", content.len());
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
