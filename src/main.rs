use clap::Parser;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct Cli{
    #[arg(long)]
    file: PathBuf,
}


fn main() {
    let args = Cli::parse();
    let file_name = args.file.to_str().unwrap();
    let file = File::open(file_name).expect("Failed to open this file");
    let reader = BufReader::new(file);
    let words: Vec<_> = reader.lines()
                        .collect();
    for word in words.iter() {
        println!("{:?}", word);
    }
}

