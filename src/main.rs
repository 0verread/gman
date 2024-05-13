use clap::Parser;
use rand::prelude::*;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


#[derive(Parser)]
struct Cli{
    #[arg(long)]
    file: PathBuf,
}

fn build_mc(words: &[String]) -> HashMap<String, Vec<String>> {
    let mut mc = HashMap::new();

    for window in words.windows(2){
        let (prefix, suffix) = (window[0], window[1]);
        mc.entry(prefix.to_string()).or_insert_with(Vec::new).push(suffix.to_string());
    }

    return mc;
}

fn generate_name(mc: &HashMap<String, Vec<String>>, max_len: usize) -> String {
    let mut rng = thread_rng();
    let mut state = mc.keys().choose(&mut rng)
                    .unwrap().to_string();
    let mut name = state.clone();

    while name.len() < max_len {
        let next_words = mc.get(&state).unwrap();
        let next_word = next_words.choose(&mut rng).unwrap();
        name.push_str(next_word);
        state = next_word.clone();
    }
    return name;
}


fn main() {
    let args = Cli::parse();
    let file_path = args.file.to_str().unwrap();
    let file = File::open(file_path).expect("Failed to open this file");
    let reader = BufReader::new(file);
    let mut words = Vec::new();
   
    for line in reader.lines() {
        let word = line;
        words.push(word);
    }

    let mc  = build_mc(&words);
    for _ in 0..10 {
        let name = generate_name(&mc, 10);
        println!("{}", name);
    }
}

